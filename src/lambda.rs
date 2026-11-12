//! The Λ Engine — document margin interpreter.
//!
//! Executes readable Λ syntax from document margins.
//! Supports variables, conditionals, loops, file operations,
//! self-modification, and random selection.
//!
//! Syntax:
//!   say "text"                     — print text
//!   say "{variable}"               — print with substitution
//!   write path "text"              — append text to a file
//!   set name "value"               — set a variable
//!   set name ← count path/*        — count files in a directory
//!   set name ← read-count          — how many times this doc was read
//!   set name ← random-line path    — pick a random line from a file
//!   set name ← random-choice path/ — pick a random file from a directory
//!   set name ← ask "prompt"        — ask the reader for input
//!   if condition:                  — conditional (checks var is non-empty and not "0")
//!     ...                          — indented body
//!   inscribe path                  — create a file with following indented lines
//!   withdraw path                  — delete a file
//!   erode self N                   — remove N lines from own Μ layer
//!   mutate self "old" "new"        — replace text in own Μ layer
//!   wait Ns                        — sleep for N seconds
//!
//! A margin is written by whoever wrote the document and runs for whoever
//! reads it, so the engine does not trust it:
//!   - every path a margin names is confined to the library root (`confine`);
//!   - one reading gets one allowance of steps and of waiting (`Budget`),
//!     shared by every loop and nested block in the margin.
//! A refused instruction is skipped and reported; a margin that outruns its
//! allowance is stopped and says so. Neither can panic the shell.

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write as IoWrite};
use std::path::{Component, Path, PathBuf};
use std::thread;
use std::time::Duration;

use crate::narrator;

/// Instructions one reading may execute, counted across every loop and block.
pub const STEP_LIMIT: u32 = 10_000;

/// Longest single `wait`. Longer requests are shortened to this, not refused.
const MAX_SINGLE_WAIT_SECS: f64 = 60.0;

/// Total time one reading may spend in `wait`.
const MAX_TOTAL_WAIT: Duration = Duration::from_secs(600);

/// How many refusals are shown to the reader before the rest are only counted.
const REFUSALS_SHOWN: usize = 5;

/// Context passed to the Λ engine for self-referential operations
pub struct LambdaContext {
    pub root: PathBuf,
    pub doc_path: PathBuf,
    pub read_count_path: PathBuf,
}

impl LambdaContext {
    pub fn new(root: &Path, doc_path: &Path) -> Self {
        let rc_path = root.join(".meta").join(
            doc_path.strip_prefix(root).unwrap_or(doc_path)
                .to_string_lossy().replace('/', "_") + ".reads"
        );
        LambdaContext {
            root: root.to_path_buf(),
            doc_path: doc_path.to_path_buf(),
            read_count_path: rc_path,
        }
    }

    pub fn increment_read_count(&self) -> u64 {
        let _ = fs::create_dir_all(self.root.join(".meta"));
        let count = self.get_read_count() + 1;
        let _ = fs::write(&self.read_count_path, count.to_string());
        count
    }

    pub fn get_read_count(&self) -> u64 {
        fs::read_to_string(&self.read_count_path)
            .ok()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0)
    }
}

// ── Confinement ──────────────────────────────────────────────────────

/// Turn a path named by a margin into a real path inside `root`, or refuse.
///
/// Refused: an empty path, an absolute path, any `..` component, and any
/// path whose nearest existing ancestor resolves (through symlinks) to
/// somewhere outside `root`. A dangling symlink is refused too, because
/// writing through it would create its target.
///
/// The symlink check reads the filesystem and the caller acts afterwards,
/// so it is not atomic: it stops a margin from following a link that is
/// already there, not a second process racing to plant one.
pub fn confine(root: &Path, rel: &str) -> Option<PathBuf> {
    let rel = rel.trim();
    if rel.is_empty() || rel.contains('\0') {
        return None;
    }
    // Only plain names and `.` may appear. This rejects `/x` (RootDir),
    // `C:\x` (Prefix) and `a/../../x` (ParentDir) without resolving anything.
    let named = Path::new(rel);
    if !named
        .components()
        .all(|c| matches!(c, Component::Normal(_) | Component::CurDir))
    {
        return None;
    }

    let full = root.join(named);

    // Walk up to the nearest thing that exists (the root itself at worst)
    // and ask where it really is.
    let mut probe = full.as_path();
    loop {
        if fs::symlink_metadata(probe).is_ok() {
            if !stays_inside(root, probe) {
                return None;
            }
            break;
        }
        probe = probe.parent()?;
    }
    Some(full)
}

