# ΦΜΛ

**A living book that is also an operating system.**

Boot a Raspberry Pi. No desktop. No login screen. Just a blinking cursor and the opening line of a library you can explore, read, write, and rebuild. The library is the computer. The computer is the library. You can read it or run it. Those are the same verb.

ΦΜΛ is three Greek letters — Phi, Mu, Lambda — and three things at once:

- **An operating system** that fits on a 32MB SD card and boots in two seconds
- **A library you explore** through text, where every file is a document that describes what it does and does what it describes
- **A programming environment** where writing a new document changes the system, and no two installations will ever contain the same collection

In Dutch, *oma* means grandmother — the keeper of stories, the one whose house you explore as a child, opening drawers, finding things you don't understand yet. That's one layer. The other layers are the Greek: Form, Message, Function — the three registers that every object in the system carries. Both meanings are always present.

---

## The Three Registers

Everything in ΦΜΛ has three layers. A document, a room, a process, the system itself — all three are always there.

**Φ (Form)** — Where something is. Its classification, its location on the shelves, its structural relationship to other documents. The skeleton.

**Μ (Message)** — What something says. The visible text. The human-readable content. What you see when you `read` a document.

**Λ (Lambda)** — What something does. The executable logic. The margin. What happens when the document is alive.

Every interaction with ΦΜΛ shows you one or more of these layers depending on what you ask:

```
> read clock

    It is Sunday, 18 May 2026, 11:47:23.
    The library has been open for 3 hours.
    Four readers are present. No documents are overdue.

> inspect clock

    Φ (Form)
      Location:     east-wing/processes/clock
      Classification: process, self-updating, public
      Acquired:     system-birth
      Condition:    good

    Μ (Message)
      "It is Sunday, 18 May 2026, {time}.
       The library has been open for {uptime}.
       {reader-count} readers are present. {overdue-status}."

    Λ (Lambda)
      loop:
        time ← query fundament clock
        uptime ← query fundament boot-time
        readers ← count reading-room/*
        format Μ with time, uptime, readers
        display
        wait 1s
```

You always know what you're looking at. The friendly surface (Μ) and the working machinery (Φ and Λ) are never hidden — you just have to ask.

---

## The Language of the Library

ΦΜΛ replaces every Unix command with a word a reader would use. You navigate a library, not a filesystem.

### Moving

| You type | What happens |
|----------|-------------|
| `walk east-wing` | Move to the east wing |
| `walk back` | Return to the room you came from |
| `walk lobby` | Return to the entrance hall |
| `browse` | See what's on the shelves around you |
| `browse -quietly` | Just the names, no descriptions |

### Reading

| You type | What happens |
|----------|-------------|
| `read welcome` | Read a document (shows the Μ layer) |
| `glance at report` | Read just the first few lines |
| `peek at logbook` | Read just the last few lines |
| `inspect clock` | See all three registers: Φ, Μ, Λ |

### Writing

| You type | What happens |
|----------|-------------|
| `inscribe new-letter` | Create a new document (opens editor) |
| `revise welcome` | Edit an existing document |
| `transcribe letter to outbox/` | Copy a document |
| `reshelve notes to archive/` | Move a document |
| `withdraw draft-3` | Remove a document from the collection |

### Organising

| You type | What happens |
|----------|-------------|
| `open-room south-wing` | Create a new room |
| `close-room south-wing` | Remove an empty room |
| `classify restricted letter` | Change access classification |
| `search title:borges` | Search the catalogue |
| `scan "infinite" in east-wing/` | Scan documents for a phrase |
| `catalogue` | View or rebuild the master index |

### System

| You type | What happens |
|----------|-------------|
| `readers` | Who is reading what right now? |
| `dismiss reader-7` | Ask a reader to leave (end a process) |
| `activity` | What's happening in the library? |
| `inventory` | How much shelf space remains? |
| `ledger` | The record of everything you've done |
| `turn-page` | Clear the screen — a fresh page |
| `leave` | Leave the library. The lights go out. |

### Connecting

| You type | What happens |
|----------|-------------|
| `say hello` | Print text |
| `say hello into greeting` | Write text to a document |
| `say more onto greeting` | Append to a document |
| `read letter then scan "secret"` | Chain actions (pipe) |
| `annex /dev/sda1 as west-wing` | Attach external storage |
| `seal west-wing` | Detach external storage |
| `as-archivist` | Act with Head Archivist authority (root) |

A session:

```
You are in the Entrance Hall.
The catalogue desk is ahead. Wings branch east and west.
A single document rests on the desk: "welcome"

> read welcome

    Welcome to the Open Manual Archive.

    You are the first reader today. The collection contains
    1,247 documents across 43 rooms. The east wing holds
    the technical collection. The west wing holds letters
    and ephemera. The basement is locked.

    To begin, browse the shelves or walk to a wing.

> walk east-wing

    You are in the East Wing — Technical Collection.
    The air is cooler here. The shelves are metal.

> browse

      networking/ .......... how the library connects to others
      processes/ ........... documents that do things when read
      devices/ ............. the building's infrastructure
      utilities/ ........... tools for the working reader

> walk processes

    You are in the Processes room.
    These documents are alive. Reading them sets them in motion.

> read heartbeat

    The building is healthy.
    Fundament: 42°C, 1.2GB free, load 0.3
    All wings open. No damage detected.
    The messenger last checked for deliveries 4 minutes ago.
```

---

## Writing and Building

You can write new documents that change how the system works. This is programming, but it feels like adding to a library collection.

### Simple: just write

Inscribe a document with visible text only. It becomes part of the collection. It's searchable, browsable, and permanent.

```
> inscribe west-wing/correspondence/note-to-self

    (editor opens)

    Remember to check the basement lock next Tuesday.
    The previous archivist mentioned something about
    a room behind the boiler.

    (save and close)

> search "basement lock"

    1 result:
      west-wing/correspondence/note-to-self
      "Remember to check the basement lock next Tuesday..."
```

### Medium: write a document that does something

Add a Λ (Lambda) layer. The document now has both text you can read AND logic that runs.

```
> inscribe east-wing/utilities/greeter

    (editor opens — two sections)

    --- Μ (Message) ---
    This document greets whoever reads it by name
    and tells them how long they've been in the library.

    --- Λ (Lambda) ---
    reader ← query reading-room/current
    duration ← query reader.entered
    say "Hello, {reader.name}."
    say "You have been here for {duration}."
    say "You have read {reader.documents-read} documents today."

    (save and close)

> read greeter

    Hello, Ned.
    You have been here for 47 minutes.
    You have read 12 documents today.
```

The Λ syntax for everyday use is readable English-like instructions:

```
Λ syntax (simple):
    name ← query source          fetch a value
    say "text {variable}"        output text
    if condition:                conditional
      do-something
    loop:                        repeat
      do-something
      wait 5s
    read other-document          invoke another document's Λ
    count room/*                 count items
    query fundament thing        ask the building (kernel)
```

### Advanced: full ΦΜΛ grid programs

For complex logic — daemons, network services, the librarian itself — the Λ layer uses the full ΦΜΛ grid language. Two-dimensional execution, three-register consensus, self-mutating instructions. This is the deep end.

```
> inspect librarian

    Φ (Form)
      Location:     east-wing/processes/librarian
      Classification: daemon, system-critical, restricted
      Condition:    mutating (cycle 4,712)

    Μ (Message)
      "The librarian catalogues new acquisitions, repairs
       damaged documents, and maintains the index."

    Λ (Lambda)
      [ΦΜΛ grid program — 24x16, degree 3, 4712 mutations]
      Use `inspect librarian -deep` to view the grid.
```

You don't need to write grid programs to use ΦΜΛ. You don't even need to know they exist. But they're there — the same way C is there under a Unix shell. The simple Λ instructions compile down to ΦΜΛ grid operations. The grid is the machine code of this library.

### Rebuilding the library

Because every part of the system is a document, you can rewrite any of it:

```
> as-archivist revise rules

    (the library's rules open in the editor)
    (you change them)
    (save and close)

    The rules have been revised.
    The librarian will enforce the new rules on the next round.
```

You can restructure rooms, change how processes work, rewrite the welcome message, alter the journal format. The system is its own source code, presented as a collection of documents.

**This is why no two ΦΜΛ installations are the same.** Same floor plan at birth. Same initial collection. But every reader adds different documents, rewrites different rules, opens different rooms. After a week, your library is yours. After a month, it's unrecognisable from someone else's.

---

## The Filesystem as Library

```
/
├── welcome                     the first document you see
├── catalogue                   the master index (searchable)
├── rules                       how the library operates
│
├── east-wing/                  technical collection
│   ├── networking/             connections to other libraries
│   ├── processes/              living documents (daemons)
│   │   ├── clock               tells the time
│   │   ├── heartbeat           monitors the building
│   │   ├── messenger           sends/receives between libraries
│   │   ├── librarian           background cataloguer
│   │   └── dormant             does nothing. yet.
│   ├── devices/                the building's infrastructure
│   └── utilities/              tools for the working reader
│
├── west-wing/                  letters, ephemera, personal
│   ├── correspondence/         messages received and sent
│   ├── drafts/                 work in progress
│   ├── journal/                the system log, as diary entries
│   └── ephemera/               temporary documents (auto-withdraw)
│
├── basement/                   restricted (as-archivist required)
│   ├── fundament/              the building itself (kernel, hardware)
│   ├── blueprints/             how the building is configured
│   └── vault/                  the archivist's private collection
│
├── acquisitions/               newly arrived, unsorted
│
├── reading-room/               active sessions
│
└── other-libraries/            mounted external storage
```

