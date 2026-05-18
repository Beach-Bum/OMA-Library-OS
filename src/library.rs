//! The Library — filesystem abstraction.
//! Maps between library concepts (rooms, shelves, documents)
//! and the underlying filesystem.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Room descriptions — stored as .room-description files
const ROOM_DESC_FILE: &str = ".room-description";

/// Document metadata separator
const META_SEPARATOR: &str = "\n--- Λ ---\n";

pub struct Document {
    pub name: String,
    pub path: PathBuf,
    pub mu: String,       // visible text (Μ)
    pub lambda: String,   // executable logic (Λ)
    pub is_process: bool,
}

impl Document {
    pub fn load(path: &Path) -> Option<Self> {
        let content = fs::read_to_string(path).ok()?;
        let name = path.file_name()?.to_string_lossy().to_string();
        let is_process = path.starts_with("east-wing/processes")
            || content.contains(META_SEPARATOR);

        let (mu, lambda) = if let Some(idx) = content.find(META_SEPARATOR) {
            let (m, l) = content.split_at(idx);
            (m.to_string(), l[META_SEPARATOR.len()..].to_string())
        } else {
            (content, String::new())
        };

        Some(Document {
            name,
            path: path.to_path_buf(),
            mu,
            lambda,
            is_process,
        })
    }
}

pub struct ShelfEntry {
    pub name: String,
    pub is_dir: bool,
    pub description: String,
}

/// List contents of a room (directory)
pub fn browse(room_path: &Path) -> Vec<ShelfEntry> {
    let mut entries = Vec::new();
    let Ok(read_dir) = fs::read_dir(room_path) else {
        return entries;
    };

    for entry in read_dir.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue; // hidden files are infrastructure
        }
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let description = if is_dir {
            // Count contents
            let count = fs::read_dir(entry.path())
                .map(|d| d.flatten().filter(|e| {
                    !e.file_name().to_string_lossy().starts_with('.')
                }).count())
                .unwrap_or(0);
            // Check for room description
            let desc_path = entry.path().join(ROOM_DESC_FILE);
            if let Ok(desc) = fs::read_to_string(desc_path) {
                desc.lines().next().unwrap_or("").to_string()
            } else {
                format!("{count} documents")
            }
        } else {
            // First line of the document as description
            fs::read_to_string(entry.path())
                .ok()
                .and_then(|c| c.lines().next().map(|l| {
                    let l = l.trim();
                    if l.len() > 60 {
                        format!("{}...", &l[..57])
                    } else {
                        l.to_string()
                    }
                }))
                .unwrap_or_default()
        };

        entries.push(ShelfEntry {
            name,
            is_dir,
            description,
        });
    }

    // Sort: directories first, then alphabetical
    entries.sort_by(|a, b| {
        b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name))
    });

    entries
}

/// Get a room's description
pub fn room_description(room_path: &Path) -> String {
    let desc_path = room_path.join(ROOM_DESC_FILE);
    fs::read_to_string(desc_path)
        .ok()
        .and_then(|c| c.lines().next().map(String::from))
        .unwrap_or_default()
}

/// Friendly name for a path relative to the library root
pub fn room_name(path: &Path, root: &Path) -> String {
    let rel = path.strip_prefix(root).unwrap_or(path);
    if rel == Path::new("") {
        return "the Entrance Hall".to_string();
    }

    let name = rel.to_string_lossy().to_string();
    match name.as_str() {
        "east-wing" => "the East Wing — Technical Collection".to_string(),
        "east-wing/stacks" => "the Stacks (East Wing)".to_string(),
        "east-wing/processes" => "the Processes Room".to_string(),
        "east-wing/devices" => "the Devices Room".to_string(),
        "east-wing/utilities" => "the Utilities Room".to_string(),
        "east-wing/networking" => "the Networking Room".to_string(),
        "west-wing" => "the West Wing — Letters & Ephemera".to_string(),
        "west-wing/correspondence" => "the Correspondence Room".to_string(),
        "west-wing/drafts" => "the Drafts Room".to_string(),
        "west-wing/journal" => "the Journal Room".to_string(),
        "west-wing/ephemera" => "the Ephemera Room".to_string(),
        "basement" => "the Basement (restricted)".to_string(),
        "basement/fundament" => "the Fundament".to_string(),
        "basement/blueprints" => "the Blueprints Room".to_string(),
        "basement/vault" => "the Vault".to_string(),
        "acquisitions" => "Acquisitions — newly arrived, unsorted".to_string(),
        "reading-room" => "the Reading Room".to_string(),
        "other-libraries" => "Other Libraries".to_string(),
        _ => format!("the {name} room"),
    }
}

/// Search documents for a query string
pub fn search(root: &Path, query: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    search_recursive(root, root, query, &mut results);
    results
}

fn search_recursive(
    base: &Path,
    dir: &Path,
    query: &str,
    results: &mut Vec<(String, String)>,
) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    let query_lower = query.to_lowercase();

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
            search_recursive(base, &path, query, results);
        } else if let Ok(content) = fs::read_to_string(&path) {
            if content.to_lowercase().contains(&query_lower)
                || name.to_lowercase().contains(&query_lower)
            {
                let rel = path.strip_prefix(base).unwrap_or(&path);
                let snippet = content
                    .lines()
                    .find(|l| l.to_lowercase().contains(&query_lower))
                    .unwrap_or("")
                    .trim();
                let snippet = if snippet.len() > 70 {
                    format!("{}...", &snippet[..67])
                } else {
                    snippet.to_string()
                };
                results.push((rel.to_string_lossy().to_string(), snippet));
            }
        }
    }
}

/// Write a journal entry
pub fn journal_write(root: &Path, text: &str) {
    let journal_dir = root.join("west-wing").join("journal");
    let _ = fs::create_dir_all(&journal_dir);

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let time = chrono::Local::now().format("%H:%M").to_string();
    let path = journal_dir.join(&today);

    let entry = format!("{time}  {text}\n");
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap_or_else(|_| panic!("Cannot write to journal"));
    let _ = file.write_all(entry.as_bytes());
}
