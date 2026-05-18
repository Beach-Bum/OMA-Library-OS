# The ΦΜΛ Installation and Operation Manual

*For the curious, the contemplative, and the constructive.*

---

## Preface

You are holding the manual for a library that is also an operating system. This is not a metaphor. When you boot ΦΜΛ, you are standing in an entrance hall. The shelves are real directories. The documents are real files. The journal is a real log. And the manual you are reading now is one of those documents.

This manual is written for three readers simultaneously. If you are exploring and want to know what to do next, you'll find that here. If you are thinking about what this system means, you'll find that here too. And if you are building and want to know how the machinery works, it's all here — every command, every register, every mechanism.

---

## I. Installation

### 1.1 Prerequisites

| Requirement | Details |
|-------------|---------|
| Operating system | Linux (any distribution), macOS, WSL2 |
| Rust toolchain | Install from https://rustup.rs (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh`) |
| Terminal | Any terminal emulator (kitty, alacritty, Terminal.app, Windows Terminal, Termius on iOS) |
| Disk space | ~50MB (build), ~1MB (binary), ~1MB (library) |
| Architecture | x86_64, aarch64 (Raspberry Pi), arm7 |

### 1.2 Build from source

```bash
git clone https://github.com/Beach-Bum/phi-mu-lambda.git
cd phi-mu-lambda
cargo build --release
```

Output: `target/release/oma` (~800KB, statically linked)

### 1.3 Install

```bash
# Option A: copy to user bin
cp target/release/oma ~/.local/bin/

# Option B: install system-wide
sudo cp target/release/oma /usr/local/bin/

# Option C: run from build directory
./target/release/oma
```

### 1.4 First boot

```bash
oma
```

Creates library at `~/oma-library/` with the founding collection (12 documents + rules).

### 1.5 Configuration

| Environment variable | Default | Purpose |
|---------------------|---------|---------|
| `OMA_ROOT` | `~/oma-library` | Library location |
| `USER` / `LOGNAME` | system username | Reader name shown in journal |

```bash
# Custom library location
OMA_ROOT=/mnt/usb/my-library oma

# Multiple independent libraries
OMA_ROOT=~/library-one oma
OMA_ROOT=~/library-two oma
```

### 1.6 Uninstall

```bash
rm ~/.local/bin/oma      # remove binary
rm -rf ~/oma-library     # remove library (irreversible)
```

There is no configuration stored outside these two locations. No system files. No registry. No hidden state.

---

## II. The Three Registers

Every object in ΦΜΛ carries three registers.

| Register | Symbol | Name | Contains | Unix equivalent |
|----------|--------|------|----------|-----------------|
| **Φ** | Phi | Form | Location, classification, structure | File path, permissions, metadata |
| **Μ** | Mu | Message | Visible text, human-readable content | File content (above separator) |
| **Λ** | Lambda | Lambda | Executable logic, margin script | File content (below `--- Λ ---` separator) |

### 2.1 Viewing registers

| Command | Registers shown | What you see |
|---------|----------------|-------------|
| `read document` | Μ only | The text, then Λ executes silently |
| `inspect document` | Φ + Μ + Λ | Location, classification, content preview, full logic source |
| `glance at document` | Μ (first 5 lines) | Quick preview |
| `peek at document` | Μ (last 5 lines) | End of document |

### 2.2 Document file format

A document is a plain text file. The Λ layer is separated by `--- Λ ---`:

```
This is the visible text (Μ layer).
The reader sees this when they "read" the document.

--- Λ ---
say "This executes when the document is read."
set visits ← read-count
write west-wing/journal/{date} "{time} Document was read."
```

Documents without a `--- Λ ---` separator have no Λ layer. They are inert — text only.

### 2.3 Register philosophy

**For the contemplative:** The three registers map to Ranganathan's faceted classification — every record has Form (what kind of thing it is), Message (what it contains), and Function (what it does in the system). Φ without Μ is a blank page. Μ without Λ is a letter. All three together is a living document. The registers are not metadata about the document — they ARE the document.