The journal reads like a diary:

```
> read journal/today

    06:00  The library opened. All wings inspected. No damage overnight.
    06:01  The heartbeat document began its rounds.
    06:03  The messenger checked for deliveries. None waiting.
    09:14  A reader arrived at the entrance hall.
    09:14  The reader walked to the east wing.
    09:15  The reader read "clock" in the processes room.
    09:22  The reader inscribed "note-to-self" in correspondence.
    09:30  The reader attempted to enter the basement. Access denied.
    11:47  The reader is still here. 12 documents read.
```

---

## The Substrate

Beneath ΦΜΛ is a Linux kernel. We call it **The Fundament**.

The Fundament is the building that houses the library. Walls, floors, electricity, plumbing. You don't catalogue concrete. But without it, the shelves have nothing to stand on.

The Fundament handles hardware, memory, scheduling, and physical storage. ΦΜΛ handles everything you see, read, write, and run. The boundary is the floor beneath your feet.

You can visit the Fundament:

```
> as-archivist walk basement/fundament

    You are in the Fundament.
    This is the building itself — what the library stands on.

> browse

      cpu ................. the building's engine
      memory .............. shelf capacity (physical)
      temperature ......... how warm the building is
      storage ............. the physical shelves
      network-cable ....... the wire to other libraries
      kernel-log .......... the building's own diary

> read temperature

    The building is 41°C. This is within normal range.
    The engine is running at 23% capacity.
```

When something goes wrong at the Fundament level:

```
    The lights in this wing have flickered.
    A shelf in east-wing/processes/ may be unstable.
    The heartbeat document is investigating.
```

---

## The Seeds

ΦΜΛ ships with a collection that hints at more than it shows:

- A letter in `west-wing/correspondence/` from "a previous archivist" referencing a room that doesn't appear on any floor plan
- A document called `dormant` in processes that does nothing when read — but its Λ layer contains logic waiting for a condition that hasn't been met
- The catalogue has entries for documents that `search` cannot locate
- The basement contains `special-collections`, requiring a classification level nobody currently holds
- An empty shelf in the east wing labelled "documents that wrote themselves"
- The librarian occasionally logs: "Reclassified 1 document. Reason: self-amended."

These are not puzzles to solve. The library is slightly larger than it should be. Some of the shelves go back further than the walls.

---

## Technical Architecture

```
┌──────────────────────────────────────────────────┐
│  /bin/φμλ  (Rust, single binary, PID 1)          │
│                                                  │
│  ├── Shell            command parser, REPL       │
│  ├── Narrator         prose output formatter     │
│  ├── Cataloguer       full-text search index     │
│  ├── Margin Engine    ΦΜΛ grid executor          │
│  ├── Λ Compiler       simple Λ → grid compiler   │
│  ├── Librarian        background daemon thread   │
│  └── Renderer         terminal styling           │
│                                                  │
│  Starts as PID 1. No other userspace exists.     │
└──────────────────────────────────────────────────┘
═══════════════ THE FUNDAMENT ═════════════════════
┌──────────────────────────────────────────────────┐
│  Linux kernel (~4MB)                             │
│  Hardware, memory, scheduling, block I/O         │
└──────────────────────────────────────────────────┘
```

**Image:** Buildroot generates a minimal Linux image. Kernel + `/bin/φμλ` + initial library collection. 32MB total.

**Language:** Rust. Single binary, no runtime dependencies, cross-compiles to ARM (Raspberry Pi) and x86.

**Core components:**
- Terminal handling (crossterm)
- Full-text search (tantivy)
- ΦΜΛ margin engine (from the spec, compiled in)
- Simple Λ compiler (readable instructions → grid programs)

---

## What Success Looks Like

Someone downloads a 32MB image. Flashes it to an SD card. Boots a Raspberry Pi. A library appears. They spend an hour exploring rooms, reading documents, finding the journal, discovering that some documents are alive. They inscribe their own document. They walk to the basement and find it locked. They come back later and the journal has recorded everything.

They `inspect` something and see the three registers — Φ, Μ, Λ — and understand that the friendly words and the Greek machinery are the same system seen from different distances.

They write a document with a Λ layer and watch it come alive. They revise the rules and the library changes. They realise the OS is the manuscript and the manuscript is the OS and they've been writing both this whole time.

They show it to someone else. That person boots their own copy. Same entrance hall. Same welcome letter. But within a day, a completely different library.