/// Whether `existing`, a path under `root` that is known to exist, really is
/// inside `root` once symlinks are followed.
#[cfg(not(target_family = "wasm"))]
fn stays_inside(root: &Path, existing: &Path) -> bool {
    match (fs::canonicalize(root), fs::canonicalize(existing)) {
        (Ok(real_root), Ok(real)) => real.starts_with(real_root),
        _ => false,
    }
}

/// The browser build. WASI has no `canonicalize`, so nothing can be resolved.
/// The name is already plain (no `..`, no root), which leaves a symlink on
/// the way as the only route out; any link between `existing` and `root`
/// refuses the path. Stricter than the native rule, which allows a link
/// that stays inside the library.
#[cfg(target_family = "wasm")]
fn stays_inside(root: &Path, existing: &Path) -> bool {
    let mut at = existing;
    loop {
        match fs::symlink_metadata(at) {
            Ok(meta) if meta.file_type().is_symlink() => return false,
            Ok(_) => {}
            Err(_) => return false,
        }
        if at == root {
            return true;
        }
        match at.parent() {
            Some(parent) => at = parent,
            None => return false,
        }
    }
}

// ── Budget ───────────────────────────────────────────────────────────

/// What one reading of one document is allowed to spend.
#[derive(Debug, Clone)]
pub struct Budget {
    steps_left: u32,
    wait_left: Duration,
}

impl Budget {
    /// The allowance the shell gives every reading.
    pub fn per_read() -> Self {
        Budget { steps_left: STEP_LIMIT, wait_left: MAX_TOTAL_WAIT }
    }

    #[cfg(test)]
    fn of(steps: u32, wait: Duration) -> Self {
        Budget { steps_left: steps, wait_left: wait }
    }
}

/// What happened when a margin ran. The reader has already been told;
/// this is the same account for the caller.
#[derive(Debug, Default)]
#[allow(dead_code)] // the shell does not act on it yet; the tests do
pub struct Outcome {
    /// One entry per instruction the engine refused to carry out.
    pub refusals: Vec<String>,
    /// True if the margin was stopped for outrunning its allowance.
    pub exhausted: bool,
    /// Instructions executed.
    pub steps: u32,
    /// The variables as the margin left them.
    pub vars: HashMap<String, String>,
}

/// Whether the margin goes on after an instruction or block.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Flow {
    Continue,
    Stop,
}

// ── Execution ────────────────────────────────────────────────────────

/// Execute a Λ script with full context and the standard allowance.
pub fn execute(
    script: &str,
    ctx: &LambdaContext,
    vars: &HashMap<String, String>,
) -> Outcome {
    execute_with_budget(script, ctx, vars, Budget::per_read())
}

/// Execute a Λ script against a given allowance.
pub fn execute_with_budget(
    script: &str,
    ctx: &LambdaContext,
    vars: &HashMap<String, String>,
    budget: Budget,
) -> Outcome {
    let mut reading = Reading {
        ctx,
        vars: vars.clone(),
        budget,
        outcome: Outcome::default(),
    };
    reading.run(script);

    let unshown = reading.outcome.refusals.len().saturating_sub(REFUSALS_SHOWN);
    if unshown > 0 {
        narrator::say(&format!("(The margin was refused {unshown} more times.)"));
    }
    reading.outcome.vars = reading.vars;
    reading.outcome
}

/// One reading of one margin. Every block, however deeply nested, runs
/// against these same variables and this same budget — that is what lets a
/// loop body change its own condition, and what keeps nested loops from
/// multiplying their allowance.
struct Reading<'a> {
    ctx: &'a LambdaContext,
    vars: HashMap<String, String>,
    budget: Budget,
    outcome: Outcome,
}

