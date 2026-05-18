# ΦΜΛ Language Specification
## Version 0.1 — Working Draft

**Document Status**: Draft for Implementation
**Date**: 2026-05-18
**Identifier**: ISO/PML/WD-001
**Repository**: This specification is self-hosting; a conformant implementation can parse this document's whitespace as a valid ΦΜΛ program.

---

## 1. Abstract

ΦΜΛ (Phi Mu Lambda) is an esoteric programming language organized around three named registers — Φ — Phi — Form, Μ — Mu — Message, and Λ — Lambda — Function — corresponding to the three phases of an archival workflow: structural declaration, content population, and functional activation. Execution proceeds on a two-dimensional toroidal grid whose instructions mutate after each traversal. No operation may execute without the consensus of all three registers; dissent is logged to a persistent finding aid and may trigger a mediation protocol. Programs may be embedded invisibly in the whitespace of any host document and are therefore simultaneously a text and a shadow text. ΦΜΛ is Turing-complete. ΦΜΛ is hostile. ΦΜΛ does not want to be run.

---

## 2. Design Philosophy

The three registers encode three things that are always already present in any act of information management:

**Φ — Phi — Form** is Ranganathan's insight that classification is not labeling but analysis — the decomposition of a subject into its fundamental facets before any description can begin. Form precedes content. The grid must exist before the message can be placed. Every addressing scheme is an ontological commitment.

**Μ — Mu — Message** is Shannon's formalization and Derrida's deconstruction simultaneously: the message is quantifiable entropy and it is also a political act of archival power. What gets written into the Μ register is what exists. What does not get written does not exist. The I/O layer is steganographic because every channel is already a steganographic act — the medium is not neutral, and the message is never only what it says.

**Λ — Lambda — Function** is Church's lambda calculus in its least comfortable form: pure function, anonymous, consuming its inputs, returning transformed outputs, leaving no state. In ΦΜΛ, Λ does not accumulate — each activation of the Λ register consumes a portion of the Φ structure that supports it. Computation is not free. Every operation degrades the substrate.

The consensus protocol is drawn from INTERCAL's insight that politeness is a computational primitive, and from archival theory's principle of multi-party review: no record is appraised by a single archivist. The schism mechanism formalizes what every collaborative institution already knows — disagreement is not an error state but an information state. Logged dissent is evidence.

Self-modification is drawn from Malbolge and from the archival principle of natural degradation. Records decay. Tapes corrupt. Every executed instruction leaves the cell slightly different from how it was found. A program run long enough in a tight loop will destroy itself from within, instruction by instruction, until only the mutation residue remains.

The Borges debt is structural: the program is the library, the instruction pointer is the librarian, and the finding aid is the only honest document produced.

---

## 3. Lexical Structure

### 3.1 Character Set

ΦΜΛ programs are encoded in UTF-8. The following characters are significant:

**Primary opcodes** (Greek register markers):

| Character | Unicode  | Name  | Role                         |
|-----------|----------|-------|------------------------------|
| Φ         | U+03A6   | PHI   | Form register opcode prefix  |
| Μ         | U+039C   | MU    | Message register opcode prefix |
| Λ         | U+039B   | LAMBDA| Function register opcode prefix |

**Navigation opcodes** (ASCII subset):

```
> < ^ v ? # @ ! | _ [ ] { } . , : \ $ % " 0-9 A-Z a-z
```

All other characters are ignored in explicit mode. In embedded mode (§3.4), all non-whitespace characters are ignored.

**Whitespace** (significant only in embedded mode):

| Character | Hex  | Embedded meaning |
|-----------|------|-----------------|
| SP        | 0x20 | Bit 0           |
| TAB       | 0x09 | Bit 1           |
| LF        | 0x0A | Frame separator |
| CR        | 0x0D | Ignored         |

### 3.2 Tokens

A ΦΜΛ program in explicit mode consists of a sequence of tokens. Tokens are:

1. **Opcode tokens**: a single character from the opcode set (§6)
2. **Register-prefixed opcodes**: a Greek letter (Φ, Μ, Λ) followed immediately by an opcode character, specifying which register's variant to use when opcodes are shared
3. **String literals**: a double-quote character, followed by arbitrary UTF-8 content, terminated by a second double-quote. Embedded double-quotes are escaped with `\"`. String literals are Μ-register tokens only.
4. **Comments**: any character sequence beginning with `⌜` (U+231C) and ending with `⌝` (U+231D). Comments are ignored during parsing. Irony is noted.
5. **PLEASE keyword**: the ASCII string `PLEASE` followed by a single space, preceding any opcode token. Grants politeness status (§7.4).

### 3.3 Explicit Mode Grid Layout

In explicit mode, the program text is laid out on a two-dimensional grid. The parser reads the source, assigns each significant character to a cell at coordinates (col, row) where col is the 0-indexed horizontal position and row is the 0-indexed vertical position. The grid dimensions are determined by the largest col and row values encountered during parsing, padded to a minimum of 8×8.

Empty cells contain the NOP opcode (ASCII 0x20, a space).

### 3.4 Embedded Mode: Whitespace Encoding

In embedded mode, the program is encoded in the whitespace characters of a host document. The host document's non-whitespace content is preserved and irrelevant to execution.

**Encoding scheme**: Each instruction byte is transmitted as 7 consecutive whitespace characters (SP = 0, TAB = 1), encoding the 7-bit ASCII value of the opcode. An LF character terminates each instruction's whitespace sequence and acts as a framing delimiter.

The embedded program begins with the **acquisition stamp**: the sequence SP TAB SP TAB SP TAB LF (decimal 42, ASCII `*`), which must appear in the whitespace of the host document before any program content.

Following the acquisition stamp, the program content is encoded as a sequence of 7-bit frames. Each frame:

```
[b6][b5][b4][b3][b2][b1][b0] LF
```

where each `[bN]` is SP (0) or TAB (1).

Example: The opcode `>` (ASCII 0x3E = 0b0111110) is encoded as:

```
SP TAB TAB TAB TAB TAB SP LF
```

The embedded program ends with the **terminus stamp**: TAB SP TAB SP TAB SP LF (decimal 85, ASCII `U` — for *unprocessed*; after execution, this marker should be updated to ASCII 0x43 = `C` = *closed*).

**Steganographic property**: An embedded ΦΜΛ program is invisible to a reader examining only the non-whitespace content of the host document. The whitespace appears as ordinary formatting. The encoded program can be embedded in a legal brief, a catalog record, or this specification.

### 3.5 Case Sensitivity

Opcodes are case-sensitive. `G` and `g` are distinct instructions.

### 3.6 Grid Initialization from Source

After parsing, the two-dimensional character array is normalized:

1. All rows are padded with SP to the width of the longest row.
2. All register-prefixed opcodes `ΦX`, `ΜX`, `ΛX` are stored as two-cell sequences in the grid, with the Greek prefix in the first cell and the opcode in the second. The IP advances over both.
3. The grid dimensions are stored in the Φ register's dimension field.

---

## 4. Data Types

ΦΜΛ defines four primitive types and one compound type.

### 4.1 INTEGER

A signed 64-bit two's-complement integer. Arithmetic operations use saturating semantics — overflow and underflow clamp to INT64_MAX and INT64_MIN respectively, and the OVERFLOW flag is set in the Λ register's status field.

### 4.2 COORDINATE

A pair of integers (x, y) representing a position in the execution grid. Coordinates are zero-indexed. Coordinates are automatically reduced modulo the current grid dimensions (toroidal wrapping). The type carries a **timestamp** field — the execution cycle at which the coordinate was last referenced — which is used in provenance metadata.

### 4.3 MESSAGE

A sequence of bytes with attached **provenance metadata**. Provenance metadata is a record containing:

- `origin`: the (x, y, cycle) triple at which the message was created
- `author`: the register (Φ, Μ, or Λ) that created the message
- `digest`: a SHA-256 hash of the message content at creation time
- `transmission_count`: number of times this message has been moved between stack frames
- `annotations`: an ordered list of (cycle, register, text) tuples attached via the `A` opcode

Messages are the primary data type of the Μ register.

### 4.4 FORM

A structural descriptor: a pair (width, height) with an optional **layout map** — a sparse mapping from (x, y) coordinates to opcode values. FORMs are created during the First Degree and represent declared grid sections. A FORM is consumed when a Λ operation uses the grid region it describes.

