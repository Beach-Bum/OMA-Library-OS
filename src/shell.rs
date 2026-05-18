//! The Shell — command parser and REPL.
//! Every Unix command reimagined as an act of reading.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use crate::{lambda, library, narrator};

pub struct ShellState {
    pub root: PathBuf,
    pub cwd: PathBuf,
    pub history: Vec<String>,
    pub reader_name: String,
    pub boot_time: chrono::DateTime<chrono::Local>,
    pub documents_read: usize,
    pub is_archivist: bool,
    pub annexed: HashMap<String, PathBuf>,
}

impl ShellState {
    pub fn new(root: PathBuf) -> Self {
        ShellState {
            cwd: root.clone(),
            root,
            history: Vec::new(),
            reader_name: whoami(),
            boot_time: chrono::Local::now(),
            documents_read: 0,
            is_archivist: false,
            annexed: HashMap::new(),
        }
    }

    pub fn boot(&mut self) {
        narrator::header("THE OPEN MANUAL ARCHIVE");

        let room = library::room_name(&self.cwd, &self.root);
        narrator::say(&format!("You are standing in {room}."));
        narrator::say("The lights are on. The shelves are full.");
        narrator::say("A document rests on the desk: \"welcome\"");

        library::journal_write(&self.root, "The library opened.");
        library::journal_write(
            &self.root,
            &format!("A reader arrived: {}", self.reader_name),
        );

        // First visit tour
        self.offer_tour();
    }

    fn offer_tour(&mut self) {
        narrator::blank();
        narrator::say("How would you like to begin?");
        narrator::blank();
        narrator::say("  1  Read the welcome letter");
        narrator::say("  2  Explore the library");
        narrator::say("  3  Start writing");
        narrator::blank();
        narrator::say("Type a number, or just start typing commands.");

        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return;
        }
        let input = input.trim();

