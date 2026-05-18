#!/usr/bin/env python3
"""
ΦΜΛ (Phi Mu Lambda) Reference Interpreter — Version 0.2
Implements ΦΜΛ Language Specification v0.1

Usage:
    python3 phi-mu-lambda-interpreter.py <program.pml>
    python3 phi-mu-lambda-interpreter.py --test
    python3 phi-mu-lambda-interpreter.py --verbose <program.pml>

Finding aid written to <program>.faid (stderr fallback).

Implementation notes vs. spec:
  - | : Befunge-style vertical conditional (pop; 0→south, else→north)
  - _ : Befunge-style horizontal conditional (pop; 0→east, else→west)
  - Multi-digit number literals: adjacent digits collapse to one integer push
  - Greek prefix opcodes (ΦX ΜX ΛX) handled at execution time, not parse time,
    preserving correct 2D column alignment
"""

import sys, os, random, hashlib, datetime, argparse
from dataclasses import dataclass, field
from typing import Optional, Any
from io import StringIO

# ── Unicode constants ──────────────────────────────────────────────────────────
PHI   = '\u03a6'   # Φ
MU    = '\u039c'   # Μ
LAMBD = '\u039b'   # Λ
CORN_L = '\u231c'  # ⌜ comment open
CORN_R = '\u231d'  # ⌝ comment close

# ── Directions ─────────────────────────────────────────────────────────────────
EAST  = ( 1,  0)
WEST  = (-1,  0)
NORTH = ( 0, -1)
SOUTH = ( 0,  1)
DIRS  = [EAST, WEST, NORTH, SOUTH]

# ── Degrees ────────────────────────────────────────────────────────────────────
D1, D2, D3 = 1, 2, 3
DEGREE_NAME = {1: 'FIRST', 2: 'SECOND', 3: 'THIRD'}

# ── Integer bounds ─────────────────────────────────────────────────────────────
I64_MAX =  9223372036854775807
I64_MIN = -9223372036854775808

def clamp(v): return max(I64_MIN, min(I64_MAX, int(v)))

# ── Mutation table ─────────────────────────────────────────────────────────────
def _build_mut():
    t = {chr(i): chr(i) for i in range(128)}
    # Direction rotation
    for a, b in [('>','v'),('v','<'),('<','^'),('^','>')]:
        t[a] = b
    # Arithmetic cycle
    for a, b in [('+','-'),('-','*'),('*','/'),(  '/','`'),('`','+')]:
        t[a] = b
    # I/O toggle
    t['.'] = ','; t[','] = '.'
    # Terminus → NOP
    t['@'] = ' '
    # Conditional toggle (| ↔ _ swap roles after mutation)
    t['|'] = '_'; t['_'] = '|'
    # Branch toggle
    t['j'] = 'J'; t['J'] = 'j'
    # Request block toggle
    t['{'] = '}'; t['}'] = '{'
    # Frame push/pop toggle
    t['['] = ']'; t[']'] = '['
    # Bridge → NOP
    t['#'] = ' '
    # Structural ops
    t['W'] = 'G'; t['S'] = 'R'; t['E'] = 'D'
    # Compare cycle
    t['='] = '('; t['('] = ')'; t[')'] = '='
    # Reverse toggle
    t['!'] = '?'; t['?'] = '!'
    return t

MUT = _build_mut()
NOP_AGE_LIMIT = 18

# ── Provenance ─────────────────────────────────────────────────────────────────
@dataclass
class Prov:
    x: int; y: int; cycle: int; author: str; digest: str
    tx_count: int = 0
    annotations: list = field(default_factory=list)

# ── Stack items ────────────────────────────────────────────────────────────────
@dataclass
class Item:
    val: Any          # int or bytes
    kind: str         # 'int' | 'bytes'
    prov: Optional[Prov] = None

    def as_int(self) -> int:
        if self.kind == 'int':
            return int(self.val)
        if self.kind == 'bytes':
            b = self.val
            if not b: return 0
            return int.from_bytes(b[:8], 'little', signed=True)
        return 0

    def as_bytes(self) -> bytes:
        if self.kind == 'bytes': return self.val
        v = self.val
        try: return v.to_bytes(8, 'little', signed=True)
        except Exception: return str(v).encode()

    def as_str(self) -> str:
        if self.kind == 'bytes':
            try: return self.val.decode('utf-8', errors='replace')
            except: return repr(self.val)
        return str(self.val)

# ── Grid cell ──────────────────────────────────────────────────────────────────
@dataclass
class Cell:
    op: str           # current opcode (may differ from original after mutation)
    orig: str         # original opcode from parse
    frozen: bool = False
    wall: bool = False
    mutation_count: int = 0
    last_exec: int = -1
    nop_age: int = 0

