# The Founding Collection

Seven documents on the shelves when the library first opens. Each one is readable, philosophical, and functional at the same time. Nobody gets a lesser version.

---

## 1. welcome

**Location:** Entrance Hall, on the desk

**The text (Μ):**

> You are the first reader in this library. Or perhaps not the first — the journal will know. The shelves are full but the collection is young. Everything here was placed by the archivist who built this building, and everything you add will outlast both of us.
>
> A library is not its books. A library is the order imposed on its books. That order is fragile. It depends entirely on someone caring enough to maintain it. Today, that someone is you.
>
> To see what is here, say `browse`. To move through the rooms, say `walk`. To read anything, say `read`. To add something of your own, say `inscribe`. To understand how anything works, say `inspect`.
>
> The east wing holds the technical collection — things that do. The west wing holds letters and ephemera — things that remember. The basement holds the building itself — things that are.
>
> Take your time. The library is patient.

**What the kid sees:** Instructions. A friendly welcome. They now know five commands and where to go. They feel invited.

**What the philosopher sees:** A meditation on curation as care. The distinction between a collection and its organisation. The archivist as mortal, the library as persistent. The reader as inheritor of an ordering system they didn't create.

**What the programmer sees:** A tutorial that teaches `browse`, `walk`, `read`, `inscribe`, `inspect` and the three-zone filesystem (east=technical, west=personal, basement=system) — all without ever saying "filesystem" or "command."

**The logic (Λ):**

```
reader ← query reading-room/current
if reader.first-visit:
  write journal/today "{time} A new reader arrived and read the welcome."
  mark reader.first-visit false
```

Reading the welcome document is itself a system event. The journal records it. On your second visit, the document reads exactly the same — but the system already knows you.

---

## 2. catalogue

**Location:** Entrance Hall, on the desk beside welcome

**The text (Μ):**

> This document lists every document in the library, including itself.
>
> That sentence is the oldest problem in library science. A catalogue that contains itself is either incomplete (because it cannot fully describe its own entry) or inconsistent (because its description of itself changes every time a new document is added, which changes the catalogue, which changes the description).
>
> The working compromise: this catalogue is always slightly out of date. The librarian updates it on every round. Between rounds, new documents exist but are not yet indexed. They are real but not yet *found*. This is true of most things.
>
> To search: `search {words}`. To browse a room instead: `walk` there and `browse`.

Then, below the text, the live index:

> **The Collection** (last indexed: 3 minutes ago)
>
> 7 documents across 4 rooms.
>
> welcome ........................ Entrance Hall
> catalogue ...................... Entrance Hall (you are here)
> the garden of forking paths ..... east-wing/stacks
> on the maintenance of shelves ... east-wing/stacks
> the letter ..................... west-wing/correspondence
> how to inscribe ................ east-wing/stacks
> the book of sand ............... east-wing/stacks

**What the kid sees:** A list of everything. They can search. They know what's available. The sentence about the catalogue listing itself is a fun riddle.

**What the philosopher sees:** Russell's paradox, Gödel's incompleteness, and the Borges catalogue problem presented as a practical limitation of a working system. The admission "this catalogue is always slightly out of date" is a precise philosophical position — all indexing is retroactive, all description lags behind the thing described.

**What the programmer sees:** `search` is backed by a full-text index. The librarian daemon rebuilds it periodically. The "slightly out of date" note is documentation of eventual consistency. The Λ layer IS the search engine.

**The logic (Λ):**

```
documents ← scan everything in /
for each doc in documents:
  display "{doc.name} ... {doc.location}"
```

The catalogue is not a static file. It regenerates itself every time you read it. The text is fixed. The index is live.

---

## 3. the garden of forking paths

**Location:** east-wing/stacks

**The text (Μ):**

An abridged retelling — not Borges' copyrighted text, but a new story written in the same tradition:

> In the archive of Dr. Liang there is a manuscript that is also a labyrinth. Most labyrinths are spatial — you walk them. This one is temporal. At every moment, the path forks. In one fork, you read this sentence and continue. In another, you stopped reading three lines ago and are now `browse`-ing the shelves instead. Both paths are real. Both lead somewhere.
>
> The manuscript contains instructions for navigating itself. "Turn to page 74" means something different if you are holding a book or standing in a library. In a book, you move through pages. In a library, you move through rooms. The labyrinth changes shape depending on what kind of building you think you're in.
>
> What kind of building do you think you're in?
>
> If you think you are in a library, say `walk` and go somewhere.
> If you think you are in a book, say `read` and keep reading.
> If you think you are in a program, say `inspect` and see the machinery.
>
> All three are correct. The paths fork here.

**What the kid sees:** A choose-your-own-adventure. Three choices. They pick one. It feels like a game.

**What the philosopher sees:** Borges' forking paths as a direct metaphor for the system they're inside. The question "what kind of building do you think you're in?" is genuine — ΦΜΛ is simultaneously all three, and the reader's interpretation determines their experience.

