# ΦΜΛ

A library that is also an operating system.

Boot it. You're standing in an entrance hall. The shelves are full. A document rests on the desk. You can read it, explore the rooms, write your own documents, and rebuild the library from the inside. Everything you do is recorded in a journal. Everything you write becomes part of the collection. No two libraries will ever be the same.

ΦΜΛ is three Greek letters — **Phi, Mu, Lambda** — and three things at once:

- **Φ — Phi — Form** — where something is. Its place on the shelves. Its structure.
- **Μ — Mu — Message** — what something says. The text you read.
- **Λ — Lambda — Function** — what something does. The logic that runs when you read it.

Every document in the library carries all three. `read` shows you the message. `inspect` shows you all three registers. Some documents are still. Some are alive.

```
> read welcome

    Welcome to the Open Manual Archive.
    You are the first reader in this library...

> inspect welcome

    Φ — Phi — Form
      Location: welcome
      Classification: document, still

    Μ — Mu — Message
      Content: "Welcome to the Open Manual Archive..."

    Λ — Lambda — Function
      Logic: (none — this document is still)
```

## Three readers, one library

ΦΜΛ is designed so that three different people can sit down at the same terminal and each have a complete experience. Nobody gets a lesser version.

### If you're curious

You explore. Walk between rooms. Read documents. Find the journal and see that the library has been recording your visit. Discover a locked basement. Find a letter from the previous archivist mentioning rooms that don't appear on any floor plan. Write your own document and watch it appear on the shelves.

```
> walk east wing
    You are in the East Wing — Technical Collection.

> browse
      stacks/ .............. The main collection.
          stacks/the-garden-of-forking-paths .. In the archive of Dr. Liang...
          stacks/the-book-of-sand ............. This document has no fixed content.
          stacks/how-to-inscribe .............. how to add to the library

> read garden
    In the archive of Dr. Liang there is a manuscript
    that is also a labyrinth...

> inscribe west-wing/drafts/my-note
    (write something — it's now part of the library forever)
```

### If you think about systems

You see a filing system that is also a philosophy of classification. The catalogue that lists itself and is therefore always incomplete. Documents whose executable logic is encoded in their whitespace — invisible unless you know to look. A journal that transforms system logs into narrative prose. The question of whether a self-modifying document should be allowed. The Borges debt is structural.

```
> catalogue
    15 documents across 18 rooms.

    The following entries have no corresponding document:
      the-unwritten .............. (location unknown)
      the-remembered ............. (location unknown)
      the-awaited ................ (location unknown)

> read the letter
    The catalogue has three entries for documents I cannot find.
    I did not create these entries...

> inspect dormant
    Λ — Lambda — Function
      (instructions waiting for a condition that hasn't been met)
```

### If you build things

You see an operating system. Rooms are directories. Documents are files with an executable margin. `browse` is `ls` that shows you nested content. `walk` is `cd` with fuzzy matching. `inscribe` is file creation with a built-in editor. The Λ layer is a scripting language with conditionals, loops, self-modification, and file operations. The journal is syslog formatted as prose. The basement is root access behind `as-archivist`. You can rewrite the rules, revise any document, and change how the system behaves — because the system IS its documents.

```
> inscribe east-wing/utilities/greeter

    --- Μ ---
    This document greets whoever reads it.

    --- Λ ---
    say "Hello, {reader}. You've read {documents-read} documents today."

> read greeter
    Hello, ned. You've read 7 documents today.

> as-archivist revise rules
    (the library's rules open — change them, the system changes)
```

## Quick start

```bash
# Build from source (requires Rust)
make build

# Run — creates a library at ~/oma-library/ on first boot
./target/release/oma

# Or specify where the library lives
OMA_ROOT=/tmp/my-library ./target/release/oma
```

On first boot, the library opens with twelve documents, a set of rules, and three phantom catalogue entries. A librarian daemon begins maintaining the collection. A dreamer begins composing in the background. After that, it's yours.

## Commands

You don't need to memorise paths. Type what you mean — "east wing", "the letter", "garden" all work.

### Explore

| You type | What happens |
|----------|-------------|
| `browse` | See what's on the shelves (including nested rooms) |
| `walk east wing` | Move to a room |
| `walk back` | Go back |
| `where` | Where am I? |