# ── Finding Aid ────────────────────────────────────────────────────────────────
class FindingAid:
    def __init__(self, name: str, path: Optional[str]):
        self.name = name
        self.path = path
        self.start = datetime.datetime.now(datetime.timezone.utc).isoformat()
        self.s1, self.s2, self.s3, self.s4 = [], [], [], []
        self.schisms = []
        self.mutations = []

    def log(self, series: str, cycle: int, msg: str, sev: str = 'NOTICE'):
        e = f"  [{cycle:06d}] {sev}: {msg}"
        getattr(self, series, self.s3).append(e)

    def schism(self, cycle, pos, op, dissenters, reasons, polite, sev):
        self.schisms.append({'cycle': cycle, 'pos': pos, 'op': op})
        lines = [f"  [{cycle:06d}] SCHISM({sev}) at {pos} op={repr(op)} polite={polite}"]
        for d, r in zip(dissenters, reasons):
            lines.append(f"    {d}: DISSENT — {r}")
        ref = f"SR-{len(self.schisms):03d}"
        lines.append(f"    Ref: {ref}")
        self.s3.append('\n'.join(lines))
        return ref

    def mutation(self, cycle, pos, old, new):
        self.mutations.append(f"  [{cycle:06d}] MUT {pos}: {repr(old)}→{repr(new)}")

    def finalize(self, degree, exit_cond, cycles, exit_code, term_hash):
        blocks = [
            "═"*72, "FINDING AID", "═"*72,
            f"Accession:    {self.name}",
            f"Date:         {self.start}",
            f"Degree:       {DEGREE_NAME.get(degree,'?')}",
            f"Exit:         {exit_cond}  (code {exit_code})",
            f"Cycles:       {cycles}",
            f"Schisms:      {len(self.schisms)}",
            f"Mutations:    {len(self.mutations)}",
            f"Term.hash:    {term_hash}",
            "", "─"*72, "SERIES 1: STRUCTURAL DECLARATIONS", "─"*72,
        ]
        blocks += self.s1 or ["  (none — UNDESCRIBED COLLECTION)"]
        blocks += ["", "─"*72, "SERIES 2: POPULATION EVENTS", "─"*72]
        blocks += self.s2 or ["  (none)"]
        blocks += ["", "─"*72, "SERIES 3: EXECUTION LOG", "─"*72]
        blocks += self.s3 or ["  (none)"]
        blocks += ["", "─"*72, "SERIES 4: MUTATIONS (first 30)", "─"*72]
        blocks += self.mutations[:30] or ["  (none)"]
        if len(self.mutations) > 30:
            blocks.append(f"  ... {len(self.mutations)-30} more")
        blocks += ["", "═"*72]
        text = '\n'.join(blocks) + '\n'
        if self.path:
            try:
                with open(self.path, 'w', encoding='utf-8') as f: f.write(text)
                return
            except OSError: pass
        print(text, file=sys.stderr)

# ── Parser ─────────────────────────────────────────────────────────────────────
def parse(source: str) -> list:
    """
    Parse source into 2D list[list[Cell]].
    One source character = one grid cell, EXCEPT:
      - Adjacent digit characters merge into a single NUM:N cell
      - ⌜...⌝ comments are stripped (replaced with spaces to preserve column positions)
      - "..." string content is stored as SC:ch cells (one per char)
    This preserves 2D alignment: Greek letters (Φ Μ Λ) occupy their own cells.
    """
    # Strip comments (replace with spaces to preserve column positions)
    out = []
    i = 0
    while i < len(source):
        if source[i] == CORN_L:
            j = source.find(CORN_R, i+1)
            if j == -1:
                out.append(' ' * (len(source) - i)); i = len(source)
            else:
                out.append(' ' * (j - i + 1)); i = j + 1
        else:
            out.append(source[i]); i += 1
    source = ''.join(out)

    rows = source.split('\n')
    grid = []
    for raw_row in rows:
        cells = []
        j = 0
        in_str = False
        while j < len(raw_row):
            ch = raw_row[j]
            # String mode toggle
            if ch == '"':
                cells.append(Cell(op='"', orig='"'))
                in_str = not in_str
                j += 1
                continue
            if in_str:
                # Escaped quote
                if ch == '\\' and j+1 < len(raw_row) and raw_row[j+1] == '"':
                    cells.append(Cell(op='SC:"', orig='SC:"'))
                    j += 2
                else:
                    cells.append(Cell(op=f'SC:{ch}', orig=f'SC:{ch}'))
                    j += 1
                continue
            # Multi-digit numbers: consume adjacent digit chars as one token
            if ch.isdigit():
                num = ch
                j += 1
                while j < len(raw_row) and raw_row[j].isdigit():
                    num += raw_row[j]; j += 1
                op = f'NUM:{num}' if len(num) > 1 else num
                cells.append(Cell(op=op, orig=op))
                continue
            # Everything else: one char → one cell
            cells.append(Cell(op=ch, orig=ch))
            j += 1
        if in_str:
            cells.append(Cell(op='"', orig='"'))
        grid.append(cells)

    # Normalize to rectangle, minimum 8×8
    w = max(8, max((len(r) for r in grid), default=0))
    h = max(8, len(grid))
    for r in grid:
        while len(r) < w:
            r.append(Cell(op=' ', orig=' '))
    while len(grid) < h:
        grid.append([Cell(op=' ', orig=' ') for _ in range(w)])
    return grid

