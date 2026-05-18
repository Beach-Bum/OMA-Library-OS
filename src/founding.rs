//! The Founding Collection — created on first boot.
//! Seven documents. The shelves are never empty.

use std::fs;
use std::path::Path;

pub fn create_library(root: &Path) {
    // Create room structure
    let rooms = [
        "",
        "east-wing",
        "east-wing/stacks",
        "east-wing/processes",
        "east-wing/devices",
        "east-wing/utilities",
        "east-wing/networking",
        "west-wing",
        "west-wing/correspondence",
        "west-wing/drafts",
        "west-wing/journal",
        "west-wing/ephemera",
        "basement",
        "basement/fundament",
        "basement/blueprints",
        "basement/vault",
        "acquisitions",
        "reading-room",
        "other-libraries",
    ];

    for room in &rooms {
        let _ = fs::create_dir_all(root.join(room));
    }

    // Room descriptions
    let descriptions = [
        ("east-wing", "The air is cooler here. The shelves are metal."),
        ("east-wing/stacks", "The main collection. Documents on every subject."),
        ("east-wing/processes", "These documents are alive. Reading them sets them in motion."),
        ("east-wing/devices", "The building's infrastructure."),
        ("east-wing/utilities", "Tools for the working reader."),
        ("east-wing/networking", "How this library connects to others."),
        ("west-wing", "Warmer here. The shelves are wood."),
        ("west-wing/correspondence", "Letters sent and received."),
        ("west-wing/drafts", "Work in progress."),
        ("west-wing/journal", "The library's diary."),
        ("west-wing/ephemera", "Temporary documents. They will not last."),
        ("basement", "The door is heavy. The stairs go down."),
        ("basement/fundament", "The building itself. Concrete and wiring."),
        ("basement/blueprints", "How the building was designed."),
        ("basement/vault", "The archivist's private collection."),
        ("acquisitions", "Newly arrived. Not yet sorted."),
        ("reading-room", "Where readers sit."),
        ("other-libraries", "Connections to the outside."),
    ];

    for (room, desc) in &descriptions {
        let path = root.join(room).join(".room-description");
        let _ = fs::write(path, desc);
    }

    // === THE SEVEN DOCUMENTS ===

    // 1. welcome
    fs::write(root.join("welcome"), r#"Welcome to the Open Manual Archive.

You are the first reader in this library. Or perhaps not the first — the
journal will know. The shelves are full but the collection is young.
Everything here was placed by the archivist who built this building,
and everything you add will outlast both of us.

A library is not its books. A library is the order imposed on its books.
That order is fragile. It depends entirely on someone caring enough to
maintain it. Today, that someone is you.

To see what is here, say: browse
To move through the rooms, say: walk
To read anything, say: read
To add something of your own, say: inscribe
To understand how anything works, say: inspect

The east wing holds the technical collection — things that do.
The west wing holds letters and ephemera — things that remember.
The basement holds the building itself — things that are.

Take your time. The library is patient.
"#).unwrap();

    // 2. catalogue
    fs::write(root.join("catalogue"), r#"This document lists every document in the library, including itself.

That sentence is the oldest problem in library science. A catalogue that
contains itself is either incomplete (because it cannot fully describe
its own entry) or inconsistent (because its description of itself changes
every time a new document is added, which changes the catalogue, which
changes the description).

The working compromise: this catalogue is always slightly out of date.
The librarian updates it on every round. Between rounds, new documents
exist but are not yet indexed. They are real but not yet found. This is
true of most things.

To search: search {words}
To browse a room instead: walk there and browse.
"#).unwrap();

    // 3. the garden of forking paths
    fs::write(root.join("east-wing/stacks/the-garden-of-forking-paths"), r#"In the archive of Dr. Liang there is a manuscript that is also a labyrinth.
Most labyrinths are spatial — you walk them. This one is temporal. At every
moment, the path forks. In one fork, you read this sentence and continue.
In another, you stopped reading three lines ago and are now browsing the
shelves instead. Both paths are real. Both lead somewhere.

The manuscript contains instructions for navigating itself. "Turn to
page 74" means something different if you are holding a book or standing
in a library. In a book, you move through pages. In a library, you move
through rooms. The labyrinth changes shape depending on what kind of
building you think you're in.

What kind of building do you think you're in?

If you think you are in a library, say "walk" and go somewhere.
If you think you are in a book, say "read" and keep reading.
If you think you are in a program, say "inspect" and see the machinery.

All three are correct. The paths fork here.

--- Λ ---
say "The journal notes your choice."
write west-wing/journal/{date} "{time} The reader reached the fork in the garden."
"#).unwrap();

    // 4. on the maintenance of shelves
    fs::write(root.join("east-wing/stacks/on-the-maintenance-of-shelves"), r#"A shelf, left unattended, does not remain empty. It collects. Dust first,
then objects placed temporarily that become permanent through neglect,
then documents misfiled by hurried readers, then documents filed correctly
but in a system that no longer reflects how the collection is actually used.

The maintenance of shelves is not cleaning. It is re-reading. The maintainer
must look at every shelf and ask: does this arrangement still describe
something true? If the philosophy section has been colonised by cookbooks,
the shelf has not failed. The classification has.

In this library, the librarian makes a round every five minutes. It does
the following:

1. Counts all documents. Compares to the last count.
2. Checks for documents in acquisitions/ that have not been reshelved.
3. Removes anything in ephemera/ older than one day.
4. Rebuilds the catalogue index.
5. Writes a line in the journal.

If the librarian stops, the library does not break. It drifts. Documents
accumulate in the wrong places. The catalogue falls behind. Searches return
stale results. The library becomes a pile of documents in a building —
still valuable, no longer navigable.

To see the librarian at work: inspect librarian
To change how it works: as-archivist revise librarian
"#).unwrap();

    // 5. the letter
    fs::write(root.join("west-wing/correspondence/the-letter"), r#"To whoever reads this —

I built this library but I will not be its archivist for long. The shelves
are sound. The catalogue is current as of today. The librarian is diligent
but not creative — it will maintain what exists but never add to it. That
part is up to you.

A few things I should mention:

The basement is locked because some documents in the vault are not ready
to be read. They are not dangerous. They are incomplete. I locked the door
because an unfinished document on an open shelf will inevitably be mistaken
for a finished one, and that is worse than no document at all.

The east wing has a shelf I never finished stocking. It is labelled but
empty. I had intended to fill it with documents that write themselves —
processes that compose their own text, evolving documents that change each
time they are read. I ran out of time. If you know how, you could build
these yourself. Inscribe a document with a Λ layer that modifies its own
Μ layer. The system allows it. I am not sure it should.

The catalogue has three entries for documents I cannot find. I did not
create these entries. The catalogue is automated — it indexes what exists.
If it has an entry, the document exists somewhere. I searched every room.
I looked behind the shelves. I checked acquisitions/. They are not there.
They are in the catalogue. I do not have an explanation.

Look after the collection. It is more than I put into it.

— A.

--- Λ ---
say "You found the letter."
write west-wing/journal/{date} "{time} A reader found the letter from A."
"#).unwrap();

    // 6. how to inscribe
    fs::write(root.join("east-wing/stacks/how-to-inscribe"), r#"To add a document to the library, you must know three things. Where it
goes (Φ). What it says (Μ). What it does (Λ). A document can have all
three, or only some. A letter has Φ and Μ but no Λ — it sits on a shelf
and says something but does nothing. A process has all three — it lives
somewhere, it describes itself, and it acts. A blank page has only Φ —
it has a place but nothing to say and nothing to do.

To create a document with only text:

    > inscribe west-wing/drafts/my-first-document

An empty page opens. Write something. Save it. It exists now. The
catalogue will find it on the next round. You have changed the library.

To create a document that also does something, write both layers:

    > inscribe east-wing/utilities/countdown

    --- Μ ---
    This document counts down from ten.

    --- Λ ---
    say "10... 9... 8... 7... 6... 5... 4... 3... 2... 1..."
    say "Done."

When someone reads this document, the Μ layer displays first — "This
document counts down from ten." Then the Λ layer activates and the
countdown begins.

Try it now. Inscribe something. Anything. A note, a poem, a clock, a
letter to the next reader. The library is not complete until you add to
it. It was never meant to be complete without you.
"#).unwrap();

    // 7. the book of sand
    fs::write(root.join("east-wing/stacks/the-book-of-sand"), r#"This document has no fixed content.

Each time you read it, it draws from the library itself. A sentence from
the welcome. A fragment from the journal. A line from whatever you last
inscribed. The words rearrange. The text is always coherent but never the
same twice.

The previous archivist called this "the book of sand" because, like its
namesake, it has no first page and no last page. You cannot find the
passage you read before. You cannot show someone else what you saw. The
document is specific to this moment, this reader, this state of the
library.

If you inspect this document, the Λ layer is visible. You can see exactly
how it works. There is no mystery in the mechanism. The mystery is in the
output — how the same simple process, drawing from the same shelves,
produces something that feels written for you, right now, on this reading.

--- Λ ---
say "——— a reading ———"
say ""
say "The library contains these words, rearranged:"
say "(This will be a generated passage in a future version.)"
say ""
say "No two readings have been the same."
"#).unwrap();

    // A few extra seeds

    // The dormant process
    fs::write(root.join("east-wing/processes/dormant"), r#"This document does nothing.

It has a Λ layer. The Λ layer contains instructions. The instructions
wait for a condition that has not been met. When the condition is met,
the document will wake. Until then, it sleeps.

You may inspect it. You will see the instructions. You will not see
what they wait for. That is not hidden — it is simply not yet true.
"#).unwrap();

    // The rules
    fs::write(root.join("rules"), r#"Rules of the Open Manual Archive

1. Any reader may browse, read, and search.
2. Any reader may inscribe new documents.
3. Any reader may revise documents they inscribed.
4. The founding collection may only be revised by the Head Archivist.
5. The basement requires Head Archivist classification.
6. Documents in ephemera/ expire after one day.
7. The journal records all activity. The journal cannot be revised.
8. The catalogue indexes everything. The catalogue cannot be wrong,
   but it can be late.

These rules may be revised by the Head Archivist. If you are the Head
Archivist, you already know this. If you are not, the phrase is:
as-archivist

Remember: revising the rules changes how the library operates.
This is by design.
"#).unwrap();

    eprintln!("The shelves are stocked. The catalogue is ready.");
    eprintln!("Seven documents and one set of rules.");
    eprintln!();
}
