//! The ΦΜΛ Grid Interpreter — two-dimensional execution engine.
//!
//! Implements the core of the ΦΜΛ Language Specification:
//!   - 2D toroidal grid with instruction pointer
//!   - Three registers (Φ, Μ, Λ) with consensus protocol
//!   - Three degrees of execution (Declaration, Population, Activation)
//!   - Self-mutation after each cell execution
//!   - Stack-based computation
//!   - Finding aid (execution log)
//!
//! Ported from the Python reference interpreter (interpreter.py).

use std::collections::HashMap;
use std::fmt::Write as FmtWrite;

// ── Directions ──────────────────────────────────────────────────────

const EAST: (i32, i32) = (1, 0);
const WEST: (i32, i32) = (-1, 0);
const NORTH: (i32, i32) = (0, -1);
const SOUTH: (i32, i32) = (0, 1);

// ── Degrees ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
enum Degree {
    First,  // Φ: structural declaration
    Second, // Μ: population
    Third,  // Λ: activation
}

// ── Stack Item ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum StackItem {
    Int(i64),
    Str(String),
}

impl StackItem {
    fn as_int(&self) -> i64 {
        match self {
            StackItem::Int(n) => *n,
            StackItem::Str(s) => s.parse().unwrap_or(0),
        }
    }

    fn as_str(&self) -> String {
        match self {
            StackItem::Int(n) => n.to_string(),
            StackItem::Str(s) => s.clone(),
        }
    }
}

// ── Grid Cell ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Cell {
    op: char,
    orig: char,
    frozen: bool,
    mutation_count: u32,
}

// ── Finding Aid ─────────────────────────────────────────────────────

#[derive(Default)]
struct FindingAid {
    log: Vec<String>,
    schisms: Vec<String>,
    mutations: Vec<String>,
}

impl FindingAid {
    fn log(&mut self, cycle: u64, msg: &str) {
        self.log.push(format!("  [{cycle:06}] {msg}"));
    }

    fn schism(&mut self, cycle: u64, pos: (usize, usize), op: char, dissenter: &str, reason: &str) {
        self.schisms.push(format!(
            "  [{cycle:06}] SCHISM at ({},{}) op='{}' — {}: {}",
            pos.0, pos.1, op, dissenter, reason
        ));
    }

    fn mutation(&mut self, cycle: u64, pos: (usize, usize), old: char, new: char) {
        self.mutations.push(format!(
            "  [{cycle:06}] MUT ({},{}): '{old}' → '{new}'",
            pos.0, pos.1
        ));
    }

    fn report(&self, width: usize, height: usize, cycles: u64, degree: Degree, exit: &str) -> String {
        let mut out = String::new();
        let bar = "═".repeat(72);
        let thin = "─".repeat(72);
        let _ = writeln!(out, "{bar}");
        let _ = writeln!(out, "FINDING AID");
        let _ = writeln!(out, "{bar}");
        let _ = writeln!(out, "Grid:       {width}x{height}");
        let _ = writeln!(out, "Degree:     {degree:?}");
        let _ = writeln!(out, "Cycles:     {cycles}");
        let _ = writeln!(out, "Schisms:    {}", self.schisms.len());
        let _ = writeln!(out, "Mutations:  {}", self.mutations.len());
        let _ = writeln!(out, "Exit:       {exit}");
        let _ = writeln!(out);
        let _ = writeln!(out, "{thin}");
        let _ = writeln!(out, "EXECUTION LOG");
        let _ = writeln!(out, "{thin}");
        for entry in &self.log {
            let _ = writeln!(out, "{entry}");
        }
        if self.log.is_empty() {
            let _ = writeln!(out, "  (none)");
        }
        let _ = writeln!(out);
        let _ = writeln!(out, "{thin}");
        let _ = writeln!(out, "MUTATIONS (first 30)");
        let _ = writeln!(out, "{thin}");
        for entry in self.mutations.iter().take(30) {
            let _ = writeln!(out, "{entry}");
        }
        if self.mutations.len() > 30 {
            let _ = writeln!(out, "  ... {} more", self.mutations.len() - 30);
        }
        if self.mutations.is_empty() {
            let _ = writeln!(out, "  (none)");
        }
        let _ = writeln!(out, "{bar}");
        out
    }
}

// ── Mutation Table ──────────────────────────────────────────────────

fn mutate_op(op: char) -> char {
    match op {
        '>' => 'v', 'v' => '<', '<' => '^', '^' => '>',
        '+' => '-', '-' => '*', '*' => '/', '/' => '`', '`' => '+',
        '.' => ',', ',' => '.',
        '@' => ' ',
        '|' => '_', '_' => '|',
        '#' => ' ',
        '[' => ']', ']' => '[',
        '{' => '}', '}' => '{',
        '!' => '?', '?' => '!',
        _ => op,
    }
}

// ── Grid State ──────────────────────────────────────────────────────

pub struct GridState {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    dir: (i32, i32),
    degree: Degree,
    stack: Vec<StackItem>,
    halted: bool,
    cycle: u64,
    exit_condition: String,
    output: String,
    in_string: bool,
    string_buf: String,
    fa: FindingAid,
    polite_next: bool,
    nop_ages: HashMap<(usize, usize), u32>,
}

