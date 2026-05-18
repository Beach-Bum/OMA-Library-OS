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

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write as IoWrite};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use crate::narrator;

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

/// Execute a Λ script with full context.
pub fn execute(
    script: &str,
    ctx: &LambdaContext,
    vars: &HashMap<String, String>,
) -> bool {
    if script.trim().is_empty() {
        return true;
    }

    let mut local_vars = vars.clone();
    let lines: Vec<&str> = script.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        i += 1;

        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }

        let line = substitute(line, &local_vars);

        if line.starts_with("say ") {
            let text = line.strip_prefix("say ").unwrap().trim_matches('"');
            narrator::say(text);

        } else if line.starts_with("write ") {
            let rest = line.strip_prefix("write ").unwrap();
            if let Some((path, text)) = rest.split_once(' ') {
                let full_path = ctx.root.join(path.trim());
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
                let expr = expr.trim();
                let value = eval_expr(expr, ctx, &local_vars);
                local_vars.insert(name, value);
            } else if let Some((name, val)) = rest.split_once(' ') {
                let name = name.trim().to_string();
                let val = val.trim().trim_matches('"').to_string();
                local_vars.insert(name, val);
            }

        } else if line.starts_with("if ") && line.ends_with(':') {
            let condition = line.strip_prefix("if ").unwrap()
                .strip_suffix(':').unwrap().trim();
            let is_true = eval_condition(condition, &local_vars);

            // Collect indented body
            let mut body = Vec::new();
            while i < lines.len() {
                let next = lines[i];
                if next.starts_with("  ") || next.starts_with("\t") {
                    body.push(next.trim());
                    i += 1;
                } else {
                    break;
                }
            }

            if is_true {
                let body_script = body.join("\n");
                execute(&body_script, ctx, &local_vars);
            }

        } else if line.starts_with("inscribe ") {
            let path = line.strip_prefix("inscribe ").unwrap().trim();
            let full_path = ctx.root.join(path);
            // Collect indented content
            let mut content = String::new();
            while i < lines.len() {
                let next = lines[i];
                if next.starts_with("  ") || next.starts_with("\t") {
                    content.push_str(next.trim_start());
                    content.push('\n');
                    i += 1;
                } else {
                    break;
                }
            }
            if let Some(parent) = full_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(full_path, content);

        } else if line.starts_with("withdraw ") {
            let path = line.strip_prefix("withdraw ").unwrap().trim();
            let full_path = ctx.root.join(path);
            let _ = fs::remove_file(full_path);

        } else if line.starts_with("erode self ") {
            let n: usize = line.strip_prefix("erode self ").unwrap()
                .trim().parse().unwrap_or(1);
            erode_document(&ctx.doc_path, n);

        } else if line.starts_with("mutate self ") {
            let rest = line.strip_prefix("mutate self ").unwrap();
            // mutate self "old" "new"
            if let Some((old, new)) = parse_two_quoted(rest) {
                mutate_document(&ctx.doc_path, &old, &new);
            }

        } else if line.starts_with("wait ") {
            let rest = line.strip_prefix("wait ").unwrap().trim();
            let secs: f64 = rest.trim_end_matches('s').parse().unwrap_or(1.0);
            thread::sleep(Duration::from_secs_f64(secs));
        }
    }

    true
}

/// Legacy execute interface (for backward compat)
pub fn execute_simple(
    script: &str,
    root: &Path,
    vars: &HashMap<String, String>,
) -> bool {
    let ctx = LambdaContext {
        root: root.to_path_buf(),
        doc_path: PathBuf::new(),
        read_count_path: PathBuf::new(),
    };
    execute(script, &ctx, vars)
}

// ── Expression evaluation ────────────────────────────────────────────

fn eval_expr(expr: &str, ctx: &LambdaContext, vars: &HashMap<String, String>) -> String {
    if expr.starts_with("count ") {
        let path = expr.strip_prefix("count ").unwrap().trim();
        let path = path.trim_end_matches("/*");
        let full = ctx.root.join(path);
        let count = fs::read_dir(&full)
            .map(|d| d.flatten().filter(|e| {
                !e.file_name().to_string_lossy().starts_with('.')
            }).count())
            .unwrap_or(0);
        count.to_string()

    } else if expr == "read-count" {
        ctx.get_read_count().to_string()

    } else if expr.starts_with("random-line ") {
        let path = expr.strip_prefix("random-line ").unwrap().trim();
        let full = ctx.root.join(path);
        random_line_from(&full).unwrap_or_default()

    } else if expr.starts_with("random-choice ") {
        let path = expr.strip_prefix("random-choice ").unwrap()
            .trim().trim_end_matches('/');
        let full = ctx.root.join(path);
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
    let idx = simple_random() % lines.len();
    Some(lines[idx].to_string())
}

fn random_file_in(dir: &Path) -> Option<String> {
    let entries: Vec<String> = fs::read_dir(dir).ok()?
        .flatten()
        .filter(|e| !e.file_name().to_string_lossy().starts_with('.'))
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    if entries.is_empty() { return None; }
    let idx = simple_random() % entries.len();
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

/// Simple deterministic-ish random using time
fn simple_random() -> usize {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    nanos as usize
}
