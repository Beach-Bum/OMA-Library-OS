//! The Λ Engine — simple lambda interpreter.
//! Executes the readable Λ syntax from document margins.
//! Full ΦΜΛ grid programs are a future extension.

use std::collections::HashMap;
use std::path::Path;

use crate::narrator;

/// Execute a simple Λ script.
/// Returns true if execution completed normally.
pub fn execute(
    script: &str,
    root: &Path,
    vars: &HashMap<String, String>,
) -> bool {
    if script.trim().is_empty() {
        return true;
    }

    for line in script.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }

        // Variable substitution
        let line = substitute(line, vars);

        if line.starts_with("say ") {
            let text = line.strip_prefix("say ").unwrap();
            let text = text.trim_matches('"');
            narrator::say(text);
        } else if line.starts_with("write ") {
            // write path "text"
            let rest = line.strip_prefix("write ").unwrap();
            if let Some((path, text)) = rest.split_once(' ') {
                let full_path = root.join(path.trim());
                let text = text.trim_matches('"');
                let _ = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(full_path)
                    .and_then(|mut f| {
                        use std::io::Write;
                        writeln!(f, "{text}")
                    });
            }
        }
        // More Λ instructions can be added here:
        // query, if/else, loop, count, read, ask, wait, etc.
    }

    true
}

fn substitute(line: &str, vars: &HashMap<String, String>) -> String {
    let mut result = line.to_string();
    for (key, value) in vars {
        result = result.replace(&format!("{{{key}}}"), value);
    }
    result
}