impl GridState {
    pub fn from_source(source: &str) -> Self {
        let grid = parse_grid(source);
        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 8 };

        let mut fa = FindingAid::default();
        fa.log(0, &format!("Grid {width}x{height} parsed"));

        GridState {
            grid,
            width,
            height,
            x: 0,
            y: 0,
            dir: EAST,
            degree: Degree::First,
            stack: Vec::new(),
            halted: false,
            cycle: 0,
            exit_condition: "TERMINUS".into(),
            output: String::new(),
            in_string: false,
            string_buf: String::new(),
            fa,
            polite_next: false,
            nop_ages: HashMap::new(),
        }
    }

    pub fn run(&mut self, max_cycles: u64) -> &str {
        while !self.halted && self.cycle < max_cycles {
            self.exec_cell();
            self.step();
            self.cycle += 1;
        }

        if !self.halted && self.cycle >= max_cycles {
            self.exit_condition = "CYCLE_LIMIT".into();
            self.fa.log(self.cycle, "Cycle limit reached");
        }

        // Print finding aid
        let report = self.fa.report(
            self.width, self.height, self.cycle, self.degree, &self.exit_condition,
        );
        eprintln!("{report}");

        // Print accumulated output
        if !self.output.is_empty() {
            println!("{}", self.output);
        }

        &self.exit_condition
    }

    fn cell(&self, x: usize, y: usize) -> &Cell {
        &self.grid[y % self.height][x % self.width]
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        let h = self.height;
        let w = self.width;
        &mut self.grid[y % h][x % w]
    }

    fn step(&mut self) {
        let (dx, dy) = self.dir;
        let nx = ((self.x as i32 + dx).rem_euclid(self.width as i32)) as usize;
        let ny = ((self.y as i32 + dy).rem_euclid(self.height as i32)) as usize;
        self.x = nx;
        self.y = ny;
    }

    fn push(&mut self, item: StackItem) {
        self.stack.push(item);
    }

    fn pop(&mut self) -> StackItem {
        self.stack.pop().unwrap_or(StackItem::Int(0))
    }

    fn pop_int(&mut self) -> i64 {
        self.pop().as_int()
    }

    fn do_mutate(&mut self, x: usize, y: usize) {
        let cell = &self.grid[y % self.height][x % self.width];
        if cell.frozen {
            return;
        }
        let old = cell.op;
        if old == ' ' {
            let pos = (x, y);
            let age = self.nop_ages.entry(pos).or_insert(0);
            *age += 1;
            if *age >= 18 {
                let new_op = '?';
                self.fa.mutation(self.cycle, pos, ' ', new_op);
                self.grid[y % self.height][x % self.width].op = new_op;
                self.grid[y % self.height][x % self.width].mutation_count += 1;
                self.nop_ages.insert(pos, 0);
            }
            return;
        }
        let new_op = mutate_op(old);
        if new_op != old {
            self.fa.mutation(self.cycle, (x, y), old, new_op);
            self.grid[y % self.height][x % self.width].op = new_op;
            self.grid[y % self.height][x % self.width].mutation_count += 1;
        }
    }

    fn exec_cell(&mut self) {
        let x = self.x;
        let y = self.y;
        let op = self.grid[y][x].op;

        // String mode
        if self.in_string {
            if op == '"' {
                self.in_string = false;
                let s = std::mem::take(&mut self.string_buf);
                self.push(StackItem::Str(s));
            } else {
                self.string_buf.push(op);
            }
            return; // No mutation inside strings
        }

        // PLEASE detection
        if op == 'P' {
            self.polite_next = true;
            return;
        }

        let _polite = self.polite_next;
        self.polite_next = false;

        match op {
            // ── Navigation ──
            '>' => self.dir = EAST,
            '<' => self.dir = WEST,
            '^' => self.dir = NORTH,
            'v' => self.dir = SOUTH,
            '?' => {
                // Random direction
                let dirs = [EAST, WEST, NORTH, SOUTH];
                let idx = (self.cycle as usize) % 4;
                self.dir = dirs[idx];
            }

            // ── Stack / Numbers ──
            '0'..='9' => {
                self.push(StackItem::Int((op as u8 - b'0') as i64));
            }
            '"' => {
                self.in_string = true;
                self.string_buf.clear();
            }
            '$' => { let _ = self.pop(); } // discard
            '\\' => {
                // swap top two
                let a = self.pop();
                let b = self.pop();
                self.push(a);
                self.push(b);
            }
            ':' => {
                // duplicate top
                let a = self.pop();
                self.push(a.clone());
                self.push(a);
            }

            // ── Arithmetic ──
            '+' => {
                let b = self.pop_int();
                let a = self.pop_int();
                self.push(StackItem::Int(a.wrapping_add(b)));
            }
            '-' => {
                let b = self.pop_int();
                let a = self.pop_int();
                self.push(StackItem::Int(a.wrapping_sub(b)));
            }
            '*' => {
                let b = self.pop_int();
                let a = self.pop_int();
                self.push(StackItem::Int(a.wrapping_mul(b)));
            }
            '/' => {
                let b = self.pop_int();
                let a = self.pop_int();
                if b == 0 {
                    self.fa.schism(self.cycle, (x, y), op, "LAMBDA", "division by zero");
                    self.push(StackItem::Int(0));
                } else {
                    self.push(StackItem::Int(a / b));
                }
            }
            '%' => {
                let b = self.pop_int();
                let a = self.pop_int();
                if b == 0 {
                    self.push(StackItem::Int(0));
                } else {
                    self.push(StackItem::Int(a % b));
                }
            }
            '`' => {
                // greater than
                let b = self.pop_int();
                let a = self.pop_int();
                self.push(StackItem::Int(if a > b { 1 } else { 0 }));
            }
            '!' => {
                // logical not
                let a = self.pop_int();
                self.push(StackItem::Int(if a == 0 { 1 } else { 0 }));
            }

            // ── Conditionals ──
            '|' => {
                // vertical conditional
                let a = self.pop_int();
                self.dir = if a == 0 { SOUTH } else { NORTH };
            }
            '_' => {
                // horizontal conditional
                let a = self.pop_int();
                self.dir = if a == 0 { EAST } else { WEST };
            }

            // ── I/O ──
            '.' => {
                // output integer
                let a = self.pop_int();
                let _ = write!(self.output, "{a} ");
            }
            ',' => {
                // output char
                let a = self.pop_int();
                if let Some(ch) = char::from_u32(a as u32) {
                    self.output.push(ch);
                }
            }

            // ── Degree transitions ──
            'Φ' => {
                if self.degree == Degree::First {
                    self.fa.log(self.cycle, "Degree transition: FIRST → SECOND (Φ complete)");
                    self.degree = Degree::Second;
                }
            }
            'Μ' => {
                if self.degree == Degree::Second {
                    self.fa.log(self.cycle, "Degree transition: SECOND → THIRD (Μ complete)");
                    self.degree = Degree::Third;
                }
            }
            'Λ' => {
                // Λ in Third degree = activation marker (no-op, just marks intent)
                self.fa.log(self.cycle, "Λ activation marker encountered");
            }

            // ── Control ──
            '@' => {
                // Terminus — halt
                self.halted = true;
                self.exit_condition = if self.degree == Degree::Third {
                    "INITIATED".into()
                } else {
                    "UNINITIATED".into()
                };
                self.fa.log(self.cycle, &format!("HALT: {}", self.exit_condition));
            }
            '#' => {
                // Bridge — skip next cell
                self.step();
            }

            ' ' | '\t' => {
                // NOP
            }

            _ => {
                // Unknown opcode — log and continue
            }
        }

        // Mutate the cell we just executed (if not frozen)
        if op != ' ' && op != '"' {
            self.do_mutate(x, y);
        }
    }

    /// Display the current grid state (for inspect -deep)
    pub fn display_grid(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "ΦΜΛ Grid — {}x{}, {} mutations, degree {:?}",
            self.width, self.height,
            self.grid.iter().flatten().map(|c| c.mutation_count).sum::<u32>(),
            self.degree,
        );
        let _ = writeln!(out);
        for (y, row) in self.grid.iter().enumerate() {
            let _ = write!(out, "  ");
            for (x, cell) in row.iter().enumerate() {
                if x == self.x && y == self.y {
                    let _ = write!(out, "[{}]", cell.op);
                } else {
                    let _ = write!(out, " {} ", cell.op);
                }
            }
            let _ = writeln!(out);
        }
        out
    }
}