impl Reading<'_> {
    /// Pay for one instruction. False means the allowance is gone and the
    /// margin must stop; the reader is told once.
    fn spend(&mut self) -> bool {
        if self.budget.steps_left == 0 {
            self.exhaust(&format!(
                "(The margin ran on past {} steps and was stopped.)",
                self.outcome.steps
            ));
            return false;
        }
        self.budget.steps_left -= 1;
        self.outcome.steps += 1;
        true
    }

    fn exhaust(&mut self, message: &str) {
        if !self.outcome.exhausted {
            self.outcome.exhausted = true;
            narrator::say(message);
        }
    }

    /// Skip an instruction and say why.
    fn refuse(&mut self, what: String) {
        if self.outcome.refusals.len() < REFUSALS_SHOWN {
            narrator::say(&format!("(The margin {what}. Refused.)"));
        }
        self.outcome.refusals.push(what);
    }

    /// Resolve a margin's path inside the library, or record the refusal.
    fn path(&mut self, rel: &str) -> Option<PathBuf> {
        let found = confine(&self.ctx.root, rel);
        if found.is_none() {
            self.refuse(format!("reached for \"{rel}\", which is outside the library"));
        }
        found
    }

    fn run(&mut self, script: &str) -> Flow {
        let lines: Vec<&str> = script.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            i += 1;

            if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
                continue;
            }
            if !self.spend() {
                return Flow::Stop;
            }

            let line = substitute(line, &self.vars);

            if line.starts_with("say ") {
                let text = line.strip_prefix("say ").unwrap().trim_matches('"');
                narrator::say(text);

            } else if line.starts_with("write ") {
                let rest = line.strip_prefix("write ").unwrap();
                if let Some((path, text)) = rest.split_once(' ')
                    && let Some(full_path) = self.path(path)
                {
                    let text = text.trim_matches('"');
                    if let Some(parent) = full_path.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    let _ = fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(full_path)
                        .and_then(|mut f| writeln!(f, "{text}"));
                }

            } else if line.starts_with("set ") {
                let rest = line.strip_prefix("set ").unwrap();
                if let Some((name, expr)) = rest.split_once(" ← ") {
                    let name = name.trim().to_string();
                    let value = self.eval_expr(expr.trim());
                    self.vars.insert(name, value);
                } else if let Some((name, val)) = rest.split_once(' ') {
                    let name = name.trim().to_string();
                    let val = val.trim().trim_matches('"').to_string();
                    self.vars.insert(name, val);
                }

            } else if line.starts_with("if ") && line.ends_with(':') {
                let condition = line.strip_prefix("if ").unwrap()
                    .strip_suffix(':').unwrap().trim();
                let body = take_block(&lines, &mut i);

                if eval_condition(condition, &self.vars) && self.run(&body) == Flow::Stop {
                    return Flow::Stop;
                }

            } else if line.starts_with("inscribe ") {
                let path = line.strip_prefix("inscribe ").unwrap().trim();
                // The indented lines are the new document's text. They are
                // taken off the script even when the path is refused, so they
                // are never mistaken for instructions.
                let mut content = substitute(&take_block(&lines, &mut i), &self.vars);
                if !content.is_empty() {
                    content.push('\n');
                }
                if let Some(full_path) = self.path(path) {
                    if let Some(parent) = full_path.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    let _ = fs::write(full_path, content);
                }

            } else if line.starts_with("withdraw ") {
                let path = line.strip_prefix("withdraw ").unwrap().trim();
                if let Some(full_path) = self.path(path) {
                    let _ = fs::remove_file(full_path);
                }

            } else if line.starts_with("erode self ") {
                let n: usize = line.strip_prefix("erode self ").unwrap()
                    .trim().parse().unwrap_or(1);
                erode_document(&self.ctx.doc_path, n);

            } else if line.starts_with("mutate self ") {
                let rest = line.strip_prefix("mutate self ").unwrap();
                // mutate self "old" "new"
                if let Some((old, new)) = parse_two_quoted(rest) {
                    mutate_document(&self.ctx.doc_path, &old, &new);
                }

            } else if line.starts_with("loop while ") && line.ends_with(':') {
                // loop while condition:
                let condition = line.strip_prefix("loop while ").unwrap()
                    .strip_suffix(':').unwrap().trim().to_string();
                let body = take_block(&lines, &mut i);

                // The condition is read from the live variables, which the
                // body shares, so the body can end the loop. Each further
                // pass costs a step of its own: an empty body still ends.
                while eval_condition(&condition, &self.vars) {
                    if self.run(&body) == Flow::Stop || !self.spend() {
                        return Flow::Stop;
                    }
                }

            } else if line == "loop:" {
                // Runs until the reading's allowance is spent.
                let body = take_block(&lines, &mut i);
                loop {
                    if self.run(&body) == Flow::Stop || !self.spend() {
                        return Flow::Stop;
                    }
                }

            } else if line.starts_with("wait ") {
                let rest = line.strip_prefix("wait ").unwrap().trim();
                match wait_duration(rest) {
                    None => self.refuse(format!("asked to wait \"{rest}\"")),
                    Some(asked) => {
                        let granted = asked.min(self.budget.wait_left);
                        self.budget.wait_left -= granted;
                        thread::sleep(granted);
                        if granted < asked {
                            self.exhaust("(The margin has waited as long as it may and was stopped.)");
                            return Flow::Stop;
                        }
                    }
                }
            }
        }

        Flow::Continue
    }

    // ── Expression evaluation ────────────────────────────────────────

    fn eval_expr(&mut self, expr: &str) -> String {
        if expr.starts_with("count ") {
            let path = expr.strip_prefix("count ").unwrap().trim();
            let path = path.trim_end_matches("/*");
            let Some(full) = self.path(path) else { return "0".to_string() };
            let count = fs::read_dir(&full)
                .map(|d| d.flatten().filter(|e| {
                    !e.file_name().to_string_lossy().starts_with('.')
                }).count())
                .unwrap_or(0);
            count.to_string()

        } else if expr == "read-count" {
            self.ctx.get_read_count().to_string()

        } else if expr.starts_with("random-line ") {
            let path = expr.strip_prefix("random-line ").unwrap().trim();
            let Some(full) = self.path(path) else { return String::new() };
            random_line_from(&full).unwrap_or_default()

        } else if expr.starts_with("random-choice ") {
            let path = expr.strip_prefix("random-choice ").unwrap()
                .trim().trim_end_matches('/');
            let Some(full) = self.path(path) else { return String::new() };
            random_file_in(&full).unwrap_or_default()

        } else if expr.starts_with("ask ") {
            let prompt = expr.strip_prefix("ask ").unwrap().trim_matches('"');
            narrator::say(prompt);
            print!("    > ");
            let _ = io::stdout().flush();
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);
            input.trim().to_string()

        } else {
            // Return as literal
            expr.trim_matches('"').to_string()
        }
    }
}