# ── Consensus ──────────────────────────────────────────────────────────────────
class Consensus:
    def __init__(self, fa: FindingAid):
        self.fa = fa
        self.schism_counts: dict = {}
        self.fatal = False
        self.total = 0
        self.polite = 0

    def check(self, cycle, pos, op, polite, phi_fn, mu_fn, lam_fn) -> bool:
        self.total += 1
        if polite: self.polite += 1
        p_ok, p_why = phi_fn()
        m_ok, m_why = mu_fn()
        l_ok, l_why = lam_fn()
        if p_ok and m_ok and l_ok:
            return True
        ds, rs = [], []
        if not p_ok: ds.append('PHI');    rs.append(p_why)
        if not m_ok: ds.append('MU');     rs.append(m_why)
        if not l_ok: ds.append('LAMBDA'); rs.append(l_why)
        cnt = self.schism_counts.get(pos, 0) + 1
        self.schism_counts[pos] = cnt
        sev = ['MINOR','MODERATE','ESCALATED','FATAL'][min(cnt-1,3)]
        self.fa.schism(cycle, pos, op, ds, rs, polite, sev)
        if sev == 'FATAL':
            self.fatal = True
            self.fa.log('s3', cycle, f'FATAL SCHISM at {pos}', 'FATAL')
        return False