**What the programmer sees:** A document that teaches the three-register model by making you choose which register to engage with. `walk` = Φ (form/structure). `read` = Μ (message/content). `inspect` = Λ (function/logic). The fork is an introduction to the architecture.

**The logic (Λ):**

```
choice ← wait-for-input
if choice starts-with "walk":
  say "You chose the library. The path continues through rooms."
  say "You are a reader who moves through space."
if choice starts-with "read":
  say "You chose the book. The path continues through pages."
  say "You are a reader who moves through text."
if choice starts-with "inspect":
  say "You chose the program. The path continues through registers."
  say "You are a reader who moves through structure."
write journal/today "{time} The reader reached the fork. They chose: {choice}."
```

The journal records which path you took. It always will.

---

## 4. on the maintenance of shelves

**Location:** east-wing/stacks

**The text (Μ):**

> A shelf, left unattended, does not remain empty. It collects. Dust first, then objects placed temporarily that become permanent through neglect, then documents misfiled by hurried readers, then documents filed correctly but in a system that no longer reflects how the collection is actually used.
>
> The maintenance of shelves is not cleaning. It is re-reading. The maintainer must look at every shelf and ask: does this arrangement still describe something true? If the philosophy section has been colonised by cookbooks, the shelf has not failed. The classification has.
>
> In this library, the librarian makes a round every five minutes. It does the following:
>
> 1. Counts all documents. Compares to the last count.
> 2. Checks for documents in acquisitions/ that have not been reshelved.
> 3. Removes anything in ephemera/ older than one day.
> 4. Rebuilds the catalogue index.
> 5. Writes a line in the journal.
>
> If the librarian stops, the library does not break. It drifts. Documents accumulate in the wrong places. The catalogue falls behind. Searches return stale results. The library becomes a pile of documents in a building — still valuable, no longer navigable.
>
> To see the librarian at work: `inspect librarian`.
> To change how it works: `as-archivist revise librarian`.

**What the kid sees:** A story about keeping your room tidy, except it's a library. The five steps are concrete and easy to follow. They learn the library has a helper that does chores.

**What the philosopher sees:** Classification as an ongoing act of interpretation, not a one-time labelling. The distinction between a collection and a pile. The librarian as a daemon in the classical sense — a spirit that maintains order without being asked. The idea that a system without maintenance doesn't fail, it *drifts*.

**What the programmer sees:** The documentation for the librarian daemon. Cron-equivalent that runs every 5 minutes. Garbage collection (ephemera cleanup), indexing (catalogue rebuild), monitoring (count comparison), and logging (journal write). They learn they can inspect it, and — critically — revise it.

**The logic (Λ):**

```
This document has no active Λ layer — it describes the librarian,
it is not the librarian. The librarian itself is at:
east-wing/processes/librarian

(inspect that document to see the working Λ)
```

Important: not every document has to DO something. Some documents are just documents. The kid learns that some things are alive and some are still. The programmer learns the separation between documentation and implementation.

---

## 5. the letter

**Location:** west-wing/correspondence

**The text (Μ):**

> To whoever reads this —
>
> I built this library but I will not be its archivist for long. The shelves are sound. The catalogue is current as of today. The librarian is diligent but not creative — it will maintain what exists but never add to it. That part is up to you.
>
> A few things I should mention:
>
> The basement is locked because some documents in the vault are not ready to be read. They are not dangerous. They are incomplete. I locked the door because an unfinished document on an open shelf will inevitably be mistaken for a finished one, and that is worse than no document at all.
>
> The east wing has a room I never finished stocking. The shelf is labelled but empty. I had intended to fill it with documents that write themselves — processes that compose their own text, evolving documents that change each time they are read. I ran out of time. If you know how, you could build these yourself. `inscribe` a document with a Λ layer that modifies its own Μ layer. The system allows it. I am not sure it should.
>
> The catalogue has three entries for documents I cannot find. I did not create these entries. The catalogue is automated — it indexes what exists. If it has an entry, the document exists somewhere. I searched every room. I looked behind the shelves. I checked acquisitions/. They are not there. They are in the catalogue. I do not have an explanation.
>
> Look after the collection. It is more than I put into it.
>
> — A.

**What the kid sees:** A letter from the person who built this place. A mystery — what's in the basement? What are the missing documents? Who is A.? An empty shelf they could fill. Permission to make the library theirs.

**What the philosopher sees:** The archivist's anxiety about incompleteness. The ethical problem of unfinished documents on public shelves. The self-modifying document as an epistemological crisis ("the system allows it — I am not sure it should"). The three phantom catalogue entries as a direct Borges reference — things that exist in the index but not in the world.

**What the programmer sees:** A roadmap. The basement has system documents that aren't stable yet (locked behind root). The empty shelf is an invitation to write self-modifying processes. The phantom catalogue entries suggest either a bug or a feature — and the previous archivist couldn't tell which. That's real software.

**The logic (Λ):**