/// Take the indented block that starts at `lines[*i]`, advance `*i` past it,
/// and return it with one level of indentation removed.
///
/// One level is the indentation of the block's first line. Deeper lines keep
/// the rest of theirs, so a block inside a block is still a block when the
/// outer one runs. Blank lines belong to the block when more of it follows.
fn take_block(lines: &[&str], i: &mut usize) -> String {
    let indent = |l: &str| l.len() - l.trim_start_matches([' ', '\t']).len();
    let indented = |l: &str| l.starts_with("  ") || l.starts_with('\t');

    let start = *i;
    let mut end = start;
    let mut j = start;
    while j < lines.len() {
        let l = lines[j];
        if l.trim().is_empty() {
            j += 1;
        } else if indented(l) {
            j += 1;
            end = j;
        } else {
            break;
        }
    }
    *i = end;

    let block = &lines[start..end];
    let level = block.iter()
        .find(|l| !l.trim().is_empty())
        .map(|&l| indent(l))
        .unwrap_or(0);
    block.iter()
        .map(|&l| &l[indent(l).min(level)..])
        .collect::<Vec<_>>()
        .join("\n")
}

/// How long `wait <arg>` should sleep, or None if the request is refused.
///
/// Policy: a negative, NaN or infinite number is refused (the instruction is
/// skipped). A finite number larger than MAX_SINGLE_WAIT_SECS is shortened to
/// that. Text that is not a number at all waits one second, as it always has.
/// The f64 is range-checked before it becomes a Duration, because
/// `Duration::from_secs_f64` panics on exactly the values refused here.
fn wait_duration(arg: &str) -> Option<Duration> {
    let secs: f64 = arg.trim().trim_end_matches('s').parse().unwrap_or(1.0);
    if !secs.is_finite() || secs < 0.0 {
        return None;
    }
    if secs == 0.0 {
        return Some(Duration::ZERO);
    }
    Some(Duration::from_secs_f64(secs.min(MAX_SINGLE_WAIT_SECS)))
}

fn eval_condition(cond: &str, vars: &HashMap<String, String>) -> bool {
    let cond = cond.trim();

    // "var > N" numeric comparison
    if let Some((name, val)) = cond.split_once(" > ") {
        let lhs: i64 = vars.get(name.trim()).and_then(|v| v.parse().ok()).unwrap_or(0);
        let rhs: i64 = val.trim().parse().unwrap_or(0);
        return lhs > rhs;
    }
    if let Some((name, val)) = cond.split_once(" < ") {
        let lhs: i64 = vars.get(name.trim()).and_then(|v| v.parse().ok()).unwrap_or(0);
        let rhs: i64 = val.trim().parse().unwrap_or(0);
        return lhs < rhs;
    }
    if let Some((name, val)) = cond.split_once(" == ") {
        let lhs = vars.get(name.trim()).map(|s| s.as_str()).unwrap_or("");
        return lhs == val.trim().trim_matches('"');
    }

    // Simple truthiness: non-empty and not "0"
    let val = vars.get(cond).map(|s| s.as_str()).unwrap_or(cond);
    !val.is_empty() && val != "0" && val != "false"
}

// ── Self-modification ────────────────────────────────────────────────

fn erode_document(path: &Path, n: usize) {
    let Ok(content) = fs::read_to_string(path) else { return };

    // Split into Μ and Λ sections
    let sep = "\n--- Λ ---\n";
    let (mu, lambda) = if let Some(idx) = content.find(sep) {
        (&content[..idx], Some(&content[idx..]))
    } else {
        (content.as_str(), None)
    };

    // Remove n lines from the end of the Μ section
    let mu_lines: Vec<&str> = mu.lines().collect();
    let keep = mu_lines.len().saturating_sub(n);
    let new_mu: String = mu_lines[..keep].join("\n");

    let mut new_content = new_mu;
    if let Some(lam) = lambda {
        new_content.push_str(lam);
    }

    let _ = fs::write(path, new_content);
}