# ── Interpreter ────────────────────────────────────────────────────────────────
class PML:
    def __init__(self, source: str, name='program', faid_path=None,
                 stdin=None, stdout=None, max_cycles=100_000, verbose=False):
        self.name = name
        self.max_cycles = max_cycles
        self.verbose = verbose
        self.stdin  = stdin  or sys.stdin
        self.stdout = stdout or sys.stdout
        self.fa  = FindingAid(name, faid_path)
        self.con = Consensus(self.fa)

        # Grid
        self.grid   = parse(source)
        self.height = len(self.grid)
        self.width  = len(self.grid[0])

        # IP
        self.x, self.y = 0, 0
        self.dir = EAST

        # State
        self.degree   = D1
        self.stack: list[Item] = []
        self.frames: list     = []
        self.shelves: dict    = {}
        self.funcs: dict      = {}
        self.calls: list      = []

        # Flags
        self.halted    = False
        self.exit_code = 0
        self.exit_cond = 'TERMINUS'
        self.cycle     = 0
        self.in_str    = False
        self.nop_ages: dict = {}
        self.all_mutated = False
        self.entropy_cycle = -1

        self.fa.log('s1', 0, f'Grid {self.width}×{self.height} parsed', 'NOTICE')

    # ── Grid helpers ───────────────────────────────────────────────────────
    def cell(self, x, y) -> Cell:
        return self.grid[y % self.height][x % self.width]

    def wrap(self, x, y):
        return x % self.width, y % self.height

    def peek_next(self) -> Cell:
        """Return the cell one step ahead of IP in current direction."""
        dx, dy = self.dir
        return self.cell(self.x + dx, self.y + dy)

    def step(self):
        dx, dy = self.dir
        nx, ny = self.wrap(self.x + dx, self.y + dy)
        if self.cell(nx, ny).wall:
            self.dir = (-dx, -dy)
            nx, ny = self.wrap(self.x - dx, self.y - dy)
        self.x, self.y = nx, ny

    # ── Stack helpers ──────────────────────────────────────────────────────
    def _prov(self, author='MU', content=b'') -> Prov:
        d = hashlib.sha256(content).hexdigest()[:16]
        return Prov(self.x, self.y, self.cycle, author, d)

    def push_int(self, v: int, author='LAMBDA'):
        v = clamp(v)
        self.stack.append(Item(val=v, kind='int',
                               prov=self._prov(author, str(v).encode())))

    def push_bytes(self, b: bytes, author='MU'):
        self.stack.append(Item(val=b, kind='bytes',
                               prov=self._prov(author, b)))

    def push_str(self, s: str, author='MU'):
        self.push_bytes(s.encode('utf-8'), author)

    def pop(self) -> Item:
        if not self.stack:
            self._err('E005', 'STACK EXHAUSTED', fatal=True)
            return Item(val=0, kind='int')
        return self.stack.pop()

    def pop_int(self) -> int:
        return self.pop().as_int()

    def peek_top(self) -> Optional[Item]:
        return self.stack[-1] if self.stack else None

    # ── Error ──────────────────────────────────────────────────────────────
    def _err(self, code, msg, fatal=False):
        sev = 'FATAL' if fatal else 'ERROR'
        self.fa.log('s3', self.cycle, f'{code} {msg}', sev)
        if self.verbose:
            print(f'[{sev}] {code} {msg}', file=sys.stderr)
        if fatal:
            self.halted = True; self.exit_cond = code; self.exit_code = 1

    # ── Consensus wrapper ──────────────────────────────────────────────────
    def _ok(self):
        return lambda: (True, 'nominal')

    def _need(self, n):
        def f():
            if len(self.stack) >= n: return True, 'stack ok'
            return False, f'stack needs {n}, has {len(self.stack)}'
        return f

    def _deg(self, *ds):
        def f():
            if self.degree in ds: return True, 'degree ok'
            return False, f'need degree {ds}, have {self.degree}'
        return f

    def _cons(self, op, polite, phi=None, mu=None, lam=None) -> bool:
        pos = (self.x, self.y)
        return self.con.check(self.cycle, pos, op, polite,
                              phi or self._ok(), mu or self._ok(), lam or self._ok())

    # ── Mutation ───────────────────────────────────────────────────────────
    def _mutate(self, x, y):
        c = self.cell(x, y)
        if c.frozen: return
        op = c.op
        # Only single-char printable ops mutate
        if len(op) != 1: return
        if op == ' ':
            pos = (x, y)
            age = self.nop_ages.get(pos, 0) + 1
            self.nop_ages[pos] = age
            if age >= NOP_AGE_LIMIT:
                c.op = '?'; c.mutation_count += 1
                self.nop_ages[pos] = 0
                self.fa.mutation(self.cycle, (x,y), ' ', '?')
            return
        nop = MUT.get(op, op)
        if nop != op:
            c.op = nop; c.mutation_count += 1
            self.fa.mutation(self.cycle, (x,y), op, nop)

    def _check_entropy(self):
        if self.all_mutated:
            if self.entropy_cycle >= 0 and self.cycle - self.entropy_cycle > 1000:
                self._err('E013', 'ENTROPIC DISSOLUTION', fatal=True)
            return
        for row in self.grid:
            for c in row:
                if c.mutation_count == 0 and c.orig not in (' ', '\t'):
                    return
        self.all_mutated = True
        self.entropy_cycle = self.cycle
        self.fa.log('s3', self.cycle, 'E012 ENTROPY THRESHOLD REACHED', 'NOTICE')

    # ── Execute one cell ───────────────────────────────────────────────────
    def exec_cell(self):
        x, y = self.x, self.y
        c = self.cell(x, y)
        op = c.op
        c.last_exec = self.cycle

        if self.verbose and op not in (' ', '\t'):
            print(f'  [{self.cycle:5d}] ({x:3d},{y:2d}) {repr(op):<12} '
                  f'deg={self.degree} stk={len(self.stack)}', file=sys.stderr)

        # ── String mode ────────────────────────────────────────────────────
        if op == '"':
            self.in_str = not self.in_str
            return
        if self.in_str:
            if op.startswith('SC:'):
                ch = op[3:]
                self.push_int(ord(ch) if ch else 0, author='MU')
            elif op == ' ':
                self.push_int(32, author='MU')
            elif len(op) == 1:
                self.push_int(ord(op), author='MU')
            return

        # ── NUM:N literal ──────────────────────────────────────────────────
        if op.startswith('NUM:'):
            n = int(op[4:])
            self.push_int(n, author='MU')
            return

        # ── SC: outside string mode: treat as NOP ─────────────────────────
        if op.startswith('SC:'):
            return

        # ── NOP ────────────────────────────────────────────────────────────
        if op == ' ' or op == '\t':
            return

        # ── Greek prefix: peek next cell, form combined opcode ─────────────
        if op in (PHI, MU, LAMBD):
            nc = self.peek_next()
            combined = op + nc.op
            # Advance IP past the suffix cell (main loop also advances after exec)
            self.step()
            self._exec_prefixed(combined)
            return

        # ── Navigation (all degrees) ────────────────────────────────────────
        if op == '>': self.dir = EAST;  return
        if op == '<': self.dir = WEST;  return
        if op == '^': self.dir = NORTH; return
        if op == 'v': self.dir = SOUTH; return
        if op == '?': self.dir = random.choice(DIRS); return
        if op == '!':
            dx, dy = self.dir; self.dir = (-dx, -dy); return
        if op == '#':
            self.step(); return   # bridge: skip next
        if op == '@':
            self.halted = True; self.exit_cond = 'TERMINUS'
            self.fa.log('s3', self.cycle, 'TERMINUS', 'NOTICE'); return

        # ── Conditional directions (Befunge-style) ─────────────────────────
        if op == '|':
            # Vertical conditional: pop; 0 → SOUTH, else → NORTH
            v = self.pop_int()
            self.dir = SOUTH if v == 0 else NORTH
            return
        if op == '_':
            # Horizontal conditional: pop; 0 → EAST, else → WEST
            v = self.pop_int()
            self.dir = EAST if v == 0 else WEST
            return

        # ── Single digit ───────────────────────────────────────────────────
        if op in '0123456789':
            self.push_int(int(op), author='MU'); return

        # ── Degree-gated ops ───────────────────────────────────────────────
        self._exec_gated(op)

    def _exec_gated(self, op: str):
        """Ops that respect degree restrictions."""
        deg = self.degree

        # ── Mu I/O (degrees 2, 3) ──────────────────────────────────────────
        if op == '.':   # EMIT
            if not self._cons('.', False, mu=self._need(1),
                               lam=self._deg(D2, D3)): return
            v = self.pop_int()
            try:
                ch = chr(v) if 0 <= v <= 0x10FFFF else '?'
            except (ValueError, OverflowError):
                ch = '?'
            self.stdout.write(ch); self.stdout.flush()
            return

        if op == ',':   # INTAKE
            ch = self.stdin.read(1)
            self.push_int(ord(ch) if ch else -1, author='INPUT')
            return

        if op == ':':   # DUP
            if not self.stack:
                self._err('E005', 'DUP on empty stack'); return
            import copy
            self.stack.append(copy.deepcopy(self.stack[-1]))
            return

        if op == '\\':  # EXCHANGE
            if len(self.stack) < 2:
                self._err('E005', 'EXCHANGE needs 2 items'); return
            self.stack[-1], self.stack[-2] = self.stack[-2], self.stack[-1]
            return

        if op == '$':   # DISCARD
            if self.stack: self.stack.pop()
            return

        if op == '%':   # PROVENANCE
            if not self.stack: return
            t = self.stack[-1]
            p = t.prov
            meta = (f"origin=({p.x},{p.y},c{p.cycle}) author={p.author}"
                    if p else "NO PROVENANCE")
            self.push_str(meta)
            return

        if op == 'A':   # ANNOTATE
            if len(self.stack) < 2: return
            ann = self.pop()
            top = self.stack[-1]
            if top.prov:
                top.prov.annotations.append((self.cycle, 'MU', ann.as_str()[:80]))
            return

        if op == 'X':   # EXTRACT
            payload = b''
            try:
                import xattr  # type: ignore
                payload = xattr.getxattr(os.getcwd(), 'user.phimulambda')
            except Exception:
                pass
            if payload:
                self.push_bytes(payload, 'INPUT')
                self.fa.log('s3', self.cycle, 'EXTRACT — payload from xattr', 'NOTICE')
            else:
                self.push_bytes(b'', 'MU')
                self.fa.log('s3', self.cycle, 'E025 SOURCE ABSENT', 'WARNING')
            return

        if op == 'I':   # INSCRIBE
            if not self.stack:
                self._err('E005', 'INSCRIBE on empty stack'); return
            item = self.pop()
            payload = item.as_bytes()
            written = False
            try:
                import xattr  # type: ignore
                xattr.setxattr(os.getcwd(), 'user.phimulambda', payload)
                self.fa.log('s3', self.cycle,
                            f'INSCRIBE {len(payload)}B → xattr', 'NOTICE')
                written = True
            except Exception:
                pass
            if not written:
                print(f'[INSCRIBE:{len(payload)}B] {payload!r}', file=sys.stderr)
                self.fa.log('s3', self.cycle, 'E022 PLAINTEXT FALLBACK CHANNEL', 'WARNING')
            return

        if op == 'K':   # FINDING AID REF
            self.push_str(f'FAID:{self.name}:C{self.cycle}')
            return

        # ── Phi structural (degree 1) ──────────────────────────────────────
        if op == 'P':   # POSITION
            self.push_int(self.x, 'PHI'); self.push_int(self.y, 'PHI')
            return
        if op == 'D':   # DIMENSION
            self.push_int(self.width, 'PHI'); self.push_int(self.height, 'PHI')
            return
        if op == 'E':   # EXPAND (D1 only)
            if deg != D1:
                self.fa.log('s3', self.cycle, 'E006 DEGREE VIOLATION — E in D2/D3', 'WARNING')
                return
            if len(self.stack) < 2: return
            h = self.pop_int(); w = self.pop_int()
            self._resize(max(8,w), max(8,h))
            self.fa.log('s1', self.cycle, f'EXPAND {w}×{h}', 'NOTICE')
            return
        if op == 'W':   # WALL (D1 only)
            if deg != D1: return
            if len(self.stack) < 2: return
            cy = self.pop_int(); cx = self.pop_int()
            cx, cy = self.wrap(cx, cy)
            self.cell(cx, cy).wall = True
            self.fa.log('s1', self.cycle, f'WALL ({cx},{cy})', 'NOTICE')
            return
        if op == 'S':   # SHELVE
            if len(self.stack) < 3: return
            nm = self.pop(); cy = self.pop_int(); cx = self.pop_int()
            name = nm.as_str()
            self.shelves[name] = self.wrap(cx, cy)
            self.fa.log('s1', self.cycle, f'SHELVE "{name}"→({cx},{cy})', 'NOTICE')
            return
        if op == 'R':   # RETRIEVE
            if not self.stack: return
            nm = self.pop(); name = nm.as_str()
            if name in self.shelves:
                self.x, self.y = self.shelves[name]
            else:
                self._err('E004', f'UNINITIATED PROCESS — no shelf "{name}"')
            return
        if op == 'G':   # GOTO
            if not self._cons('G', False, mu=self._need(2)): return
            cy = self.pop_int(); cx = self.pop_int()
            self.x, self.y = self.wrap(cx, cy)
            return
        if op == '[':   # PUSH FRAME
            self.frames.append({'x':self.x,'y':self.y,'dir':self.dir,'deg':self.degree})
            return
        if op == ']':   # POP FRAME
            if self.frames:
                f = self.frames.pop()
                self.x, self.y, self.dir, self.degree = f['x'],f['y'],f['dir'],f['deg']
            return

        # ── Lambda compute (degree 3) ──────────────────────────────────────
        if op in '+-*/`=()&~':
            self._arith(op); return
        if op == 'j':   # BRANCH IF ZERO
            self._branch_if_zero(); return
        if op == 'J':   # END BRANCH target
            return
        if op == '{':   return  # request open (tracked implicitly)
        if op == '}':   return  # request close
        if op == 'f':   # CALL FUNCTION
            if not self.stack: return
            nm = self.pop().as_str()
            if nm in self.funcs:
                self.calls.append({'x':self.x,'y':self.y,'dir':self.dir})
                self.x, self.y = self.funcs[nm]['start']
            else:
                self._err('E004', f'UNINITIATED PROCESS — func "{nm}"')
            return
        if op == 'r':   # RETURN
            if self.calls:
                c = self.calls.pop()
                self.x, self.y, self.dir = c['x'], c['y'], c['dir']
            return

    def _exec_prefixed(self, op: str):
        """Handle ΦX, ΜX, ΛX opcodes."""
        prefix, suffix = op[0], op[1:]

        if prefix == LAMBD:
            if suffix == 'A':   # ADVANCE DEGREE
                if self.degree == D3:
                    self._err('E016', 'ADVANCEMENT REFUSED', fatal=True); return
                old = self.degree; self.degree += 1
                series = 's1' if old == D1 else 's2'
                self.fa.log(series, self.cycle,
                            f'ΛA {DEGREE_NAME[old]}→{DEGREE_NAME[self.degree]}', 'NOTICE')
                return
            if suffix == 'E':   # EXIT WITH CODE
                self.exit_code = self.pop_int()
                self.halted = True; self.exit_cond = 'EXIT_CODE'
                return
            if suffix == 'D':   # DEGREE
                self.push_int(self.degree, 'LAMBDA'); return
            if suffix == 'Q':   # QUERY CONSENSUS
                ok = 1 if self.con.schism_counts.get((self.x,self.y),0)==0 else 0
                self.push_int(ok, 'LAMBDA'); return
            if suffix == 'S':   # SCHISM LOG
                if self.con.schisms:
                    self.push_str(str(self.con.schisms[-1]))
                else:
                    self.push_str('NO SCHISMS')
                return
            if suffix == 'M':   # MEDIATE (stub)
                self.fa.log('s3', self.cycle, 'ΛM MEDIATE (stub)', 'NOTICE'); return

        if prefix == MU:
            if suffix == 'R':   # RECORD to FAID
                top = self.peek_top()
                if top:
                    self.fa.log('s3', self.cycle,
                                f'ΜR RECORD: {repr(top.as_str()[:60])}', 'NOTICE')
                return
            if suffix == 'D':   # DIGEST
                item = self.pop()
                b = item.as_bytes()
                self.push_str(hashlib.sha256(b).hexdigest()); return
            if suffix == 'M':   # MARK retention
                ret = self.pop_int()
                top = self.peek_top()
                if top: top.val  # retention stored conceptually; stub
                return
            if suffix == 'T':   # TRANSMIT (stub)
                if len(self.stack) >= 2: self.pop(); self.pop()
                self.fa.log('s3', self.cycle, 'ΜT TRANSMIT (stub)', 'NOTICE'); return
            if suffix == 'H':   # HEAR (stub)
                self.push_bytes(b'', 'INPUT')
                self.fa.log('s3', self.cycle, 'ΜH HEAR (stub)', 'NOTICE'); return

        if prefix == PHI:
            if suffix == 'N':   # NAMED GRID (stub)
                if self.stack: self.pop()
                return
            if suffix == 'F':   # FORM DECLARE
                if len(self.stack) >= 5:
                    nm = self.pop(); x2=self.pop_int(); y2=self.pop_int()
                    x1=self.pop_int(); y1=self.pop_int()
                    self.fa.log('s1', self.cycle,
                                f'ΦF FORM "{nm.as_str()}" ({x1},{y1})→({x2},{y2})', 'NOTICE')
                return
            if suffix == 'C':   # CONSULT FORM
                self.push_int(self.width,'PHI'); self.push_int(self.height,'PHI'); return

    def _arith(self, op: str):
        if op == '~':  # unary not
            if not self._cons('~', False, lam=self._deg(D3), mu=self._need(1)): return
            self.push_int(~self.pop_int())
            return
        if not self._cons(op, False, lam=self._deg(D3), mu=self._need(2)): return
        b = self.pop_int(); a = self.pop_int()
        if op == '+': r = a + b
        elif op == '-': r = a - b
        elif op == '*': r = a * b
        elif op == '/':
            if b == 0: self._err('E007','DIVISION UNDEFINED'); self.push_int(0); return
            r = int(a / b)
        elif op == '`':
            if b == 0: self._err('E007','MODULO UNDEFINED'); self.push_int(0); return
            r = a % b
        elif op == '=': r = 1 if a == b else 0
        elif op == '(': r = 1 if a < b else 0
        elif op == ')': r = 1 if a > b else 0
        elif op == '&': r = a & b
        else: r = a + b
        self.push_int(clamp(r))

    def _branch_if_zero(self):
        """j: pop top; if 0, scan forward in current direction for J."""
        if not self.stack:
            self._err('E005', 'j on empty stack'); return
        val = self.pop_int()
        if val != 0: return
        # Scan forward for matching J
        dx, dy = self.dir
        nx, ny = self.wrap(self.x + dx, self.y + dy)
        depth = 1
        for _ in range(self.width * self.height):
            c = self.cell(nx, ny)
            if c.op == 'j': depth += 1
            elif c.op == 'J':
                depth -= 1
                if depth == 0:
                    self.x, self.y = nx, ny
                    return
            nx, ny = self.wrap(nx + dx, ny + dy)
        self.fa.log('s3', self.cycle, 'j: no matching J found', 'WARNING')

    def _resize(self, w, h):
        for row in self.grid:
            while len(row) < w:
                row.append(Cell(op=' ', orig=' '))
        while len(self.grid) < h:
            self.grid.append([Cell(op=' ', orig=' ') for _ in range(w)])
        self.width = w; self.height = h

    # ── Main loop ──────────────────────────────────────────────────────────
    def run(self) -> int:
        while not self.halted and self.cycle < self.max_cycles:
            x, y = self.x, self.y
            self.exec_cell()
            if self.halted: break
            if self.con.fatal: self.halted=True; self.exit_cond='FATAL_SCHISM'; break
            self._mutate(x, y)
            self.step()
            self.cycle += 1
            if self.cycle % 1000 == 0:
                self._check_entropy()

        if self.cycle >= self.max_cycles:
            self.exit_cond = 'MAX_CYCLES'
            self.fa.log('s3', self.cycle, f'MAX CYCLES ({self.max_cycles})', 'WARNING')

        # Terminal hash
        grid_bytes = ''.join(c.op for r in self.grid for c in r).encode()
        stk_bytes  = str([(i.kind, i.as_int()) for i in self.stack]).encode()
        th = hashlib.sha256(grid_bytes + stk_bytes).hexdigest()[:32]

        self.fa.finalize(self.degree, self.exit_cond, self.cycle,
                         self.exit_code, th)
        return self.exit_code