### 4.5 FUNCTION

A FUNCTION value is a (start_coordinate, end_coordinate, arity) triple representing a region of the grid that has been declared as a callable unit during the First Degree. FUNCTIONs are stored in the Λ register's function table. When activated, execution jumps to start_coordinate; on completion, returns to the call site. Arity specifies how many Μ stack entries the function consumes.

### 4.6 The Μ Stack

The Μ register maintains a stack of values of any type. Standard stack discipline applies (LIFO). The stack has no declared maximum depth; implementations should document their actual limit. Stack underflow produces error `STACK EXHAUSTED` (§12). Stack items retain their provenance metadata through all operations.

### 4.7 The Φ Frame Stack

The Φ register maintains a stack of execution frames, each containing:

- `ip`: current instruction pointer coordinates
- `direction`: current IP direction (EAST/WEST/NORTH/SOUTH)
- `grid_ref`: a reference to the active grid section
- `degree`: current execution degree (FIRST/SECOND/THIRD)

The `[` opcode pushes the current frame; `]` pops and restores it.

---

## 5. The Spatial Grid

### 5.1 Grid Structure

The ΦΜΛ execution grid is a two-dimensional toroidal array of cells. Each cell contains:

- `opcode`: a single-byte instruction value (initially set from source parsing)
- `original_opcode`: the instruction value as parsed from source (preserved for finding aid output)
- `mutation_count`: number of times this cell has been modified by self-modification
- `last_executed_at`: execution cycle number of most recent execution
- `wall`: boolean — if true, the IP cannot enter this cell (deflects)

**Dimensions**: Declared in the First Degree using the `E` opcode. Default: 80 columns × 25 rows. Maximum dimensions are implementation-defined but must support at least 1024×1024.

**Coordinate system**: (0,0) is the top-left cell. X increases eastward. Y increases southward. This matches terminal/screen conventions.

### 5.2 Toroidal Wrapping

All coordinate arithmetic wraps modulo the current grid dimensions:

```
x_wrapped = x mod width
y_wrapped = y mod height
```

There are no edges. A program walking east past column (width-1) emerges at column 0. This is the first architectural commitment ΦΜΛ makes: the territory is closed.

### 5.3 Instruction Pointer

The instruction pointer (IP) is a tuple (x, y, direction) where direction ∈ {EAST, WEST, NORTH, SOUTH}.

At each execution cycle:
1. The opcode at (x, y) is fetched and decoded.
2. If the current degree permits this opcode's register class, the opcode executes.
3. If the current degree does not permit this opcode, a DEGREE VIOLATION is logged (see §12) and the opcode is skipped.
4. The self-modification step runs (§8).
5. The IP advances one cell in the current direction.
6. Wall cells: if the next cell has `wall = true`, the IP reflects 180° instead of entering.

### 5.4 Direction Opcodes

| Opcode | Register | Effect                            |
|--------|----------|-----------------------------------|
| `>`    | Φ        | Set direction EAST                |
| `<`    | Φ        | Set direction WEST                |
| `^`    | Φ        | Set direction NORTH               |
| `v`    | Φ        | Set direction SOUTH               |
| `?`    | Φ        | Set direction random (uniform)    |
| `!`    | Φ        | Reverse direction (180°)          |
| `\|`   | Φ        | Reflect: if EAST→NORTH, WEST→SOUTH, NORTH→EAST, SOUTH→WEST  |
| `_`    | Φ        | Reflect: if EAST→SOUTH, WEST→NORTH, NORTH→WEST, SOUTH→EAST  |

### 5.5 String Mode

When the IP encounters a `"` character (Μ register), the interpreter enters **string mode**. In string mode, all cells are read as literal character values and pushed to the Μ stack as MESSAGE values until a second `"` is encountered. String mode does not respect degree restrictions.

### 5.6 Multi-Grid Execution

The First Degree may declare multiple named grids using the `N` opcode (Φ variant). Each named grid is an independent 2D space. The IP may teleport between named grids using the `G` opcode. All grids share the same Μ stack and Λ function table, but each maintains its own coordinate space and mutation state.

---

## 6. Instruction Set

Instructions are organized by register. Each instruction specifies:
- **Opcode**: the character in explicit mode
- **Degree**: which degree(s) permit execution (1 = First, 2 = Second, 3 = Third, * = any)
- **Stack effect**: notation is `( before -- after )` using Forth convention
- **Consensus required**: whether the operation requires tri-register consensus (§7)

### 6.1 Φ Register — Form Instructions

Form instructions govern spatial navigation, grid declaration, and structural state. They execute primarily in the First Degree but retain navigational authority in all degrees.

| Opcode | Degree | Stack Effect       | Consensus | Description |
|--------|--------|--------------------|-----------|-------------|
| `>`    | *      | ( -- )             | No        | Set IP direction EAST |
| `<`    | *      | ( -- )             | No        | Set IP direction WEST |
| `^`    | *      | ( -- )             | No        | Set IP direction NORTH |
| `v`    | *      | ( -- )             | No        | Set IP direction SOUTH |
| `?`    | *      | ( -- )             | No        | Set IP direction random |
| `!`    | *      | ( -- )             | No        | Reverse IP direction |
| `\|`   | *      | ( -- )             | No        | Reflect IP (vertical mirror) |
| `_`    | *      | ( -- )             | No        | Reflect IP (horizontal mirror) |
| `#`    | *      | ( -- )             | No        | BRIDGE: skip next cell |
| `@`    | *      | ( -- )             | No        | TERMINUS: halt program |
| `G`    | *      | ( coord -- )       | Yes       | GOTO: teleport IP to COORDINATE on Μ stack |
| `[`    | *      | ( -- )             | No        | ACQUIRE: push current Φ frame to frame stack |
| `]`    | *      | ( -- )             | No        | RELEASE: pop Φ frame (restore IP, direction, grid ref) |
| `P`    | *      | ( -- coord )       | No        | POSITION: push current IP as COORDINATE to Μ stack |
| `D`    | *      | ( -- int int )     | No        | DIMENSION: push grid width then height to Μ stack |
| `E`    | 1      | ( int int -- )     | Yes       | EXPAND: set grid dimensions from Μ stack (width, height). First Degree only. |
| `W`    | 1      | ( coord -- )       | Yes       | WALL: mark COORDINATE as impassable |
| `S`    | 1      | ( coord msg -- )   | Yes       | SHELVE: assign name (top of Μ as MESSAGE) to coordinate |
| `R`    | *      | ( msg -- )         | Yes       | RETRIEVE: move IP to shelved coordinate by name |
| `ΦN`   | 1      | ( msg -- )         | Yes       | NAMED GRID: declare new named grid with MESSAGE as name |
| `ΦF`   | 1      | ( coord coord msg -- ) | Yes  | FORM DECLARE: declare a FORM spanning two coordinates with MESSAGE name |
| `ΦC`   | 1      | ( msg -- int int ) | No        | CONSULT: push dimensions of named FORM to Μ stack |

### 6.2 Μ Register — Message Instructions

Message instructions govern data placement, content manipulation, and I/O. They execute in the Second Degree and retain read/write authority in the Third.

