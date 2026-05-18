//! ΦΜΛ — The Open Manual Archive
//!
//! A living book that is also an operating system.
//! Single binary. Replaces /bin/sh. Becomes PID 1.
//!
//! Every file is a document with three registers:
//!   Φ (Form)    — where it is, what it's classified as
//!   Μ (Message) — what it says (visible text)
//!   Λ (Lambda)  — what it does (executable logic)

mod founding;
mod lambda;
mod library;
mod narrator;
mod shell;

use std::env;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

fn main() {
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

    let mut state = shell::ShellState::new(library_root);
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
}
