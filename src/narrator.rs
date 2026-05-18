//! The Narrator — prose output formatting.
//! Everything the reader sees passes through here.

pub fn say(text: &str) {
    for line in text.lines() {
        println!("    {line}");
    }
}

pub fn blank() {
    println!();
}

pub fn room_description(name: &str, description: &str) {
    println!("    You are in {name}.");
    if !description.is_empty() {
        println!("    {description}");
    }
}

pub fn shelf_entry(name: &str, description: &str, is_dir: bool) {
    let suffix = if is_dir { "/" } else { "" };
    let dots = 40_usize.saturating_sub(name.len() + suffix.len());
    let padding: String = std::iter::repeat_n('.', dots.max(2)).collect();
    println!("      {name}{suffix} {padding} {description}");
}

pub fn header(text: &str) {
    println!();
    let border = "═".repeat(text.len() + 8);
    println!("    ╔{border}╗");
    println!("    ║    {text}    ║");
    println!("    ╚{border}╝");
    println!();
}

pub fn register_header(glyph: &str, name: &str, meaning: &str) {
    println!("    {glyph} — {name} — {meaning}");
}

pub fn register_field(key: &str, value: &str) {
    println!("      {key}: {value}");
}

pub fn error(text: &str) {
    println!("    {text}");
}