// ── Parser ──────────────────────────────────────────────────────────

fn parse_grid(source: &str) -> Vec<Vec<Cell>> {
    // Strip comments (⌜...⌝)
    let mut cleaned = String::new();
    let mut in_comment = false;
    for ch in source.chars() {
        if ch == '\u{231C}' {
            in_comment = true;
            cleaned.push(' ');
        } else if ch == '\u{231D}' {
            in_comment = false;
            cleaned.push(' ');
        } else if in_comment {
            cleaned.push(if ch == '\n' { '\n' } else { ' ' });
        } else {
            cleaned.push(ch);
        }
    }

    let rows: Vec<&str> = cleaned.split('\n').collect();
    let mut grid: Vec<Vec<Cell>> = Vec::new();

    for row_str in &rows {
        let mut cells = Vec::new();
        for ch in row_str.chars() {
            cells.push(Cell {
                op: ch,
                orig: ch,
                frozen: false,
                mutation_count: 0,
            });
        }
        grid.push(cells);
    }

    // Normalize to rectangle, minimum 8x8
    let width = grid.iter().map(|r| r.len()).max().unwrap_or(0).max(8);
    let height = grid.len().max(8);

    for row in &mut grid {
        while row.len() < width {
            row.push(Cell { op: ' ', orig: ' ', frozen: false, mutation_count: 0 });
        }
    }
    while grid.len() < height {
        grid.push(vec![Cell { op: ' ', orig: ' ', frozen: false, mutation_count: 0 }; width]);
    }

    grid
}