**For the constructive:** Φ is `stat()` + path. Μ is file content above separator. Λ is file content below separator, interpreted line-by-line by the Λ engine. The separator is the literal string `\n--- Λ ---\n`. Files without this separator have empty Λ. The `inspect` command is `stat` + `head` + showing the Λ source.

---

## III. Command Reference

### 3.1 Navigation

#### `walk`

Move to a room (directory).

**Syntax:**
```
walk <destination>
walk back
walk lobby
```

**Arguments:**

| Argument | Effect |
|----------|--------|
| `<name>` | Walk to a room by name. Fuzzy-matched. |
| `back` or `..` | Return to previous room (parent directory) |
| `lobby` or `home` or `~` | Return to entrance hall (library root) |
| `to <name>` | Same as `walk <name>` (natural language variant) |

**Name resolution:** Names are matched in this order:
1. Exact match in current room
2. Exact match from library root
3. Space-to-hyphen conversion (`east wing` → `east-wing`)
4. Partial/substring match in current room
5. Partial match from library root
6. Recursive search through all rooms

**Examples:**
```
> walk east wing          walks to /east-wing
> walk stacks             walks to east-wing/stacks (if in east-wing)
> walk correspondence     walks to west-wing/correspondence (from anywhere)
> walk back               returns to parent room
> walk lobby              returns to entrance hall
```

**Errors:**
- `"There is no room called X here."` — no match found
- `"X is a document, not a room. Try: read X"` — matched a file, not a directory
- `"The basement door is locked."` — requires `as-archivist` elevation

**Unix equivalent:** `cd`

---

#### `browse`

Look at the shelves — list contents of current or specified room.

**Syntax:**
```
browse
browse <room>
browse -quietly
```

**Arguments:**

| Argument | Effect |
|----------|--------|
| (none) | Browse current room |
| `<room>` | Browse a specific room (fuzzy-matched) |
| `-quietly` | Names only, no descriptions |

**Output format:**
```
name/ .......................... room description (directories)
    name/sub-item .............. sub-item description (nested one level)
name ........................... first line of document (files)
```

Rooms show one level of nesting — you see the documents inside sub-rooms without having to walk into them.

**Examples:**
```
> browse                  current room contents
> browse east wing        east wing contents + nested documents
> browse -quietly         names only
```

**Unix equivalent:** `ls -la` with recursive depth 1 for directories

---

#### `where`

Show your current location.

**Syntax:**
```
where
```

**Output:** `"You are in the East Wing — Technical Collection."`

**Unix equivalent:** `pwd`

---

### 3.2 Reading

#### `read`

Read a document. Shows the Μ (Message) layer. If the document has a Λ layer, it executes after display.

**Syntax:**
```
read <document>
read at <document>
```

**Arguments:**

| Argument | Effect |
|----------|--------|
| `<name>` | Document name (fuzzy-matched) |
| `at <name>` | Same (natural language variant) |

**Name resolution:** Same fuzzy matching as `walk` — spaces become hyphens, partial names work, recursive search from any location.

**Behaviour:**
1. Display full Μ (Message) content
2. Increment read counter (stored in `.meta/`)
3. Execute Λ (Lambda) layer if present
4. Write journal entry

**Examples:**
```
> read welcome            reads the welcome document
> read the letter         fuzzy → west-wing/correspondence/the-letter
> read garden             fuzzy → east-wing/stacks/the-garden-of-forking-paths
> read maintenance        fuzzy → on-the-maintenance-of-shelves
```

**Side effects:** Living documents may modify themselves (`erode self`), create new documents (`inscribe`), delete themselves (`withdraw`), or write to the journal.

**Unix equivalent:** `cat` + executing embedded script

---

#### `glance`

Read only the first 5 lines of a document.

**Syntax:**
```
glance at <document>
glance <document>
```

**Does not** execute the Λ layer. Does not increment read counter.

**Unix equivalent:** `head -5`

---

#### `peek`

Read only the last 5 lines of a document.

**Syntax:**
```
peek at <document>
peek <document>
```

**Does not** execute the Λ layer. Does not increment read counter.

**Unix equivalent:** `tail -5`

---

#### `inspect`

Show all three registers of a document or room.

**Syntax:**
```
inspect <document>
inspect <room>
```

