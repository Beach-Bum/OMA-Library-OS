//! Embedded mode — extract and run ΦΜΛ programs from whitespace.
//!
//! A ΦΜΛ program can be hidden in the whitespace of any host document.
//! The encoding:
//!   - Acquisition stamp: SP TAB SP TAB SP TAB LF  (ASCII 42 = '*')
//!   - Each instruction: 7 whitespace chars (SP=0, TAB=1) + LF
//!   - The 7 bits encode the ASCII value of the opcode
//!
//! Usage: oma --embedded path/to/document

use std::fs;
use std::path::Path;

/// Extract a ΦΜΛ program from the whitespace of a host document.
pub fn extract(content: &str) -> Option<Vec<u8>> {
    // Find acquisition stamp: SP TAB SP TAB SP TAB LF
    let ws: Vec<u8> = content
        .bytes()
        .filter(|b| matches!(b, b' ' | b'\t' | b'\n'))
        .collect();

    let stamp = [b' ', b'\t', b' ', b'\t', b' ', b'\t', b'\n'];

    let start = ws.windows(stamp.len()).position(|w| w == stamp)?;
    let program_ws = &ws[start + stamp.len()..];

    // Decode 7-bit frames
    let mut opcodes = Vec::new();
    let mut frame = Vec::new();

    for &byte in program_ws {
        if byte == b'\n' {
            if frame.len() == 7 {
                let mut value: u8 = 0;
                for (i, &bit) in frame.iter().enumerate() {
                    let bit_val = if bit == b'\t' { 1 } else { 0 };
                    value |= bit_val << (6 - i);
                }
                opcodes.push(value);
            }
            frame.clear();
        } else {
            frame.push(byte);
        }
    }

    if opcodes.is_empty() {
        None
    } else {
        Some(opcodes)
    }
}

/// Check if a document contains an embedded ΦΜΛ program.
pub fn has_embedded(content: &str) -> bool {
    let ws: Vec<u8> = content
        .bytes()
        .filter(|b| matches!(b, b' ' | b'\t' | b'\n'))
        .collect();
    let stamp = [b' ', b'\t', b' ', b'\t', b' ', b'\t', b'\n'];
    ws.windows(stamp.len()).any(|w| w == stamp)
}

/// Run an embedded program from a file.
pub fn run_embedded(path: &Path) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Cannot read {}: {e}", path.display()))?;

    let opcodes = extract(&content)
        .ok_or_else(|| "No embedded ΦΜΛ program found (missing acquisition stamp).".to_string())?;

    let program: String = opcodes.iter().map(|&b| b as char).collect();
    println!("Extracted embedded program ({} opcodes):", opcodes.len());
    println!();

    // For now, display the extracted program.
    // Full grid execution will be handled by the grid interpreter.
    for (i, line) in program.lines().enumerate() {
        println!("  {:3}: {}", i + 1, line);
    }

    // Try to run it through the grid interpreter if it looks like grid code
    if program.contains('Φ') || program.contains('Μ') || program.contains('Λ') {
        println!();
        println!("This appears to be a ΦΜΛ grid program.");
        println!("Running through the grid interpreter...");
        println!();
        let mut grid_state = crate::grid::GridState::from_source(&program);
        grid_state.run(10000);
    } else {
        println!();
        println!("Extracted {} bytes of opcode data.", opcodes.len());
    }

    Ok(())
}