        match input {
            "1" => self.tour_read(),
            "2" => self.tour_explore(),
            "3" => self.tour_write(),
            _ => {
                // They typed a command — run it
                if !input.is_empty() {
                    self.execute(input);
                }
            }
        }
    }

    fn tour_read(&mut self) {
        narrator::blank();
        self.execute("read welcome");
        narrator::blank();
        narrator::say("To see what else is here, type: browse");
        narrator::say("To explore the wings, type: walk east wing");
    }

    fn tour_explore(&mut self) {
        narrator::blank();
        narrator::say("Let's walk through the library.");

        self.execute("browse");

        narrator::blank();
        narrator::say("The east wing holds the technical collection. Let's go there.");
        pause();

        self.execute("walk east wing");
        self.execute("browse");

        narrator::blank();
        narrator::say("The stacks have the main collection. Let's look inside.");
        pause();

        self.execute("walk stacks");
        self.execute("browse");

        narrator::blank();
        narrator::say("Try reading one — type: read garden");
        narrator::say("Or see how a document works — type: inspect garden");
        narrator::blank();
        narrator::say("You can also try: read the letter (it's in the west wing,");
        narrator::say("but you don't need to walk there — names work from anywhere).");
    }

    fn tour_write(&mut self) {
        narrator::blank();
        narrator::say("Let's add something to the library.");
        narrator::blank();
        narrator::say("The west wing has a drafts room for work in progress.");

        self.execute("walk west wing");
        self.execute("browse");

        narrator::blank();
        narrator::say("To create your first document, type:");
        narrator::say("  inscribe drafts/my-first-document");
        narrator::blank();
        narrator::say("Write anything — a note, a poem, a thought. Type .end when done.");
        narrator::say("It becomes part of the library. The journal will record it.");
        narrator::blank();
        narrator::say("To make a document that DOES something, add a Λ layer:");
        narrator::say("  read how to inscribe");
    }

    pub fn shutdown(&self) {
        narrator::blank();
        narrator::say("The lights go out. The library closes.");
        narrator::say("The journal will remember your visit.");
        library::journal_write(&self.root, "The library closed.");
    }

    /// Execute a command. Returns false if the reader wants to leave.
    pub fn execute(&mut self, input: &str) -> bool {
        self.history.push(input.to_string());

        // Handle as-archivist prefix
        let (elevated, input) = if input.starts_with("as-archivist ") {
            (true, input.strip_prefix("as-archivist ").unwrap())
        } else if input == "as-archivist" {
            self.is_archivist = !self.is_archivist;
            if self.is_archivist {
                narrator::say("You now act with the authority of the Head Archivist.");
            } else {
                narrator::say("You return to being an ordinary reader.");
            }
            return true;
        } else {
            (self.is_archivist, input)
        };

        // Handle "then" chains (pipes)
        if input.contains(" then ") {
            let parts: Vec<&str> = input.splitn(2, " then ").collect();
            self.execute(parts[0]);
            return self.execute(parts[1]);
        }

        // Parse command
        let (cmd, args) = match input.split_once(' ') {
            Some((c, a)) => (c, a.trim()),
            None => (input, ""),
        };

        match cmd {
            "walk" => self.cmd_walk(args),
            "browse" => self.cmd_browse(args),
            "read" => self.cmd_read(args),
            "glance" | "glance-at" => self.cmd_glance(args),
            "peek" | "peek-at" => self.cmd_peek(args),
            "inspect" => self.cmd_inspect(args),
            "inscribe" => self.cmd_inscribe(args),
            "revise" => self.cmd_revise(args),
            "transcribe" => self.cmd_transcribe(args),
            "reshelve" => self.cmd_reshelve(args),
            "withdraw" => self.cmd_withdraw(args, elevated),
            "open-room" => self.cmd_open_room(args),
            "close-room" => self.cmd_close_room(args),
            "search" => self.cmd_search(args),
            "scan" => self.cmd_scan(args),
            "say" => self.cmd_say(args),
            "readers" => self.cmd_readers(),
            "activity" => self.cmd_activity(),
            "inventory" => self.cmd_inventory(),
            "catalogue" => self.cmd_catalogue(),
            "ledger" => self.cmd_ledger(),
            "turn-page" => self.cmd_turn_page(),
            "annex" => self.cmd_annex(args, elevated),
            "seal" => self.cmd_seal(args, elevated),
            "classify" => self.cmd_classify(args, elevated),
            "leave" | "exit" | "quit" => return false,
            "help" | "what" | "how" | "?" => self.cmd_help(),
            "where" | "whereami" => self.cmd_where(),
            _ => {
                // Try to be helpful
                narrator::say(&format!(
                    "The library does not understand \"{input}\"."
                ));
                narrator::say("Type \"help\" to see what you can do, or \"browse\" to look around.");
            }
        }

        true
    }

    // ── Navigation ───────────────────────────────────────────────────

    fn cmd_walk(&mut self, args: &str) {
        if args.is_empty() {
            narrator::error("Walk where? Try: walk east-wing");
            return;
        }

        let target = match args {
            "back" | ".." => {
                if self.cwd == self.root {
                    narrator::say("You are already in the Entrance Hall. There is nowhere further back.");
                    return;
                }
                self.cwd.parent().unwrap_or(&self.root).to_path_buf()
            }
            "lobby" | "home" | "~" => self.root.clone(),
            _ => {
                let target = args.strip_prefix("to ").unwrap_or(args);
                if let Some(path) = self.resolve_room(target) {
                    path
                } else if self.resolve_document(target).is_some() {
                    narrator::error(&format!(
                        "\"{target}\" is a document, not a room. Try: read {target}"
                    ));
                    return;
                } else {
                    narrator::error(&format!(
                        "There is no room called \"{target}\" here."
                    ));
                    narrator::error("Try: browse (to see what's nearby)");
                    return;
                }
            }
        };

        // Check basement access
        if target.starts_with(self.root.join("basement")) && !self.is_archivist {
            narrator::say("The basement door is locked.");
            narrator::say("Only the Head Archivist may enter. Try: as-archivist walk basement");
            return;
        }

        self.cwd = target;
        let room = library::room_name(&self.cwd, &self.root);
        let desc = library::room_description(&self.cwd);
        narrator::blank();
        narrator::room_description(&room, &desc);

        library::journal_write(
            &self.root,
            &format!(
                "The reader walked to {}.",
                library::room_name(&self.cwd, &self.root)
            ),
        );
    }

    fn cmd_where(&self) {
        let room = library::room_name(&self.cwd, &self.root);
        narrator::say(&format!("You are in {room}."));
    }

    // ── Reading ──────────────────────────────────────────────────────

    fn cmd_browse(&self, args: &str) {
        let quiet = args == "-quietly";
        let target_arg = if quiet { "" } else { args };

        let target = if target_arg.is_empty() {
            self.cwd.clone()
        } else if let Some(path) = self.resolve_room(target_arg) {
            path
        } else {
            // Maybe they meant a document
            if self.resolve_document(target_arg).is_some() {
                narrator::error(&format!("\"{target_arg}\" is a document. Try: read {target_arg}"));
                return;
            }
            narrator::error(&format!("There is no room called \"{target_arg}\" here."));
            return;
        };

        let entries = library::browse(&target);
        if entries.is_empty() {
            narrator::say("The shelves here are empty.");
            return;
        }

        narrator::blank();

        for entry in &entries {
            if quiet {
                let suffix = if entry.is_dir { "/" } else { "" };
                println!("      {}{suffix}", entry.name);
            } else if entry.is_dir {
                // Show the room AND its contents (one level deep)
                narrator::shelf_entry(&entry.name, &entry.description, true);
                let sub_entries = library::browse(&target.join(&entry.name));
                for sub in &sub_entries {
                    if sub.name.starts_with('.') {
                        continue;
                    }
                    let prefix = format!("  {}/{}", entry.name, sub.name);
                    let suffix = if sub.is_dir { "/" } else { "" };
                    let dots = 38_usize.saturating_sub(prefix.len() + suffix.len());
                    let padding: String = std::iter::repeat_n('.', dots.max(2)).collect();
                    println!("        {prefix}{suffix} {padding} {}", sub.description);
                }
            } else {
                narrator::shelf_entry(&entry.name, &entry.description, false);
            }
        }
    }

    fn cmd_read(&mut self, args: &str) {
        let name = args.strip_prefix("at ").unwrap_or(args);
        if name.is_empty() {
            narrator::error("Read what? Try: read welcome");
            return;
        }

        let path = self.resolve_document(name);
        let Some(path) = path else {
            narrator::error(&format!("There is no document called \"{name}\" here."));
            return;
        };

        let Some(doc) = library::Document::load(&path) else {
            narrator::error("This document cannot be read. It may be damaged.");
            return;
        };

        narrator::blank();
        for line in doc.mu.lines() {
            narrator::say(line);
        }

        self.documents_read += 1;
        library::journal_write(
            &self.root,
            &format!("The reader read \"{}\".", doc.name),
        );

        // Execute Λ layer if present
        if !doc.lambda.is_empty() {
            let mut vars = self.make_vars();
            vars.insert("document".into(), doc.name.clone());
            let ctx = lambda::LambdaContext::new(&self.root, &path);
            let count = ctx.increment_read_count();
            vars.insert("read-count".into(), count.to_string());
            lambda::execute(&doc.lambda, &ctx, &vars);
        }
    }

    fn cmd_glance(&mut self, args: &str) {
        let name = args.strip_prefix("at ").unwrap_or(args);
        let path = self.resolve_document(name);
        let Some(path) = path else {
            narrator::error(&format!("There is no document called \"{name}\" here."));
            return;
        };

        if let Ok(content) = fs::read_to_string(&path) {
            narrator::blank();
            for line in content.lines().take(5) {
                narrator::say(line);
            }
            narrator::say("    ...");
        }
    }

    fn cmd_peek(&mut self, args: &str) {
        let name = args.strip_prefix("at ").unwrap_or(args);
        let path = self.resolve_document(name);
        let Some(path) = path else {
            narrator::error(&format!("There is no document called \"{name}\" here."));
            return;
        };

        if let Ok(content) = fs::read_to_string(&path) {
            let lines: Vec<&str> = content.lines().collect();
            let start = lines.len().saturating_sub(5);
            narrator::blank();
            narrator::say("    ...");
            for line in &lines[start..] {
                narrator::say(line);
            }
        }
    }

    fn cmd_inspect(&self, args: &str) {
        if args.is_empty() {
            narrator::error("Inspect what? Try: inspect clock");
            return;
        }

        // Handle -deep flag
        let (deep, name) = if args.starts_with("-deep ") {
            (true, args.strip_prefix("-deep ").unwrap())
        } else if args.ends_with(" -deep") {
            (true, args.strip_suffix(" -deep").unwrap())
        } else {
            (false, args)
        };

        let path = self.resolve_document(name);
        let Some(path) = path else {
            // Maybe it's a room
            if let Some(room) = self.resolve_room(name) {
                self.inspect_room(&room);
                return;
            }
            narrator::error(&format!("There is no document or room called \"{name}\" here."));
            return;
        };

        let Some(doc) = library::Document::load(&path) else {
            narrator::error("This document cannot be inspected.");
            return;
        };

        let rel = path.strip_prefix(&self.root).unwrap_or(&path);

        // Check classification
        let class_path = self.root.join(".meta").join(
            rel.to_string_lossy().replace('/', "_") + ".classification"
        );
        let classification = fs::read_to_string(&class_path)
            .unwrap_or_else(|_| if doc.is_process { "process, living document".into() } else { "document, still".into() });

        narrator::blank();
        narrator::register_header("Φ", "Phi", "Form");
        narrator::register_field("Location", &rel.to_string_lossy());
        narrator::register_field("Classification", &classification);

        let metadata = fs::metadata(&path);
        if let Ok(meta) = metadata {
            let size = meta.len();
            narrator::register_field("Size", &format!("{size} bytes"));
        }

        // Check for embedded whitespace program
        if let Ok(content) = fs::read_to_string(&path)
            && crate::embedded::has_embedded(&content) {
                narrator::register_field("Embedded", "contains a ΦΜΛ whitespace program");
            }

        narrator::blank();
        narrator::register_header("Μ", "Mu", "Message");
        let preview: String = doc.mu.lines().take(3).collect::<Vec<_>>().join("\n");
        narrator::register_field("Content", &format!("\"{}\"", preview));

        narrator::blank();
        narrator::register_header("Λ", "Lambda", "Function");
        if doc.lambda.is_empty() {
            narrator::register_field("Logic", "(none — this document is still)");
        } else if deep {
            // Deep inspection: if it looks like grid code, show the grid
            if doc.lambda.contains('Φ') || doc.lambda.contains('Μ') || doc.lambda.contains('Λ')
                || doc.lambda.contains('>') || doc.lambda.contains('@')
            {
                let grid = crate::grid::GridState::from_source(&doc.lambda);
                println!("{}", grid.display_grid());
            } else {
                for line in doc.lambda.lines() {
                    println!("      {line}");
                }
            }
        } else {
            for line in doc.lambda.lines().take(10) {
                println!("      {line}");
            }
            let total = doc.lambda.lines().count();
            if total > 10 {
                println!("      ... ({total} lines total)");
                println!("      Use: inspect {name} -deep  to see the full grid.");
            }
        }
    }

    fn inspect_room(&self, path: &Path) {
        let name = library::room_name(path, &self.root);
        let entries = library::browse(path);
        let docs = entries.iter().filter(|e| !e.is_dir).count();
        let rooms = entries.iter().filter(|e| e.is_dir).count();

        narrator::blank();
        narrator::register_header("Φ", "Phi", "Form");
        narrator::register_field("Room", &name);
        narrator::register_field("Documents", &docs.to_string());
        narrator::register_field("Sub-rooms", &rooms.to_string());

        narrator::blank();
        narrator::register_header("Μ", "Mu", "Message");
        let desc = library::room_description(path);
        narrator::register_field(
            "Description",
            if desc.is_empty() { "(no description)" } else { &desc },
        );

        narrator::blank();
        narrator::register_header("Λ", "Lambda", "Function");
        narrator::register_field("Logic", "(rooms do not execute)");
    }

    // ── Writing ──────────────────────────────────────────────────────

    fn cmd_inscribe(&mut self, args: &str) {
        if args.is_empty() {
            narrator::error("Inscribe what? Try: inscribe west-wing/drafts/my-note");
            return;
        }

        let path = if args.contains('/') {
            self.root.join(args)
        } else {
            self.cwd.join(args)
        };

        if path.exists() {
            narrator::error("A document with that name already exists. Try: revise");
            return;
        }

        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        narrator::say("An empty page opens. Write your document.");
        narrator::say("(Enter text. Type .end on a line by itself to finish.)");
        narrator::say("(To add a Λ layer, write --- Λ --- on a line, then your logic.)");
        narrator::blank();

        let content = read_multiline();
        let _ = fs::write(&path, &content);

        let name = path.file_name().unwrap_or_default().to_string_lossy();
        narrator::blank();
        narrator::say(&format!("\"{name}\" has been inscribed and placed on the shelf."));
        narrator::say("The catalogue will find it on the next round.");

        library::journal_write(
            &self.root,
            &format!("The reader inscribed \"{}\".", name),
        );
    }

    fn cmd_revise(&mut self, args: &str) {
        if args.is_empty() {
            narrator::error("Revise what?");
            return;
        }

        let path = self.resolve_document(args);
        let Some(path) = path else {
            narrator::error(&format!("There is no document called \"{args}\" here."));
            return;
        };

        let current = fs::read_to_string(&path).unwrap_or_default();
        narrator::say("The current document reads:");
        narrator::blank();
        for line in current.lines() {
            narrator::say(line);
        }
        narrator::blank();
        narrator::say("Write the revised version.");
        narrator::say("(Type .end on a line by itself to finish.)");
        narrator::blank();

        let content = read_multiline();
        let _ = fs::write(&path, &content);

        let name = path.file_name().unwrap_or_default().to_string_lossy();
        narrator::say(&format!("\"{name}\" has been revised."));

        library::journal_write(
            &self.root,
            &format!("The reader revised \"{}\".", name),
        );
    }

    fn cmd_transcribe(&self, args: &str) {
        // transcribe SOURCE to DEST
        let Some((source, dest)) = args.split_once(" to ") else {
            narrator::error("Usage: transcribe document to destination/");
            return;
        };

        let src_path = self.resolve_document(source.trim());
        let Some(src_path) = src_path else {
            narrator::error(&format!("Cannot find \"{source}\"."));
            return;
        };

        let dest_path = if dest.trim().contains('/') {
            self.root.join(dest.trim())
        } else {
            self.cwd.join(dest.trim())
        };

        let dest_final = if dest_path.is_dir() {
            dest_path.join(src_path.file_name().unwrap_or_default())
        } else {
            dest_path
        };

        match fs::copy(&src_path, &dest_final) {
            Ok(_) => narrator::say("The document has been transcribed."),
            Err(e) => narrator::error(&format!("Transcription failed: {e}")),
        }
    }

    fn cmd_reshelve(&self, args: &str) {
        let Some((source, dest)) = args.split_once(" to ") else {
            narrator::error("Usage: reshelve document to destination/");
            return;
        };

        let src_path = self.resolve_document(source.trim());
        let Some(src_path) = src_path else {
            narrator::error(&format!("Cannot find \"{source}\"."));
            return;
        };

        let dest_path = if dest.trim().contains('/') {
            self.root.join(dest.trim())
        } else {
            self.cwd.join(dest.trim())
        };

        let dest_final = if dest_path.is_dir() {
            dest_path.join(src_path.file_name().unwrap_or_default())
        } else {
            dest_path
        };

        match fs::rename(&src_path, &dest_final) {
            Ok(_) => narrator::say("The document has been reshelved."),
            Err(e) => narrator::error(&format!("Could not reshelve: {e}")),
        }
    }

    fn cmd_withdraw(&self, args: &str, elevated: bool) {
        if args.is_empty() {
            narrator::error("Withdraw what?");
            return;
        }

        let path = self.resolve_document(args);
        let Some(path) = path else {
            narrator::error(&format!("There is no document called \"{args}\" here."));
            return;
        };

        // Protect founding collection unless archivist
        if !elevated {
            let rel = path.strip_prefix(&self.root).unwrap_or(&path);
            let name = rel.to_string_lossy();
            if matches!(
                name.as_ref(),
                "welcome" | "catalogue" | "rules"
            ) {
                narrator::error("This document is part of the founding collection.");
                narrator::error("Only the Head Archivist may withdraw it.");
                return;
            }
        }

        match fs::remove_file(&path) {
            Ok(_) => {
                narrator::say(&format!("\"{args}\" has been withdrawn from the collection."));
                library::journal_write(
                    &self.root,
                    &format!("The reader withdrew \"{}\".", args),
                );
            }
            Err(e) => narrator::error(&format!("Could not withdraw: {e}")),
        }
    }

    // ── Organising ───────────────────────────────────────────────────

    fn cmd_open_room(&self, args: &str) {
        if args.is_empty() {
            narrator::error("Open which room?");
            return;
        }

        let path = self.cwd.join(args);
        match fs::create_dir_all(&path) {
            Ok(_) => narrator::say(&format!("A new room opens: \"{args}\".")),
            Err(e) => narrator::error(&format!("Could not open room: {e}")),
        }
    }

    fn cmd_close_room(&self, args: &str) {
        if args.is_empty() {
            narrator::error("Close which room?");
            return;
        }

        let path = self.cwd.join(args);
        match fs::remove_dir(&path) {
            Ok(_) => narrator::say(&format!("The room \"{args}\" has been closed.")),
            Err(_) => narrator::error("The room is not empty. Remove its contents first."),
        }
    }

    fn cmd_search(&self, args: &str) {
        if args.is_empty() {
            narrator::error("Search for what? Try: search borges");
            return;
        }

        let results = library::search(&self.root, args);
        if results.is_empty() {
            narrator::say(&format!("No documents match \"{args}\"."));
        } else {
            narrator::say(&format!("{} results:", results.len()));
            narrator::blank();
            for (path, snippet) in &results {
                narrator::shelf_entry(path, snippet, false);
            }
        }
    }

    fn cmd_scan(&self, args: &str) {
        // scan "phrase" in path/
        let args = args.trim_matches('"');
        if let Some((phrase, location)) = args.split_once("\" in ") {
            let path = self.root.join(location.trim());
            let results = library::search(&path, phrase);
            if results.is_empty() {
                narrator::say(&format!("No documents contain \"{phrase}\"."));
            } else {
                for (path, snippet) in &results {
                    narrator::shelf_entry(path, snippet, false);
                }
            }
        } else {
            // Scan in current room
            let results = library::search(&self.cwd, args);
            if results.is_empty() {
                narrator::say(&format!("No documents here contain \"{args}\"."));
            } else {
                for (path, snippet) in &results {
                    narrator::shelf_entry(path, snippet, false);
                }
            }
        }
    }

    fn cmd_catalogue(&self) {
        narrator::blank();
        narrator::say("This document lists every document in the library, including itself.");
        narrator::say("");
        narrator::say("The Collection:");
        narrator::blank();
        let mut count = 0;
        let mut rooms = 0;
        self.catalogue_walk(&self.root, &self.root, &mut count, &mut rooms);
        narrator::blank();
        narrator::say(&format!("{count} documents across {rooms} rooms."));

        // Show phantom entries
        narrator::blank();
        narrator::say("The following entries have no corresponding document:");
        narrator::blank();
        for name in &["the-unwritten", "the-remembered", "the-awaited"] {
            narrator::say(&format!("  {name} .............. (location unknown)"));
        }
    }

    fn catalogue_walk(&self, dir: &Path, root: &Path, count: &mut usize, rooms: &mut usize) {
        let Ok(entries) = fs::read_dir(dir) else { return };
        let mut sorted: Vec<_> = entries.flatten().collect();
        sorted.sort_by_key(|e| e.file_name());
        for entry in sorted {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') { continue; }
            let path = entry.path();
            if path.is_dir() {
                *rooms += 1;
                self.catalogue_walk(&path, root, count, rooms);
            } else {
                *count += 1;
                let rel = path.strip_prefix(root).unwrap_or(&path);
                let display = rel.to_string_lossy();
                // First line as description
                let desc = fs::read_to_string(&path)
                    .ok()
                    .and_then(|c| c.lines().next().map(|l| {
                        let l = l.trim();
                        if l.len() > 50 { format!("{}...", &l[..47]) } else { l.to_string() }
                    }))
                    .unwrap_or_default();
                let dots = 45_usize.saturating_sub(display.len());
                let padding: String = std::iter::repeat_n('.', dots.max(2)).collect();
                narrator::say(&format!("  {display} {padding} {desc}"));
            }
        }
    }

    // ── System ───────────────────────────────────────────────────────

    fn cmd_say(&self, args: &str) {
        // Handle "into" and "onto" redirects
        if let Some((text, path)) = args.split_once(" into ") {
            let path = self.cwd.join(path.trim());
            let text = text.trim_matches('"');
            let _ = fs::write(&path, format!("{text}\n"));
            narrator::say(&format!("Written into {}.", path.file_name().unwrap_or_default().to_string_lossy()));
        } else if let Some((text, path)) = args.split_once(" onto ") {
            let path = self.cwd.join(path.trim());
            let text = text.trim_matches('"');
            let _ = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .and_then(|mut f| writeln!(f, "{text}"));
            narrator::say(&format!("Appended onto {}.", path.file_name().unwrap_or_default().to_string_lossy()));
        } else {
            let text = args.trim_matches('"');
            narrator::say(text);
        }
    }

    fn cmd_readers(&self) {
        let sessions = crate::session::list_active(&self.root);
        let count = sessions.len();
        narrator::say(&format!("{count} reader{} present:", if count == 1 { "" } else { "s" }));
        narrator::blank();
        for (user, time, pid) in &sessions {
            let is_me = *pid == std::process::id();
            let suffix = if is_me { " (you)" } else { "" };
            narrator::say(&format!("  {user}{suffix} — arrived {time}"));
        }
        narrator::blank();
        narrator::say(&format!("You have read {} documents this session.", self.documents_read));
    }

    fn cmd_activity(&self) {
        let uptime = chrono::Local::now() - self.boot_time;
        let minutes = uptime.num_minutes();
        narrator::say(&format!(
            "The library has been open for {} minutes.",
            minutes
        ));
        narrator::say(&format!("1 reader present. {} documents read.", self.documents_read));

        narrator::say("The collection spans the east and west wings.");
    }

    fn cmd_inventory(&self) {
        // Disk space as shelf space
        #[cfg(target_os = "linux")]
        {
            // Simple: check root partition
            narrator::say("Checking shelf capacity...");
            if let Ok(output) = std::process::Command::new("df")
                .arg("-h")
                .arg(&self.root)
                .output()
            {
                let out = String::from_utf8_lossy(&output.stdout);
                if let Some(line) = out.lines().nth(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        narrator::say(&format!("Total shelf space: {}", parts[1]));
                        narrator::say(&format!("Used: {}", parts[2]));
                        narrator::say(&format!("Available: {}", parts[3]));
                    }
                }
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            narrator::say("Shelf capacity information is only available from the Fundament.");
        }
    }

    fn cmd_ledger(&self) {
        narrator::say("Your actions today:");
        narrator::blank();
        for (i, entry) in self.history.iter().enumerate() {
            println!("      {: >3}. {entry}", i + 1);
        }
    }

    fn cmd_turn_page(&self) {
        // Clear screen
        print!("\x1B[2J\x1B[H");
        let _ = io::stdout().flush();
        let room = library::room_name(&self.cwd, &self.root);
        narrator::say(&format!("A fresh page. You are in {room}."));
    }

    fn cmd_help(&self) {
        narrator::say("What you can do here:");
        narrator::blank();
        narrator::say("  Explore");
        narrator::say("    browse              look at the shelves around you");
        narrator::say("    browse east wing    look at a specific wing");
        narrator::say("    walk east wing      walk to a room");
        narrator::say("    walk back           go back to the previous room");
        narrator::say("    walk lobby          return to the entrance hall");
        narrator::say("    where               where am I?");
        narrator::blank();
        narrator::say("  Read");
        narrator::say("    read welcome        read a document");
        narrator::say("    read the letter     you don't need exact names");
        narrator::say("    read garden         partial names work too");
        narrator::say("    glance at welcome   just the first few lines");
        narrator::say("    peek at welcome     just the last few lines");
        narrator::say("    inspect welcome     see all three registers (Φ Μ Λ)");
        narrator::blank();
        narrator::say("  Write");
        narrator::say("    inscribe my-note    create a new document");
        narrator::say("    revise welcome      edit an existing document");
        narrator::say("    say hello           say something");
        narrator::say("    say hello into note write text to a document");
        narrator::blank();
        narrator::say("  Find");
        narrator::say("    search library      search all documents");
        narrator::say("    catalogue           view the master index");
        narrator::blank();
        narrator::say("  Connect");
        narrator::say("    annex /path as name attach external storage");
        narrator::say("    seal name           detach external storage");
        narrator::say("    classify level doc  set access level (public/restricted/classified)");
        narrator::blank();
        narrator::say("  Other");
        narrator::say("    ledger              everything you've done today");
        narrator::say("    turn-page           clear the screen");
        narrator::say("    as-archivist        unlock restricted areas");
        narrator::say("    leave               the lights go out");
    }

    // ── Connecting ────────────────────────────────────────────────────

    fn cmd_annex(&mut self, args: &str, elevated: bool) {
        if !elevated {
            narrator::error("Only the Head Archivist may annex external storage.");
            narrator::error("Try: as-archivist annex /path as name");
            return;
        }
        // annex /path as name
        let Some((source, name)) = args.split_once(" as ") else {
            narrator::error("Usage: annex /path/to/storage as wing-name");
            return;
        };
        let source = source.trim();
        let name = name.trim();
        let source_path = PathBuf::from(source);
        if !source_path.exists() || !source_path.is_dir() {
            narrator::error(&format!("\"{source}\" does not exist or is not a directory."));
            return;
        }
        let link_path = self.root.join("other-libraries").join(name);
        #[cfg(unix)]
        {
            let _ = std::os::unix::fs::symlink(&source_path, &link_path);
        }
        #[cfg(not(unix))]
        {
            let _ = fs::create_dir_all(&link_path);
            narrator::error("Symlinks not supported on this platform. Created empty room instead.");
            return;
        }
        self.annexed.insert(name.to_string(), source_path.clone());
        narrator::say(&format!("External storage annexed as other-libraries/{name}."));
        narrator::say(&format!("Source: {source}"));
        library::journal_write(
            &self.root,
            &format!("The archivist annexed external storage as \"{}\".", name),
        );
    }

    fn cmd_seal(&mut self, args: &str, elevated: bool) {
        if !elevated {
            narrator::error("Only the Head Archivist may seal storage.");
            return;
        }
        let name = args.trim();
        if name.is_empty() {
            narrator::error("Seal what? Usage: seal wing-name");
            return;
        }
        let link_path = self.root.join("other-libraries").join(name);
        if link_path.exists() {
            let _ = fs::remove_file(&link_path); // remove symlink
            let _ = fs::remove_dir(&link_path);  // or empty dir
            self.annexed.remove(name);
            narrator::say(&format!("other-libraries/{name} has been sealed."));
            library::journal_write(
                &self.root,
                &format!("The archivist sealed \"{}\".", name),
            );
        } else {
            narrator::error(&format!("There is no annexed storage called \"{name}\"."));
        }
    }

    fn cmd_classify(&self, args: &str, elevated: bool) {
        if !elevated {
            narrator::error("Only the Head Archivist may classify documents.");
            narrator::error("Try: as-archivist classify restricted document-name");
            return;
        }
        // classify LEVEL DOCUMENT
        let Some((level, doc_name)) = args.split_once(' ') else {
            narrator::error("Usage: classify [public|restricted|classified] document-name");
            return;
        };
        let level = level.trim();
        if !matches!(level, "public" | "restricted" | "classified") {
            narrator::error("Classification levels: public, restricted, classified");
            return;
        }
        let Some(path) = self.resolve_document(doc_name.trim()) else {
            narrator::error(&format!("Cannot find \"{}\".", doc_name.trim()));
            return;
        };
        // Store classification in .meta
        let rel = path.strip_prefix(&self.root).unwrap_or(&path);
        let meta_path = self.root.join(".meta").join(
            rel.to_string_lossy().replace('/', "_") + ".classification"
        );
        let _ = fs::create_dir_all(self.root.join(".meta"));
        let _ = fs::write(&meta_path, level);
        narrator::say(&format!("\"{}\" classified as {level}.", doc_name.trim()));
        library::journal_write(
            &self.root,
            &format!("The archivist classified \"{}\" as {}.", doc_name.trim(), level),
        );
    }

    // ── Helpers ──────────────────────────────────────────────────────

    /// Resolve a name to a path. Handles:
    ///   "east wing" → "east-wing"
    ///   "the letter" → "the-letter"
    ///   "garden" → "the-garden-of-forking-paths" (partial match)
    ///   "east-wing/stacks/garden" → partial match within path
    fn resolve_path(&self, name: &str) -> Option<PathBuf> {
        let name = name.trim();

        // Direct match (exact path)
        for base in [&self.cwd, &self.root] {
            let path = base.join(name);
            if path.exists() {
                return Some(path);
            }
        }

        // Replace spaces with hyphens: "east wing" → "east-wing"
        let hyphenated = name.replace(' ', "-");
        for base in [&self.cwd, &self.root] {
            let path = base.join(&hyphenated);
            if path.exists() {
                return Some(path);
            }
        }

        // Partial match: "garden" matches "the-garden-of-forking-paths"
        // Search in cwd, then root, then recursively
        if let Some(found) = self.fuzzy_find(name) {
            return Some(found);
        }

        None
    }

    fn resolve_document(&self, name: &str) -> Option<PathBuf> {
        self.resolve_path(name).filter(|p| p.is_file())
    }

    fn resolve_room(&self, name: &str) -> Option<PathBuf> {
        // If we're already in the room they named, return cwd
        let cwd_name = self.cwd.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let needle = name.to_lowercase().replace(' ', "-");
        if cwd_name.to_lowercase() == needle {
            return Some(self.cwd.clone());
        }
        self.resolve_path(name).filter(|p| p.is_dir())
    }

    /// Fuzzy find: search for partial name matches in cwd, root, then recursively
    fn fuzzy_find(&self, name: &str) -> Option<PathBuf> {
        let needle = name.to_lowercase().replace(' ', "-");

        // Search in cwd first
        if let Some(found) = self.fuzzy_find_in(&self.cwd, &needle) {
            return Some(found);
        }

        // Then root
        if self.cwd != self.root
            && let Some(found) = self.fuzzy_find_in(&self.root, &needle) {
                return Some(found);
            }

        // Then recursively from root
        self.fuzzy_find_recursive(&self.root, &needle)
    }

    fn fuzzy_find_in(&self, dir: &Path, needle: &str) -> Option<PathBuf> {
        let entries = fs::read_dir(dir).ok()?;
        for entry in entries.flatten() {
            let fname = entry.file_name().to_string_lossy().to_lowercase();
            if fname == *needle || fname.contains(needle) {
                return Some(entry.path());
            }
        }
        None
    }

    fn fuzzy_find_recursive(&self, dir: &Path, needle: &str) -> Option<PathBuf> {
        let entries = fs::read_dir(dir).ok()?;
        for entry in entries.flatten() {
            let fname = entry.file_name().to_string_lossy().to_lowercase();
            if fname.starts_with('.') {
                continue;
            }
            if fname == *needle || fname.contains(needle) {
                return Some(entry.path());
            }
            if entry.path().is_dir()
                && let Some(found) = self.fuzzy_find_recursive(&entry.path(), needle) {
                    return Some(found);
                }
        }
        None
    }

    fn make_vars(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        vars.insert("reader".into(), self.reader_name.clone());
        vars.insert("time".into(), chrono::Local::now().format("%H:%M:%S").to_string());
        vars.insert("date".into(), chrono::Local::now().format("%Y-%m-%d").to_string());
        vars.insert(
            "documents-read".into(),
            self.documents_read.to_string(),
        );
        vars
    }
}

fn read_multiline() -> String {
    let stdin = io::stdin();
    let mut content = String::new();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim() == ".end" {
            break;
        }
        content.push_str(&line);
        content.push('\n');
    }
    content
}

fn whoami() -> String {
    env::var("USER")
        .or_else(|_| env::var("LOGNAME"))
        .unwrap_or_else(|_| "a reader".into())
}

fn pause() {
    print!("\n    (press enter to continue)\n");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
}