fn mutate_document(path: &Path, old: &str, new: &str) {
    let Ok(content) = fs::read_to_string(path) else { return };

    // Only mutate the Μ section
    let sep = "\n--- Λ ---\n";
    let (mu, lambda) = if let Some(idx) = content.find(sep) {
        (&content[..idx], Some(&content[idx..]))
    } else {
        (content.as_str(), None)
    };

    let new_mu = mu.replacen(old, new, 1);
    let mut new_content = new_mu;
    if let Some(lam) = lambda {
        new_content.push_str(lam);
    }

    let _ = fs::write(path, new_content);
}

// ── Utilities ────────────────────────────────────────────────────────

fn substitute(line: &str, vars: &HashMap<String, String>) -> String {
    let mut result = line.to_string();
    for (key, value) in vars {
        result = result.replace(&format!("{{{key}}}"), value);
    }
    result
}

fn random_line_from(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let lines: Vec<&str> = content.lines()
        .filter(|l| !l.trim().is_empty())
        .collect();
    if lines.is_empty() { return None; }
    let idx = random_usize() % lines.len();
    Some(lines[idx].to_string())
}

fn random_file_in(dir: &Path) -> Option<String> {
    let entries: Vec<String> = fs::read_dir(dir).ok()?
        .flatten()
        .filter(|e| !e.file_name().to_string_lossy().starts_with('.'))
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    if entries.is_empty() { return None; }
    let idx = random_usize() % entries.len();
    Some(entries[idx].clone())
}

fn parse_two_quoted(s: &str) -> Option<(String, String)> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;

    for ch in s.chars() {
        if ch == '"' {
            if in_quote {
                parts.push(current.clone());
                current.clear();
                in_quote = false;
            } else {
                in_quote = true;
            }
        } else if in_quote {
            current.push(ch);
        }
    }

    if parts.len() >= 2 {
        Some((parts[0].clone(), parts[1].clone()))
    } else {
        None
    }
}