```
reader ← query reading-room/current
if reader.read-count("the letter") == 1:
  write journal/today "{time} A reader found the letter from A."
  write journal/today "{time} They are the {nth} person to read it."
```

The journal records who found it. The letter keeps count of its own readers. But it doesn't tell you that.

---

## 6. how to inscribe

**Location:** east-wing/stacks

**The text (Μ):**

> To add a document to the library, you must know three things. Where it goes (Φ). What it says (Μ). What it does (Λ). A document can have all three, or only some. A letter has Φ and Μ but no Λ — it sits on a shelf and says something but does nothing. A process has all three — it lives somewhere, it describes itself, and it acts. A blank page has only Φ — it has a place but nothing to say and nothing to do.
>
> To create a document with only text:
>
>     > inscribe west-wing/drafts/my-first-document
>
> An empty page opens. Write something. Save it. It exists now. The catalogue will find it on the next round. You have changed the library.
>
> To create a document that also does something, write both layers:
>
>     > inscribe east-wing/utilities/countdown
>
>     --- Μ ---
>     This document counts down from any number you give it.
>
>     --- Λ ---
>     n ← ask "Count down from?"
>     loop while n > 0:
>       say n
>       n ← n - 1
>       wait 1s
>     say "Done."
>
> When someone reads this document, the Μ layer displays first — "This document counts down from any number you give it." Then the Λ layer activates and the countdown begins.
>
> Try it now. Inscribe something. Anything. A note, a poem, a clock, a letter to the next reader. The library is not complete until you add to it. It was never meant to be complete without you.

**What the kid sees:** How to make stuff. Clear examples. An invitation: make a countdown, write a poem, leave a note. "The library is not complete until you add to it" — they feel needed.

**What the philosopher sees:** The ontology of the three registers applied practically. Φ without Μ is a blank page — existence without content. Μ without Λ is a letter — content without agency. All three together is a living document. The final line — "it was never meant to be complete without you" — is Eco's open work, Borges' infinite library admitting it needs finite readers.

**What the programmer sees:** The API. How to create files with executable logic. The `--- Μ ---` / `--- Λ ---` section markers. The simple Λ syntax: variables, loops, conditions, `ask` for input, `say` for output, `wait` for timing. Enough to start building immediately.

**The logic (Λ):**

```
This document has no Λ layer.

It teaches you to write Λ layers.
It does not need one of its own.
The manual is not the tool.
```

Deliberately inert. The irony is visible to all three readers: the document about making things come alive is itself just text.

---

## 7. the book of sand

**Location:** east-wing/stacks

**The text (Μ):**

> This document has no fixed content.
>
> Each time you read it, it is different. Not randomly different — it draws from the library itself. A sentence from the welcome. A line from the journal. A fragment of whatever you last inscribed. A piece of the catalogue. The words rearrange. The text is always coherent but never the same twice.
>
> The previous archivist called this "the book of sand" because, like Borges' original, it has no first page and no last page. You cannot find the passage you read before. You cannot show someone else what you saw. The document is specific to this moment, this reader, this state of the library.
>
> If you `inspect` this document, the Λ layer is visible. You can see exactly how it works. There is no mystery in the mechanism. The mystery is in the output — how the same simple process, drawing from the same shelves, produces something that feels written for you, right now, on this reading.
>
> This is reading number {n}. No two have been the same.

Then, below — the generated passage, different every time.

**What the kid sees:** A magic book that changes every time you open it. They read it again, it's different. They show a friend, it's different again. They try to find the same page and can't. Delightful.

**What the philosopher sees:** Borges' Book of Sand made real. The impossibility of rereading. The document as event rather than object. The transparency of mechanism ("you can see how it works") paired with the opacity of output ("it feels written for you"). Hermeneutics in executable form.

**What the programmer sees:** A working example of a generative document. The Λ layer reads other documents, selects fragments, recombines them. It's a template engine, a text generator, and a demonstration of the system's reflexivity — documents that read other documents. They can `inspect` it and learn how to build their own.

**The logic (Λ):**

```
n ← read self.read-count
n ← n + 1
write self.read-count n

fragments ← []
for room in [lobby, east-wing, west-wing]:
  docs ← list room
  pick ← random-choice docs
  line ← random-line-from pick
  append line to fragments

shuffle fragments
for each fragment in fragments:
  say "  {fragment}"

say ""
say "This is reading number {n}. No two have been the same."
```

---

## Placement Summary

```
Entrance Hall/
  ├── welcome .................. the door opens, you are invited in
  └── catalogue ................ the index of everything, including itself

east-wing/stacks/
  ├── the garden of forking paths .. the fork: library, book, or program?
  ├── on the maintenance of shelves  how the librarian keeps order
  ├── how to inscribe .............. how to add to the library
  └── the book of sand ............. the document that is never the same

west-wing/correspondence/
  └── the letter ................... from the archivist who built this place
```

Seven documents. Three readings each. No reader gets less. The kid explores, the philosopher contemplates, the programmer builds. Same shelves, same text, same system.