# ── Test programs ──────────────────────────────────────────────────────────────
# Design rules:
#   - ΛA occupies 2 cells (Λ at col N, A at col N+1)
#   - Multi-digit nums like 72 are 1 cell
#   - | = vertical conditional: pop; 0→SOUTH, non-zero→NORTH
#   - _ = horizontal conditional: pop; 0→EAST,  non-zero→WEST
#   - All degrees: ΛA ΛA to reach Third Degree immediately

# Hello World:
#   ΛA → D2, push 10(newline) + reversed string (H on top), ΛA → D3, emit 14 times.
#   Stack after Second Degree (top→bottom): H e l l o , sp W o r l d ! \n
#   Pop order: H e l l o , sp W o r l d ! \n  → "Hello, World!\n"
HW = (
    "Hello World",
    'ΛA 10 "!dlroW ,olleH" ΛA . . . . . . . . . . . . . . @',
    '',
    'Hello, World!\n',
)

# Cat: direct read+emit, no loop (avoids | mutation issue)
CAT = (
    "Cat (echo stdin, 5 chars)",
    'ΛA ΛA , . , . , . , . , . @',
    'Hello',
    'Hello',
)

# Self-consuming countdown (5 to 1 then newline):
#   Push in D2: 10(newline) 49(1) 50(2) 51(3) 52(4) 53(5)
#   Stack top→bottom: 53 52 51 50 49 10
#   D3: emit each in pop order → "54321\n"
COUNTDOWN = (
    "Self-consuming countdown",
    'ΛA 10 49 50 51 52 53 ΛA . . . . . . @',
    '',
    '54321\n',
)

