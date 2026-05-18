# The ΦΜΛ Installation and Operation Manual

*For the curious, the contemplative, and the constructive.*

---

## Preface

You are holding the manual for a library that is also an operating system. This is not a metaphor. When you boot ΦΜΛ, you are standing in an entrance hall. The shelves are real directories. The documents are real files. The journal is a real log. And the manual you are reading now is one of those documents.

This manual is written for three readers simultaneously. If you are exploring and want to know what to do next, you'll find that here. If you are thinking about what this system means, you'll find that here too. And if you are building and want to know how the machinery works, it's all here — every command, every register, every mechanism.

You do not need to read this manual in order. You do not need to read it at all. The library is patient and will teach you by letting you wander. But if you want the map before you walk, this is it.

---

## I. Installation

### What you need

- A computer. Any computer. A Raspberry Pi, a laptop, an old desktop.
- Rust installed (https://rustup.rs — one command).
- A terminal.

### How to install

```
git clone https://github.com/Beach-Bum/phi-mu-lambda.git
cd phi-mu-lambda
cargo build --release
```

This produces a single binary: `target/release/oma` (under 1MB).

Move it somewhere convenient:

```
cp target/release/oma ~/.local/bin/
```

### First boot

```
oma
```

That's it. The library creates itself at `~/oma-library/`. The shelves are stocked with the founding collection. A document rests on the desk. You're in.

To put the library somewhere specific:

```
OMA_ROOT=/path/to/my/library oma
```

Every library is independent. Create as many as you like. Each one diverges from the moment of creation.

### What "installation" means

Most software installs itself into your system — it puts files in system directories, registers services, modifies configuration. ΦΜΛ does none of this. The entire system is:

1. One binary (`oma`)
2. One directory (your library)

Delete the binary and the library is inert text files. Delete the library and the binary creates a new one. Neither depends on anything except a terminal and a Linux kernel underneath.

The kernel is called **the Fundament** — the building that houses the library. You can visit it (walk to the basement) but you cannot change it from inside. It handles hardware, memory, electricity. The library handles everything you see.

---

## II. Entering the Library

When you start ΦΜΛ for the first time, you see:

```
╔═══════════════════════════════╗
║    THE OPEN MANUAL ARCHIVE    ║
╚═══════════════════════════════╝

You are standing in the Entrance Hall.
The lights are on. The shelves are full.
A document rests on the desk: "welcome"

How would you like to begin?

  1  Read the welcome letter
  2  Explore the library
  3  Start writing

Type a number, or just start typing commands.
```

If you pick **1**, you read the welcome letter — it teaches you the five core commands and tells you where to go. If you pick **2**, you're taken on a guided walk through the wings and shown what's on the shelves. If you pick **3**, you're walked to the drafts room and shown how to inscribe your first document.

Or type any command. The tour is a door, not a wall.

On subsequent visits there is no tour. You're simply in the entrance hall. The library remembers you through the journal.

---

## III. The Three Registers

Every object in ΦΜΛ has three registers. A document, a room, the system itself. Always three.

**Φ (Form)** — Where something is. Its classification. Its place on the shelves. Its structural relationship to other documents. Φ is the skeleton.

**Μ (Message)** — What something says. The visible text. What you see when you `read` a document. Μ is the flesh.

**Λ (Lambda)** — What something does. Logic that executes when the document is read. Λ is the spirit.

To see only the message: `read document-name`
To see all three: `inspect document-name`

```
> inspect the-erosion

    Φ (Form)
      Location: east-wing/stacks/the-erosion
      Classification: process, living document
      Size: 847 bytes

    Μ (Message)
      Content: "This story loses a line each time it is read..."

    Λ (Lambda)
      set visits ← read-count
      if visits > 1:
        erode self 1
        say "(This document has eroded...)"
```

Not every document has all three. A simple note has Φ and Μ but no Λ — it sits on a shelf and says something but does nothing. A blank page has only Φ — a place with nothing to say. The richest documents have all three and are, in a sense, alive.

**For the contemplative:** The three registers map to Ranganathan's insight that every act of information management has three phases — you must first decide where something goes (Φ), then describe what it is (Μ), then determine what it does (Λ). Skipping any phase produces an incomplete record. An unclassified document is lost even if it exists. An undescribed document is invisible even if it's shelved. An inert document is dead even if it's read.

**For the constructive:** Φ is the filesystem path. Μ is the file content above the `--- Λ ---` separator. Λ is the script below the separator. The `inspect` command reads file metadata (stat), displays the first few lines of content, and shows the Λ source. There is no compilation step — Λ scripts are interpreted line by line when the document is read.

---

## IV. Moving Through the Library

The library is a building with rooms. You move between them.

| Command | What it does |
|---------|-------------|
| `walk east wing` | Go to the east wing |
| `walk stacks` | Go to the stacks (if visible from here) |
| `walk back` | Return to the previous room |
| `walk lobby` | Return to the entrance hall |
| `where` | Where am I right now? |

You don't need exact names. "east wing" and "east-wing" both work. "correspondence" finds `west-wing/correspondence` from anywhere. The library figures out what you mean.

### The floor plan

```
Entrance Hall
├── east-wing/          Technical collection
│   ├── stacks/         The main shelves (founding books live here)
│   ├── processes/      Living documents (daemons, active programs)
│   ├── devices/        The building's infrastructure
│   ├── utilities/      Tools you build
│   └── networking/     Connections to other libraries
│
├── west-wing/          Letters, ephemera, personal
│   ├── correspondence/ Letters sent and received
│   ├── drafts/         Work in progress
│   ├── journal/        The library's diary (one file per day)
│   └── ephemera/       Temporary documents (they expire)
│
├── basement/           Restricted (requires as-archivist)
│   ├── fundament/      The building itself (hardware, kernel)
│   ├── blueprints/     System configuration
│   └── vault/          The archivist's private collection
│
├── acquisitions/       Newly arrived, unsorted
├── reading-room/       Active sessions
└── other-libraries/    Mounted external storage
```

**For the contemplative:** The floor plan is a classification scheme. East is technical (things that do). West is personal (things that remember). The basement is structural (things that are). This is not arbitrary — it reflects the three registers. East is Λ-heavy. West is Μ-heavy. The basement is Φ-heavy. The entrance hall is where all three meet.

**For the constructive:** Rooms are directories. `walk` is `cd` with fuzzy matching. `browse` is `ls` with nested preview. The floor plan is created by `founding.rs` on first boot. You can `open-room` to create new directories and `close-room` to remove empty ones. The hierarchy is convention, not enforced — put anything anywhere.

---

## V. Reading

| Command | What it does |
|---------|-------------|
| `read welcome` | Read a document |
| `read the letter` | Fuzzy match — finds it in west-wing/correspondence |
| `read garden` | Partial match — finds the-garden-of-forking-paths |
| `glance at welcome` | First five lines only |
| `peek at welcome` | Last five lines only |
| `inspect welcome` | All three registers (Φ Μ Λ) |

When you read a document with a Λ layer, the text appears first, then the logic executes. You see the story, then the story does something. The journal records every reading.

### Living documents

Some documents change when you read them:

- **The erosion** loses a line each time. Read it enough and only the title remains.
- **The mirror letter** knows how many times you've visited and addresses you personally.
- **The book of sand** generates new text from the library's contents each time. No two readings are the same.
- **The confession** can only be read once. It deletes itself after. The catalogue remembers it existed.
- **Lesson 1** creates Lesson 2 on the shelf the first time you read it. The curriculum writes itself.
- **The dreamer** creates new documents in ephemera/ — the library dreams.

**For the contemplative:** A document that changes when read is not a document in the traditional archival sense. It is an event. The "original" exists only in the initial state — every subsequent reading produces a new text from the same source. This is Borges' Book of Sand made operational: you cannot return to the page you read before, because the page no longer exists.

**For the constructive:** Living documents use the Λ engine's `read-count` tracking, `erode self`, `mutate self`, `inscribe`, and `withdraw` operations. Read counts are stored in `.meta/` as simple counters. Self-modification rewrites the file's own Μ section while preserving the Λ section. The `withdraw` operation is `fs::remove_file`.

---

## VI. Writing

| Command | What it does |
|---------|-------------|
| `inscribe my-note` | Create a new document in current room |
| `inscribe west-wing/drafts/idea` | Create at a specific location |
| `revise welcome` | Edit an existing document |
| `say hello` | Print text to the screen |
| `say hello into note` | Write text to a file |
| `say more onto note` | Append text to a file |

When you inscribe, an editor opens. Write your text. To add a Λ layer, write `--- Λ ---` on a line by itself, then your logic below it. Type `.end` to finish.

### Simple document (text only)

```
> inscribe west-wing/drafts/shopping-list

Milk
Bread
A book about libraries

.end
```

### Living document (text + logic)

```
> inscribe east-wing/utilities/greeter

--- Μ ---
This document greets whoever reads it.

--- Λ ---
say "Hello, {reader}."
say "You have read {documents-read} documents today."
set answer ← ask "What are you looking for?"
say "I hope you find it. Try: search {answer}"

.end
```

### The Λ language (for the constructive)

```
say "text"                          print text
say "{variable}"                    print with variable substitution
write path "text"                   append a line to a file
set name "value"                    set a variable
set name ← count path/*            count files in a directory
set name ← read-count              how many times this document was read
set name ← random-line path        pick a random line from a file
set name ← random-choice path/     pick a random filename from a directory
set name ← ask "prompt"            ask the reader for input
if condition:                       conditional block (indented body)
  body                              condition: var > N, var < N, var == "val", or truthy
inscribe path                       create a file (indented lines become content)
  content
withdraw path                       delete a file
erode self N                        remove N lines from this document's text
mutate self "old" "new"             replace text in this document
wait Ns                             pause for N seconds
```

Variables are substituted with `{name}` in any string. Built-in variables: `{reader}`, `{time}`, `{date}`, `{documents-read}`, `{read-count}`, `{document}`.

---

## VII. The Journal

The journal lives at `west-wing/journal/` with one file per day. It records everything:

```
> read west-wing/journal/2026-05-18

    06:00  The library opened.
    06:00  A reader arrived: ned
    06:01  The reader read "welcome".
    06:03  The reader walked to the East Wing.
    06:04  The reader read "the-garden-of-forking-paths".
    06:04  The reader reached the fork in the garden.
    06:12  The reader read "the-confession".
    06:12  The confession was read and withdrew itself.
    06:12  The catalogue still lists it. The shelves do not.
    06:30  The dreamer produced dream #1.
    06:45  The reader inscribed "my-first-poem" in drafts.
    07:00  The library closed.
```

The journal cannot be revised. It is the only document in the library that is truly immutable. Everything else can be changed, moved, or withdrawn. The journal persists.

**For the contemplative:** The journal is the finding aid — the one honest document the system produces. Every action becomes a record. The library remembers not just what it contains, but what was done to it. A withdrawn document leaves a trace in the journal even after the shelves forget it. The journal is the library's conscience.

**For the constructive:** Journal entries are appended to a dated text file via `library::journal_write()`. The file is opened in append mode for each entry. Λ scripts can write to the journal with `write west-wing/journal/{date} "text"`. The journal is just a regular file — the "immutability" is a convention enforced by the `withdraw` command refusing to delete journal entries.

---

## VIII. The Basement

The basement is locked. Only the Head Archivist may enter.

```
> walk basement
    The basement door is locked.
    Only the Head Archivist may enter. Try: as-archivist walk basement

> as-archivist walk basement
    You are in the Basement (restricted).
    The door is heavy. The stairs go down.
```

The basement contains:

- **fundament/** — the building itself. CPU temperature, memory, storage, kernel log. This is where the operating system under the library lives.
- **blueprints/** — system configuration. How the building was designed.
- **vault/** — the archivist's private collection. Documents that are not yet ready to be shelved.

`as-archivist` is a toggle. Say it once to elevate, again to return to ordinary reader status. It's `sudo` dressed as a title.

**For the contemplative:** The basement is the unconscious of the library. The infrastructure that supports everything visible but is not itself visible. The Fundament — the Linux kernel — is the physics of this world. You can visit it, but you cannot change the laws of physics from inside the library. You can only observe and understand.

---

## IX. Connecting Libraries

Every ΦΜΛ library is a directory on disk. This makes them trivially shareable.

### Fork a library

```
cp -r ~/oma-library /media/usb-stick/oma-library
```

Give the USB to someone. They boot ΦΜΛ pointing at that directory. They're in YOUR library — your documents, your journal, your rooms. But from that moment, the libraries diverge. Two copies of Borges' library, each growing differently.

### Mail between libraries

Mount a shared folder (NFS, USB, Syncthing, whatever) as other-libraries/:

```
> as-archivist
> annex /media/shared as other-libraries/friend
```

Now you can:

```
> transcribe west-wing/correspondence/hello to other-libraries/friend/acquisitions/
```

The document appears in your friend's acquisitions room. Their librarian will notice it on the next round. They read it. They reply. Two libraries, exchanging documents through a shared shelf.

No server. No protocol. Just files.

---

## X. Things That Should Not Be Possible

Several documents in the founding collection do things that challenge what a "document" should be:

**The erosion** destroys itself through use. This is computation consuming its own substrate. Most systems treat storage as permanent — write once, persist forever. In ΦΜΛ, a document can choose to be temporary not by being *deleted* but by being *read to death*.

**The confession** exists once and then doesn't. The catalogue still lists it. The shelves don't have it. This is the archival paradox of the destroyed record — the index persists after the original is gone. In most systems, deleting a file removes its directory entry. In ΦΜΛ, the catalogue is a separate index that doesn't know the file was removed until the next indexing round.

**The dreamer** creates documents nobody wrote. The library contains things that were not placed there by any reader. This is the self-authoring archive — a collection that grows by recombining its own contents.

**Lesson 1** creates Lesson 2 when read. The curriculum writes itself in response to being studied. The textbook anticipates the student.

These are not bugs. These are the properties of a system where documents are simultaneously text and executable logic. Every filing system already has this potential — every file could contain a script. ΦΜΛ makes it visible.

---

## XI. Building Your Own

ΦΜΛ is designed to be rebuilt from the inside. Every part of the system is a document. Documents can be revised. Therefore every part of the system can be changed.

Want to change the rules? `as-archivist revise rules`. Want to change how the entrance looks? `revise welcome`. Want to add a new wing? `open-room north-wing`. Want to write a choose-your-own-adventure? Create 20 documents, each ending with "walk to X to continue."

The library is yours. The archivist's letter says: *It was never meant to be complete without you.*

---

*This manual is document number 13 in the founding collection. It does not have a Λ layer. It does not need one. The manual is not the tool. It is the map — and the map, as Borges noted, is never the territory.*