**Output for documents:**
```
Φ (Form)
  Location: east-wing/stacks/the-erosion
  Classification: process, living document
  Size: 847 bytes

Μ (Message)
  Content: "This story loses a line each time..."

Λ (Lambda)
  set visits ← read-count
  if visits > 1:
    erode self 1
```

**Output for rooms:**
```
Φ (Form)
  Room: the Stacks (East Wing)
  Documents: 7
  Sub-rooms: 0

Μ (Message)
  Description: "The main collection. Documents on every subject."

Λ (Lambda)
  Logic: (rooms do not execute)
```

**Does not** execute the Λ layer. Shows source only.

**Unix equivalent:** `stat` + `cat` + `head`

---

### 3.3 Writing

#### `inscribe`

Create a new document. Opens an interactive editor.

**Syntax:**
```
inscribe <name>
inscribe <path/name>
```

**Arguments:**

| Argument | Effect |
|----------|--------|
| `<name>` | Create in current room |
| `<path/name>` | Create at specific location (directories created automatically) |

**Editor:**
- Type text freely (Μ layer)
- Type `--- Λ ---` on a line to begin the Λ layer
- Type `.end` on a line to save and close

**Example:**
```
> inscribe east-wing/utilities/counter

This document counts from 1 to 5.

--- Λ ---
say "1... 2... 3... 4... 5."

.end
```

**Errors:**
- `"A document with that name already exists."` — use `revise` instead

**Unix equivalent:** `cat > file` with heredoc

---

#### `revise`

Edit an existing document. Shows current content, then opens editor for replacement.

**Syntax:**
```
revise <document>
```

**Behaviour:**
1. Display current content
2. Open editor for new content
3. Type `.end` to save

**Note:** Revising a document replaces its entire content (both Μ and Λ layers).

**Unix equivalent:** `$EDITOR file`

---

#### `transcribe`

Copy a document.

**Syntax:**
```
transcribe <source> to <destination>
```

**Examples:**
```
> transcribe welcome to west-wing/drafts/
> transcribe the letter to other-libraries/friend/acquisitions/
```

**Unix equivalent:** `cp`

---

#### `reshelve`

Move a document to a new location.

**Syntax:**
```
reshelve <source> to <destination>
```

**Examples:**
```
> reshelve drafts/idea to east-wing/stacks/
```

**Unix equivalent:** `mv`

---

#### `withdraw`

Remove a document from the collection.

**Syntax:**
```
withdraw <document>
```

**Restrictions:**
- Founding collection documents (welcome, catalogue, rules) require `as-archivist` elevation
- Journal files cannot be withdrawn
- Withdrawal is recorded in the journal

**Unix equivalent:** `rm`

---

#### `say`

Output text, optionally to a file.

**Syntax:**
```
say <text>
say <text> into <document>
say <text> onto <document>
```

**Arguments:**

| Variant | Effect |
|---------|--------|
| `say hello` | Print "hello" to screen |
| `say hello into note` | Write "hello" to file (overwrites) |
| `say hello onto note` | Append "hello" to file |

**Unix equivalent:** `echo`, `echo >`, `echo >>`

---

### 3.4 Organisation

#### `open-room`

Create a new room (directory).

**Syntax:**
```
open-room <name>
```

**Unix equivalent:** `mkdir`

---

#### `close-room`

Remove an empty room.

**Syntax:**
```
close-room <name>
```

**Error:** `"The room is not empty."` — withdraw contents first.

**Unix equivalent:** `rmdir`

---

#### `classify`

Change access classification of a document.

**Syntax:**
```
classify <level> <document>
```

*Currently not fully implemented. Planned: restricted, public, archivist-only.*

**Unix equivalent:** `chmod`

---

### 3.5 Search

#### `search`

Search all documents in the library by content or name.

**Syntax:**
```
search <query>
```

**Behaviour:** Searches document names and content recursively from library root. Case-insensitive.

**Output:**
```
3 results:
  path/to/document ......... matching line from document
  path/to/another .......... matching line
```

**Unix equivalent:** `grep -r`

---

#### `scan`

Search documents for a phrase, optionally within a specific room.

**Syntax:**
```
scan <phrase>
scan "<phrase>" in <room>/
```

