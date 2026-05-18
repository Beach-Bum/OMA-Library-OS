# ΦΜΛ (Phi Mu Lambda)

An esoteric programming language organized around three named registers — **Φ (Form)**, **Μ (Message)**, **Λ (Lambda)** — corresponding to the three phases of an archival workflow: structural declaration, content population, and functional activation.

## What this is

ΦΜΛ is a Turing-complete esolang where:

- Programs execute on a **2D toroidal grid** (Befunge-inspired)
- Instructions **mutate after each execution** (Malbolge-inspired)
- Every operation requires **consensus between three registers** — dissent is logged, not suppressed
- Programs can be **embedded invisibly in document whitespace** (steganographic I/O)
- Every execution produces a **finding aid** — archival metadata is the primary output; stdout is secondary
- Computation **degrades the substrate** — each Λ operation consumes Φ structure

The design draws from library science (Ranganathan's faceted classification), information theory (Shannon), archival theory (provenance, original order, Foucault's archive), and the esolang tradition (Befunge, Malbolge, INTERCAL, Whitespace).

## Files

| File | Description |
|------|-------------|
| `SPEC.md` | Full language specification (1132 lines). Implementable — defines opcodes, memory model, execution semantics, BNF grammar. |
| `interpreter.py` | Reference interpreter in Python (~970 lines). Runs all spec examples. |
| `research/book-research.md` | Source research document — library science, secret languages, esolangs, literary connections. |

## Quick start

```bash
# Run built-in test suite (5 examples including Hello World)
python3 interpreter.py --test

# Run a program file
python3 interpreter.py program.pml

# Run with verbose execution trace
python3 interpreter.py --verbose program.pml

# Extract and run program from document whitespace
python3 interpreter.py --embedded document.txt
```

## Status

This is a creative/research artifact — an esolang designed as source material for a novel about libraries, classification systems, and invisible languages. The spec is real and the interpreter works, but this is not production software.

## Origin

Emerged from research into library science × engineering × secret languages for a literary project. The three registers encode Form/Message/Function — a complete theory of information science compressed into three Greek letters.

## License

MIT