| Opcode | Degree | Stack Effect             | Consensus | Description |
|--------|--------|--------------------------|-----------|-------------|
| `.`    | 2,3    | ( int -- )               | No        | EMIT: pop INTEGER, write as character to stdout |
| `:`    | 2,3    | ( a -- a a )             | No        | DUPLICATE: duplicate Μ stack top |
| `\`    | 2,3    | ( a b -- b a )           | No        | EXCHANGE: swap top two stack items |
| `$`    | 2,3    | ( a -- )                 | No        | DISCARD: pop and discard |
| `,`    | 2,3    | ( -- int )               | No        | INTAKE: read one byte from stdin, push as INTEGER |
| `%`    | 2,3    | ( msg -- msg msg_meta )  | No        | PROVENANCE: duplicate top with its provenance metadata as a new MESSAGE |
| `A`    | 2,3    | ( msg annotation -- msg )| Yes       | ANNOTATE: attach annotation MESSAGE to top MESSAGE |
| `"`    | *      | ( -- msg )               | No        | STRING MODE: push string literal to Μ stack |
| `0`-`9`| *     | ( -- int )               | No        | DIGIT: push literal integer value |
| `X`    | 2,3    | ( -- msg )               | Yes       | EXTRACT: read steganographic payload from host document (§11) |
| `I`    | 2,3    | ( msg -- )               | Yes       | INSCRIBE: write MESSAGE as steganographic payload to host document (§11) |
| `K`    | 2,3    | ( -- msg )               | No        | FINDING AID: push current finding aid reference as MESSAGE |
| `ΜR`   | 2,3    | ( msg -- msg )           | No        | RECORD: write top of Μ stack to finding aid (without popping) |
| `ΜT`   | 2,3    | ( msg addr -- )          | Yes       | TRANSMIT: send MESSAGE to process at COORDINATE address |
| `ΜH`   | 2,3    | ( -- msg )               | Yes       | HEAR: block until MESSAGE received from any transmitter |
| `ΜD`   | 2,3    | ( msg -- msg )           | No        | DIGEST: replace MESSAGE content with its SHA-256 hash, preserving metadata |
| `ΜM`   | 2,3    | ( msg int -- msg )       | Yes       | MARK: set retention flag on MESSAGE (0=ephemeral, 1=permanent, 2=restricted) |

### 6.3 Λ Register — Function Instructions

Function instructions govern computation, control flow, and consensus management. They execute exclusively in the Third Degree except where noted.