**Examples:**
```
> scan library                     search current room
> scan "infinite" in east-wing/    search specific room
```

**Unix equivalent:** `grep -r "phrase" path/`

---

#### `catalogue`

View the master catalogue (reads the `catalogue` document).

**Syntax:**
```
catalogue
```

---

### 3.6 System

#### `readers`

Show active readers (sessions) and statistics.

**Syntax:**
```
readers
```

**Output:**
```
1 reader present: ned (you)
Documents read this session: 7
```

**Unix equivalent:** `who` + `w`

---

#### `activity`

Show library activity summary.

**Syntax:**
```
activity
```

**Output:**
```
The library has been open for 47 minutes.
1 reader present. 7 documents read.
```

**Unix equivalent:** `uptime` + `w`

---

#### `inventory`

Show available shelf space (disk usage).

**Syntax:**
```
inventory
```

**Output:**
```
Total shelf space: 29G
Used: 1.2M
Available: 28G
```

**Unix equivalent:** `df -h`

---

#### `ledger`

Show all commands entered this session.

**Syntax:**
```
ledger
```

**Output:**
```
  1. read welcome
  2. walk east wing
  3. browse
  4. read garden
```

**Unix equivalent:** `history`

---

#### `turn-page`

Clear the screen.

**Syntax:**
```
turn-page
```

**Unix equivalent:** `clear`

---

#### `as-archivist`

Toggle Head Archivist (root) elevation. Required for: basement access, withdrawing founding documents, revising system rules.

**Syntax:**
```
as-archivist                       toggle elevation on/off
as-archivist <command>             run one command elevated
```

**Examples:**
```
> as-archivist walk basement       enter the basement
> as-archivist revise rules        edit the library rules
> as-archivist                     toggle (stays elevated until toggled off)
```

**Unix equivalent:** `sudo`

---

#### `leave`

Exit the library. Aliases: `exit`, `quit`.

**Syntax:**
```
leave
```

**Output:**
```
The lights go out. The library closes.
The journal will remember your visit.
```

**Unix equivalent:** `exit`

---

#### `help`

Show available commands grouped by activity.

**Syntax:**
```
help
```

Also triggered by: `what`, `how`, `?`

---

### 3.7 Operators

| Operator | Syntax | Effect | Unix equivalent |
|----------|--------|--------|-----------------|
| then | `read X then scan "word"` | Pipe output to next command | `\|` |
| into | `say text into file` | Write to file (overwrite) | `>` |
| onto | `say text onto file` | Append to file | `>>` |

---

## IV. The Λ Language Reference

### 4.1 Overview

Λ is the scripting language embedded in document margins. It executes when a document is `read`. Scripts are line-based, sequential, with simple indentation for blocks.

### 4.2 Instruction set

#### Output

| Instruction | Syntax | Effect |
|-------------|--------|--------|
| `say` | `say "text"` | Print text to reader. Supports `{variable}` substitution. |
| `write` | `write path "text"` | Append a line to a file. Creates parent directories. |

#### Variables

| Instruction | Syntax | Effect |
|-------------|--------|--------|
| `set` | `set name "value"` | Set a variable to a literal value. |
| `set` | `set name ← expression` | Set a variable from an expression (see §4.3). |

#### Control flow

| Instruction | Syntax | Effect |
|-------------|--------|--------|
| `if` | `if condition:` | Execute indented block if condition is true (see §4.4). |
| `wait` | `wait Ns` | Pause execution for N seconds. |

#### File operations

| Instruction | Syntax | Effect |
|-------------|--------|--------|
| `inscribe` | `inscribe path` | Create a file. Following indented lines become content. |
| `withdraw` | `withdraw path` | Delete a file. |

#### Self-modification

| Instruction | Syntax | Effect |
|-------------|--------|--------|
| `erode self` | `erode self N` | Remove N lines from end of this document's Μ layer. |
| `mutate self` | `mutate self "old" "new"` | Replace first occurrence of "old" with "new" in Μ layer. |

### 4.3 Expressions (right-hand side of `set name ← expr`)

