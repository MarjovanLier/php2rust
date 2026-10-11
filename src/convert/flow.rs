//! Definite assignment and liveness for one body, a function or `fn main()` (P2R-34,
//! P2R-35).
//!
//! The converter reports every read and write of a variable to a [`Flow`], in the order
//! PHP evaluates them: for `$x = e` the reads in `e` come first and then the write; for
//! `$x += e` and `$x++` the read of `$x` and then the write. A parameter's value counts
//! as a write before the first statement (`write(var, None)`).
//!
//! The events sit in blocks of straight-line code, and edges join the blocks into a
//! control-flow graph. [`Flow::finish`] runs three data-flow analyses over the graph until
//! nothing changes, so its answers hold for any graph, loops included:
//! - definite assignment (forward, intersection at joins): a read is refused unless every
//!   path to it writes the variable first (P2R-34);
//! - liveness (backward, union at joins): a write is refused when no path from it reads the
//!   value before the next write or the end of the body (P2R-35);
//! - possible assignment (forward, union at joins): a variable needs `mut` when one of its
//!   writes can run while it may already hold a value.
//!
//! Every condition counts as unknown, so every edge can be taken. A block with no
//! successor is an exit of the body.
//!
//! Version 1 so far has straight-line code only, so a body is one block. Branches and
//! loops add three methods and leave `finish` as it is: `block()` adds an empty
//! block, `edge(from, to)` lets control pass from one block to another, and `enter(block)`
//! makes later events land in that block. A `return`, `break` or `continue` gives its
//! block no edge to the next statement. A `foreach` key or value is a new
//! variable id written at the start of the loop body, so a later loop that reuses the PHP
//! name gets a variable of its own (P2R-41).

use mago_span::Span;

/// A variable of the body, numbered from 0 by the caller.
pub(super) type Var = usize;

#[derive(Clone, Copy)]
enum Event {
    Read(Var, Span),
    /// `None` is a parameter's value on entry.
    Write(Var, Option<Span>),
}

#[derive(Default)]
struct Block {
    events: Vec<Event>,
    next: Vec<usize>,
}

pub(super) struct Flow {
    blocks: Vec<Block>,
    /// The block that new events go into.
    current: usize,
}

/// What [`Flow::finish`] found.
pub(super) struct Facts {
    /// Reads that some path reaches with the variable unassigned (P2R-34).
    pub unassigned: Vec<(Var, Span)>,
    /// Writes whose value no path reads (P2R-35). `None` is a parameter's value on entry.
    pub unread: Vec<(Var, Option<Span>)>,
    /// By variable: some write can run while the variable may already hold a value.
    pub reassigned: Vec<bool>,
}

/// One flag per variable.
type Set = Vec<bool>;

impl Default for Flow {
    fn default() -> Self {
        Self {
            blocks: vec![Block::default()],
            current: 0,
        }
    }
}

impl Flow {
    pub fn read(&mut self, var: Var, span: Span) {
        self.blocks[self.current]
            .events
            .push(Event::Read(var, span));
    }

    pub fn write(&mut self, var: Var, span: Option<Span>) {
        self.blocks[self.current]
            .events
            .push(Event::Write(var, span));
    }

    /// Runs the analyses for variables `0..vars`.
    pub fn finish(&self, vars: usize) -> Facts {
        let mut previous: Vec<Vec<usize>> = vec![Vec::new(); self.blocks.len()];
        for (from, block) in self.blocks.iter().enumerate() {
            for &to in &block.next {
                previous[to].push(from);
            }
        }
        // Definitely and possibly assigned at the end of each block. Definite starts full,
        // so a block that no path reaches yet refuses nothing.
        let mut definite: Vec<Set> = vec![vec![true; vars]; self.blocks.len()];
        let mut possible: Vec<Set> = vec![vec![false; vars]; self.blocks.len()];
        let mut changed = true;
        while changed {
            changed = false;
            for (index, block) in self.blocks.iter().enumerate() {
                let (mut def, mut pos) = arriving(index, &previous, &definite, &possible, vars);
                for event in &block.events {
                    if let Event::Write(var, _) = *event {
                        def[var] = true;
                        pos[var] = true;
                    }
                }
                changed |= def != definite[index] || pos != possible[index];
                definite[index] = def;
                possible[index] = pos;
            }
        }
        // Live at the start of each block: some path from there reads the variable before
        // it writes it.
        let mut live: Vec<Set> = vec![vec![false; vars]; self.blocks.len()];
        changed = true;
        while changed {
            changed = false;
            for (index, block) in self.blocks.iter().enumerate().rev() {
                let mut set = leaving(block, &live, vars);
                for event in block.events.iter().rev() {
                    match *event {
                        Event::Read(var, _) => set[var] = true,
                        Event::Write(var, _) => set[var] = false,
                    }
                }
                changed |= set != live[index];
                live[index] = set;
            }
        }
        let mut facts = Facts {
            unassigned: Vec::new(),
            unread: Vec::new(),
            reassigned: vec![false; vars],
        };
        for (index, block) in self.blocks.iter().enumerate() {
            let (mut def, mut pos) = arriving(index, &previous, &definite, &possible, vars);
            for event in &block.events {
                match *event {
                    Event::Read(var, span) if !def[var] => facts.unassigned.push((var, span)),
                    Event::Read(..) => {}
                    Event::Write(var, _) => {
                        facts.reassigned[var] |= pos[var];
                        def[var] = true;
                        pos[var] = true;
                    }
                }
            }
            let mut set = leaving(block, &live, vars);
            for event in block.events.iter().rev() {
                match *event {
                    Event::Read(var, _) => set[var] = true,
                    Event::Write(var, span) => {
                        if !set[var] {
                            facts.unread.push((var, span));
                        }
                        set[var] = false;
                    }
                }
            }
        }
        facts
    }
}