/// Xorshift64 PRNG seeded from time + thread ID for decent randomness
fn random_usize() -> usize {
    use std::time::SystemTime;
    use std::sync::atomic::{AtomicU64, Ordering};
    static STATE: AtomicU64 = AtomicU64::new(0);

    let mut s = STATE.load(Ordering::Relaxed);
    if s == 0 {
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        s = nanos ^ 0x517cc1b727220a95;
        if s == 0 { s = 1; }
    }
    // xorshift64
    s ^= s << 13;
    s ^= s >> 7;
    s ^= s << 17;
    STATE.store(s, Ordering::Relaxed);
    s as usize
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::Instant;

    /// A scratch area under the system temp dir, removed on drop.
    /// The library root is `<area>/outer/library`, two levels down, so
    /// `../../x` from the root still lands inside the area.
    struct Area {
        dir: PathBuf,
        root: PathBuf,
    }

    impl Area {
        fn new(tag: &str) -> Self {
            static N: AtomicU32 = AtomicU32::new(0);
            let dir = std::env::temp_dir().join(format!(
                "oma-test-{}-{}-{tag}",
                std::process::id(),
                N.fetch_add(1, Ordering::Relaxed),
            ));
            let _ = fs::remove_dir_all(&dir);
            let root = dir.join("outer").join("library");
            fs::create_dir_all(&root).unwrap();
            Area { dir, root }
        }

        fn run(&self, script: &str) -> Outcome {
            self.run_with(script, Budget::per_read())
        }

        fn run_with(&self, script: &str, budget: Budget) -> Outcome {
            let ctx = LambdaContext::new(&self.root, &self.root.join("doc"));
            execute_with_budget(script, &ctx, &HashMap::new(), budget)
        }

        fn count(&self, room: &str) -> usize {
            fs::read_dir(self.root.join(room)).map(|d| d.count()).unwrap_or(0)
        }
    }

    impl Drop for Area {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    // ── A: confinement ──

    #[test]
    fn confine_accepts_ordinary_relative_paths() {
        let a = Area::new("confine-ok");
        assert_eq!(confine(&a.root, "notes/today"), Some(a.root.join("notes/today")));
        assert_eq!(confine(&a.root, "./welcome"), Some(a.root.join("./welcome")));
    }

    #[test]
    fn confine_refuses_escapes() {
        let a = Area::new("confine-no");
        assert_eq!(confine(&a.root, ""), None);
        assert_eq!(confine(&a.root, "../x"), None);
        assert_eq!(confine(&a.root, "../../x"), None);
        assert_eq!(confine(&a.root, "notes/../../x"), None);
        assert_eq!(confine(&a.root, "/etc/x"), None);
        assert_eq!(confine(&a.root, a.dir.join("x").to_str().unwrap()), None);
    }

    #[test]
    fn a_normal_relative_write_still_works() {
        let a = Area::new("write-ok");
        let out = a.run("write west-wing/journal/today \"hello\"");
        assert!(out.refusals.is_empty(), "{:?}", out.refusals);
        let written = fs::read_to_string(a.root.join("west-wing/journal/today")).unwrap();
        assert_eq!(written, "hello\n");
    }

    #[test]
    fn write_with_dotdot_is_refused_and_creates_nothing_outside() {
        let a = Area::new("write-dotdot");
        let out = a.run("write ../../x \"escaped\"\nsay \"still running\"");
        assert_eq!(out.refusals.len(), 1);
        assert!(!a.dir.join("x").exists());
        assert!(!out.exhausted);
        assert_eq!(out.steps, 2, "the margin goes on after a refusal");
    }

    #[test]
    fn inscribe_outside_is_refused_and_its_text_is_not_run() {
        let a = Area::new("inscribe-out");
        let out = a.run("inscribe ../leak\n  write inside \"x\"\nwrite after \"y\"");
        assert_eq!(out.refusals.len(), 1);
        assert!(!a.root.parent().unwrap().join("leak").exists());
        assert!(!a.root.join("inside").exists(), "document text was executed");
        assert!(a.root.join("after").exists());
    }

    #[test]
    fn absolute_paths_are_refused_for_write_and_withdraw() {
        let a = Area::new("absolute");
        let victim = a.dir.join("victim");
        fs::write(&victim, "keep me").unwrap();
        let target = a.dir.join("planted");

        let script = format!(
            "withdraw {}\nwrite {} \"x\"\nwithdraw /etc/x",
            victim.display(),
            target.display(),
        );
        let out = a.run(&script);

        assert_eq!(out.refusals.len(), 3, "{:?}", out.refusals);
        assert_eq!(fs::read_to_string(&victim).unwrap(), "keep me");
        assert!(!target.exists());
    }

    #[test]
    fn reads_are_confined_too() {
        let a = Area::new("reads");
        fs::write(a.dir.join("secret"), "the secret line\n").unwrap();
        let out = a.run("set s ← random-line ../../secret\nset n ← count ../..");
        assert_eq!(out.refusals.len(), 2);
        assert_eq!(out.vars["s"], "");
        assert_eq!(out.vars["n"], "0");
    }

    #[cfg(unix)]
    #[test]
    fn symlinks_that_leave_the_library_are_refused() {
        use std::os::unix::fs::symlink;

        let a = Area::new("symlink");
        let outside = a.dir.join("outside");
        fs::create_dir_all(&outside).unwrap();
        let victim = outside.join("victim");
        fs::write(&victim, "keep me").unwrap();

        symlink(&outside, a.root.join("door")).unwrap(); // directory link
        symlink(&victim, a.root.join("alias")).unwrap(); // file link
        symlink(outside.join("unborn"), a.root.join("dangling")).unwrap();

        let out = a.run(
            "write door/planted \"x\"\n\
             write door/deeper/planted \"x\"\n\
             write alias \"x\"\n\
             withdraw door/victim\n\
             write dangling \"x\"\n\
             inscribe door/page\n  text",
        );

        assert_eq!(out.refusals.len(), 6, "{:?}", out.refusals);
        assert_eq!(fs::read_to_string(&victim).unwrap(), "keep me");
        let left_outside: Vec<_> = fs::read_dir(&outside).unwrap()
            .flatten().map(|e| e.file_name()).collect();
        assert_eq!(left_outside, ["victim"], "something was created outside the library");
    }

    #[cfg(unix)]
    #[test]
    fn symlinks_that_stay_inside_the_library_still_work() {
        use std::os::unix::fs::symlink;

        let a = Area::new("symlink-in");
        fs::create_dir_all(a.root.join("stacks")).unwrap();
        symlink(a.root.join("stacks"), a.root.join("shortcut")).unwrap();

        let out = a.run("write shortcut/note \"x\"");
        assert!(out.refusals.is_empty(), "{:?}", out.refusals);
        assert!(a.root.join("stacks/note").exists());
    }

    // ── B: one budget, shared variables, nested blocks ──

    #[test]
    fn loop_while_ends_when_its_body_changes_the_condition() {
        let a = Area::new("loop-while");
        let out = a.run(
            "set n ← count box/*\n\
             loop while n < 3:\n\
             \x20\x20write box/item-{n} \"x\"\n\
             \x20\x20set n ← count box/*\n\
             say \"done\"",
        );
        assert!(!out.exhausted);
        assert_eq!(a.count("box"), 3, "exactly three passes");
        assert_eq!(out.vars["n"], "3");
        // set, loop header, 3 × (2 body lines + the re-check), say
        assert_eq!(out.steps, 12);
    }

    #[test]
    fn a_false_loop_while_never_runs() {
        let a = Area::new("loop-zero");
        let out = a.run("set go \"0\"\nloop while go:\n  write ran \"x\"");
        assert!(!a.root.join("ran").exists());
        assert!(!out.exhausted);
    }

    #[test]
    fn variables_set_inside_a_block_are_seen_after_it() {
        let a = Area::new("scope");
        let out = a.run("set a \"1\"\nif a:\n  set b \"from inside\"\nset c \"{b}\"");
        assert_eq!(out.vars["c"], "from inside");
    }

    #[test]
    fn nested_blocks_keep_their_indentation() {
        let a = Area::new("nested");
        let out = a.run(
            "set outer \"1\"\n\
             set off \"0\"\n\
             loop while outer:\n\
             \x20\x20set inner ← count box/*\n\
             \x20\x20loop while inner < 2:\n\
             \x20\x20\x20\x20write box/f-{inner} \"x\"\n\
             \x20\x20\x20\x20set inner ← count box/*\n\
             \x20\x20if off:\n\
             \x20\x20\x20\x20write box/never \"x\"\n\
             \n\
             \x20\x20set outer \"0\"\n\
             write out/after \"x\"",
        );
        assert!(!out.exhausted);
        assert_eq!(a.count("box"), 2, "inner loop ran twice, the false `if` did not run");
        assert_eq!(out.vars["outer"], "0");
        assert!(a.root.join("out/after").exists());
    }

    #[test]
    fn inscribe_keeps_blank_lines_and_inner_indentation_and_fills_in_variables() {
        let a = Area::new("inscribe");
        a.run(
            "set who \"A.\"\n\
             if who:\n\
             \x20\x20inscribe page\n\
             \x20\x20\x20\x20Title\n\
             \n\
             \x20\x20\x20\x20\x20\x20> quoted\n\
             \x20\x20\x20\x20signed {who}\n\
             \x20\x20write marker \"x\"",
        );
        let page = fs::read_to_string(a.root.join("page")).unwrap();
        assert_eq!(page, "Title\n\n  > quoted\nsigned A.\n");
        assert!(a.root.join("marker").exists());
    }

    #[test]
    fn an_endless_loop_stops_at_the_budget() {
        let a = Area::new("endless");
        let started = Instant::now();
        let out = a.run("loop:\n  set x \"1\"\nwrite after \"x\"");
        assert!(out.exhausted);
        assert_eq!(out.steps, STEP_LIMIT);
        assert!(!a.root.join("after").exists(), "a stopped margin must not carry on");
        assert!(started.elapsed() < Duration::from_secs(20));
    }

    #[test]
    fn an_endless_loop_while_with_an_empty_body_stops_too() {
        let a = Area::new("endless-while");
        let out = a.run("set go \"1\"\nloop while go:\n  # nothing\nsay \"unreachable\"");
        assert!(out.exhausted);
        assert_eq!(out.steps, STEP_LIMIT);
    }

    #[test]
    fn nested_loops_share_one_budget() {
        let a = Area::new("nested-budget");
        let out = a.run_with(
            "loop:\n  loop:\n    loop:\n      write log \"x\"",
            Budget::of(500, Duration::ZERO),
        );
        assert!(out.exhausted);
        assert_eq!(out.steps, 500, "nesting must not multiply the allowance");
        let written = fs::read_to_string(a.root.join("log")).unwrap();
        assert!(written.lines().count() < 500);
    }

    #[test]
    fn a_margin_that_fits_its_budget_is_not_flagged() {
        let a = Area::new("fits");
        let out = a.run_with("say \"one\"\nsay \"two\"", Budget::of(2, Duration::ZERO));
        assert!(!out.exhausted);
        assert_eq!(out.steps, 2);
    }

    // ── C: wait ──

    #[test]
    fn wait_parses_defensively() {
        assert_eq!(wait_duration("2s"), Some(Duration::from_secs(2)));
        assert_eq!(wait_duration("0.5s"), Some(Duration::from_millis(500)));
        assert_eq!(wait_duration("3"), Some(Duration::from_secs(3)));
        assert_eq!(wait_duration("0s"), Some(Duration::ZERO));
        assert_eq!(wait_duration("-0s"), Some(Duration::ZERO));
        assert_eq!(wait_duration("a moment"), Some(Duration::from_secs(1)));

        assert_eq!(wait_duration("-1s"), None);
        assert_eq!(wait_duration("nans"), None);
        assert_eq!(wait_duration("NaN"), None);
        assert_eq!(wait_duration("infs"), None);
        assert_eq!(wait_duration("-inf"), None);
        assert_eq!(wait_duration("1e400s"), None); // overflows to infinity

        let longest = Duration::from_secs_f64(MAX_SINGLE_WAIT_SECS);
        assert_eq!(wait_duration("61s"), Some(longest));
        assert_eq!(wait_duration("1e300s"), Some(longest));
        assert_eq!(wait_duration("99999999999999999999s"), Some(longest));
    }

    #[test]
    fn bad_waits_are_refused_without_panicking_or_sleeping() {
        let a = Area::new("wait-bad");
        let started = Instant::now();
        let out = a.run("wait -1s\nwait nans\nwait infs\nwait 0s\nwrite after \"x\"");
        assert_eq!(out.refusals.len(), 3, "{:?}", out.refusals);
        assert!(a.root.join("after").exists(), "the margin goes on after a refused wait");
        assert!(started.elapsed() < Duration::from_secs(1));
    }

    #[test]
    fn waiting_draws_on_one_allowance_and_stops_the_margin_when_it_runs_out() {
        let a = Area::new("wait-budget");
        let started = Instant::now();
        let out = a.run_with(
            "loop:\n  wait 1e300s\nwrite after \"x\"",
            Budget::of(1_000, Duration::from_millis(30)),
        );
        let took = started.elapsed();
        assert!(out.exhausted);
        assert!(took >= Duration::from_millis(30), "it did wait what it was allowed");
        assert!(took < Duration::from_secs(5), "and no longer: {took:?}");
        assert!(!a.root.join("after").exists());
    }

    // ── The founding collection ──

    #[test]
    fn every_founding_margin_runs_clean() {
        let a = Area::new("founding");
        crate::founding::create_library(&a.root);

        fn documents(dir: &Path, found: &mut Vec<PathBuf>) {
            for entry in fs::read_dir(dir).unwrap().flatten() {
                let path = entry.path();
                if entry.file_name().to_string_lossy().starts_with('.') {
                    continue;
                }
                if path.is_dir() {
                    documents(&path, found);
                } else {
                    found.push(path);
                }
            }
        }
        let mut found = Vec::new();
        documents(&a.root, &mut found);
        found.sort();

        let mut margins = 0;
        for path in &found {
            let doc = crate::library::Document::load(path).unwrap();
            if doc.lambda.is_empty() {
                continue;
            }
            margins += 1;

            // What the shell's `read` provides.
            let ctx = LambdaContext::new(&a.root, path);
            let mut vars = HashMap::new();
            vars.insert("reader".to_string(), "a reader".to_string());
            vars.insert("time".to_string(), "12:00:00".to_string());
            vars.insert("date".to_string(), "2026-01-01".to_string());
            vars.insert("documents-read".to_string(), "1".to_string());
            vars.insert("document".to_string(), doc.name.clone());
            vars.insert("read-count".to_string(), ctx.increment_read_count().to_string());

            let out = execute(&doc.lambda, &ctx, &vars);
            assert!(out.refusals.is_empty(), "{}: {:?}", doc.name, out.refusals);
            assert!(!out.exhausted, "{} ran out of budget", doc.name);
            assert!(out.steps < 50, "{} took {} steps", doc.name, out.steps);
        }
        assert_eq!(margins, 8, "margins found in the founding collection");

        // And they did what they say they do.
        let journal = fs::read_to_string(a.root.join("west-wing/journal/2026-01-01")).unwrap();
        assert_eq!(journal.lines().count(), 9);
        assert!(!a.root.join("east-wing/stacks/the-confession").exists());

        let dream = fs::read_to_string(a.root.join("west-wing/ephemera/dream-1")).unwrap();
        assert!(dream.starts_with("A dream of the library (dream #1)\n\n"), "{dream}");
        assert!(dream.ends_with("It will expire with the ephemera.\n"), "{dream}");
        assert!(!dream.contains("{frag"), "{dream}");

        let lesson = fs::read_to_string(
            a.root.join("east-wing/stacks/lesson-2-the-three-registers"),
        ).unwrap();
        assert!(lesson.starts_with("Lesson 2: The three registers\n\n"), "{lesson}");
        assert!(lesson.contains("\n  > inspect lesson 1\n"), "{lesson}");
        assert!(lesson.ends_with("The document \"how to inscribe\" explains how.\n"), "{lesson}");
    }
}