| Expression | Returns | Example |
|------------|---------|---------|
| `count path/*` | Number of items in directory | `set n ← count east-wing/stacks/*` |
| `read-count` | Times this document has been read | `set visits ← read-count` |
| `random-line path` | A random non-empty line from a file | `set line ← random-line welcome` |
| `random-choice path/` | A random filename from a directory | `set pick ← random-choice east-wing/stacks/` |
| `ask "prompt"` | Reader's typed input | `set answer ← ask "What is your name?"` |
| `"literal"` | The literal string | `set greeting ← "hello"` |

### 4.4 Conditions (inside `if condition:`)

| Form | True when |
|------|-----------|
| `var > N` | Variable is numerically greater than N |
| `var < N` | Variable is numerically less than N |
| `var == "value"` | Variable equals the string |
| `var` | Variable is non-empty and not "0" or "false" |

### 4.5 Built-in variables

Available in any Λ script via `{name}` substitution:

| Variable | Contains |
|----------|----------|
| `{reader}` | Current reader's username |
| `{time}` | Current time (HH:MM:SS) |
| `{date}` | Current date (YYYY-MM-DD) |
| `{documents-read}` | Documents read this session |
| `{read-count}` | Times THIS document has been read |
| `{document}` | Name of the current document |

### 4.6 Comments

Lines starting with `#` or `//` are ignored.

### 4.7 Example: a complete living document

```
A greeting that remembers you.

--- Λ ---
# Track visits
set visits ← read-count

# First-time greeting
if visits == "1":
  say "Welcome, {reader}. This is your first visit."
  write west-wing/journal/{date} "{time} New reader: {reader}"

# Returning greeting
if visits > 1:
  say "Welcome back, {reader}. Visit number {visits}."

# Interactive
set mood ← ask "How are you today?"
say "The library notes: {reader} is {mood}."
write west-wing/journal/{date} "{time} {reader} reports: {mood}"
```

---

## V. The Library Filesystem

### 5.1 Floor plan

```
/ (Entrance Hall)
├── welcome                            the first document
├── catalogue                          the self-referencing index
├── rules                              how the library operates
│
├── east-wing/                         Technical collection
│   ├── stacks/                        The main shelves
│   │   ├── the-garden-of-forking-paths
│   │   ├── on-the-maintenance-of-shelves
│   │   ├── how-to-inscribe
│   │   ├── the-book-of-sand           (generative — different each read)
│   │   ├── the-erosion                (loses a line each read)
│   │   ├── the-confession             (read-once — self-deleting)
│   │   └── lesson-1-what-is-a-document (creates lesson-2 when read)
│   ├── processes/                     Living documents (daemons)
│   │   ├── dormant                    (waiting for a condition)
│   │   └── the-dreamer                (generates dream documents)
│   ├── devices/                       Building infrastructure
│   ├── utilities/                     Reader-created tools
│   └── networking/                    Connections to other libraries
│
├── west-wing/                         Letters & ephemera
│   ├── correspondence/                Letters
│   │   ├── the-letter                 (from archivist "A.")
│   │   └── the-mirror-letter          (knows your visit count)
│   ├── drafts/                        Work in progress
│   ├── journal/                       System diary (one file per day)
│   └── ephemera/                      Temporary (dreams, notes)
│
├── basement/                          Restricted (as-archivist)
│   ├── fundament/                     Kernel, hardware, CPU, memory
│   ├── blueprints/                    System configuration
│   └── vault/                         Archivist's private collection
│
├── acquisitions/                      Newly arrived, unsorted
├── reading-room/                      Active sessions
├── other-libraries/                   External/mounted storage
└── .meta/                             Read counters (hidden)
```

### 5.2 Room descriptions

Each room has a `.room-description` file (hidden) that provides the narrative text shown when you enter.

### 5.3 The journal

| Path | Content |
|------|---------|
| `west-wing/journal/YYYY-MM-DD` | One file per day, appended |

Format: `HH:MM  Event description in prose.`

The journal is append-only by convention. Λ scripts write to it with `write west-wing/journal/{date} "text"`.

### 5.4 Read counters

| Path | Content |
|------|---------|
| `.meta/<path_underscored>.reads` | Integer: number of times read |