# Dead drop: push string in D2, inscribe in D3.
DEAD_DROP = (
    "Dead drop (steganographic write)",
    'ΛA "RENDEZ-VOUS: ZERO THREE HUNDRED" ΛA I @',
    '',
    '',   # no stdout; payload goes to xattr or stderr
)

# Consensus demo: push "OK\n" as numeric literals in D2, record to faid, emit in D3.
#   Push: 10(\n) 75(K) 79(O) → stack top→bottom: 79 75 10
#   ΜR records top item (O) to finding aid without popping.
#   D3: . . . @ → emit O, K, \n → "OK\n"
CONSENSUS = (
    "Consensus demonstration",
    'ΛA 10 75 79 ΛA ΜR . . . @',
    '',
    'OK\n',
)

TESTS = [HW, CAT, COUNTDOWN, DEAD_DROP, CONSENSUS]

# ── Test runner ────────────────────────────────────────────────────────────────
def run_tests(verbose=False):
    print('=' * 72)
    print('ΦΜΛ Reference Interpreter — Test Suite v0.2')
    print('=' * 72)
    passed = failed = 0
    for desc, src, stdin_data, expected_out in TESTS:
        print(f'\n── {desc}')
        if verbose:
            print('   Source:')
            for i, line in enumerate(src.split('\n')):
                print(f'   {i}: {repr(line)}')
        faid = f'/tmp/pml_{desc.split()[0].lower()}.faid'
        out_buf = StringIO()
        try:
            interp = PML(src, name=desc, faid_path=faid,
                         stdin=StringIO(stdin_data), stdout=out_buf,
                         max_cycles=50_000, verbose=verbose)
            code = interp.run()
            out = out_buf.getvalue()
            ok = (expected_out == '' or out == expected_out)
            short = repr(out) if len(out) < 60 else repr(out[:57]) + '...'
            print(f'   Exit: {code}  Output: {short}')
            if expected_out:
                exp_short = repr(expected_out) if len(expected_out)<60 else repr(expected_out[:57])+'...'
                match = '✓' if ok else '✗'
                print(f'   Expected: {exp_short}  {match}')
            if os.path.exists(faid):
                print(f'   FAID: {faid}')
            status = 'PASS' if ok else 'FAIL'
            print(f'   Status: {status}')
            if ok: passed += 1
            else:  failed += 1
        except Exception as e:
            import traceback
            print(f'   Status: ERROR — {e}')
            if verbose: traceback.print_exc()
            failed += 1
    print(f'\n{"="*72}')
    print(f'Results: {passed} passed, {failed} failed')
    print('=' * 72)
    return failed == 0

# ── CLI ────────────────────────────────────────────────────────────────────────
def main():
    ap = argparse.ArgumentParser(description='ΦΜΛ Reference Interpreter')
    ap.add_argument('program', nargs='?')
    ap.add_argument('--test', action='store_true')
    ap.add_argument('--verbose', '-v', action='store_true')
    ap.add_argument('--faid', metavar='PATH')
    ap.add_argument('--max-cycles', type=int, default=100_000)
    args = ap.parse_args()

    if args.test:
        ok = run_tests(verbose=args.verbose)
        sys.exit(0 if ok else 1)

    if not args.program:
        ap.print_help(); sys.exit(1)

    with open(args.program, encoding='utf-8') as f:
        src = f.read()
    name = os.path.splitext(os.path.basename(args.program))[0]
    faid = args.faid or f'{os.path.splitext(args.program)[0]}.faid'
    interp = PML(src, name=name, faid_path=faid,
                 max_cycles=args.max_cycles, verbose=args.verbose)
    sys.exit(interp.run())

if __name__ == '__main__':
    main()
