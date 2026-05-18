# ΦΜΛ

A library that is also an operating system.

Boot it. You're standing in an entrance hall. The shelves are full. A document rests on the desk. You can read it, explore the rooms, write your own documents, and rebuild the library from the inside. Everything you do is recorded in a journal. Everything you write becomes part of the collection. No two libraries will ever be the same.

ΦΜΛ is three Greek letters — **Phi, Mu, Lambda** — and three things at once:

- **Φ (Form)** — where something is. Its place on the shelves. Its structure.
- **Μ (Message)** — what something says. The text you read.
- **Λ (Lambda)** — what something does. The logic that runs when you read it.

Every document in the library carries all three. `read` shows you the message. `inspect` shows you all three registers. Some documents are still. Some are alive.

```
> read welcome

    Welcome to the Open Manual Archive.
    You are the first reader in this library...

> inspect welcome

    Φ (Form)
      Location: welcome
      Classification: document, still

    Μ (Message)
      Content: "Welcome to the Open Manual Archive..."

    Λ (Lambda)
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
> read catalogue
    This document lists every document in the library,
    including itself.

    That sentence is the oldest problem in library science...

> read the letter
    The catalogue has three entries for documents I cannot find.
    I did not create these entries...

> inspect dormant
    Λ (Lambda)
      (instructions waiting for a condition that hasn't been met)
```

### If you build things

You see an operating system. Rooms are directories. Documents are files with an executable margin. `browse` is `ls` that shows you nested content. `walk` is `cd` with fuzzy matching. `inscribe` is file creation with a built-in editor. The Λ layer is a scripting language. The journal is syslog formatted as prose. The basement is root access behind `as-archivist`. You can rewrite the rules, revise any document, and change how the system behaves — because the system IS its documents.

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
cargo build --release

# Run — creates a library at ~/oma-library/ on first boot
./target/release/oma

# Or specify where the library lives
OMA_ROOT=/tmp/my-library ./target/release/oma
```

On first boot, the library opens with seven documents and a set of rules. After that, it's yours.

## Commands

You don't need to memorise paths. Type what you mean — "east wing", "the letter", "garden" all work.

| You type | What happens |
|----------|-------------|
| `browse` | See what's on the shelves (including nested rooms) |
| `walk east wing` | Move to a room |
| `walk back` | Go back |
| `read the letter` | Read a document |
| `inspect clock` | See all three registers (Φ Μ Λ) |
| `inscribe my-note` | Create a new document |
| `revise welcome` | Edit a document |
| `search borges` | Search the whole catalogue |
| `say hello` | Say something |
| `say hello into note` | Write to a document |
| `as-archivist` | Unlock restricted areas |
| `help` | See all commands |
| `leave` | The lights go out |

## What's in the repo

| File | What it is |
|------|-----------|
| `src/` | The shell — Rust, single binary, ~1500 lines |
| `BRIEF.md` | The full project brief — architecture, filesystem, design |
| `FOUNDING-COLLECTION.md` | The seven founding documents, designed for three readers |
| `SPEC.md` | The ΦΜΛ esolang specification (the deep Λ layer) |
| `interpreter.py` | Reference esolang interpreter in Python |
| `research/` | Source research — library science, Borges, secret languages |

## The name

**ΦΜΛ** — Phi Mu Lambda. Form, Message, Function.

In Dutch, *oma* means grandmother. The keeper of stories. The one whose house you explore as a child, opening drawers, finding things that don't quite make sense yet.

Both meanings are always present.

## License

MIT