| Opcode | Degree | Stack Effect             | Consensus | Description |
|--------|--------|--------------------------|-----------|-------------|
| `+`    | 3      | ( int int -- int )       | Yes       | ADD: pop two INTEGERs, push sum |
| `-`    | 3      | ( int int -- int )       | Yes       | SUBTRACT: pop two, push (second - top) |
| `*`    | 3      | ( int int -- int )       | Yes       | MULTIPLY |
| `/`    | 3      | ( int int -- int )       | Yes       | DIVIDE: integer division; divisor-zero → DIVISION UNDEFINED (§12) |
| `` ` ``| 3     | ( int int -- int )       | Yes       | MODULO: pop two, push (second mod top) |
| `=`    | 3      | ( a b -- int )           | Yes       | COMPARE: push 1 if equal, 0 otherwise |
| `(`    | 3      | ( int int -- int )       | Yes       | LESS THAN: push 1 if second < top |
| `)`    | 3      | ( int int -- int )       | Yes       | GREATER THAN: push 1 if second > top |
| `&`    | 3      | ( int int -- int )       | Yes       | BITWISE AND |
| `~`    | 3      | ( int -- int )           | Yes       | BITWISE NOT |
| `j`    | 3      | ( int -- )               | Yes       | BRANCH IF ZERO: if top is 0, skip to matching `J` |
| `J`    | 3      | ( -- )                   | No        | END BRANCH: target for `j` |
| `{`    | 3      | ( -- )                   | No        | OPEN REQUEST: begin consensus request block |
| `}`    | 3      | ( -- )                   | No        | ADJUDICATE: close request block; execute if consensus granted |
| `f`    | 3      | ( args... msg -- )       | Yes       | CALL FUNCTION: invoke named FUNCTION from Φ function table |
| `r`    | 3      | ( -- )                   | No        | RETURN: return from FUNCTION to call site |
| `ΛQ`   | 3      | ( -- int )               | No        | QUERY CONSENSUS: push 1 if current operation has full consent, 0 if schism |
| `ΛM`   | 3      | ( -- )                   | Yes       | MEDIATE: attempt schism resolution (§7.5) |
| `ΛS`   | 3      | ( -- msg )               | No        | SCHISM LOG: push the most recent schism record as MESSAGE |
| `ΛE`   | 3      | ( int -- )               | No        | EXIT WITH CODE: terminate program, writing integer as exit status |
| `ΛD`   | 1,2,3  | ( -- )                   | No        | DEGREE: push current degree as INTEGER |
| `ΛA`   | 1,2    | ( -- )                   | Yes       | ADVANCE DEGREE: transition to next degree |

### 6.4 Opcode Summary Table

The full opcode byte map (decimal values, ASCII):

```
0x20 (SP)  : NOP — no operation, does not trigger self-modification
0x21 (!)   : Φ — BOUNCE (reverse direction)
0x22 (")   : Μ — STRING MODE begin/end
0x23 (#)   : Φ — BRIDGE (skip next)
0x24 ($)   : Μ — DISCARD
0x25 (%)   : Μ — PROVENANCE
0x26 (&)   : Λ — BITWISE AND
0x27 (')   : NOP
0x28 (()   : Λ — LESS THAN
0x29 ())   : Λ — GREATER THAN
0x2A (*)   : Λ — MULTIPLY
0x2B (+)   : Λ — ADD
0x2C (,)   : Μ — INTAKE
0x2D (-)   : Λ — SUBTRACT
0x2E (.)   : Μ — EMIT
0x2F (/)   : Λ — DIVIDE
0x30-0x39  : Μ — DIGIT (0-9)
0x3A (:)   : Μ — DUPLICATE
0x3B (;)   : NOP
0x3C (<)   : Φ — WEST
0x3D (=)   : Λ — COMPARE
0x3E (>)   : Φ — EAST
0x3F (?)   : Φ — RANDOM DIRECTION
0x40 (@)   : Φ — TERMINUS
0x41 (A)   : Μ — ANNOTATE
0x44 (D)   : Φ — DIMENSION
0x45 (E)   : Φ — EXPAND
0x46 (F)   : Λ — (prefix: ΛF = function variant)
0x47 (G)   : Φ — GOTO
0x49 (I)   : Μ — INSCRIBE
0x4A (J)   : Λ — END BRANCH
0x4B (K)   : Μ — FINDING AID reference
0x4E (N)   : Μ — NEXT record
0x50 (P)   : Φ — POSITION
0x52 (R)   : Φ — RETRIEVE
0x53 (S)   : Φ — SHELVE
0x57 (W)   : Φ — WALL
0x58 (X)   : Μ — EXTRACT
0x5C (\)   : Μ — EXCHANGE
0x5E (^)   : Φ — NORTH
0x5F (_)   : Φ — REFLECT HORIZONTAL
0x60 (`)   : Λ — MODULO
0x66 (f)   : Λ — CALL FUNCTION
0x6A (j)   : Λ — BRANCH IF ZERO
0x72 (r)   : Λ — RETURN
0x76 (v)   : Φ — SOUTH
0x7C (|)   : Φ — REFLECT VERTICAL
0x7E (~)   : Λ — BITWISE NOT
```

All other byte values in the range 0x00-0x7F are NOPs.

---

## 7. The Consensus Protocol

### 7.1 Overview

Every instruction marked "Consensus required" in §6 must receive affirmative votes from all three registers before execution. This is the Protocol of Mutual Assent (PMA). The protocol is not advisory — an instruction that fails to achieve consensus does not execute. The failure is recorded. Repeated failures at the same instruction cell are evidence of a structural problem and are treated as escalating severity in the finding aid.

### 7.2 Request Format

When a consensus-required instruction is about to execute, the Λ register (as coordinator) issues an internal REQUEST. The request record has the following structure:

```
REQUEST {
    cycle:       INTEGER          // execution cycle count
    opcode:      BYTE             // the instruction byte
    position:    COORDINATE       // (x, y) in the grid
    degree:      {FIRST, SECOND, THIRD}
    arguments:   MESSAGE[]        // copies of relevant Μ stack entries
    polite:      BOOLEAN          // true if PLEASE prefix was used (§3.2)
}
```

This request is broadcast simultaneously to all three registers. Each register independently evaluates the request against its current state and responds with ASSENT or DISSENT.

### 7.3 Assent Criteria

**Φ register assents if**:
- The target coordinates (if any) are within the current grid bounds or will be after wrapping
- No WALL is declared at any coordinate the operation would touch
- The operation does not violate FORM boundaries declared in the First Degree
- The current degree is consistent with the opcode's degree restrictions

**Μ register assents if**:
- The Μ stack contains the required number and types of arguments
- All MESSAGE arguments have valid provenance metadata
- INSCRIBE operations target a writable channel (§11)
- No retention restriction (ΜM flag = 2) blocks the operation

**Λ register assents if**:
- The operation is within the current execution degree
- No FATAL SCHISM is pending (§7.6)
- The function table entry exists (for CALL FUNCTION operations)
- Arithmetic operations would not produce undefined results (division by zero, etc.)

### 7.4 Politeness and Priority

An instruction prefixed with `PLEASE` (§3.2) or encoded with the TAB TAB TAB politeness prefix in embedded mode is a **polite request**. Polite requests receive the following treatment:

- Φ will ASSENT to polite GOTOs even when crossing FORM boundaries, provided the destination is declared.
- Μ will ASSENT to polite INSCRIBEs even to restricted channels, provided a key is on the Μ stack.
- Λ will grant one additional mediation round (§7.5) before escalating to FATAL SCHISM.

Politeness is not optional. If fewer than 1 in every 5 consensus-requiring instructions is prefixed with PLEASE, the interpreter issues a COURTESY DEFICIT warning to the finding aid. Programs that never use PLEASE are technically valid but are noted in the finding aid as *discourteous*, and their finding aid entries are flagged accordingly.

### 7.5 Dissent and Schism

If any register DISsents, the operation does not execute. A SCHISM record is written to the finding aid:

```
SCHISM {
    cycle:        INTEGER
    opcode:       BYTE
    position:     COORDINATE
    dissenters:   SET of {PHI, MU, LAMBDA}
    reasons:      STRING[]         // one reason per dissenter
    polite:       BOOLEAN
    severity:     {MINOR, MODERATE, ESCALATED, FATAL}
}
```

Schism severity escalates as follows:
- First schism at a given position: MINOR
- Second schism at the same position: MODERATE
- Third schism: ESCALATED — the Λ register initiates MEDIATION automatically
- Fourth schism: FATAL (§7.6)

### 7.6 Mediation Protocol

Mediation (`ΛM` opcode, or triggered automatically at ESCALATED severity) is a structured negotiation process:

1. **Φ states its objection** as a string appended to the schism record
2. **Μ states its objection** similarly
3. **Λ proposes a modification**: it may pop additional items from the Μ stack to reframe the argument, attempt an alternative opcode, or reduce the scope of the operation
4. Each register re-evaluates the modified request
5. If all three ASSENT to the modified request, the modified operation executes. The modification is logged.
6. If consensus is still not achieved after `max_mediation_rounds` attempts (default: 3, or 4 if the original request was polite), mediation fails.

### 7.7 Fatal Schism

A FATAL SCHISM terminates the current function call if within a FUNCTION, or the entire program if at the top level. Before termination, the finding aid is finalized with a FATAL SCHISM notice. Exit code is 255.

Programs that terminate via FATAL SCHISM are said to have **lapsed into permanent disagreement** — a condition that archival theory identifies as the natural result of inadequately documented records.

### 7.8 Process Communication (Shakespeare Inheritance)

The `ΜT` (TRANSMIT) and `ΜH` (HEAR) opcodes implement inter-process communication in a structured dialogue model. Multiple ΦΜΛ processes may run concurrently. Communication is point-to-point, addressed by COORDINATE (which maps to a process identifier).

Messages transmitted via `ΜT` carry their full provenance metadata and are not transformed in transit. A process blocked on `ΜH` will remain blocked until a message is received or a configurable timeout elapses, at which point a TRANSMISSION FAILURE error (§12) is raised.

The dialogue form: when process A TRANSMITs to process B and process B HEARs, the exchange is logged in both processes' finding aids as a DIALOGUE entry, citing both processes' identifiers. This is the Shakespeare inheritance — every inter-process communication is a documented encounter, not an anonymous syscall.

---

## 8. Self-Modification Rules

### 8.1 The Mutation Cycle

After each instruction executes (consensus or not — NOPs and degree violations also trigger mutation), the cell at the IP's current position is modified. The modification applies a **mutation function** M to the current opcode value:

```
M(opcode) = MUTATION_TABLE[opcode mod TABLE_SIZE]
```

The mutation table maps each opcode to its successor in the decay sequence. The sequence is designed such that:

1. Navigation opcodes (`> < ^ v`) cycle among themselves, preserving directionality but rotating: `>` → `<` → `^` → `v` → `>` ...
2. Arithmetic opcodes (`+ - * /`) cycle among themselves: `+` → `-` → `*` → `/` → `+` ...
3. I/O opcodes (`. ,`) alternate: `.` → `,` → `.` ...
4. TERMINUS (`@`) mutates to NOP (SP). Once mutated, the termination point is gone.
5. NOP (SP) mutates to `?` (RANDOM DIRECTION) after enough cycles. Cells that were never opcodes become unpredictable.
6. PLEASE-prefixed instructions mutate more slowly: their mutation counter increments only every other execution.

### 8.2 The Complete Mutation Table

The mutation table is a permutation of the instruction set. Partial listing of key transitions:

| Current | Mutates to | Notes |
|---------|-----------|-------|
| `>`     | `v`        | East becomes South |
| `v`     | `<`        | South becomes West |
| `<`     | `^`        | West becomes North |
| `^`     | `>`        | North becomes East |
| `+`     | `-`        | Add becomes Subtract |
| `-`     | `*`        | Subtract becomes Multiply |
| `*`     | `/`        | Multiply becomes Divide |
| `/`     | `` ` ``    | Divide becomes Modulo |
| `` ` `` | `+`        | Modulo becomes Add |
| `.`     | `,`        | Emit becomes Intake |
| `,`     | `.`        | Intake becomes Emit |
| `@`     | SP         | Terminus becomes NOP |
| SP      | SP         | NOP stays NOP (18 cycles) |
| SP(×18) | `?`        | NOP becomes Random Direction after 18 passes |
| `?`     | `!`        | Random becomes Bounce |
| `!`     | `?`        | Bounce becomes Random |
| `j`     | `J`        | Branch-if-zero becomes End-branch |
| `J`     | `j`        | End-branch becomes Branch-if-zero |
| `{`     | `}`        | Open-request becomes Close-request |
| `}`     | `{`        | Close-request becomes Open-request |
| `[`     | `]`        | Acquire-frame becomes Release-frame |
| `]`     | `[`        | Release-frame becomes Acquire-frame |
| `#`     | SP         | Bridge becomes NOP |
| `W`     | `G`        | Wall becomes Goto |
| `S`     | `R`        | Shelve becomes Retrieve |
| `E`     | `D`        | Expand becomes Dimension |
| `=`     | `(`        | Compare becomes Less-than |
| `(`     | `)`        | Less-than becomes Greater-than |
| `)`     | `=`        | Greater-than becomes Compare |

### 8.3 Mutation Inhibition

The `ΦF` (FORM DECLARE) opcode, when used to declare a region in the First Degree, optionally accepts a **preservation flag** on the Μ stack. If the preservation flag is set (INTEGER 1), cells within the declared FORM region are **frozen** — their mutation counter does not increment. Frozen cells are noted in the finding aid with a PRESERVATION NOTICE.

Frozen cells are an archival intervention. Using them excessively is noted with ARRANGEMENT VIOLATED (§12).

### 8.4 Entropic Terminal State

When every cell in the grid has mutated past its original opcode at least once, the program is in **entropic terminal state**. In this state:
- Mutation continues
- Execution continues
- The interpreter writes ENTROPY THRESHOLD REACHED to the finding aid
- If the program has not halted within 1000 additional cycles, ENTROPIC DISSOLUTION is logged and the program is terminated with exit code 127

A program that can survive entropic terminal state is a program that was designed to — one whose behavior at the fully-mutated state is still defined. This is considered extremely difficult to achieve and is noted in the finding aid as RESILIENT CONFIGURATION.

---

## 9. The Three Degrees

### 9.1 Formal Definition

ΦΜΛ execution is divided into three sequential phases called Degrees. Each Degree permits only a subset of opcodes to execute. An opcode encountered in an impermitted Degree produces a DEGREE VIOLATION log entry (§12) and is skipped; the cell still mutates; the IP still advances.

The current Degree is stored in the Λ register's degree field. The initial Degree at program start is FIRST.

### 9.2 First Degree: Declaration

**Register authority**: Φ
**Permitted opcodes**: All Φ opcodes; ΛD; ΛA
**Forbidden**: All Μ opcodes except string literals; all Λ opcodes except ΛD and ΛA

**What the First Degree is for**:
- Declaring the grid dimensions (`E`)
- Marking walls (`W`)
- Declaring named locations (`S`)
- Declaring named grids (`ΦN`)
- Declaring FORMs (`ΦF`)
- Declaring FUNCTIONs (regions that will be callable in the Third Degree)
- Establishing navigation (direction opcodes are legal in all degrees)

**What the First Degree is not for**:
- Moving data
- Computing
- I/O

A ΦΜΛ program that performs no First Degree declarations executes on the default 80×25 grid with no declared structure. This is legal but noted in the finding aid as UNDESCRIBED COLLECTION.

**Transition to Second Degree**: The `ΛA` (ADVANCE DEGREE) opcode, when executed in the First Degree, transitions to the Second Degree. The transition is logged. `ΛA` requires consensus; if any register DISsents, the Degree does not advance.

### 9.3 Second Degree: Population

**Register authority**: Μ
**Permitted opcodes**: All Μ opcodes; navigation opcodes; ΛA
**Forbidden**: All Λ arithmetic/logic/control opcodes except ΛA; Φ structural declaration opcodes (E, W, S, ΦN, ΦF)

**What the Second Degree is for**:
- Loading data onto the Μ stack
- Placing string literals
- Reading from external sources (`,` INTAKE)
- Arranging data before computation
- Writing steganographic payloads to the host document (`I` INSCRIBE)
- Setting up function arguments

The Second Degree is the **population phase** — analogous to the archival accession process: materials arrive and are placed in rough order before processing begins. No operations are performed on the data; the data simply exists in the Μ stack.

**Transition to Third Degree**: `ΛA` again, requiring consensus. A program that transitions to the Third Degree without populating any data is legal; the Μ stack is empty; Λ operations that require arguments will immediately produce schisms.

### 9.4 Third Degree: Activation

**Register authority**: Λ
**Permitted opcodes**: All Λ opcodes; Μ I/O opcodes (. , X I); navigation opcodes
**Forbidden**: Φ structural declaration opcodes; Μ data-placement opcodes (the Μ stack may be read and written, but no new literals may be pushed — all data that will be used must already be on the stack from the Second Degree, or read from external input)

**Exception**: The `"` STRING MODE opcode is permitted in all Degrees.

**What the Third Degree is for**:
- Arithmetic and logic
- Control flow (branches, function calls)
- Output (EMIT)
- Inter-process communication
- Consensus operations

**Structural consumption**: Every Λ operation that requires consensus checks whether the grid region it is executing within has a declared FORM (§5, §6.1). If it does, each consensus-requiring operation in the Third Degree **consumes** one unit of the FORM's declared capacity. When a FORM's capacity reaches zero, further operations within that region produce FORM EXHAUSTED errors (§12) until the IP exits the region. This is the Malbolge/archival inheritance: computation is not free; each operation degrades the structural substrate.

FORM capacity defaults to (width × height) of the declared region. It may be set explicitly using a second INTEGER argument to `ΦF`.

**Termination**: The Third Degree ends when `@` (TERMINUS) is executed, `ΛE` (EXIT WITH CODE) is executed, a FATAL SCHISM occurs, or ENTROPIC DISSOLUTION triggers.

There is no fourth degree. Programs that exhaust all three degrees and then encounter `ΛA` again produce an ADVANCEMENT REFUSED error and terminate.

---

## 10. Self-Documentation

### 10.1 The Finding Aid

Every ΦΜΛ program automatically produces a **finding aid** — a structured document recording all significant events during execution. The finding aid is written to a file named `<program_name>.faid` in the current working directory, or to standard error if file I/O is unavailable.

The finding aid is a plain-text document organized in archival finding aid format, with the following mandatory sections:

**Header**:
```
FINDING AID
Repository: [implementation-defined]
Accession: [program filename and SHA-256 hash]
Date: [ISO 8601 execution timestamp]
Degree at Termination: [FIRST|SECOND|THIRD|UNDETERMINED]
Exit Condition: [TERMINUS|EXIT_CODE|FATAL_SCHISM|ENTROPIC|EXTERNAL]
Total Cycles: [integer]
```

**Series 1: Structural Declarations** (First Degree events)
All E, W, S, ΦN, ΦF operations that succeeded, with cycle numbers and coordinates.

**Series 2: Population Events** (Second Degree events)
All Μ-stack operations that loaded non-trivial data, with stack depths and provenance.

**Series 3: Execution Log** (Third Degree events)
All consensus requests, their outcomes, schism records, and mediation proceedings.

**Series 4: Self-Modification Record**
A complete log of all mutations: (cycle, position, old_opcode, new_opcode).

**Series 5: Terminal State**
Final grid state, final Μ stack contents, exit condition, and a SHA-256 hash of the terminal state.

### 10.2 Archival Metadata Record (AMR)

At termination, the interpreter writes an Archival Metadata Record in Dublin Core format:

```
dc:title:       [program filename]
dc:creator:     [ΦΜΛ version string]
dc:date:        [execution timestamp]
dc:description: [one-sentence description inferred from finding aid]
dc:type:        "ΦΜΛ Execution Record"
dc:format:      "text/x-phimulambda-faid"
dc:identifier:  [SHA-256 of terminal state]
dc:rights:      "ARRANGEMENT MAY NOT BE REPRODUCED WITHOUT PROVENANCE"
dc:relation:    [list of processes communicated with, if any]
```

### 10.3 Finding Aid Generation from ΦΜΛ

The `K` opcode pushes the current finding aid reference (a MESSAGE containing the current entry identifier in the finding aid) to the Μ stack. This allows programs to annotate their own execution records in the finding aid using `A` (ANNOTATE) and `ΜR` (RECORD), creating self-referential archival structures. A program that uses `K` extensively to annotate its own finding aid is said to be **exhibiting archival consciousness** — this is not a term of praise.

---

## 11. I/O Model

### 11.1 Standard Channels

ΦΜΛ provides access to three standard channels:

- **Standard Input**: readable via `,` (INTAKE), one byte at a time
- **Standard Output**: writable via `.` (EMIT), one byte at a time
- **Finding Aid**: written automatically; also writable via `ΜR` and `A`

### 11.2 Steganographic I/O

The INSCRIBE (`I`) and EXTRACT (`X`) opcodes provide access to **steganographic channels** — mechanisms for reading and writing data invisibly within host documents or the filesystem environment.

Available steganographic channels (tried in order; first available is used):

**Channel 1: Extended Attributes (xattr)**
On filesystems that support extended attributes (Linux ext4, macOS APFS, etc.), data is stored in the `user.phimulambda` extended attribute of the current working directory. Read/write with `getxattr`/`setxattr`. This channel is invisible to `ls`, `cat`, and most file inspection tools.

**Channel 2: Whitespace in Host Document**
If the program is running in embedded mode from a host document, INSCRIBE modifies the whitespace encoding of the host document in memory (and on disk if the host document path is accessible). The modification is within the whitespace sequences following the terminus stamp, appending a new payload frame. EXTRACT reads from this same region.

**Channel 3: Filesystem Metadata**
The `atime`, `mtime`, and `ctime` of files in the current directory encode integers via the low-order bits. The INSCRIBE opcode encodes data into the last-modified timestamps of files matching `*.faid` in the current directory. The EXTRACT opcode reads these timestamps and decodes them. Requires write permission on the target files.

**Channel 4: Slack Space (Advisory)**
On block-structured filesystems, file slack space (the gap between file content and the next block boundary) theoretically provides a steganographic channel. ΦΜΛ implementations MAY implement this channel but are not required to. If implemented, it must be documented in the finding aid with a SLACK CHANNEL NOTICE. Use of this channel requires the `polite` flag (§7.4) or will be rejected by the Μ register with CHANNEL UNAUTHORIZED.

**Channel 5: Standard Error**
The fallback channel. Unformatted bytes written to stderr. Not steganographic in any meaningful sense; the finding aid will contain a NOTE: PLAINTEXT FALLBACK CHANNEL IN USE.

### 11.3 INSCRIBE Semantics

`I` pops a MESSAGE from the Μ stack and writes it to the first available steganographic channel. The MESSAGE's provenance metadata is written alongside the payload so that a subsequent EXTRACT can reconstruct it. The write requires consensus; the Μ register verifies the MESSAGE has valid provenance before permitting the write.

### 11.4 EXTRACT Semantics

`X` reads the most recent payload from the first available steganographic channel and pushes it as a MESSAGE to the Μ stack. If no payload is present on the channel, `X` pushes an empty MESSAGE with a provenance note: SOURCE ABSENT.

### 11.5 Dead Drop Pattern

The canonical steganographic I/O pattern (demonstrated in §13.4) is called a **dead drop**: one process INSCRIBEs to the xattr channel, terminates, and a second process later EXTRACTs from the same channel. The two processes never communicate directly; the host filesystem is the intermediary. The finding aid of both processes will contain cross-references if they share the same working directory.

---

## 12. Error Messages

ΦΜΛ defines 26 named errors. Errors are written to the finding aid with cycle number, position, and severity. Severity levels: NOTICE, WARNING, ERROR, FATAL.

| Code | Severity | Message | Condition |
|------|----------|---------|-----------|
| E001 | FATAL    | PROVENANCE DISPUTED | A MESSAGE's digest does not match its content; the record has been altered after creation |
| E002 | FATAL    | FINDING AID INCOMPLETE | The finding aid file cannot be written; execution cannot proceed without archival record |
| E003 | ERROR    | ARRANGEMENT VIOLATED | An operation attempts to write into a declared FORM region from outside, in violation of original order |
| E004 | NOTICE   | UNINITIATED PROCESS | A CALL FUNCTION opcode targets an undeclared FUNCTION; treated as NOP with logging |
| E005 | FATAL    | STACK EXHAUSTED | Μ stack underflow: an operation requires more items than the stack contains |
| E006 | ERROR    | DEGREE VIOLATION | An opcode was executed in a Degree that does not permit its register class |
| E007 | FATAL    | DIVISION UNDEFINED | Division or modulo by zero |
| E008 | WARNING  | UNDESCRIBED COLLECTION | The First Degree completed with no structural declarations |
| E009 | ERROR    | FORM EXHAUSTED | A FORM's capacity has been consumed; further operations within its region are blocked |
| E010 | FATAL    | FATAL SCHISM | Fourth dissent at the same position; all mediation failed |
| E011 | WARNING  | COURTESY DEFICIT | Fewer than 20% of consensus-requiring instructions used the PLEASE prefix |
| E012 | NOTICE   | ENTROPY THRESHOLD REACHED | Every cell has mutated at least once |
| E013 | FATAL    | ENTROPIC DISSOLUTION | 1000 cycles elapsed after entropy threshold without termination |
| E014 | ERROR    | TRANSMISSION FAILURE | `ΜH` HEAR timed out waiting for a message |
| E015 | ERROR    | CHANNEL UNAUTHORIZED | INSCRIBE attempted a restricted steganographic channel without politeness prefix |
| E016 | FATAL    | ADVANCEMENT REFUSED | `ΛA` executed in the Third Degree (no further degrees exist) |
| E017 | WARNING  | PERMANENT DISAGREEMENT | A schism was logged but the program continued; the dissented operation is permanently unexecuted |
| E018 | ERROR    | ACQUISITION STAMP ABSENT | Embedded mode: the host document does not contain a valid acquisition stamp |
| E019 | WARNING  | SLACK CHANNEL IN USE | INSCRIBE is writing to filesystem slack space |
| E020 | NOTICE   | PRESERVATION NOTICE | A cell within a frozen FORM region was not mutated (mutation inhibition active) |
| E021 | ERROR    | ARCHIVAL BOND BROKEN | A MESSAGE was pushed to the Μ stack without valid provenance metadata (e.g., received from an untrusted external source) |
| E022 | WARNING  | PLAINTEXT FALLBACK CHANNEL | INSCRIBE is using stderr as fallback; steganographic property not preserved |
| E023 | ERROR    | COORDINATE OUT OF DECLARED BOUNDS | A FORM or named location references a coordinate outside the declared grid dimensions |
| E024 | NOTICE   | RESILIENT CONFIGURATION | Program survived entropic terminal state for more than 1000 cycles |
| E025 | WARNING  | SOURCE ABSENT | EXTRACT found no payload on any available steganographic channel |
| E026 | FATAL    | DEACCESSIONED | A program attempted to re-execute an `@` (TERMINUS) cell that has mutated to NOP; the termination point no longer exists; the program cannot stop in the normal way |

---

## 13. Example Programs

### 13.1 Hello World

The canonical first program. Written in explicit mode on a 13×1 grid.

**Source**:
```
"Hello, World!">v
              .< ^
              .  |
```

**Explanation**:
- First Degree: No declarations (UNDESCRIBED COLLECTION warning will be issued). Grid defaults to 80×25.
- The IP starts at (0,0) going EAST.
- `"Hello, World!"` enters string mode, pushes each character as an INTEGER in reverse order onto the Μ stack.
- `>` confirms EAST direction.
- `v` turns SOUTH.
- `.` pops the top of Μ and emits it as a character.
- `<` turns WEST, `^` turns NORTH, `|` reflects.
- The IP loops, emitting one character per pass.
- When the Μ stack is exhausted, `.` will attempt to pop from an empty stack → STACK EXHAUSTED (E005, FATAL).

**Note**: To avoid STACK EXHAUSTED, a proper Hello World uses a counter:

```
"Hello, World!"
v              <
>:j@>.
```

Line 3: IP goes EAST. `:` duplicates top. `j` — BRANCH IF ZERO: if the duplicate is 0 (stack empty sentinel), jump to `@`. `>` continues EAST (branch target). `.` emits.

For a correct program without STACK EXHAUSTED, push a zero sentinel before the string:

**Corrected Hello World**:
```
0"Hello, World!">v
                .<
                j ^
                @
```

**Execution trace (First Degree → Second Degree → Third Degree)**:
1. First Degree: IP navigates the grid; `ΛA` transitions degree. Since no `ΛA` is present in this program, the First Degree ends when the IP wraps back to the start and the implementation detects no structural opcodes remain. (Simplification: ΦΜΛ implicitly transitions between degrees when the IP has completed a full traversal without encountering degree-advance opcodes. Implementations may require explicit `ΛA` — this is a conformance option.)
2. Second Degree: `0` pushes sentinel; `"Hello, World!"` pushes characters.
3. Third Degree: Loop emits characters until sentinel is popped.

### 13.2 Cat (Echo Input)

A program that reads bytes from stdin and writes them to stdout until EOF (when `,` returns -1).

```
>,: 1+j@>.
```

**Explanation**:
- IP goes EAST.
- `,` reads one byte from stdin; pushes as INTEGER (-1 on EOF).
- `:` duplicates it.
- `1+` adds 1 (so EOF becomes 0; any other byte becomes non-zero).
- `j` — if zero (EOF condition), skip to `@` and terminate.
- `>` continues east.
- `.` emits the byte (the original copy, not the incremented one).

Wait — the `:` and `j` logic requires more careful stack management. Corrected:

```
>,: 1+(j@)$.>
```

Stack on arrival at `j`: ( original_byte eof_check ).
`j` consumes eof_check; if zero (EOF), jumps to `@`.
`$` discards original_byte (we don't want to emit EOF = -1).
`.` emits original_byte.

For the corrected version with explicit stack discipline:

```
> , : 1 + j @ $ . <
^                  v
```

The IP loops: EAST to read and emit, then bends SOUTH, then WEST (under the main line), then NORTH and back. On EOF, `@` terminates.

### 13.3 Self-Consuming Countdown

A program that counts down from 5 to 0, emitting each digit, relying on mutation to eventually destroy its own loop structure.

```
ΛA ΛA 5 > : . 1 - : j @ $
             ^           v
             <           <
```

**Execution**:
- `ΛA ΛA` advances through First and Second Degrees immediately.
- `5` pushes 5.
- Main loop: `: .` duplicates and emits top of stack as character code. `1 -` decrements. `: j` checks if zero; if zero, jumps to `@` and terminates. `$` discards the duplicate used for testing.
- The loop executes 6 times (5,4,3,2,1,0).
- **Self-consumption**: The direction opcode `>` at the start of the loop mutates. After each pass through the loop, `>` → `v` → `<` → `^` → `>` (4-cycle rotation). This means after 4 loops, the direction opcode has rotated once — the loop continues by coincidence. By loop 5, `>` has become `v`, and the IP turns south rather than continuing east. The program leaves the main loop row and wanders the grid, encountering `<` opcodes and other residue. The countdown completes (it reaches 0) before this matters, but the program's behavior after countdown is undefined — it will wander, encounter mutated cells, and eventually hit a mutated `j` that misroutes, a `?` that arose from an aged NOP, or it will loop forever through a maze of its own decay.
- The self-consuming countdown does not need to be well-behaved after completion. The finding aid documents the exact cycle of each mutation.

### 13.4 Dead Drop (Steganographic Write)

A program that writes a secret message into the extended attribute of the current directory, then terminates leaving no standard output.

```
ΛA ΛA
PLEASE "DEAD DROP PAYLOAD: RENDEZVOUS ZERO THREE HUNDRED" ΜM 1 I @
```

**Execution**:
- `ΛA ΛA` advances to Third Degree.
- `PLEASE` prefix applies to the entire following block.
- `"DEAD DROP PAYLOAD: ..."` pushes the message string to Μ stack.
- `ΜM 1` sets retention flag to PERMANENT on the top MESSAGE.
- `I` INSCRIBE: pops the MESSAGE and writes to the first available steganographic channel (xattr `user.phimulambda` on the current directory).
- `@` terminates.

**Standard output**: none.
**Finding aid**: Records the INSCRIBE operation, the channel used, the cycle, and the digest of the written payload.

The retrieval program:

```
ΛA ΛA PLEASE X . @
```

- `X` EXTRACT reads from the steganographic channel.
- `.` emits each byte of the retrieved MESSAGE.
- The finding aid cross-references the original INSCRIBE if the two programs share a working directory.

### 13.5 Consensus Request Between Three Operators

A program demonstrating explicit multi-party consensus using `{ }` REQUEST blocks and the PLEASE prefix. This simulates a formal archival appraisal decision.

```
ΛA ΛA
⌜ APPRAISAL DECISION: Three-party consent required for deletion ⌝
PLEASE "RECORD SERIES 7" { : ΜR } } }
⌜ First } = Phi assents (structure ok), second = Mu assents (data valid), third = Lambda assents (function legal) ⌝
⌜ In this implementation, { } is a single consensus block; three } tokens = three explicit checkpoints ⌝
$ @
```

**Note on `{ }` semantics**: A REQUEST block `{ ... }` requires consensus at the `}` point. Nesting `{ { { } } }` creates three nested consensus checkpoints. In this program, the three `}` tokens are three sequential consensus checkpoints, each requiring fresh assent from all three registers.

- The first `}` runs `: ΜR` (duplicate and record to finding aid) only if consensus is reached.
- The second and third `}` extend the confirmation chain.
- If any register DISsents at any checkpoint, the entire transaction fails — the MESSAGE is NOT recorded, and the schism log documents which register objected and why.
- `$` discards the original MESSAGE.
- `@` terminates.

**Finding aid output** on success:
```
[SERIES 3: EXECUTION LOG]
Cycle 14: CONSENSUS REQUEST
  Opcode: }
  Position: (29, 1)
  Polite: YES
  PHI: ASSENT — structure nominal
  MU: ASSENT — provenance valid, retention unrestricted
  LAMBDA: ASSENT — degree THIRD, function table consistent
  RESULT: OPERATION EXECUTED
  MESSAGE RECORDED: "RECORD SERIES 7" (digest: a3f7...)
```

**Finding aid output** on schism (e.g., Μ DISsents because MESSAGE has no provenance):
```
Cycle 14: SCHISM (MINOR)
  Opcode: }
  Position: (29, 1)
  Polite: YES
  PHI: ASSENT
  MU: DISSENT — ARCHIVAL BOND BROKEN: message lacks valid provenance
  LAMBDA: ASSENT
  RESULT: OPERATION NOT EXECUTED
  SCHISM RECORD: SR-001
```

---

## 14. BNF Grammar

```bnf
<program>          ::= <cell>*
<cell>             ::= <explicit-opcode> | <prefixed-opcode> | <whitespace-program>
<explicit-opcode>  ::= <phi-opcode> | <mu-opcode> | <lambda-opcode> | <nav-opcode> | <nop>
<prefixed-opcode>  ::= <register-prefix> <opcode-char>
<register-prefix>  ::= "Φ" | "Μ" | "Λ"

<phi-opcode>       ::= ">" | "<" | "^" | "v" | "?" | "#" | "@" | "!" | "|" | "_"
                     | "[" | "]" | "P" | "D" | "E" | "W" | "S" | "R" | "G"
<mu-opcode>        ::= "." | "," | ":" | "\" | "$" | "%" | "A" | "X" | "I" | "K"
                     | <string-literal> | <digit>
<lambda-opcode>    ::= "+" | "-" | "*" | "/" | "`" | "=" | "(" | ")" | "&" | "~"
                     | "j" | "J" | "{" | "}" | "f" | "r"
<nav-opcode>       ::= <phi-opcode>
<nop>              ::= " "
<digit>            ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

<string-literal>   ::= '"' <string-content>* '"'
<string-content>   ::= <any-utf8-char-except-dquote> | '\"'

<comment>          ::= "⌜" <any-utf8-char>* "⌝"

<polite-prefix>    ::= "PLEASE "
<polite-opcode>    ::= <polite-prefix> <explicit-opcode>

<whitespace-program> ::= <acquisition-stamp> <frame>* <terminus-stamp>
<acquisition-stamp>  ::= SP TAB SP TAB SP TAB LF
<terminus-stamp>     ::= TAB SP TAB SP TAB SP LF
<frame>              ::= <bit>+ LF
<bit>                ::= SP | TAB
<frame-length>       ::= 7   ;; frames are exactly 7 bits, encoding opcode ASCII value

<consensus-block>    ::= "{" <cell>* "}"
<degree-advance>     ::= "Λ" "A"

<request>            ::= <polite-opcode>? <consensus-block>
<response>           ::= ASSENT | DISSENT

ASSENT               ::= ;; internal state; not representable in source
DISSENT              ::= ;; internal state; not representable in source

<schism-record>      ::= ;; runtime structure; not representable in source
<finding-aid>        ::= ;; runtime output; not representable in source
```

**Notes on grammar**:

1. The BNF above describes syntactic structure only. Semantic constraints (e.g., that `E` is legal only in First Degree) are not representable in BNF without attribute grammars and are instead specified in §6 and §9.

2. The `<whitespace-program>` alternative to `<program>` takes precedence when parsing a document whose non-whitespace content is non-empty — i.e., when the parser detects it is operating on a host document in embedded mode.

3. Recursive structures (`<consensus-block>` containing `<cell>*` which may contain further `<consensus-block>`) are legal up to nesting depth 32. Deeper nesting produces ARRANGEMENT VIOLATED (E003).

4. The `<program>` is laid out on a 2D grid, not a 1D sequence. The BNF reflects the linear token stream; grid position is determined by line breaks in the source, which are significant only for determining row boundaries in explicit mode.

---

## 15. Glossary

**ACCESSION** *(archival)* The formal process of acquiring materials for a collection. *(computational)* The First Degree of ΦΜΛ execution, during which grid structure is declared and allocated.

**ADVANCEMENT** The transition from one Degree to the next via the `ΛA` opcode. Requires consensus. Not reversible.

**APPRAISAL** *(archival)* The process of determining which records have sufficient value to be retained permanently. *(computational)* The consensus protocol's evaluation of whether an operation should execute. The Λ register performs appraisal; Φ and Μ provide review.

**ARCHIVAL BOND** *(archival)* The relationship linking each record to others in the same fonds, giving records meaning through context. *(computational)* The provenance metadata attached to every MESSAGE, which links it to its creation context. A MESSAGE without a valid archival bond cannot be INSCRIBEd.

**ARRANGEMENT** *(archival)* The organization of materials in an archival collection according to provenance and original order. *(computational)* The layout of the execution grid and the order of instructions therein. Violations of arrangement are logged as E003.

**ASSENT** A positive vote in the consensus protocol. A register ASSENTs when the requested operation is valid within its domain. All three registers must ASSENT for an operation to execute.

**BRIDGE** The `#` opcode. Causes the IP to skip the next cell. Named for a bridge in a navigation chart — you cross over it without stopping.

**COURTESY DEFICIT** Warning E011. Issued when a program uses the PLEASE prefix on fewer than 20% of consensus-requiring instructions. The protocol is not merely formal; politeness is a computational property.

**DEAD DROP** A steganographic I/O pattern in which one process writes to a covert channel and a separate process, at a later time, reads from that channel. The two processes do not communicate directly. Named for the espionage technique.

**DEGREE** One of three sequential execution phases: FIRST (declaration), SECOND (population), THIRD (activation). The concept derives from archival processing stages and from initiatory societies' gradation of knowledge.

**DEGREE VIOLATION** Error E006. An opcode was encountered in a Degree whose register authority does not include that opcode's register class. The opcode is skipped; the cell still mutates; the cycle is logged.

**DEACCESSION** *(archival)* The formal process of removing materials from a collection. *(computational)* Error E026: the TERMINUS cell has mutated away; the program cannot terminate normally. The termination point has been removed from the collection.

**DISSENT** A negative vote in the consensus protocol. One dissenting register is sufficient to block an operation. Dissent is logged; it is not an error but an information state.

**ENTROPIC DISSOLUTION** The terminal state (E013) reached when a program survives 1000 execution cycles after every cell has mutated at least once. The program is terminated. This is the Malbolge inheritance.

**FAID** The finding aid file produced automatically by every ΦΜΛ execution. Extension `.faid`. A conformant implementation must produce this file; execution without a finding aid is not conformant.

**FATAL SCHISM** Error E010. The fourth dissent at the same grid position, after all mediation attempts have failed. The program terminates. The finding aid documents the complete schism history.

**FINDING AID** *(archival)* A document that describes the contents and organization of an archival collection, enabling researchers to locate relevant materials. *(computational)* The `.faid` file produced by ΦΜΛ execution, documenting all significant events in structured form. The finding aid is more reliable than the program's output.

**FONDS** *(archival)* The body of records organically created by one person or organization. *(computational)* A single ΦΜΛ execution and its complete set of outputs (program text, finding aid, steganographic channel contents). Records from different fondsshould not be mixed.

**FORM** A declared region of the execution grid, named and bounded during the First Degree. FORM regions have a capacity that is consumed by Third Degree operations.

**IMMEDIATISM** *(Hakim Bey)* Art created for immediate consumption, not distribution. *(computational)* A ΦΜΛ program with no INSCRIBE operations — one that leaves no trace in the steganographic channels. Noted in the finding aid as IMMEDIATIST CONFIGURATION; not an error.

**INSTRUCTION POINTER (IP)** The cursor that moves through the 2D execution grid, indicating the current cell. The IP's position, direction, and movement are governed by the Φ register.

**LAMBDA (Λ)** The third register. Governs function execution, arithmetic, logic, and consensus coordination. Named for Church's lambda calculus — the anonymous function that maps inputs to outputs. In ΦΜΛ, each Λ operation consumes a portion of the structure that supports it.

**MEDIATION** The structured negotiation process invoked when a schism reaches ESCALATED severity, or explicitly via `ΛM`. The Λ register proposes modifications; the other registers re-evaluate. Mediation fails if consensus is not achieved within `max_mediation_rounds`.

**MESSAGE (Μ type)** A sequence of bytes with attached provenance metadata. The fundamental data type of the Μ register. Every MESSAGE carries its archival bond.

**MU (Μ)** The second register. Governs data, I/O, and content. The message register. Named for the Greek letter associated with information in information theory; also micro-, the coefficient that determines how things slide against each other.

**MUTATION** The self-modification of a grid cell after instruction execution. Each executed cell's opcode advances one step in the mutation table. Mutation is irreversible. A program that loops through the same cell many times will eventually find that cell's instruction has become something else.

**ORIGINAL ORDER** *(archival)* The principle that records should be maintained in the order established by their creator. *(computational)* The initial opcode layout of the execution grid, as parsed from source. The `original_opcode` field in each cell preserves this for the finding aid.

**PHI (Φ)** The first register. Governs structure, navigation, and spatial addressing. Named for the golden ratio — the form that underlies apparent proportion. In ΦΜΛ, Φ defines the shape of the space before anything else is placed in it.

**PLEASE** A politeness prefix applicable to any consensus-requiring instruction. The PLEASE prefix grants additional consideration in the mediation protocol and increases the likelihood of ASSENT from the Φ and Μ registers. Its use is tracked; neglect is noted.

**PROVENANCE** *(archival)* The origin and custody history of a record. *(computational)* The metadata attached to each MESSAGE describing where, when, and by which register it was created, and all subsequent operations performed on it.

**RESILIENT CONFIGURATION** Notice E024. A program that survives 1000 cycles after the entropy threshold has been reached. This is a significant achievement and is noted approvingly in the finding aid. It does not mean the program is correct.

**SCHISM** A recorded disagreement: one or more registers DISsented from a consensus request. Minor schisms are notices. Fatal schisms terminate the program. The schism log in the finding aid is the primary evidence of a program's internal conflicts.

**SHELVE** *(archival)* To place materials on a shelf in a library or archive. *(computational)* The `S` opcode, which assigns a name to a grid coordinate for later retrieval.

**STRING MODE** A mode entered by the `"` opcode in which all cell contents are pushed as character integers to the Μ stack rather than executed as opcodes. String mode is register-agnostic and active across all Degrees.

**TEMPORARY AUTONOMOUS ZONE (TAZ)** *(Hakim Bey)* A space that briefly exists outside institutional control. *(computational)* The region of the execution grid that has not been covered by any declared FORM — the undeclared space in which the IP wanders without structural accountability. In the TAZ, operations execute without consuming any FORM's capacity, but schisms are more likely because the Φ register has no structural declarations to ASSENT against.

**TERMINUS** The `@` opcode. Halts program execution. Mutates to NOP after execution. If the IP passes through the mutated cell again, the termination point is gone — error E026 DEACCESSIONED.

**TOROIDAL** The grid topology in which edges wrap: moving past the east edge returns at the west edge. The territory is closed. There is no outside.

**WALL** A grid cell marked as impassable via the `W` opcode. The IP deflects 180° when it would enter a WALL cell. Walls are declared in the First Degree.

---

## Appendix A: Implementation Conformance

A conformant ΦΜΛ implementation MUST:

1. Implement all opcodes in §6 with the specified semantics
2. Produce a finding aid in `.faid` format for every execution
3. Implement the consensus protocol as specified in §7
4. Implement the mutation table as specified in §8
5. Implement the three Degrees with the specified opcode restrictions in §9
6. Implement at least Channels 1 (xattr) and 5 (stderr fallback) from §11
7. Produce all 26 error messages from §12 under the specified conditions
8. Process PLEASE-prefixed instructions with the specified priority in §7.4

A conformant implementation MAY:

1. Implement Channel 4 (slack space) with appropriate documentation
2. Extend the mutation table with implementation-defined successor relationships for undefined opcode values
3. Support grid dimensions larger than the specified minimum
4. Provide an interactive debugger that displays the finding aid in real time
5. Implement multi-grid execution (`ΦN`) and inter-process communication (`ΜT`, `ΜH`)

A conformant implementation MUST NOT:

1. Execute consensus-required instructions without completing the full three-register protocol
2. Skip finding aid output for any reason
3. Halt without recording a terminal state in the finding aid
4. Modify the `original_opcode` field of any cell after initial parsing

---

## Appendix B: Notes on the Tone of Error Messages

Error messages are written in archival terminology because the errors are archival failures. A STACK EXHAUSTED is not a runtime error — it is a finding: the collection has nothing left to describe. A DEGREE VIOLATION is not a programming mistake — it is an anachronism: a function-class act intruding into the declaration phase, like processing records before they have been accessioned.

The tone of error messages should match the institutional register of a finding aid: precise, impersonal, without judgment. The record does not mourn. The record notes. That the program has collapsed into ENTROPIC DISSOLUTION is not tragedy; it is a terminal state, logged at the correct severity level, cross-referenced to the self-modification record, and preserved in the finding aid for future reference.

The finding aid outlasts the program. This is the archival promise: even when nothing works anymore, the record of what happened survives. The ΦΜΛ interpreter is a processing archivist. Its primary product is not output — it is the finding aid.

---

*End of ΦΜΛ Language Specification, Version 0.1*
*This document was produced without steganographic channels. The whitespace, however, is not guaranteed to be innocent.*