Read counters are stored in a hidden `.meta/` directory at the library root. Counters persist across sessions.

---

## VI. The Founding Collection

Twelve documents placed by the archivist at the library's creation. Each works on three levels.

| # | Document | Location | Type | Key feature |
|---|----------|----------|------|-------------|
| 1 | welcome | Entrance Hall | still | Teaches 5 commands |
| 2 | catalogue | Entrance Hall | still | Self-referencing index |
| 3 | rules | Entrance Hall | still | Editable system policy |
| 4 | the-garden-of-forking-paths | east-wing/stacks | living | The fork: library/book/program |
| 5 | on-the-maintenance-of-shelves | east-wing/stacks | still | Librarian daemon documentation |
| 6 | how-to-inscribe | east-wing/stacks | still | Λ tutorial — the manual for writing |
| 7 | the-book-of-sand | east-wing/stacks | living | Generative text from library contents |
| 8 | the-erosion | east-wing/stacks | living | Loses a line each reading |
| 9 | the-confession | east-wing/stacks | living | Read-once, self-deleting |
| 10 | lesson-1 | east-wing/stacks | living | Creates lesson-2 when read |
| 11 | the-letter | west-wing/correspondence | living | From archivist "A." — mentions phantom entries |
| 12 | the-mirror-letter | west-wing/correspondence | living | Tracks and addresses the reader |
| — | dormant | east-wing/processes | inert | Waiting for a condition |
| — | the-dreamer | east-wing/processes | living | Generates dream documents in ephemera/ |

---

## VII. Connecting Libraries

### 7.1 Fork a library

```bash
cp -r ~/oma-library ~/oma-library-fork
OMA_ROOT=~/oma-library-fork oma
```

Or to a USB stick:
```bash
cp -r ~/oma-library /media/usb/oma-library
```

The fork contains everything: documents, journal, read counters, room structure. From the moment of forking, the libraries diverge.

### 7.2 Mail between libraries

Mount a shared directory:
```
> as-archivist
> annex /media/shared as other-libraries/friend
```

Send a document:
```
> transcribe west-wing/correspondence/hello to other-libraries/friend/acquisitions/
```

The document appears in the other library's acquisitions room.

### 7.3 Shared folders

Any directory can be annexed:
```
> annex /path/to/folder as other-libraries/name
> seal name                                        # detach
```

NFS, Syncthing, USB, SMB — any shared filesystem works. No protocol needed. Documents are files. Sharing is copying.

---

## VIII. Administration

### 8.1 The Fundament

```
> as-archivist walk basement/fundament
> browse
    cpu ................. the building's engine
    memory .............. shelf capacity
    temperature ......... how warm the building is
    storage ............. physical shelves
    kernel-log .......... the building's diary
```

*Note: Fundament documents are not yet auto-populated. Future versions will map /proc and /sys.*

### 8.2 Editing rules

```
> as-archivist revise rules
```

The rules document governs library policy. Changing it changes the system's behaviour (when the librarian daemon is implemented).

### 8.3 Backup

```bash
# Backup entire library
tar czf oma-backup-$(date +%Y%m%d).tar.gz ~/oma-library/

# Restore
tar xzf oma-backup-20260518.tar.gz -C ~/
```

### 8.4 Reset library

```bash
rm -rf ~/oma-library
oma    # creates fresh founding collection
```

---

## IX. Unix Equivalence Table

Complete mapping between ΦΜΛ and Unix commands.

