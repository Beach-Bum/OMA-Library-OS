//! ΦΜΛ — The Open Manual Archive
//!
//! A living book that is also an operating system.
//! Single binary. Replaces /bin/sh. Becomes PID 1.
//!
//! Every file is a document with three registers:
//!   Φ — Phi — Form    — where it is, what it's classified as
//!   Μ — Mu — Message — what it says (visible text)
//!   Λ — Lambda — Function  — what it does (executable logic)

mod daemon;
mod embedded;
mod founding;
mod grid;
mod lambda;
mod library;
mod narrator;
mod session;
mod shell;

use std::env;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Handle --embedded mode
    if args.len() >= 3 && args[1] == "--embedded" {
        let path = PathBuf::from(&args[2]);
        match embedded::run_embedded(&path) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {e}"),
        }
        return;
    }

    let library_root = env::var("OMA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            env::var("HOME")
                .map(|h| PathBuf::from(h).join("oma-library"))
                .unwrap_or_else(|_| PathBuf::from("/oma-library"))
        });

    // First boot: seed the founding collection
    if !library_root.join("welcome").exists() {
        eprintln!("Opening the library for the first time...\n");
        founding::create_library(&library_root);
    }

    // Register this session
    let session_id = session::register(&library_root);

    // Start background daemons
    let running = Arc::new(AtomicBool::new(true));
    let librarian = daemon::start_librarian(library_root.clone(), Arc::clone(&running));
    let dreamer = daemon::start_dreamer(library_root.clone(), Arc::clone(&running));

    let mut state = shell::ShellState::new(library_root.clone());
    state.boot();

    let stdin = io::stdin();
    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }

        let input = line.trim();
        if input.is_empty() {
            continue;
        }

        if !state.execute(input) {
            break;
        }
    }

    state.shutdown();

    // Stop daemons
    running.store(false, Ordering::Relaxed);
    let _ = librarian.join();
    let _ = dreamer.join();

    // Unregister session
    session::unregister(&library_root, &session_id);
}