/// Definitely and possibly assigned where control enters block `index`. Block 0 is also
/// entered from the start of the body, where nothing is assigned.
fn arriving(
    index: usize,
    previous: &[Vec<usize>],
    definite: &[Set],
    possible: &[Set],
    vars: usize,
) -> (Set, Set) {
    let mut def = vec![index != 0; vars];
    let mut pos = vec![false; vars];
    for &from in &previous[index] {
        for (d, &f) in def.iter_mut().zip(&definite[from]) {
            *d &= f;
        }
        for (p, &f) in pos.iter_mut().zip(&possible[from]) {
            *p |= f;
        }
    }
    (def, pos)
}

/// Live where control leaves `block`: some path from a successor reads the variable.
fn leaving(block: &Block, live: &[Set], vars: usize) -> Set {
    let mut set = vec![false; vars];
    for &to in &block.next {
        for (s, &l) in set.iter_mut().zip(&live[to]) {
            *s |= l;
        }
    }
    set
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(offset: u32) -> Span {
        Span::dummy(offset, offset + 1)
    }

    #[test]
    fn straight_line_reads_after_writes() {
        let mut flow = Flow::default();
        flow.write(0, Some(at(1)));
        flow.read(0, at(2));
        flow.write(0, Some(at(3)));
        flow.read(0, at(4));
        let facts = flow.finish(1);
        assert!(facts.unassigned.is_empty());
        assert!(facts.unread.is_empty());
        assert_eq!(facts.reassigned, vec![true]);
    }

    #[test]
    fn read_before_write_and_unread_writes() {
        let mut flow = Flow::default();
        flow.read(0, at(1));
        flow.write(0, Some(at(2)));
        flow.write(0, Some(at(3)));
        flow.read(0, at(4));
        flow.write(1, Some(at(5)));
        let facts = flow.finish(2);
        assert_eq!(facts.unassigned, vec![(0, at(1))]);
        let mut unread = facts.unread;
        unread.sort();
        assert_eq!(unread, vec![(0, Some(at(2))), (1, Some(at(5)))]);
        assert_eq!(facts.reassigned, vec![true, false]);
    }

    #[test]
    fn parameter_written_before_it_is_read() {
        let mut flow = Flow::default();
        flow.write(0, None);
        flow.write(0, Some(at(1)));
        flow.read(0, at(2));
        let facts = flow.finish(1);
        assert_eq!(facts.unread, vec![(0, None)]);
        assert_eq!(facts.reassigned, vec![true]);
    }

    /// `if (c) { $x = 1; } else { $x = 2; } echo $x; echo $y;` built by hand, as the control flow code
    /// will build it: each branch assigns once, so `$x` needs no `mut`, and `$y`, assigned
    /// in one branch only, is not assigned on every path.
    #[test]
    fn branches_join_without_a_rewrite() {
        let blocks = vec![
            Block {
                events: vec![],
                next: vec![1, 2],
            },
            Block {
                events: vec![Event::Write(0, Some(at(1))), Event::Write(1, Some(at(2)))],
                next: vec![3],
            },
            Block {
                events: vec![Event::Write(0, Some(at(3)))],
                next: vec![3],
            },
            Block {
                events: vec![Event::Read(0, at(4)), Event::Read(1, at(5))],
                next: vec![],
            },
        ];
        let facts = Flow { blocks, current: 0 }.finish(2);
        assert_eq!(facts.unassigned, vec![(1, at(5))]);
        assert!(facts.unread.is_empty());
        assert_eq!(facts.reassigned, vec![false, false]);
    }

    /// `$i = 0; while (c) { echo $i; $i = 1; }` built by hand: the back edge carries the
    /// loop's write to the read, so the write is read and `$i` needs `mut`.
    #[test]
    fn loops_reach_a_fixed_point() {
        let blocks = vec![
            Block {
                events: vec![Event::Write(0, Some(at(1)))],
                next: vec![1],
            },
            Block {
                events: vec![],
                next: vec![2, 3],
            },
            Block {
                events: vec![Event::Read(0, at(2)), Event::Write(0, Some(at(3)))],
                next: vec![1],
            },
            Block::default(),
        ];
        let facts = Flow { blocks, current: 0 }.finish(1);
        assert!(facts.unassigned.is_empty());
        assert!(facts.unread.is_empty());
        assert_eq!(facts.reassigned, vec![true]);
    }
}