| ΦΜΛ command | Unix equivalent | Notes |
|-------------|----------------|-------|
| `walk <dir>` | `cd <dir>` | Fuzzy matching, no exact path needed |
| `walk back` | `cd ..` | |
| `walk lobby` | `cd ~` | Returns to library root |
| `browse` | `ls -la` | Shows nested contents one level deep |
| `browse -quietly` | `ls` | Names only |
| `read <file>` | `cat <file>` | Also executes Λ layer |
| `glance at <file>` | `head -5 <file>` | No Λ execution |
| `peek at <file>` | `tail -5 <file>` | No Λ execution |
| `inspect <file>` | `stat <file>; cat <file>` | Shows all three registers |
| `inscribe <file>` | `cat > <file>` | Interactive editor with `.end` |
| `revise <file>` | `$EDITOR <file>` | Shows current, then replaces |
| `transcribe A to B` | `cp A B` | |
| `reshelve A to B` | `mv A B` | |
| `withdraw <file>` | `rm <file>` | Journal records it |
| `open-room <dir>` | `mkdir <dir>` | |
| `close-room <dir>` | `rmdir <dir>` | Must be empty |
| `search <query>` | `grep -ri <query> .` | Recursive from root |
| `scan <phrase>` | `grep -r <phrase> .` | In current room |
| `say text` | `echo text` | |
| `say text into file` | `echo text > file` | |
| `say text onto file` | `echo text >> file` | |
| `read A then scan X` | `cat A \| grep X` | |
| `readers` | `who; w` | |
| `activity` | `uptime` | |
| `inventory` | `df -h` | |
| `ledger` | `history` | |
| `turn-page` | `clear` | |
| `as-archivist` | `sudo` | Toggle, not per-command |
| `annex /dev/X as Y` | `mount /dev/X Y` | |
| `seal Y` | `umount Y` | |
| `where` | `pwd` | |
| `catalogue` | `locate` / `find` | Full-text search index |
| `leave` | `exit` | |
| `help` / `?` | `man` | Also: `what`, `how` |

---

## X. Emergent Behaviours

Properties that arise from documents being simultaneously text and executable logic.

| Behaviour | How it works | Example document |
|-----------|-------------|-----------------|
| Self-eroding documents | Λ removes lines from own Μ on each read | `the-erosion` |
| Self-deleting documents | Λ calls `withdraw` on its own path | `the-confession` |
| Self-replicating curriculum | Λ calls `inscribe` to create the next lesson | `lesson-1` |
| Library dreaming | Λ reads random fragments, inscribes new documents | `the-dreamer` |
| Generative text | Λ reads random lines from other documents | `the-book-of-sand` |
| Personalised letters | Λ reads visit count and reader name | `the-mirror-letter` |
| Phantom catalogue entries | Document deleted but catalogue retains entry until re-index | `the-confession` (after read) |
| Divergent libraries | Fork a library, both evolve independently | `cp -r` the root |
| Inter-library mail | Shared mount as `other-libraries/`, transcribe documents | `annex` + `transcribe` |
| Self-modifying rules | `revise rules` changes system behaviour | `rules` document |

---

## Appendix A: Building for Raspberry Pi

```bash
# Cross-compile for ARM
rustup target add aarch64-unknown-linux-musl
cargo build --release --target aarch64-unknown-linux-musl

# Copy to Pi
scp target/aarch64-unknown-linux-musl/release/oma pi@raspberrypi:~/
```

For a standalone boot image (Buildroot), see `BRIEF.md` §Technical Architecture.

---

## Appendix B: Glossary

| Term | In the library | In Unix | In philosophy |
|------|---------------|---------|---------------|
| Document | A file with up to three registers | A text file | A record with provenance |
| Room | A directory containing documents | A directory | A classification facet |
| Wing | A top-level section of the library | A top-level directory | An ontological category |
| Shelf | The visible listing of a room | Directory listing | The visible arrangement |
| The Fundament | The Linux kernel | The kernel | The substrate of reality |
| The Journal | System activity log | syslog | The finding aid |
| The Catalogue | Full-text search index | locate database | The meta-document |
| The Archivist | Root user | root / sudo | The epistemic authority |
| The Librarian | Background maintenance daemon | cron + janitor | The maintenance of order |
| Founding collection | Initial documents | Default config files | The canonical texts |
| Λ layer | Executable margin script | Embedded shell script | Agency |
| Μ layer | Visible text content | File content | Message |
| Φ layer | File metadata + location | Path + stat | Form / structure |
| Ephemera | Temporary files | /tmp | The impermanent |
| Acquisitions | Incoming unsorted | /tmp or ~/Downloads | The unclassified |
| Read counter | Per-document access count | Access time | The trace of attention |

---

*This manual is document number 13 in the founding collection. It does not have a Λ layer. It does not need one. The manual is not the tool. It is the map — and the map, as Borges noted, is never the territory.*
