'use client';

import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { Badge } from '@/components/ui/badge';
import { CodeBlock } from '@/components/ui/code-block';

const foundingDocs = [
  { name: 'welcome', desc: 'The door opens. Five words to begin.', type: 'still' },
  { name: 'catalogue', desc: 'The index that lists itself.', type: 'still' },
  { name: 'the-garden-of-forking-paths', desc: 'The fork: library, book, or program?', type: 'living' },
  { name: 'the-book-of-sand', desc: 'Different text every time you read it.', type: 'living' },
  { name: 'the-erosion', desc: 'Loses a line each reading until empty.', type: 'living' },
  { name: 'the-confession', desc: 'Read once. Then it\'s gone.', type: 'living' },
  { name: 'the-letter', desc: 'From archivist A. Three documents are missing.', type: 'living' },
  { name: 'the-mirror-letter', desc: 'Knows how many times you\'ve visited.', type: 'living' },
  { name: 'the-dreamer', desc: 'Generates dream documents while you sleep.', type: 'living' },
  { name: 'lesson-1', desc: 'Creates lesson-2 when you read it.', type: 'living' },
];

export default function LandingScreen() {
  return (
    <div className="min-h-screen bg-background">
      {/* Nav */}
      <header className="border-b">
        <div className="mx-auto flex max-w-5xl items-center justify-between px-5 py-3">
          <div className="flex items-center gap-3">
            <span className="text-2xl font-bold tracking-tight">ΦΜΛ</span>
            <Separator orientation="vertical" className="h-5" />
            <span className="text-sm text-muted-foreground">The Open Manual Archive</span>
          </div>
          <nav className="flex items-center gap-4 text-sm">
            <Link href="/docs" className="text-muted-foreground hover:text-foreground transition-colors">Manual</Link>
            <Link href="https://github.com/Beach-Bum/phi-mu-lambda" className="text-muted-foreground hover:text-foreground transition-colors">Source</Link>
          </nav>
        </div>
      </header>

      {/* Hero */}
      <section className="mx-auto max-w-5xl px-5 py-16">
        <div className="max-w-2xl">
          <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">
            A library that is also an operating system.
          </h1>
          <p className="mt-4 text-lg text-muted-foreground leading-relaxed">
            Boot a terminal. You are standing in an entrance hall. The shelves are full.
            A document rests on the desk. You can read it, explore the rooms, write your
            own documents, and rebuild the library from the inside. No two libraries
            will ever be the same.
          </p>
          <div className="mt-8 flex gap-3">
            <Button asChild size="lg">
              <Link href="/docs/quickstart">Get started</Link>
            </Button>
            <Button variant="outline" size="lg" asChild>
              <Link href="/docs">Read the manual</Link>
            </Button>
          </div>
        </div>
      </section>

      <Separator />

      {/* Install */}
      <section className="mx-auto max-w-5xl px-5 py-12">
        <div className="grid gap-8 md:grid-cols-2">
          <div>
            <h2 className="text-sm font-medium text-muted-foreground uppercase tracking-wider">Install</h2>
            <div className="mt-4 rounded-lg border bg-muted/40 p-4 font-mono text-sm">
              <p className="text-muted-foreground">$ git clone https://github.com/Beach-Bum/phi-mu-lambda.git</p>
              <p className="text-muted-foreground">$ cd phi-mu-lambda && cargo build --release</p>
              <p className="text-foreground font-semibold mt-2">$ oma</p>
            </div>
            <p className="mt-3 text-sm text-muted-foreground">
              One binary. 800KB. No dependencies beyond a terminal.
            </p>
          </div>
          <div>
            <h2 className="text-sm font-medium text-muted-foreground uppercase tracking-wider">First words</h2>
            <div className="mt-4 rounded-lg border bg-muted/40 p-4 font-mono text-sm space-y-1">
              <p className="text-muted-foreground">{'>'} <span className="text-foreground">browse</span> — look at the shelves</p>
              <p className="text-muted-foreground">{'>'} <span className="text-foreground">walk east wing</span> — move to a room</p>
              <p className="text-muted-foreground">{'>'} <span className="text-foreground">read the letter</span> — read a document</p>
              <p className="text-muted-foreground">{'>'} <span className="text-foreground">inscribe my-note</span> — write something new</p>
              <p className="text-muted-foreground">{'>'} <span className="text-foreground">inspect welcome</span> — see all three registers</p>
            </div>
          </div>
        </div>
      </section>

      <Separator />

      {/* Three registers */}
      <section className="mx-auto max-w-5xl px-5 py-12">
        <h2 className="text-2xl font-bold">Three registers. Every document.</h2>
        <p className="mt-2 text-muted-foreground max-w-2xl">
          Every document in the library carries three layers. You see what you ask to see.
        </p>
        <div className="mt-8 grid gap-6 md:grid-cols-3">
          <div className="rounded-lg border p-5">
            <div className="text-2xl font-bold">Φ</div>
            <div className="text-sm font-medium mt-1">Form</div>
            <p className="mt-2 text-sm text-muted-foreground">
              Where it is. Its room, its shelf, its place in the classification.
              A document without form is lost — it exists but cannot be found.
            </p>
          </div>
          <div className="rounded-lg border p-5">
            <div className="text-2xl font-bold">Μ</div>
            <div className="text-sm font-medium mt-1">Message</div>
            <p className="mt-2 text-sm text-muted-foreground">
              What it says. The text you read. A document without message is blank —
              it has a place but nothing to say.
            </p>
          </div>
          <div className="rounded-lg border p-5">
            <div className="text-2xl font-bold">Λ</div>
            <div className="text-sm font-medium mt-1">Lambda</div>
            <p className="mt-2 text-sm text-muted-foreground">
              What it does. Logic that runs when you read it. A document without
              lambda is still — it says something but does nothing.
            </p>
          </div>
        </div>
      </section>

      <Separator />

      {/* Three readers */}
      <section className="mx-auto max-w-5xl px-5 py-12">
        <h2 className="text-2xl font-bold">Three readers. One library.</h2>
        <p className="mt-2 text-muted-foreground max-w-2xl">
          Same shelves. Same documents. Different experiences. Nobody gets a lesser version.
        </p>
        <div className="mt-8 grid gap-6 md:grid-cols-3">
          <div className="rounded-lg border p-5">
            <div className="text-sm font-medium text-muted-foreground uppercase tracking-wider">Curious</div>
            <p className="mt-2 text-sm">
              Explore rooms. Read stories. Find the locked basement. Discover a letter from
              a previous archivist mentioning documents that can&apos;t be found. Write your
              own and watch it appear on the shelves.
            </p>
          </div>
          <div className="rounded-lg border p-5">
            <div className="text-sm font-medium text-muted-foreground uppercase tracking-wider">Contemplative</div>
            <p className="mt-2 text-sm">
              The catalogue that lists itself and is therefore always incomplete.
              Documents whose logic is encoded in their whitespace. A story that
              erodes with each reading. The Borges debt is structural.
            </p>
          </div>
          <div className="rounded-lg border p-5">
            <div className="text-sm font-medium text-muted-foreground uppercase tracking-wider">Constructive</div>
            <p className="mt-2 text-sm">
              Rooms are directories. Documents are files with executable margins.
              The Λ layer is a scripting language. Rewrite the rules and the system
              changes. The library is its own source code.
            </p>
          </div>
        </div>
      </section>

      <Separator />

      {/* Founding collection */}
      <section className="mx-auto max-w-5xl px-5 py-12">
        <h2 className="text-2xl font-bold">The founding collection</h2>
        <p className="mt-2 text-muted-foreground">
          Documents on the shelves when the library first opens. Some are still. Some are alive.
        </p>
        <div className="mt-6 space-y-1">
          {foundingDocs.map((doc) => (
            <div key={doc.name} className="flex items-center justify-between rounded-md px-3 py-2 hover:bg-muted/50 transition-colors">
              <div className="flex items-center gap-3">
                <code className="text-sm font-mono">{doc.name}</code>
                <span className="text-sm text-muted-foreground">{doc.desc}</span>
              </div>
              <Badge variant={doc.type === 'living' ? 'default' : 'secondary'} className="text-xs">
                {doc.type}
              </Badge>
            </div>
          ))}
        </div>
      </section>

      <Separator />

      {/* Living documents */}
      <section className="mx-auto max-w-5xl px-5 py-12">
        <h2 className="text-2xl font-bold">Things that should not be possible</h2>
        <div className="mt-6 grid gap-4 md:grid-cols-2">
          <div className="rounded-lg border p-5">
            <div className="font-medium">Documents that erode</div>
            <p className="mt-1 text-sm text-muted-foreground">
              A story that loses a line each time it&apos;s read. Twelve readings
              and only the title remains. The logic survives after the text is gone.
            </p>
          </div>
          <div className="rounded-lg border p-5">
            <div className="font-medium">Documents that self-destruct</div>
            <p className="mt-1 text-sm text-muted-foreground">
              Read once, then deleted. The catalogue still lists it. The shelves
              don&apos;t have it. A phantom in the index.
            </p>
          </div>
          <div className="rounded-lg border p-5">
            <div className="font-medium">Documents that create documents</div>
            <p className="mt-1 text-sm text-muted-foreground">
              Read lesson 1 and lesson 2 appears on the shelf. The curriculum
              writes itself as you study it.
            </p>
          </div>
          <div className="rounded-lg border p-5">
            <div className="font-medium">A library that dreams</div>
            <p className="mt-1 text-sm text-muted-foreground">
              The dreamer reads random fragments from across the collection and
              inscribes new documents in ephemera. The library authors itself.
            </p>
          </div>
        </div>
      </section>

      <Separator />

      {/* The name */}
      <section className="mx-auto max-w-5xl px-5 py-12">
        <h2 className="text-2xl font-bold">The name</h2>
        <p className="mt-4 text-muted-foreground max-w-2xl leading-relaxed">
          <strong>ΦΜΛ</strong> — Phi Mu Lambda. Form, Message, Function.
          In Dutch, <em>oma</em> means grandmother — the keeper of stories,
          the one whose house you explore as a child, opening drawers, finding
          things that don&apos;t quite make sense yet. Both meanings are always present.
        </p>
      </section>

      {/* Footer */}
      <footer className="border-t py-6">
        <div className="mx-auto max-w-5xl px-5 flex items-center justify-between text-sm text-muted-foreground">
          <span>ΦΜΛ — The Open Manual Archive</span>
          <div className="flex gap-4">
            <Link href="/docs" className="hover:text-foreground transition-colors">Manual</Link>
            <Link href="https://github.com/Beach-Bum/phi-mu-lambda" className="hover:text-foreground transition-colors">GitHub</Link>
          </div>
        </div>
      </footer>
    </div>
  );
}
