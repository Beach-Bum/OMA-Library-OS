//! Multi-reader session management.
//!
//! Each oma instance registers itself in .sessions/ on boot.
//! The `readers` command lists all active sessions.
//! Sessions are cleaned up on shutdown or detected as stale.

use std::fs;
use std::path::Path;

/// Register a new session. Returns the session ID.
pub fn register(root: &Path) -> String {
    let sessions_dir = root.join(".sessions");
    let _ = fs::create_dir_all(&sessions_dir);

    let pid = std::process::id();
    let user = std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .unwrap_or_else(|_| "unknown".into());
    let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let session_id = format!("{pid}");

    let content = format!("{user}\n{time}\n{pid}\n");
    let _ = fs::write(sessions_dir.join(&session_id), content);

    session_id
}

/// Unregister a session on shutdown.
pub fn unregister(root: &Path, session_id: &str) {
    let path = root.join(".sessions").join(session_id);
    let _ = fs::remove_file(path);
}

/// List all active sessions. Returns (username, start_time, pid) tuples.
/// Cleans up stale sessions (where the PID is no longer running).
pub fn list_active(root: &Path) -> Vec<(String, String, u32)> {
    let sessions_dir = root.join(".sessions");
    let Ok(entries) = fs::read_dir(&sessions_dir) else {
        return Vec::new();
    };

    let mut active = Vec::new();

    for entry in entries.flatten() {
        let content = match fs::read_to_string(entry.path()) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let lines: Vec<&str> = content.lines().collect();
        if lines.len() < 3 {
            continue;
        }

        let user = lines[0].to_string();
        let time = lines[1].to_string();
        let pid: u32 = match lines[2].parse() {
            Ok(p) => p,
            Err(_) => continue,
        };

        // Check if PID is still running
        if is_process_alive(pid) {
            active.push((user, time, pid));
        } else {
            // Stale session — clean up
            let _ = fs::remove_file(entry.path());
        }
    }

    active
}

/// Check if a process is still running
fn is_process_alive(pid: u32) -> bool {
    #[cfg(unix)]
    {
        // kill(pid, 0) checks existence without sending a signal
        unsafe { libc::kill(pid as i32, 0) == 0 }
    }
    #[cfg(not(unix))]
    {
        // Assume alive on non-unix
        let _ = pid;
        true
    }
}