### Read

| You type | What happens |
|----------|-------------|
| `read the letter` | Read a document |
| `glance at welcome` | First five lines |
| `peek at welcome` | Last five lines |
| `inspect clock` | See all three registers (Φ Μ Λ) |
| `inspect clock -deep` | Full grid view (for grid programs) |
| `catalogue` | Live index of every document + phantom entries |

### Write

| You type | What happens |
|----------|-------------|
| `inscribe my-note` | Create a new document |
| `revise welcome` | Edit a document |
| `say hello` | Say something |
| `say hello into note` | Write to a document |
| `say hello onto note` | Append to a document |

### Organise

| You type | What happens |
|----------|-------------|
| `transcribe A to B/` | Copy a document |
| `reshelve A to B/` | Move a document |
| `withdraw draft` | Remove a document |
| `open-room south-wing` | Create a room |
| `close-room south-wing` | Remove empty room |
| `search borges` | Search all documents |

### Connect

| You type | What happens |
|----------|-------------|
| `annex /path as name` | Attach external storage |
| `seal name` | Detach external storage |
| `classify restricted doc` | Set access level |
| `readers` | Who else is in the library? |

### System

| You type | What happens |
|----------|-------------|
| `as-archivist` | Unlock restricted areas |
| `activity` | Uptime and session info |
| `inventory` | Shelf space remaining |
| `ledger` | Everything you've done today |
| `turn-page` | Clear the screen |
| `leave` | The lights go out |

## Background daemons

Two processes run silently while you read:

**The librarian** makes a round every five minutes. It counts documents, compares to the last count, removes expired ephemera (older than one day), and writes a line in the journal.

**The dreamer** wakes every hour. It reads random fragments from across the collection, recombines them, and inscribes a new document in `west-wing/ephemera/`. The library authors itself while you're away. Dreams expire with the ephemera.

## Embedded programs

ΦΜΛ programs can be hidden in the whitespace of any host document. Spaces and tabs encode bits. The visible text says one thing. The invisible text does another.

```bash
# Extract and run an embedded program
oma --embedded path/to/document
```

`inspect` will detect embedded programs: "contains a ΦΜΛ whitespace program."

## The grid language

Beneath the simple Λ instructions is the full ΦΜΛ grid language — a two-dimensional esoteric programming language with:

- Toroidal grid execution (wraps at edges)
- Three degrees: Declaration (Φ) → Population (Μ) → Activation (Λ)
- Consensus protocol: all three registers must agree before an operation executes
- Self-mutation: every executed instruction changes after use
- Finding aid: the system logs every decision, dissent, and mutation

Use `inspect -deep` on any document with a grid program to see the execution state.

## Multi-reader sessions

Multiple terminals can connect to the same library. Each session registers on boot and unregisters on shutdown. `readers` shows everyone currently present. The journal records all of them. Stale sessions (dead processes) are cleaned up automatically.

## What's in the repo

| File | What it is |
|------|-----------|
| `src/` | The shell — Rust, 3,452 lines across 10 modules |
| `site/` | Static documentation site (HTML + CSS, zero JS) |
| `BRIEF.md` | The full project brief — architecture, filesystem, design |
| `FOUNDING-COLLECTION.md` | The founding documents, designed for three readers |
| `SPEC.md` | The ΦΜΛ esolang specification (the deep Λ layer) |
| `interpreter.py` | Reference esolang interpreter in Python |
| `research/` | Source research — library science, Borges, secret languages |
| `Makefile` | Build, install, cross-compile, bootable image |

## Cross-compile and boot

```bash
# Cross-compile for Raspberry Pi
make cross-pi

# Build a bootable initramfs
make image

# Test with QEMU
qemu-system-aarch64 -M virt -cpu cortex-a72 \
  -kernel <your-kernel> -initrd build/initramfs.cpio.gz \
  -append 'console=ttyAMA0' -nographic
```

821KB binary. One Rust crate beyond std (chrono). Boots in two seconds on a Pi.

## The name

**ΦΜΛ** — Phi Mu Lambda. Form, Message, Function.

In Dutch, *oma* means grandmother. The keeper of stories. The one whose house you explore as a child, opening drawers, finding things that don't quite make sense yet.

Both meanings are always present.

## License

MIT
