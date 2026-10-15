//! Variables: reads, assignments, `++` and `--`, and the `let` and `mut` they need
//! (P2R-20 to P2R-22, P2R-31, P2R-34, P2R-35, P2R-43, P2R-44, P2R-50).
//!
//! A variable is declared by a `let`, at its first write in the source when that write
//! sits in the Rust block that holds every read and write of it. Otherwise a typed `let`
//! without a value goes in that block, on a slot line before the block statement that
//! holds the first write (P2R-34). A variable first written inside a loop gets its typed
//! `let` above the outermost loop that holds that write, even when only the loop reads
//! it: `reassigned` counts back edges, so an inline `let` in the loop body would get
//! rustc's unused_mut (see flow.rs). Where the `let` goes and whether it needs `mut` are
//! known only at the end of the body, so those lines carry a mark until
//! [`Converter::finish_variables`] replaces it. The type of a variable whose first write
//! is `[]` comes from the other reads and writes of the body, which `list_types` scans
//! before it converts; when they disagree, a later read or write gives it (P2R-21).
//!
//! A `foreach` key or value is a new entry, declared by the `for` pattern and never by a
//! `let`, so a later loop can reuse its PHP name with another type (P2R-41). `lookup`
//! finds the latest entry of a name whose loop has not ended; after the loop, a read of
//! the name finds an earlier entry or none, and a write declares a new one. The `for`
//! line waits for the end of the body like a first write: its pattern gets `_` when the
//! binding is never read and `mut` when the body assigns it again (P2R-35).
//!
//! P2R-50 stops int arithmetic that the tool can prove overflows, and `%` by a divisor it
//! can prove is 0. So each entry keeps the latest value of its variable when it is known,
//! and a read of it is an `RExpr::Known` that `checked_constant` and `binary` check.
//! rustc 1.99 rejects many of these cases at compile time (arithmetic_overflow,
//! unconditional_panic), but not all: it never tracks a variable that `print!` borrows,
//! and tracks a variable written twice only inside one MIR block. The spec's rule is
//! wider, so the output never depends on that. The values must never be weaker than
//! rustc's, because a missed value lets through code that rustc rejects. In straight-line
//! code the latest write in source order gives the value, and a call between does not
//! change it. control.rs keeps the values right at loops and joins:
//! - Before it converts a loop's condition, body and step, it clears the value of every
//!   variable written anywhere in that loop (a pre-scan), and keeps all other values.
//!   Clearing more would be weaker than rustc: `let x = 0; while c() { print!("{}", 10 %
//!   x); }` is a rustc error.
//! - At an `if` join, a value stays only when every path that reaches the join agrees.
//!   Paths that end in `return`, `break` or `continue` do not count.
//! - After a loop, the value is the meet of the value at the loop head and at every
//!   `break`.

use std::collections::{HashMap, HashSet};

use heck::ToSnakeCase;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use super::control::{Loop, emit};
use super::flow::{Flow, Var};
use super::lists::unparenthesized;
use super::maps::{MAP_APPEND, MAP_MAY_BE_NULL, MAP_PROPERTY_CHANGE, UNSET};
use super::{
    Converter, Mode, ParameterSignature, classes, helpers, lists, nulls, numbers, strings,
};
use crate::printer::{self, BinOp, RExpr};
use crate::types::{self, RustType};

/// Starts a line whose text is known only at the end of the body, followed by its index
/// in `Variables::pending`. `printer::quote` escapes every control character, so no other
/// output text holds it.
const MARK: char = '\u{1}';

/// The PHP name of `$this` without `$`, which a method holds as its first parameter, `self`
/// (P2R-135).
pub(super) const THIS: &[u8] = b"this";

/// The problem of a copy of a value of a changing type (P2R-145).
const NO_COPY: &str = "an object whose class changes its objects is never copied; it moves only from a local variable or an owned parameter that is not read again (P2R-145)";

/// The problem of a write to the binding of a null check (P2R-104).
const CHECKED: &str = "a variable that a check against null binds cannot be assigned until the check's block ends (P2R-104)";

pub(super) use crate::printer::COPY;

/// A line whose text is known only at the end of the body.
enum Pending {
    /// The typed `let` lines of the variables declared before a block statement.
    Slot,
    /// A variable's first write: `let x = v;`, or `x = v;` when its `let` is in a slot.
    First {
        var: Var,
        /// The `let` names the variable's type, which `[]` gets only from a later read or
        /// write (P2R-21).
        typed: bool,
        value: String,
        /// The value opens a `match`, so no `;` ends the line (P2R-123).
        open: bool,
    },
    /// A `[]` assigned to a declared variable: `x = Vec::new();`, or `x = IndexMap::new();`
    /// when the variable holds a map at the end of the body (P2R-21, P2R-241).
    Empty { var: Var },
    /// The opening line of a foreach: `for pattern in list {` (P2R-41).
    Foreach {
        /// The key, and whether it is read through `&`, as from a borrowed map (P2R-242).
        key: Option<(Var, bool)>,
        value: Var,
        /// The loop borrows a list of Copy values, so the value is read through `&`.
        by_ref: bool,
        /// The text after the pattern: ` in list {`.
        rest: String,
    },
    /// The opening line of an `if` or `elseif` whose condition checks variables against
    /// null (P2R-104).
    Condition {
        /// `if` or `} else if`.
        keyword: &'static str,
        parts: Vec<Part>,
        /// `{`, or `{}` for an empty body.
        open: &'static str,
    },
    /// The opening line of `if ($v === null)`: `let Some(v) = v else {` when its body
    /// always leaves and `binding` is read after it, otherwise `if v.is_none() {`
    /// (P2R-104).
    Guard {
        binding: Option<Var>,
        /// `let Some(v) = v`.
        pattern: String,
        /// `v.is_none()`.
        test: String,
    },
    /// The closing line of a guard: `};` after a let-else, otherwise `}`.
    GuardEnd { guard: usize },
}

/// A Rust block of the body: the body itself, a branch or a loop body.
struct Scope {
    /// The enclosing block, and the slot before the statement that opens this one.
    parent: Option<(usize, usize)>,
    looped: bool,
}

struct Entry {
    /// The PHP name without `$`.
    php_name: Vec<u8>,
    rust_name: String,
    /// `None` once a problem with the name or the type is recorded. Later uses then
    /// convert to nothing, and that problem fails the conversion.
    ty: Option<RustType>,
    /// The snake_case name of a parameter, for the `_` of an unused one (P2R-35).
    parameter: Option<String>,
    /// The first write in the body.
    assigned: Option<Span>,
    /// The value the variable holds here, when rustc knows it too (P2R-50).
    value: Option<i64>,
    /// The block of the first write.
    first: usize,
    /// The innermost block that holds every read and write.
    common: usize,
    /// The variable holds `[]` and has no type yet; `ty` is `None` until a later read or
    /// write gives it (P2R-21).
    empty: bool,
    /// A string or list that Rust borrows, as it does a parameter (P2R-24): the binding of
    /// a null check on an `Option<String>` is a `&str` (P2R-104).
    borrowed: bool,
    binding: Option<Binding>,
    /// How a parameter takes its argument (P2R-143).
    mode: Mode,
    /// A change goes through the variable, so it needs `mut` (P2R-148).
    changed: bool,
    /// A string or list parameter that the body changes, so the body starts with a copy
    /// line and the variable owns its value from there on (P2R-181).
    copied: bool,
    /// A `mixed` parameter of an `ArrayAccess` method that K or V types (P2R-220).
    templated: bool,
}

/// A foreach key or value (P2R-41), or the binding of a null check (P2R-104).
struct Binding {
    /// The snake_case name without `r#`, for the `_` of an unused binding (P2R-35).
    snake: String,
    /// The loop body assigns it again, so the pattern needs `mut`.
    rewritten: bool,
    /// The loop or the check has ended, so `lookup` no longer finds it.
    closed: bool,
    /// The binding of `if ($v !== null)` or of a let-else, which nothing may assign
    /// (P2R-104).
    null_check: bool,
}

/// One operand of the top-level `&&` chain of a condition that checks a variable against
/// null (P2R-104).
pub(super) enum Part {
    /// Any other operand.
    Plain(String),
    /// `$v !== null`: `pattern` (`let Some(v) = v`) when the binding is read, otherwise
    /// `test` (`v.is_some()`).
    Let {
        binding: Var,
        pattern: String,
        test: String,
    },
}

/// The variables of the body being converted, a function or `fn main()`, and the state
/// of its control flow.
pub(super) struct Variables {
    /// Indexed by [`Var`] in order of first write, so the output never depends on
    /// hashing (P2R-06).
    entries: Vec<Entry>,
    pub(super) flow: Flow,
    /// By copy mark: the read that moves, the statement that holds it, and whether the
    /// value may never be copied (P2R-145).
    copies: Vec<(Span, Span, bool)>,
    /// The `=` property writes on a local variable or an owned parameter: the read of the
    /// root and the write, which stops when no path reads the root again (P2R-142).
    field_writes: Vec<(Span, Span)>,
    /// The Rust blocks, indexed from 0, the body itself.
    scopes: Vec<Scope>,
    /// The Rust block that new lines go into.
    pub(super) scope: usize,
    pending: Vec<Pending>,
    /// Type problems of reads, reported at the end unless the read may be unassigned.
    read_problems: Vec<(Span, String)>,
    /// Why no path reaches the next statement; `None` while one does (P2R-40).
    pub(super) left: Option<&'static str>,
    /// The loops around the statement being converted, innermost last.
    pub(super) loops: Vec<Loop>,
    /// The variables that hold the lists of the foreach loops around the statement being
    /// converted, which their bodies must not change (P2R-41).
    iterated: Vec<Var>,
    /// By foreach key or value: the flow block after its loop (P2R-41).
    exits: Vec<(usize, Var)>,
    /// The list type that the reads and writes of the body give each PHP name, for a `[]`
    /// (P2R-21); see `Converter::site_types`.
    pub(super) list_types: HashMap<Vec<u8>, Option<RustType>>,
    /// The `Option` type of each PHP name that may hold null, for a `null` and for a first
    /// write of a value that is not null (P2R-102); `None` when its reads and writes give
    /// different types. See `Converter::site_types`.
    pub(super) null_types: HashMap<Vec<u8>, Option<RustType>>,
    /// The bindings of let-else guards, with the Rust block each holds for until it ends
    /// (P2R-104).
    guards: Vec<(usize, Var)>,
    /// While a `for` step converts: the PHP names that a let-else in the loop body guards.
    /// The step also runs at each `continue`, where the binding may shadow the variable or
    /// not exist yet, so the step must not read or write them (P2R-104).
    pub(super) step_guards: Vec<Vec<u8>>,
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            flow: Flow::default(),
            copies: Vec::new(),
            field_writes: Vec::new(),
            scopes: vec![Scope {
                parent: None,
                looped: false,
            }],
            scope: 0,
            pending: Vec::new(),
            read_problems: Vec::new(),
            left: None,
            loops: Vec::new(),
            iterated: Vec::new(),
            exits: Vec::new(),
            list_types: HashMap::new(),
            null_types: HashMap::new(),
            guards: Vec::new(),
            step_guards: Vec::new(),
        }
    }
}

impl Variables {
    /// A function body, where each parameter holds its argument from the start.
    /// The parameters whose PHP names are in `copied` own a copy of their value (P2R-181).
    pub(super) fn with_parameters(
        parameters: &[ParameterSignature],
        copied: &HashSet<Vec<u8>>,
    ) -> Self {
        let mut variables = Self::default();
        for (var, parameter) in parameters.iter().enumerate() {
            variables.flow.write(var, None);
            variables.entries.push(Entry {
                copied: copied.contains(&parameter.php_name),
                php_name: parameter.php_name.clone(),
                rust_name: parameter.rust_name.clone(),
                ty: Some(parameter.ty.clone()),
                parameter: Some(parameter.snake.clone()),
                assigned: None,
                value: None,
                first: 0,
                common: 0,
                empty: false,
                borrowed: false,
                binding: None,
                mode: parameter.mode,
                changed: false,
                templated: false,
            });
        }
        variables
    }

    /// Marks the parameters at `positions` as typed by the `ArrayAccess` template
    /// (P2R-220).
    pub(super) fn template(&mut self, positions: &[usize]) {
        for &at in positions {
            if let Some(entry) = self.entries.get_mut(at) {
                entry.templated = true;
            }
        }
    }

    pub(super) fn read(&mut self, var: Var, span: Span) {
        self.flow.read(var, span);
        self.touch(var);
    }

    fn write(&mut self, var: Var, span: Option<Span>) {
        self.flow.write(var, span);
        self.touch(var);
    }

    /// Widens the block that must hold the `let` of `var` to the current one.
    fn touch(&mut self, var: Var) {
        let common = common(&self.scopes, self.entries[var].common, self.scope);
        self.entries[var].common = common;
    }

    /// Starts a block of the statement after `slot`, a loop body when `looped`. Gives the
    /// block to return to with [`Variables::close`].
    pub(super) fn open(&mut self, slot: usize, looped: bool) -> usize {
        self.scopes.push(Scope {
            parent: Some((self.scope, slot)),
            looped,
        });
        std::mem::replace(&mut self.scope, self.scopes.len() - 1)
    }

    /// Goes back to block `outer`. A let-else binding holds until its block ends, so every
    /// guard whose block is no longer open ends too (P2R-104).
    pub(super) fn close(&mut self, outer: usize) {
        self.scope = outer;
        let open = chain(&self.scopes, outer);
        for &(block, var) in &self.guards {
            if !open.contains(&block)
                && let Some(binding) = &mut self.entries[var].binding
            {
                binding.closed = true;
            }
        }
    }

    /// Holds the binding of a let-else until the current block ends (P2R-104).
    pub(super) fn guard(&mut self, binding: Var) {
        self.guards.push((self.scope, binding));
    }

    /// The pending opening line of `if ($v === null)`, at `depth`, and its index for
    /// [`Variables::guard_end`] and [`Variables::set_guard`] (P2R-104).
    pub(super) fn guard_line(
        &mut self,
        depth: usize,
        pattern: String,
        test: String,
    ) -> (String, usize) {
        self.pending.push(Pending::Guard {
            binding: None,
            pattern,
            test,
        });
        let index = self.pending.len() - 1;
        (format!("{}{MARK}{index}", "    ".repeat(depth)), index)
    }

    /// The pending closing line of the guard at `guard`.
    pub(super) fn guard_end(&mut self, depth: usize, guard: usize) -> String {
        self.pending.push(Pending::GuardEnd { guard });
        format!("{}{MARK}{}", "    ".repeat(depth), self.pending.len() - 1)
    }

    /// Gives the guard at `guard` its binding, once its body is known to always leave.
    pub(super) fn set_guard(&mut self, guard: usize, var: Var) {
        if let Some(Pending::Guard { binding, .. }) = self.pending.get_mut(guard) {
            *binding = Some(var);
        }
    }

    /// The known value of every variable (P2R-50).
    pub(super) fn values(&self) -> Vec<Option<i64>> {
        self.entries.iter().map(|entry| entry.value).collect()
    }

    /// Sets every value from `values`; a variable added since they were taken is unknown.
    pub(super) fn set_values(&mut self, values: &[Option<i64>]) {
        for (var, entry) in self.entries.iter_mut().enumerate() {
            entry.value = values.get(var).copied().flatten();
        }
    }

    /// The value `var` holds here, when it is known (P2R-50).
    pub(super) fn value(&self, var: Var) -> Option<i64> {
        self.entries[var].value
    }

    /// Sets the value `var` holds here; `None` forgets it.
    pub(super) fn set_value(&mut self, var: Var, value: Option<i64>) {
        self.entries[var].value = value;
    }

    /// The copy mark for a statement that moves the variable read at `read`. A `strict`
    /// mark is for a value of a changing type, which stops when it is read again instead of
    /// taking a `.clone()` (P2R-145).
    pub(super) fn copy_mark(&mut self, read: Span, statement: Span, strict: bool) -> String {
        self.copies.push((read, statement, strict));
        format!("{COPY}{:06x}{COPY}", self.copies.len() - 1)
    }

    /// True when `rust_name` is a parameter or a borrowed binding: a string is then a
    /// `&str` and a list a `&[T]` (P2R-24, P2R-104). A foreach key or value can shadow a
    /// parameter of the same name, so the entry in scope decides.
    pub(super) fn is_borrowed(&self, rust_name: &str) -> bool {
        self.entries
            .iter()
            .rev()
            .find(|e| e.rust_name == rust_name && !closed(e))
            .is_some_and(|e| {
                (e.parameter.is_some() && e.mode != Mode::Owned && !e.copied) || e.borrowed
            })
    }

    /// The mode of the parameter or foreach value named `rust_name`, or `Owned` for a
    /// plain variable, which owns its value (P2R-143, P2R-144).
    pub(super) fn mode_of(&self, rust_name: &str) -> Mode {
        let entry = self
            .entries
            .iter()
            .rev()
            .find(|e| e.rust_name == rust_name && !closed(e));
        match entry {
            Some(e) if e.parameter.is_some() || e.borrowed => e.mode,
            _ => Mode::Owned,
        }
    }

    /// Records that a change goes through `var`, so it needs `mut` (P2R-148).
    pub(super) fn set_changed(&mut self, var: Var) {
        self.entries[var].changed = true;
    }

    /// Records the `=` property write `write` whose root, read at `read`, is the variable
    /// named `rust_name`, when that variable owns its object: rustc rejects a field write
    /// that no path reads afterwards (P2R-142).
    pub(super) fn field_write(&mut self, rust_name: &str, read: Span, write: Span) {
        if self.mode_of(rust_name) != Mode::Owned {
            return;
        }
        let found = self
            .entries
            .iter()
            .rposition(|e| e.rust_name == rust_name && !closed(e));
        if let Some(var) = found {
            self.flow.field(var, read);
            self.field_writes.push((read, write));
        }
    }

    /// Makes the foreach value `var` a reference into the list it iterates, `&mut` when
    /// `mode` is `Mut`: it never moves and is never assigned (P2R-146).
    pub(super) fn lend(&mut self, var: Var, mode: Mode) {
        self.entries[var].borrowed = true;
        self.entries[var].mode = mode;
    }

    /// The pending opening line of an `if` or `elseif` that checks variables against null,
    /// at `depth` (P2R-104).
    pub(super) fn condition_line(
        &mut self,
        depth: usize,
        keyword: &'static str,
        parts: Vec<Part>,
        open: &'static str,
    ) -> String {
        self.pending.push(Pending::Condition {
            keyword,
            parts,
            open,
        });
        format!("{}{MARK}{}", "    ".repeat(depth), self.pending.len() - 1)
    }

    /// The Rust type of `var`; `None` once a problem with it is recorded.
    pub(super) fn ty(&self, var: Var) -> Option<&RustType> {
        self.entries[var].ty.as_ref()
    }

    /// The Rust name of `var`.
    pub(super) fn name_of(&self, var: Var) -> &str {
        &self.entries[var].rust_name
    }

    /// Ends the bindings of a null check, when the block they hold for ends (P2R-104).
    pub(super) fn close_bindings(&mut self, bindings: &[Var]) {
        for &var in bindings {
            if let Some(binding) = &mut self.entries[var].binding {
                binding.closed = true;
            }
        }
    }

    /// The pending `for` line of a foreach, at `depth` (P2R-41).
    pub(super) fn for_line(
        &mut self,
        depth: usize,
        key: Option<(Var, bool)>,
        value: Var,
        by_ref: bool,
        rest: String,
    ) -> String {
        self.pending.push(Pending::Foreach {
            key,
            value,
            by_ref,
            rest,
        });
        format!("{}{MARK}{}", "    ".repeat(depth), self.pending.len() - 1)
    }

    /// Starts the body of a foreach over the list in `list`, which the body must not
    /// change (P2R-41).
    pub(super) fn iterate(&mut self, list: Option<Var>) {
        self.iterated.extend(list);
    }

    /// Ends a foreach: its key and value are gone, and no other variable of their names may
    /// be live at `exit`, the flow block after the loop (P2R-41).
    pub(super) fn end_foreach(&mut self, list: Option<Var>, bindings: &[Var], exit: usize) {
        if list.is_some() {
            self.iterated.pop();
        }
        for &var in bindings {
            if let Some(binding) = &mut self.entries[var].binding {
                binding.closed = true;
            }
            self.exits.push((exit, var));
        }
    }
}

/// True for a foreach key or value whose loop has ended.
fn closed(entry: &Entry) -> bool {
    entry.binding.as_ref().is_some_and(|b| b.closed)
}

/// The Rust name of the variable a chain of field and index reads starts from: `x` for
/// `x`, `x.p` and `x.p[0].q` (P2R-142). `None` for any other expression.
pub(super) fn chain_root(expr: &RExpr) -> Option<&str> {
    match expr {
        RExpr::Var(name) | RExpr::Known(name, _) => Some(name),
        RExpr::Field(object, _) => chain_root(object),
        RExpr::Index(list, _) | RExpr::Key(list, _) => chain_root(list),
        _ => None,
    }
}

/// True for the expressions that change a variable: an assignment, `++` and `--`.
pub(super) fn changes_variable(expression: &Expression<'_>) -> bool {
    match expression {
        Expression::Assignment(_) | Expression::UnaryPostfix(_) => true,
        Expression::UnaryPrefix(unary) => matches!(
            unary.operator,
            UnaryPrefixOperator::PreIncrement(_) | UnaryPrefixOperator::PreDecrement(_)
        ),
        _ => false,
    }
}

impl Converter<'_> {
    /// An assignment, `++` or `--` as a statement, or as a `for` initialisation or step
    /// (P2R-43).
    pub(super) fn assignment_statement(
        &mut self,
        expression: &Expression<'_>,
        depth: usize,
        body: &mut Vec<String>,
    ) {
        if let Some(text) = self.change(expression) {
            body.push(emit(depth, text));
        }
    }

    /// Writes the slot line where the typed `let` of a variable first written inside the
    /// block statement that follows goes (P2R-34). Gives the slot.
    pub(super) fn slot(&mut self, depth: usize, body: &mut Vec<String>) -> usize {
        self.variables.pending.push(Pending::Slot);
        let slot = self.variables.pending.len() - 1;
        body.push(format!("{}{MARK}{slot}", "    ".repeat(depth)));
        slot
    }

    /// Forgets the value of every variable that `statement` writes, before the loop that
    /// holds it converts (P2R-50).
    pub(super) fn forget_writes(&mut self, statement: &Statement<'_>) {
        let mut written = Vec::new();
        Written.walk_statement(statement, &mut written);
        self.forget(written);
    }

    /// The same for a loop condition or a `for` step.
    pub(super) fn forget_writes_in(&mut self, expression: &Expression<'_>) {
        let mut written = Vec::new();
        Written.walk_expression(expression, &mut written);
        self.forget(written);
    }

    fn forget(&mut self, written: Vec<&DirectVariable<'_>>) {
        for variable in written {
            if let Some(var) = self.lookup(variable) {
                self.variables.set_value(var, None);
            }
        }
    }

    /// Refuses an assignment, `++` or `--` used as a value (P2R-44). Its write still
    /// counts, so a later read is not reported as unassigned.
    pub(super) fn change_as_value(
        &mut self,
        expression: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        self.change(expression);
        let message = if matches!(expression, Expression::Assignment(_)) {
            "an assignment used as a value is not supported (P2R-44)"
        } else {
            "++ and -- used as a value are not supported (P2R-44)"
        };
        self.refuse(expression, message);
        None
    }

    /// A read of a variable (P2R-34).
    pub(super) fn variable(&mut self, variable: &DirectVariable<'_>) -> Option<(RExpr, RustType)> {
        let in_static = self
            .current_method
            .is_some_and(|(class, at)| self.classes[class].methods[at].is_static);
        if in_static && variable.name == b"$this" {
            self.refuse(
                variable,
                "$this does not exist in a static method, and PHP throws an Error (P2R-152)",
            );
            return None;
        }
        let Some(var) = self.lookup(variable) else {
            let php_name = &variable.name[1..];
            // A foreach key or value is gone after its loop, where PHP keeps its last value
            // (P2R-41). Any other name has no write before this read in the source, so a path
            // reaches it unassigned (P2R-34).
            let binding = self.variables.entries.iter().rev().find_map(|e| {
                (e.php_name == php_name && closed(e))
                    .then_some(e.assigned)
                    .flatten()
            });
            match binding {
                Some(binding) => self.refuse(&binding, after_loop(php_name)),
                None => self.refuse(variable, unassigned(php_name)),
            }
            return None;
        };
        self.variables.read(var, variable.span);
        if self
            .variables
            .step_guards
            .contains(&self.variables.entries[var].php_name)
        {
            self.refuse(
                variable,
                "a for step cannot read a variable that a let-else in the loop body checks against null (P2R-104)",
            );
            return None;
        }
        if self.variables.entries[var].empty && self.empty_list(variable) {
            // The converter needs the read's type, and no read or write has given it yet.
            self.fail(var, variable, lists::UNKNOWN_ELEMENT.to_owned());
            return None;
        }
        self.settle(var, variable);
        let entry = &self.variables.entries[var];
        let known = entry.ty.clone()?;
        match self.later_type(var, &known, variable, true) {
            Ok(ty) => {
                let name = entry.rust_name.clone();
                let read = match self.variables.value(var) {
                    Some(value) => RExpr::Known(name, value),
                    None => RExpr::Var(name),
                };
                Some((read, ty))
            }
            Err(message) => {
                // Mago types a read that may be unassigned as mixed, so the P2R-34 problem
                // found at the end of the body replaces this one.
                self.variables.entries[var].ty = None;
                self.variables.read_problems.push((variable.span, message));
                None
            }
        }
    }

    /// `expr`, converted from `value`, as a value that `statement` stores: assigns,
    /// appends or puts in a list. `owned` gives the owned form Rust needs, and a string or
    /// list variable read alone moves, with a copy mark for the `.clone()` it needs when it
    /// is read again (P2R-26).
    ///
    /// A value of a changing type is never copied: it moves, or the conversion stops
    /// (P2R-145).
    pub(super) fn stored(
        &mut self,
        value: &Expression<'_>,
        expr: RExpr,
        ty: &RustType,
        statement: Span,
    ) -> Option<RExpr> {
        match (self.kept(value, expr, ty)?, strings::sole_read(value)) {
            (RExpr::Var(name), Some(read)) if !ty.is_copy() => {
                // The mark rides in the variable's text, so it lands right after the
                // variable wherever the value is rendered.
                let strict = self.is_changing(ty);
                let mark = self.variables.copy_mark(read, statement, strict);
                Some(RExpr::Var(format!("{name}{mark}")))
            }
            (expr, _) => Some(expr),
        }
    }

    /// Records a change of the object that the chain `expr` starts from: a local variable
    /// or an owned parameter then needs `mut` (P2R-148). False, with a problem, when the
    /// forms decided for the whole file give that root no way to change, which would be a
    /// fault of the tool (P2R-143). True for an expression that is no chain.
    pub(super) fn change_root(&mut self, node: &impl HasSpan, expr: &RExpr) -> bool {
        let Some(name) = chain_root(expr) else {
            return true;
        };
        let Some(var) = self
            .variables
            .entries
            .iter()
            .rposition(|e| e.rust_name == name && !closed(e))
        else {
            return true;
        };
        let entry = &self.variables.entries[var];
        // The binding of a null check holds a value of a nullable changing type, which
        // P2R-141 already refused where the type was declared.
        if entry.binding.as_ref().is_some_and(|b| b.null_check) && entry.mode == Mode::Shared {
            return false;
        }
        match (entry.parameter.is_some() || entry.borrowed, entry.mode) {
            (true, Mode::Mut) => true,
            (true, Mode::Shared) => {
                self.refuse(
                    node,
                    "the tool decided that nothing changes this object here, so it cannot convert the change; please report this (P2R-143)",
                );
                false
            }
            _ => {
                self.variables.set_changed(var);
                true
            }
        }
    }

    /// `owned`, refused where it would copy a value of a changing type: `$this`, a
    /// borrowed parameter, a property read and a list element never move (P2R-145).
    pub(super) fn kept(
        &mut self,
        node: &impl HasSpan,
        expr: RExpr,
        ty: &RustType,
    ) -> Option<RExpr> {
        let owned = self.owned(expr, ty);
        if self.is_changing(ty) && matches!(owned, RExpr::Method(_, "clone" | "to_vec")) {
            self.refuse(node, NO_COPY);
            return None;
        }
        Some(owned)
    }

    /// The Rust statement for an assignment, `++` or `--`, or `None` when a problem is
    /// recorded.
    fn change(&mut self, expression: &Expression<'_>) -> Option<String> {
        // `None` is an operator that does not convert yet.
        let (target, operator, value) = match expression {
            Expression::Assignment(assignment) => {
                let operator = match assignment.operator {
                    AssignmentOperator::Assign(_) => Some("="),
                    AssignmentOperator::Addition(_) => Some("+="),
                    AssignmentOperator::Subtraction(_) => Some("-="),
                    AssignmentOperator::Multiplication(_) => Some("*="),
                    AssignmentOperator::LeftShift(_) => Some("<<="),
                    AssignmentOperator::RightShift(_) => Some(">>="),
                    AssignmentOperator::Concat(_) => Some(".="),
                    _ => None,
                };
                (assignment.lhs, operator, Some(assignment.rhs))
            }
            Expression::UnaryPostfix(unary) => {
                let operator = match unary.operator {
                    UnaryPostfixOperator::PostIncrement(_) => "+=",
                    UnaryPostfixOperator::PostDecrement(_) => "-=",
                };
                (unary.operand, Some(operator), None)
            }
            Expression::UnaryPrefix(unary) if changes_variable(expression) => {
                let increment = matches!(unary.operator, UnaryPrefixOperator::PreIncrement(_));
                let operator = if increment { "+=" } else { "-=" };
                (unary.operand, Some(operator), None)
            }
            other => {
                self.refuse(other, "this expression statement is not supported (P2R-43)");
                return None;
            }
        };
        if let (Expression::ArrayAppend(append), Some("="), Some(value)) = (target, operator, value)
            && let Expression::Variable(Variable::Direct(list)) = append.array
        {
            return self.append(expression, list, value);
        }
        if let Some(converted) = self.property_change(expression, target, operator, value) {
            return converted;
        }
        // Mago types a map that may be null as a map after the write, and a write by a key
        // of another type as another map, so either type sends it here (P2R-242).
        let map = |ty: &RustType| matches!(ty.inner(), RustType::Map(..));
        if let Expression::ArrayAccess(access) = target
            && (self.site_type(access.array).is_ok_and(|ty| map(&ty))
                || self.chain_type(access.array).is_some_and(|ty| map(&ty)))
        {
            return self.map_write(expression, target, access, operator, value);
        }
        let Expression::Variable(Variable::Direct(variable)) = target else {
            // The value's reads and the target's writes still count, so no false P2R-34 or
            // P2R-35 line follows. The refusal covers the whole assignment, so it is the one
            // line printed for it (P2R-03).
            if let Some(value) = value {
                self.expression(value);
            }
            self.record_refused_target(target);
            let message = match target {
                Expression::ArrayAccess(access)
                    if self.site_type(access.array) == Ok(RustType::String) =>
                {
                    "writing to a string offset is not supported (P2R-165)"
                }
                Expression::ArrayAccess(access)
                    if matches!(self.site_type(access.array), Ok(RustType::Shape(_))) =>
                {
                    "a change to an array shape is not supported (P2R-201)"
                }
                Expression::ArrayAccess(_) | Expression::ArrayAppend(_) => {
                    "writing to a list index is not supported in version 1 (P2R-47)"
                }
                _ => "only a plain variable can be assigned to in version 1 (P2R-07)",
            };
            self.refuse(expression, message);
            return None;
        };
        // PHP rejects a write to `$this` in a method or a function when it compiles the file.
        if variable.name == b"$this" {
            if let Some(value) = value {
                self.expression(value);
            }
            self.refuse(expression, "$this cannot be assigned (P2R-135)");
            return None;
        }
        // `[]` has no element type of its own: it has the variable's list type, which
        // `write` finds now or a later read or write gives (P2R-21).
        if operator == Some("=") && value.is_some_and(lists::is_empty_literal) {
            let (var, declares) = self.write(variable, expression, None)?;
            if !declares {
                self.variables.pending.push(Pending::Empty { var });
                return Some(format!("{MARK}{}", self.variables.pending.len() - 1));
            }
            return Some(self.first_write(var, true, EMPTY_LIST.to_owned(), false));
        }
        // `null` has no type of its own either: it has the variable's `Option` type, which
        // the other reads and writes of its name give (P2R-102).
        if let (Some("="), Some(null)) = (operator, value)
            && nulls::is_null(null)
        {
            let (var, declares) = self.write_null(variable, expression, null)?;
            if !declares {
                let name = &self.variables.entries[var].rust_name;
                return Some(format!("{name} = None;"));
            }
            return Some(self.first_write(var, true, "None".to_owned(), false));
        }
        // PHP evaluates the value first. Every operator but `=` then reads the variable
        // before it writes it.
        let source = value;
        let value = match value {
            Some(value) if operator == Some(".=") => self.joined_string(value),
            // A list literal takes the variable's list of `Option` type, so its elements
            // become `Some(..)` and `None` (P2R-102).
            Some(value) if operator == Some("=") => {
                let known = self
                    .lookup(variable)
                    .and_then(|var| self.variables.entries[var].ty.clone());
                self.value(value, known.as_ref())
            }
            Some(value) => self.expression(value),
            None => Some((RExpr::Int(1), RustType::I64)),
        };
        let current = (operator != Some("=")).then(|| self.variable(variable));
        // `++` and `--` on a float add or take away 1.0 (P2R-43).
        let value = match (source, &current) {
            (None, Some(Some((_, RustType::F64)))) => Some((RExpr::Float(1.0), RustType::F64)),
            _ => value,
        };
        let converted = match (operator, value, current) {
            (Some(operator), Some(value), None) => Some((operator, value, None)),
            (Some(operator), Some(value), Some(Some(current))) => {
                Some((operator, value, Some(current)))
            }
            _ => None,
        };
        let Some((operator, (expr, value_type), current)) = converted else {
            // The write still counts, so later reads see it. Its type is not checked: a
            // problem with it would hide the one found inside this expression (P2R-03).
            self.record_write(variable);
            if operator.is_none() {
                self.refuse(
                    expression,
                    "this assignment operator is not supported in version 1 (P2R-07)",
                );
            }
            return None;
        };
        // The value of `.=` is a joined string; the type of its variable is checked as a
        // write (P2R-34).
        if let (Some(op), Some((_, current_type))) = (operation(operator), &current)
            && let Some(message) = numbers::arithmetic_refusal(op, current_type, &value_type)
                .or_else(|| numbers::shift_count_refusal(op, &expr))
        {
            self.record_write(variable);
            self.refuse(expression, message);
            return None;
        }
        // PHP joins null as an empty string (P2R-106).
        if operator == ".=" && matches!(&current, Some((_, RustType::Option(_)))) {
            self.record_write(variable);
            self.refuse(expression, nulls::NULL_JOINED);
            return None;
        }
        let assigned = (operator == "=").then_some(&value_type);
        let (var, declares) = self.write(variable, expression, assigned)?;
        // The value the variable now holds, when rustc can work it out (P2R-50).
        let value = match (current, operation(operator)) {
            (Some((current, _)), Some(op)) => {
                let known = printer::literal_value(&current).is_some()
                    && printer::literal_value(&expr).is_some();
                let combined = RExpr::Binary(Box::new(current), op, Box::new(expr.clone()));
                let after = printer::literal_value(&combined);
                // rustc 1.99 rejects this only when nothing borrows the variable (`print!`
                // does), so it stops whenever the tool can prove it (P2R-50).
                if known && after.is_none() {
                    self.refuse(expression, "this int arithmetic always overflows (P2R-50)");
                    return None;
                }
                after
            }
            _ => printer::literal_value(&expr),
        };
        // Only an int variable has a value rustc works out.
        let nullable = matches!(self.variables.entries[var].ty, Some(RustType::Option(_)));
        self.variables.set_value(var, value.filter(|_| !nullable));
        let name = &self.variables.entries[var].rust_name;
        let read_itself = matches!(&expr, RExpr::Var(read) | RExpr::Known(read, _) if read == name);
        if operator == "=" && read_itself {
            self.refuse(
                expression,
                "an assignment of a variable to itself does nothing, and rustc rejects it (P2R-50)",
            );
            return None;
        }
        let (operator, expr) = if operator == ".=" {
            // `s += &s` borrows `s` twice, which rustc rejects (E0502).
            let expr = if read_itself {
                RExpr::Method(Box::new(expr), "clone")
            } else {
                expr
            };
            ("+=", self.borrowed(expr, &value_type))
        } else if let Some(source) = source {
            // A value that cannot be null becomes `Some(..)` in an `Option` variable
            // (P2R-102).
            let stored = self.stored(source, expr, &value_type, expression.span())?;
            let expr = match &self.variables.entries[var].ty {
                Some(target) => nulls::wrapped(stored, &value_type, target),
                None => stored,
            };
            (operator, expr)
        } else {
            (operator, expr)
        };
        let rendered = printer::expr(&expr, false);
        let name = &self.variables.entries[var].rust_name;
        if !declares {
            return Some(format!("{name} {operator} {rendered};"));
        }
        // A literal alone would make Rust infer i32 (P2R-49).
        Some(self.first_write(var, !printer::anchored(&expr), rendered, false))
    }

    /// The pending line of the first write of `var`, whose `let` names the type when
    /// `typed`. The type is the one the variable has at the end of the body, which a later
    /// read or write gives a variable that holds `[]` (P2R-21).
    pub(super) fn first_write(
        &mut self,
        var: Var,
        typed: bool,
        value: String,
        open: bool,
    ) -> String {
        self.variables.pending.push(Pending::First {
            var,
            typed,
            value,
            open,
        });
        format!("{MARK}{}", self.variables.pending.len() - 1)
    }

    /// `$a[] = x`, a change of the list in `$a`: PHP evaluates the value, then reads the list
    /// and writes it back (P2R-47).
    fn append(
        &mut self,
        expression: &Expression<'_>,
        list: &DirectVariable<'_>,
        value: &Expression<'_>,
    ) -> Option<String> {
        let source = value;
        let known = self
            .lookup(list)
            .and_then(|var| self.variables.entries[var].ty.clone());
        if let Some(RustType::Vec(element)) = &known
            && matches!(**element, RustType::Option(_))
        {
            return self.append_option(expression, list, source, (**element).clone());
        }
        // Mago types the map after the append as a list or a map, which has no Rust type.
        if matches!(known, Some(RustType::Map(..))) {
            self.record_refused(value);
            if let Some(var) = self.lookup(list) {
                self.variables.read(var, list.span);
            }
            let (var, _) = self.record_write(list);
            self.fail(var, expression, MAP_APPEND.to_owned());
            return None;
        }
        // A list is a list of `Option` only when its first write holds null (P2R-102).
        if matches!(known, Some(RustType::Vec(_))) && nulls::is_null(value) {
            if let Some(var) = self.lookup(list) {
                self.variables.read(var, list.span);
            }
            // Later reads of the list would report the same cause as a type change.
            let (var, _) = self.record_write(list);
            self.fail(
                var,
                expression,
                "null cannot be appended to a list of values that cannot be null (P2R-102)"
                    .to_owned(),
            );
            return None;
        }
        let value = self.expression(value);
        let current = self.variable(list);
        let (Some((value, value_type)), Some((_, current_type))) = (value, current) else {
            // The write still counts, so later reads see it.
            self.record_write(list);
            return None;
        };
        let RustType::Vec(element) = &current_type else {
            self.record_write(list);
            let message = match current_type {
                RustType::Option(_) => "a value that may be null cannot be appended to (P2R-106)",
                RustType::Map(..) => MAP_APPEND,
                _ => "only a list can be appended to (P2R-47)",
            };
            self.refuse(expression, message);
            return None;
        };
        // Mago may type the value as `T` where it knows more (P2R-104).
        if nulls::null_for_plain(&value_type, element) {
            self.record_write(list);
            self.refuse(expression, nulls::NULL_ELEMENT);
            return None;
        }
        // Mago types the `$a` of `$a[]` as the list after the append, so a value of another
        // type shows as a type change.
        let (var, _) = self.write(list, list, None)?;
        let value = self.stored(source, value, &value_type, expression.span())?;
        let name = &self.variables.entries[var].rust_name;
        let push = RExpr::Call(format!("{name}.push"), vec![value]);
        Some(format!("{};", printer::expr(&push, false)))
    }

    /// `$m[k] = v;` on a map in a variable or parameter: `m.insert(k, v);`, which keeps the
    /// position of a key the map has and adds a new one at the end, as PHP does (P2R-242).
    fn map_write(
        &mut self,
        expression: &Expression<'_>,
        target: &Expression<'_>,
        access: &ArrayAccess<'_>,
        operator: Option<&str>,
        value: Option<&Expression<'_>>,
    ) -> Option<String> {
        let holder = unparenthesized(access.array);
        let problem = match holder {
            _ if operator != Some("=") => {
                "a compound assignment, ++, -- or ??= on a map element is not supported (P2R-242)"
            }
            Expression::Variable(Variable::Direct(_)) => "",
            Expression::Access(mago_syntax::cst::Access::Property(_)) => MAP_PROPERTY_CHANGE,
            _ => "a map element can change only on a map in a variable or parameter (P2R-242)",
        };
        let (Some(value), "", Expression::Variable(Variable::Direct(variable))) =
            (value, problem, holder)
        else {
            // The reads and writes still count, so no false P2R-34 or P2R-35 line follows.
            if let Some(value) = value {
                self.record_refused(value);
            }
            self.record_refused_target(target);
            self.refuse(expression, problem);
            return None;
        };
        // A key or value of another type gives the map another type in Mago, so they convert
        // before the map is read, and their problem is the one reported (P2R-241).
        let known = self
            .lookup(variable)
            .and_then(|var| self.variables.entries[var].ty.clone());
        // A map that may be null is refused before Mago's type of the write is read.
        let read_first = !matches!(known, Some(RustType::Map(..) | RustType::Option(_)));
        let current = match known {
            Some(ty @ (RustType::Map(..) | RustType::Option(_))) => Some(ty),
            _ => self.variable(variable).map(|(_, ty)| ty),
        };
        let (key_type, value_type) = match current {
            Some(RustType::Map(key, value)) => (key, value),
            other => {
                if !read_first {
                    self.record_refused(access.array);
                }
                self.record_refused(access.index);
                self.record_refused(value);
                self.record_write(variable);
                match other {
                    Some(RustType::Option(_)) => self.refuse(expression, MAP_MAY_BE_NULL),
                    Some(_) => self.refuse(
                        expression,
                        "writing to a list index is not supported in version 1 (P2R-47)",
                    ),
                    None => {}
                }
                return None;
            }
        };
        let statement = expression.span();
        let key = self.map_key(access.index, &key_type, Some(statement));
        let stored = self.map_value(value, &value_type, statement);
        let (Some(key), Some(stored)) = (key, stored) else {
            if !read_first {
                self.record_refused(access.array);
            }
            // Mago gives the map another type after this write, so its later reads would
            // report the same cause (P2R-03).
            let (var, _) = self.record_write(variable);
            self.variables.entries[var].ty = None;
            return None;
        };
        if !read_first && self.variable(variable).is_none() {
            self.record_write(variable);
            return None;
        }
        let (var, _) = self.write(variable, variable, None)?;
        let map = RExpr::Var(self.variables.entries[var].rust_name.clone());
        let insert = RExpr::MethodCall(Box::new(map), "insert".to_owned(), vec![key, stored]);
        Some(format!("{};", printer::expr(&insert, false)))
    }

    /// `unset($m[k]);` on a map in a variable or parameter: `m.shift_remove(&k);`, which
    /// keeps the order of the other keys, as PHP does (P2R-242).
    pub(super) fn unset_statement(
        &mut self,
        unset: &Unset<'_>,
        depth: usize,
        body: &mut Vec<String>,
    ) {
        for value in unset.values.iter() {
            if let Some(text) = self.unset_element(value) {
                body.push(emit(depth, text));
            }
        }
    }

    fn unset_element(&mut self, value: &Expression<'_>) -> Option<String> {
        let problem = match unparenthesized(value) {
            Expression::ArrayAccess(access) => match unparenthesized(access.array) {
                Expression::Variable(Variable::Direct(variable)) => {
                    return self.map_remove(value, access, variable);
                }
                Expression::Access(mago_syntax::cst::Access::Property(_))
                    if self
                        .site_type(access.array)
                        .is_ok_and(|ty| matches!(ty.inner(), RustType::Map(..))) =>
                {
                    MAP_PROPERTY_CHANGE
                }
                _ => UNSET,
            },
            _ => UNSET,
        };
        self.record_refused(value);
        self.refuse(value, problem);
        None
    }

    fn map_remove(
        &mut self,
        value: &Expression<'_>,
        access: &ArrayAccess<'_>,
        variable: &DirectVariable<'_>,
    ) -> Option<String> {
        let current = self.variable(variable);
        let key_type = match current.map(|(_, ty)| ty) {
            Some(RustType::Map(key, _)) => key,
            other => {
                self.record_refused(access.index);
                self.record_write(variable);
                match other {
                    Some(RustType::Option(_)) => self.refuse(value, MAP_MAY_BE_NULL),
                    Some(_) => self.refuse(value, UNSET),
                    None => {}
                }
                return None;
            }
        };
        let Some(key) = self.map_key(access.index, &key_type, None) else {
            self.record_write(variable);
            return None;
        };
        let (var, _) = self.write(variable, variable, None)?;
        let map = RExpr::Var(self.variables.entries[var].rust_name.clone());
        let remove = RExpr::MethodCall(Box::new(map), "shift_remove".to_owned(), vec![key]);
        Some(format!("{};", printer::expr(&remove, false)))
    }

    /// `$a[] = x` on a list of `Option`, whose elements are `element`: `null` is `None` and
    /// any other value `Some(..)` (P2R-102). Mago types the `$a` of such an append as a
    /// union of list shapes that has no one Rust type, so the element type alone decides.
    fn append_option(
        &mut self,
        expression: &Expression<'_>,
        list: &DirectVariable<'_>,
        value: &Expression<'_>,
        element: RustType,
    ) -> Option<String> {
        let converted = self.value(value, Some(&element));
        let var = self.lookup(list)?;
        self.variables.read(var, list.span);
        let Some((expr, value_type)) = converted else {
            self.record_write(list);
            return None;
        };
        if value_type != element && value_type != *element.inner() {
            self.record_write(list);
            self.refuse(
                expression,
                format!(
                    "a {} cannot be appended to a list of {} (P2R-47)",
                    value_type.owned(),
                    element.owned()
                ),
            );
            return None;
        }
        let (var, _) = self.record_write(list);
        if self.refuse_borrowed_parameter(var, list) {
            return None;
        }
        let stored = self.stored(value, expr, &value_type, expression.span())?;
        let pushed = nulls::wrapped(stored, &value_type, &element);
        let name = &self.variables.entries[var].rust_name;
        let push = RExpr::Call(format!("{name}.push"), vec![pushed]);
        Some(format!("{};", printer::expr(&push, false)))
    }

    /// Records a write of `variable` by `node` and checks the type Mago gives the write.
    /// `assigned` is the type of the value an `=` stores. Gives the variable and whether
    /// this write declares it.
    pub(super) fn write(
        &mut self,
        variable: &DirectVariable<'_>,
        node: &impl HasSpan,
        assigned: Option<&RustType>,
    ) -> Option<(Var, bool)> {
        let empty = self.empty_list(node);
        let found = match (self.site_type(node), assigned) {
            // A union value gives a new variable its union type where Mago types the write
            // as one or more of its members (P2R-232).
            (_, Some(value))
                if matches!(value.inner(), RustType::Union(name)
                if self.expression_type(node).is_some_and(|site| {
                    self.within_union(site, name, matches!(value, RustType::Option(_)))
                })) =>
            {
                Ok(value.clone())
            }
            (found, _) => found,
        };
        let (var, declared) = self.record_write(variable);
        if self.refuse_borrowed_parameter(var, node) {
            return None;
        }
        let checked = match declared {
            // The name problem is recorded.
            Some(false) => return None,
            // `[]` has no element type: the other reads and writes of the variable give it,
            // or a later read or write does (P2R-21).
            Some(true) if empty => {
                let name = &self.variables.entries[var].php_name;
                match self.variables.list_types.get(name) {
                    Some(Some(ty)) => Ok(ty.clone()),
                    _ => {
                        self.variables.entries[var].empty = true;
                        return Some((var, true));
                    }
                }
            }
            // A value that cannot be null starts a variable that may hold null later as an
            // `Option` (P2R-102).
            Some(true) => found.map(|found| {
                let name = &self.variables.entries[var].php_name;
                match (self.variables.null_types.get(name), assigned) {
                    (Some(Some(nullable)), _) if *nullable.inner() == found => nullable.clone(),
                    // Mago types the write as `T` where it knows the value is not null, but
                    // the value is an `Option` in Rust, so the variable is one too.
                    (_, Some(value @ RustType::Option(inner))) if **inner == found => value.clone(),
                    _ => found,
                }
            }),
            None => {
                self.settle(var, node);
                if self.variables.entries[var].empty {
                    return Some((var, false));
                }
                let known = self.variables.entries[var].ty.clone()?;
                match assigned {
                    // A value of the variable's union type keeps it (P2R-232).
                    Some(value)
                        if *value == known && matches!(known.inner(), RustType::Union(_)) =>
                    {
                        Ok(known)
                    }
                    _ => self.later_type(var, &known, node, false),
                }
            }
        };
        // The value must have the variable's type, or be the `T` of an `Option<T>`: Mago's
        // type of the write can say less than the value's Rust type (P2R-34, P2R-102).
        let checked = checked.and_then(|ty| match assigned {
            Some(value) if *value != ty && value != ty.inner() => {
                same_type(&self.variables.entries[var].php_name, &ty, value.clone())
            }
            _ => Ok(ty),
        });
        match checked.and_then(built_yet) {
            Ok(ty) if self.nullable_changing(&ty) => {
                self.fail(var, node, classes::NULLABLE_CHANGING.to_owned());
                None
            }
            Ok(ty) => {
                self.variables.entries[var].ty = Some(ty);
                Some((var, declared.is_some()))
            }
            Err(message) => {
                self.fail(var, node, message);
                None
            }
        }
    }

    /// Records `$x = null` at `node`. Gives the variable and whether this write declares
    /// it. The `Option` type comes from the other reads and writes of the name, as a `[]`
    /// takes its list type, or from the variable's type (P2R-102).
    fn write_null(
        &mut self,
        variable: &DirectVariable<'_>,
        node: &impl HasSpan,
        null: &Expression<'_>,
    ) -> Option<(Var, bool)> {
        let (var, declared) = self.record_write(variable);
        if self.refuse_borrowed_parameter(var, node) {
            return None;
        }
        let entry = &self.variables.entries[var];
        let ty = match (declared, &entry.ty) {
            // The name problem is recorded.
            (Some(false), _) => return None,
            (Some(true), _) => match self.variables.null_types.get(&entry.php_name) {
                Some(Some(ty)) => Some(ty.clone()),
                Some(None) => {
                    let message = format!(
                        "the type of this null is not known, because the reads and writes of ${} give different types (P2R-102)",
                        String::from_utf8_lossy(&entry.php_name)
                    );
                    self.fail(var, null, message);
                    return None;
                }
                None => None,
            },
            (None, Some(known @ RustType::Option(_))) => Some(known.clone()),
            (None, Some(known)) => {
                let message = format!(
                    "the variable ${} changes type from {} to null (P2R-34)",
                    String::from_utf8_lossy(&entry.php_name),
                    known.owned()
                );
                self.fail(var, null, message);
                return None;
            }
            // A problem with the variable's type is recorded.
            (None, None) => return None,
        };
        let Some(ty) = ty else {
            self.fail(
                var,
                null,
                "the type null has no Rust type (P2R-22)".to_owned(),
            );
            return None;
        };
        if self.nullable_changing(&ty) {
            self.fail(var, null, classes::NULLABLE_CHANGING.to_owned());
            return None;
        }
        self.variables.entries[var].ty = Some(ty);
        Some((var, declared.is_some()))
    }

    /// Refuses a change of `var` at `node` when it is a string, list or object parameter, or
    /// `$this`: PHP passes the function a copy, but Rust borrows it (P2R-24, P2R-132,
    /// P2R-135).
    fn refuse_borrowed_parameter(&mut self, var: Var, node: &impl HasSpan) -> bool {
        let entry = &self.variables.entries[var];
        if entry.borrowed && entry.ty.as_ref().is_some_and(|ty| self.is_changing(ty)) {
            self.refuse(
                node,
                "an assignment to a foreach value of a class that changes its objects is not supported, because PHP then leaves the list unchanged (P2R-142)",
            );
            return true;
        }
        // A copied parameter owns its value (P2R-181).
        let borrowed = entry.parameter.is_some()
            && !entry.copied
            && entry
                .ty
                .as_ref()
                .is_some_and(|ty| ty.parameter() != ty.owned());
        if borrowed {
            let message = if entry.php_name == THIS {
                "$this cannot be assigned (P2R-135)"
            } else if matches!(
                entry.ty.as_ref().map(RustType::inner),
                Some(RustType::Class(_))
            ) {
                "an object parameter cannot be assigned, because Rust borrows it (P2R-132)"
            } else if matches!(
                entry.ty.as_ref().map(RustType::inner),
                Some(RustType::Shape(_))
            ) {
                "an array shape parameter cannot be assigned, because Rust borrows it (P2R-201)"
            } else if matches!(entry.ty.as_ref().map(RustType::inner), Some(RustType::Vec(element)) if self.is_changing(element))
            {
                // Its copy would copy objects that never copy (P2R-145).
                "a list parameter of objects whose class changes them cannot be changed, because its copy would copy them (P2R-143)"
            } else {
                "a string or list parameter cannot be changed, because PHP passes a copy (P2R-24)"
            };
            self.refuse(node, message);
        }
        borrowed
    }

    /// Records a write of `variable`, and declares the variable at its first write. The
    /// flag is `Some` for that first write, and `Some(false)` when the name does not
    /// convert. The write is placed at the variable, not the whole assignment, so a
    /// problem with it does not hide one in the value (P2R-03).
    pub(super) fn record_write(&mut self, variable: &DirectVariable<'_>) -> (Var, Option<bool>) {
        let (var, declared) = match self.lookup(variable) {
            Some(var) => (var, None),
            None => {
                let (var, named) = self.declare(variable);
                (var, Some(named))
            }
        };
        if self
            .variables
            .step_guards
            .contains(&self.variables.entries[var].php_name)
        {
            self.fail(
                var,
                variable,
                "a for step cannot write a variable that a let-else in the loop body checks against null (P2R-104)".to_owned(),
            );
        } else if self.checked(var) {
            self.fail(var, variable, CHECKED.to_owned());
        }
        if let Some(binding) = &mut self.variables.entries[var].binding {
            binding.rewritten = true;
        }
        if self.variables.iterated.contains(&var) {
            self.refuse(
                variable,
                "the body of a foreach cannot change the list it iterates (P2R-41)",
            );
        }
        self.variables.write(var, Some(variable.span));
        self.variables.entries[var]
            .assigned
            .get_or_insert(variable.span);
        // A write whose value is not worked out leaves the value unknown.
        self.variables.set_value(var, None);
        (var, declared)
    }

    /// Records the reads and writes inside an expression that is refused before it
    /// converts, so no false P2R-34 or P2R-35 line follows the refusal (P2R-03). Call it
    /// only on a part that was not converted: a write recorded twice makes the first one
    /// look unread.
    pub(super) fn record_refused(&mut self, expression: &Expression<'_>) {
        let mut accesses = Vec::new();
        Accesses.walk_expression(expression, &mut accesses);
        self.record_accesses(accesses);
    }

    /// The same for the target of a refused assignment, such as `[$a, $b]` or `$a[]`.
    pub(super) fn record_refused_target(&mut self, target: &Expression<'_>) {
        let mut accesses = Vec::new();
        Accesses.target(target, &mut accesses);
        self.record_accesses(accesses);
    }

    fn record_accesses(&mut self, accesses: Vec<Access<'_, '_>>) {
        for access in accesses {
            match access {
                // A read of a variable with no write yet is not reported: the refusal is.
                Access::Read(variable) => {
                    if let Some(var) = self.lookup(variable) {
                        self.variables.read(var, variable.span);
                    }
                }
                Access::Write(variable) => {
                    self.record_write(variable);
                }
            }
        }
    }

    /// Declares a foreach key or value as a new variable of type `ty`, written at the start
    /// of the loop body (P2R-34, P2R-41). `ty` is `None` when a problem with the list is
    /// recorded.
    pub(super) fn bind(&mut self, variable: &DirectVariable<'_>, ty: Option<RustType>) -> Var {
        if let Some(checked) = self.lookup(variable)
            && self.checked(checked)
        {
            self.refuse(variable, CHECKED);
        }
        // A key or value named like a parameter assigns the parameter in PHP: a string or
        // list parameter cannot change (P2R-24), and the parameter's value must be read
        // first, which the end of the body checks from its first assignment (P2R-35).
        if let Some(parameter) = self.lookup(variable)
            && self.variables.entries[parameter].parameter.is_some()
            && !self.refuse_borrowed_parameter(parameter, variable)
        {
            self.variables.entries[parameter]
                .assigned
                .get_or_insert(variable.span);
        }
        let (var, named) = self.declare(variable);
        let ty = match ty.map(built_yet) {
            Some(Ok(ty)) => Some(ty),
            Some(Err(message)) => {
                self.refuse(variable, message);
                None
            }
            None => None,
        };
        let entry = &mut self.variables.entries[var];
        entry.binding = Some(Binding {
            snake: String::from_utf8_lossy(&entry.php_name).to_snake_case(),
            rewritten: false,
            closed: false,
            null_check: false,
        });
        entry.assigned = Some(variable.span);
        entry.ty = ty.filter(|_| named);
        self.variables.write(var, Some(variable.span));
        var
    }

    /// Adds a variable at its first write. The flag is `false` when the name has no Rust
    /// form, or gives the Rust name of a function or of a variable with another PHP name
    /// (P2R-30, P2R-31). An earlier variable with the same PHP name is a foreach key or
    /// value, or one that a foreach key or value shadows (P2R-41).
    fn declare(&mut self, variable: &DirectVariable<'_>) -> (Var, bool) {
        let php_name = &variable.name[1..];
        let rust_name = self.rust_name(php_name, variable);
        let named = match &rust_name {
            None => false,
            Some(name)
                if self.functions.values().any(|f| f.rust_name == *name)
                    || self
                        .variables
                        .entries
                        .iter()
                        .any(|e| e.rust_name == *name && e.php_name != php_name) =>
            {
                self.refuse(
                    variable,
                    format!("{name} is already the Rust name of a function or variable (P2R-31)"),
                );
                false
            }
            Some(name) if helpers::is_helper(name) => {
                self.refuse(variable, helpers::HELPER_NAME);
                false
            }
            Some(_) => true,
        };
        self.variables.entries.push(Entry {
            php_name: php_name.to_vec(),
            rust_name: rust_name.unwrap_or_default(),
            ty: None,
            parameter: None,
            assigned: None,
            value: None,
            first: self.variables.scope,
            common: self.variables.scope,
            empty: false,
            borrowed: false,
            binding: None,
            mode: Mode::Shared,
            changed: false,
            copied: false,
            templated: false,
        });
        (self.variables.entries.len() - 1, named)
    }

    /// Declares the binding of a null check on `variable` (P2R-104): a new variable of the
    /// inner type `inner`, written at the check, which `lookup` finds until the block it
    /// holds for ends. A string or list binding borrows, as a parameter does.
    pub(super) fn bind_null(&mut self, variable: &DirectVariable<'_>, inner: RustType) -> Var {
        let (var, named) = self.declare(variable);
        let entry = &mut self.variables.entries[var];
        entry.borrowed = !inner.is_copy();
        entry.ty = named.then_some(inner);
        entry.assigned = Some(variable.span);
        entry.binding = Some(Binding {
            snake: String::from_utf8_lossy(&entry.php_name).to_snake_case(),
            rewritten: false,
            closed: false,
            null_check: true,
        });
        self.variables.write(var, Some(variable.span));
        var
    }

    /// True when `var` is the binding of a null check that is still open (P2R-104).
    fn checked(&self, var: Var) -> bool {
        self.variables.entries[var]
            .binding
            .as_ref()
            .is_some_and(|b| b.null_check && !b.closed)
    }

    /// The variable a PHP name refers to, or `None` before its first write. A foreach key
    /// or value refers to the name until its loop ends (P2R-41).
    pub(super) fn lookup(&self, variable: &DirectVariable<'_>) -> Option<Var> {
        let php_name = &variable.name[1..];
        self.variables
            .entries
            .iter()
            .rposition(|e| e.php_name == php_name && !closed(e))
    }

    /// The Rust type of the value Mago found at `node` (P2R-21, P2R-22).
    pub(super) fn site_type(&self, node: &impl HasSpan) -> Result<RustType, String> {
        let union = self
            .expression_type(node)
            .ok_or_else(|| "Mago found no type here (P2R-34)".to_owned())?;
        types::rust_type(&lists::without_empty(union), &self.type_names)
    }

    /// True when Mago types `node` as `[]` only, which has no element type (P2R-21).
    pub(super) fn empty_list(&self, node: &impl HasSpan) -> bool {
        self.expression_type(node).is_some_and(lists::only_empty)
    }

    /// The type of a later read, or a write when `read` is false, of `var` at `node`,
    /// refused when it differs from the variable's type `known` (P2R-34). `[]` fits a list
    /// of any element type (P2R-21).
    fn later_type(
        &self,
        var: Var,
        known: &RustType,
        node: &impl HasSpan,
        read: bool,
    ) -> Result<RustType, String> {
        if matches!(known, RustType::Vec(_)) && self.empty_list(node) {
            return Ok(known.clone());
        }
        // Mago records no type for a variable read again after a property read, as in
        // `$e->name . $e->name` or `$xs[$i]->n + $xs[$i]->n` (the `$i` too), nor for `$this`
        // as the object of a property read; every write keeps the one type (P2R-34,
        // P2R-134).
        if self.expression_type(node).is_none() {
            return Ok(known.clone());
        }
        // A read or write of an `Option` variable has its type, also where Mago knows that
        // the value is null there, or that it is not (P2R-102, P2R-104).
        if let RustType::Option(inner) = known
            && (self.expression_type(node).is_some_and(types::only_null)
                || self.site_type(node).is_ok_and(|found| found == **inner))
        {
            return Ok(known.clone());
        }
        // A list of `Option` after an append: every element fits (P2R-102).
        if let RustType::Vec(element) = known
            && matches!(**element, RustType::Option(_))
            && let Some(elements) = self.expression_type(node).and_then(types::list_elements)
            && (types::only_null(&elements)
                || types::rust_type(&elements, &self.type_names)
                    .is_ok_and(|found| found == **element || found == *element.inner()))
        {
            return Ok(known.clone());
        }
        let entry = &self.variables.entries[var];
        let found = self.site_type(node);
        // A read of a union value keeps its type where Mago narrows it to some of its
        // members, one included (P2R-232); a write such as `.=` keeps P2R-34.
        if let RustType::Union(name) = known.inner()
            && read
            && self.expression_type(node).is_some_and(|site| {
                self.within_union(site, name, matches!(known, RustType::Option(_)))
            })
        {
            return Ok(known.clone());
        }
        // A parameter that the `ArrayAccess` template types is `mixed` to Mago, or
        // `scalar` after `is_scalar()`; a read has the parameter's type (P2R-220).
        if found.is_err()
            && entry.templated
            && !entry.copied
            && self
                .expression_type(node)
                .is_some_and(types::mixed_or_scalar)
        {
            return Ok(known.clone());
        }
        found.and_then(|found| same_type(&entry.php_name, known, found))
    }

    /// Gives a variable that holds `[]` the list type Mago finds at `node`, a later read or
    /// write of it (P2R-21). Nothing changes while that type is still `[]`.
    fn settle(&mut self, var: Var, node: &impl HasSpan) {
        if !self.variables.entries[var].empty || self.empty_list(node) {
            return;
        }
        match self.site_type(node).and_then(built_yet) {
            Ok(ty) => {
                let entry = &mut self.variables.entries[var];
                entry.ty = Some(ty);
                entry.empty = false;
            }
            Err(message) => self.fail(var, node, message),
        }
    }

    /// Records a problem with a variable's type. Later uses of the variable convert to
    /// nothing, so one problem is reported per variable.
    fn fail(&mut self, var: Var, node: &impl HasSpan, message: String) {
        self.variables.entries[var].ty = None;
        self.variables.entries[var].empty = false;
        self.refuse(node, message);
    }

    /// Ends the body: refuses reads that may be unassigned and values that are never
    /// read, writes each `mut`, and gives a function's parameter list, with a leading `_`
    /// on an unused parameter so rustc does not warn (P2R-34, P2R-35).
    pub(super) fn finish_variables(&mut self, body: &mut Vec<String>) -> Vec<String> {
        let Variables {
            entries,
            flow,
            copies,
            field_writes,
            scopes,
            pending,
            read_problems,
            exits,
            ..
        } = std::mem::take(&mut self.variables);
        let facts = flow.finish(entries.len());
        for (span, message) in read_problems {
            if !facts.unassigned.iter().any(|&(_, read)| read == span) {
                self.refuse(&span, message);
            }
        }
        for (var, span) in facts.unassigned {
            self.refuse(&span, unassigned(&entries[var].php_name));
        }
        for (read, write) in field_writes {
            if facts.dead_fields.contains(&read) {
                self.refuse(
                    &write,
                    "the value written to this property is never read, which rustc rejects (P2R-142)",
                );
            }
        }
        // A read after the loop that reaches an earlier variable of the same name, through
        // an outer loop or past an empty list, would see the last key or value in PHP.
        for (exit, binding) in exits {
            let entry = &entries[binding];
            let live = entries.iter().enumerate().any(|(var, other)| {
                var != binding && other.php_name == entry.php_name && facts.live[exit][var]
            });
            if live && let Some(span) = entry.assigned {
                self.refuse(&span, after_loop(&entry.php_name));
            }
        }
        let mut unused = vec![false; entries.len()];
        for (var, span) in facts.unread {
            let entry = &entries[var];
            let name = String::from_utf8_lossy(&entry.php_name);
            match (span, entry.assigned) {
                // A foreach key or value that is never read gets a `_` (P2R-35).
                (Some(span), Some(first))
                    if span == first && entry.binding.as_ref().is_some_and(|b| !b.rewritten) =>
                {
                    unused[var] = true
                }
                (Some(span), _) => self.refuse(
                    &span,
                    format!("the value assigned to ${name} is never read (P2R-35)"),
                ),
                (None, Some(first)) => self.refuse(
                    &first,
                    format!("the parameter ${name} is assigned before it is read (P2R-35)"),
                ),
                (None, None) => unused[var] = true,
            }
        }
        // A foreach key or value is declared by its `for` pattern.
        let slots: Vec<Option<usize>> = entries
            .iter()
            .map(|entry| {
                slot(&scopes, entry.first, entry.common).filter(|_| entry.binding.is_none())
            })
            .collect();
        let let_keyword = |var: Var| {
            if facts.reassigned[var] || entries[var].changed {
                "let mut "
            } else {
                "let "
            }
        };
        let mut lines = Vec::with_capacity(body.len());
        for line in body.drain(..) {
            let mut finished: Vec<String> = Vec::new();
            let marked = line
                .find(MARK)
                .and_then(|at| Some((at, line[at + 1..].parse::<usize>().ok()?)));
            match marked.and_then(|(at, index)| Some((at, index, pending.get(index)?))) {
                Some((at, index, Pending::Slot)) => {
                    let indent = &line[..at];
                    for (var, entry) in entries.iter().enumerate() {
                        if let (Some(ty), None) = (&entry.ty, &entry.parameter)
                            && slots[var] == Some(index)
                        {
                            let name = &entry.rust_name;
                            let text =
                                format!("{indent}{}{name}: {};", let_keyword(var), ty.owned());
                            finished.push(text);
                        }
                    }
                }
                Some((
                    at,
                    _,
                    Pending::First {
                        var,
                        typed,
                        value,
                        open,
                    },
                )) => {
                    let indent = &line[..at];
                    let entry = &entries[*var];
                    let name = &entry.rust_name;
                    // A variable whose type never came is refused where it is read
                    // (P2R-21), or it is never read (P2R-35).
                    let annotation = match (typed, &entry.ty) {
                        (true, Some(ty)) => format!(": {}", ty.owned()),
                        _ => String::new(),
                    };
                    let end = if *open { "" } else { ";" };
                    let value = first_value(value, entry.ty.as_ref());
                    let text = match slots[*var] {
                        Some(_) => format!("{indent}{name} = {value}{end}"),
                        None => format!(
                            "{indent}{}{name}{annotation} = {value}{end}",
                            let_keyword(*var)
                        ),
                    };
                    finished.push(text);
                }
                Some((at, _, Pending::Empty { var })) => {
                    let entry = &entries[*var];
                    let value = first_value(EMPTY_LIST, entry.ty.as_ref());
                    finished.push(format!("{}{} = {value};", &line[..at], entry.rust_name));
                }
                Some((
                    at,
                    _,
                    Pending::Foreach {
                        key,
                        value,
                        by_ref,
                        rest,
                    },
                )) => {
                    let pattern = for_pattern(&entries, &unused, *key, *value, *by_ref);
                    let text = format!("{}for {pattern}{rest}", &line[..at]);
                    finished.push(text);
                }
                Some((
                    at,
                    _,
                    Pending::Condition {
                        keyword,
                        parts,
                        open,
                    },
                )) => {
                    finished.push(condition_line(&line[..at], keyword, parts, open, &unused));
                }
                Some((
                    at,
                    _,
                    Pending::Guard {
                        binding,
                        pattern,
                        test,
                    },
                )) => {
                    let indent = &line[..at];
                    let text = if let_else(*binding, &unused) {
                        format!("{indent}{pattern} else {{")
                    } else {
                        format!("{indent}if {test} {{")
                    };
                    finished.push(text);
                }
                Some((at, _, Pending::GuardEnd { guard })) => {
                    let after_let = matches!(pending.get(*guard),
                        Some(Pending::Guard { binding, .. }) if let_else(*binding, &unused));
                    let end = if after_let { "};" } else { "}" };
                    finished.push(format!("{}{end}", &line[..at]));
                }
                None => finished.push(line),
            }
            for mut text in finished {
                let mut copied = Vec::new();
                resolve_copy(&mut text, &copies, &facts.read_again, &mut copied);
                for span in copied {
                    self.refuse(&span, NO_COPY);
                }
                lines.push(text);
            }
        }
        *body = lines;
        entries
            .iter()
            .enumerate()
            .filter_map(|(var, entry)| {
                let snake = entry.parameter.as_ref()?;
                // A method takes `&self`, read or not, or `&mut self` when it changes its
                // object (P2R-135, P2R-143).
                if entry.php_name == THIS {
                    // An enum value is Copy, so its methods take `self` (P2R-155).
                    let this = match (&entry.ty, entry.mode) {
                        (Some(RustType::Enum(_)), _) => "self",
                        (_, Mode::Mut) => "&mut self",
                        _ => "&self",
                    };
                    return Some(this.to_owned());
                }
                // An owned parameter that a change goes through needs `mut`; a `&mut`
                // parameter is a reference, which needs none (P2R-148).
                let changed = entry.changed && entry.mode == Mode::Owned;
                // A copied parameter's `mut` is on its copy line (P2R-181).
                let name = if unused[var] {
                    format!("_{snake}")
                } else if (facts.reassigned[var] || changed) && !entry.copied {
                    format!("mut {}", entry.rust_name)
                } else {
                    entry.rust_name.clone()
                };
                Some(format!(
                    "{name}: {}",
                    entry.mode.parameter(entry.ty.as_ref()?)
                ))
            })
            .collect()
    }
}

/// The blocks from `block` out to the body, `block` first.
fn chain(scopes: &[Scope], mut block: usize) -> Vec<usize> {
    let mut chain = vec![block];
    while let Some((parent, _)) = scopes[block].parent {
        chain.push(parent);
        block = parent;
    }
    chain
}

/// The innermost block that holds both `a` and `b`.
fn common(scopes: &[Scope], a: usize, b: usize) -> usize {
    let outer = chain(scopes, a);
    chain(scopes, b)
        .into_iter()
        .find(|block| outer.contains(block))
        .unwrap_or(0)
}

/// The slot that holds the typed `let` of a variable first written in block `first` and
/// used in blocks under `common`, or `None` when its first write declares it. The `let`
/// goes in `common`, or above the outermost loop that holds the first write when that
/// is further out (see the module doc).
fn slot(scopes: &[Scope], first: usize, common: usize) -> Option<usize> {
    let chain = chain(scopes, first);
    let mut home = chain.iter().position(|&block| block == common)?;
    if let Some(looped) = chain.iter().rposition(|&block| scopes[block].looped) {
        home = home.max(looped + 1);
    }
    let inner = chain[home.checked_sub(1)?];
    scopes[inner].parent.map(|(_, slot)| slot)
}

/// Collects the variables that an assignment, `++` or `--` writes, for the loop pre-scan
/// of the P2R-50 values. Reads are left out: forgetting their values would make the tool
/// weaker than rustc.
struct Written;

impl<'ast, 'arena> Walker<'ast, 'arena, Vec<&'ast DirectVariable<'arena>>> for Written {
    fn walk_in_assignment(
        &self,
        assignment: &'ast Assignment<'arena>,
        written: &mut Vec<&'ast DirectVariable<'arena>>,
    ) {
        if let Expression::Variable(Variable::Direct(variable)) = assignment.lhs {
            written.push(variable);
        }
    }

    fn walk_in_unary_prefix(
        &self,
        unary: &'ast UnaryPrefix<'arena>,
        written: &mut Vec<&'ast DirectVariable<'arena>>,
    ) {
        if let UnaryPrefixOperator::PreIncrement(_) | UnaryPrefixOperator::PreDecrement(_) =
            unary.operator
            && let Expression::Variable(Variable::Direct(variable)) = unary.operand
        {
            written.push(variable);
        }
    }

    fn walk_in_unary_postfix(
        &self,
        unary: &'ast UnaryPostfix<'arena>,
        written: &mut Vec<&'ast DirectVariable<'arena>>,
    ) {
        if let Expression::Variable(Variable::Direct(variable)) = unary.operand {
            written.push(variable);
        }
    }
}

/// Replaces each copy mark with `.clone()` when the variable it moves is read again, or
/// with nothing (P2R-26). A list literal can move several variables on one line. A strict
/// mark read again is pushed to `copied` instead, for its P2R-145 problem.
fn resolve_copy(
    line: &mut String,
    copies: &[(Span, Span, bool)],
    read_again: &[(Var, Span)],
    copied: &mut Vec<Span>,
) {
    while let Some(start) = line.find(COPY) {
        let Some(end) = line[start + 1..].find(COPY).map(|end| start + 1 + end) else {
            return;
        };
        let number = usize::from_str_radix(&line[start + 1..end], 16).ok();
        let Some(&(read, _, strict)) = number.and_then(|number| copies.get(number)) else {
            return;
        };
        let again = read_again.iter().any(|&(_, span)| span == read);
        if again && strict {
            copied.push(read);
        }
        line.replace_range(start..=end, if again && !strict { ".clone()" } else { "" });
    }
}

/// True when `expression` reads or writes the variable `$name` (`name` with its `$`).
pub(super) fn mentions(expression: &Expression<'_>, name: &[u8]) -> bool {
    let mut accesses = Vec::new();
    Accesses.walk_expression(expression, &mut accesses);
    accesses.iter().any(|access| match access {
        Access::Read(variable) | Access::Write(variable) => variable.name == name,
    })
}

/// A variable that a refused expression reads or writes.
enum Access<'ast, 'arena> {
    Read(&'ast DirectVariable<'arena>),
    Write(&'ast DirectVariable<'arena>),
}

/// Collects the variables of a refused expression in PHP's order of evaluation: every
/// variable is a read, except the target of an assignment, which is written after its
/// value.
struct Accesses;

impl<'ast, 'arena> Walker<'ast, 'arena, Vec<Access<'ast, 'arena>>> for Accesses {
    fn walk_in_direct_variable(
        &self,
        variable: &'ast DirectVariable<'arena>,
        accesses: &mut Vec<Access<'ast, 'arena>>,
    ) {
        accesses.push(Access::Read(variable));
    }

    fn walk_assignment(
        &self,
        assignment: &'ast Assignment<'arena>,
        accesses: &mut Vec<Access<'ast, 'arena>>,
    ) {
        self.walk_expression(assignment.rhs, accesses);
        // Every operator but `=` reads the target before it writes it.
        if !matches!(assignment.operator, AssignmentOperator::Assign(_)) {
            self.walk_expression(assignment.lhs, accesses);
        }
        self.target(assignment.lhs, accesses);
    }
}

impl Accesses {
    fn target<'ast, 'arena>(
        &self,
        target: &'ast Expression<'arena>,
        accesses: &mut Vec<Access<'ast, 'arena>>,
    ) {
        let elements = match target {
            Expression::Variable(Variable::Direct(variable)) => {
                accesses.push(Access::Write(variable));
                return;
            }
            // `$a[] = x` and `$a[$i] = x` change the list in `$a`: they read it, then write it.
            Expression::ArrayAppend(append) => {
                self.walk_expression(append.array, accesses);
                self.target(append.array, accesses);
                return;
            }
            Expression::ArrayAccess(access) => {
                self.walk_expression(access.index, accesses);
                self.walk_expression(access.array, accesses);
                self.target(access.array, accesses);
                return;
            }
            Expression::Array(array) => &array.elements,
            Expression::LegacyArray(array) => &array.elements,
            Expression::List(list) => &list.elements,
            // Any other target, such as a property, only reads its variables.
            other => {
                self.walk_expression(other, accesses);
                return;
            }
        };
        for element in elements.iter() {
            match element {
                ArrayElement::KeyValue(element) => self.target(element.value, accesses),
                ArrayElement::Value(element) => self.target(element.value, accesses),
                _ => {}
            }
        }
    }
}

/// The operation of a compound assignment, `++` or `--`; `None` for `=`.
pub(super) fn operation(operator: &str) -> Option<BinOp> {
    match operator {
        "+=" => Some(BinOp::Add),
        "-=" => Some(BinOp::Sub),
        "*=" => Some(BinOp::Mul),
        "<<=" => Some(BinOp::Shl),
        ">>=" => Some(BinOp::Shr),
        _ => None,
    }
}

/// True when a guard with this binding becomes a let-else: its body always leaves and
/// the binding is read after it (P2R-104).
fn let_else(binding: Option<Var>, unused: &[bool]) -> bool {
    binding.is_some_and(|var| !unused[var])
}

/// The opening line of an `if` or `elseif` that checks variables against null, at
/// `indent`: each binding that is read becomes `let Some(v) = ..`, each other one
/// `v.is_some()` (P2R-104).
fn condition_line(
    indent: &str,
    keyword: &str,
    parts: &[Part],
    open: &str,
    unused: &[bool],
) -> String {
    let operands: Vec<&str> = parts
        .iter()
        .map(|part| match part {
            Part::Plain(text) => text.as_str(),
            Part::Let { binding, test, .. } if unused[*binding] => test.as_str(),
            Part::Let { pattern, .. } => pattern.as_str(),
        })
        .collect();
    format!("{indent}{keyword} {} {open}", operands.join(" && "))
}

/// The pattern of a foreach: its value, or its key and value (P2R-41).
/// The Rust text of `[]` assigned to a variable before its type is known (P2R-21).
const EMPTY_LIST: &str = "Vec::new()";

/// The value of a first write, with `[]` as `IndexMap::new()` for a variable that holds a
/// map at the end of the body (P2R-241).
fn first_value<'v>(value: &'v str, ty: Option<&RustType>) -> &'v str {
    match ty.map(RustType::inner) {
        Some(RustType::Map(..)) if value == EMPTY_LIST => "IndexMap::new()",
        _ => value,
    }
}

fn for_pattern(
    entries: &[Entry],
    unused: &[bool],
    key: Option<(Var, bool)>,
    value: Var,
    by_ref: bool,
) -> String {
    let value = pattern(&entries[value], unused[value], by_ref);
    match key {
        Some((key, by_ref)) => {
            format!("({}, {value})", pattern(&entries[key], unused[key], by_ref))
        }
        None => value,
    }
}

/// A foreach key or value in its `for` pattern: `_` when it is never read, `mut` when the
/// body assigns it again, behind `&` when `by_ref` (P2R-35, P2R-41).
fn pattern(entry: &Entry, unused: bool, by_ref: bool) -> String {
    let name = match &entry.binding {
        Some(binding) if unused => format!("_{}", binding.snake),
        _ => entry.rust_name.clone(),
    };
    let rewritten = entry.binding.as_ref().is_some_and(|b| b.rewritten);
    match (by_ref, rewritten) {
        (true, true) => format!("&(mut {name})"),
        (true, false) => format!("&{name}"),
        (false, true) => format!("mut {name}"),
        (false, false) => name,
    }
}

fn after_loop(php_name: &[u8]) -> String {
    format!(
        "${} is read after its foreach loop, where PHP keeps the last value (P2R-41)",
        String::from_utf8_lossy(php_name)
    )
}

fn unassigned(php_name: &[u8]) -> String {
    format!(
        "the variable ${} is not assigned on every path to this read (P2R-34)",
        String::from_utf8_lossy(php_name)
    )
}

/// The type of a later read or write, refused when it differs from the variable's type
/// (P2R-34).
fn same_type(php_name: &[u8], known: &RustType, found: RustType) -> Result<RustType, String> {
    if found == *known {
        return Ok(found);
    }
    Err(format!(
        "the variable ${} changes type from {} to {} (P2R-34)",
        String::from_utf8_lossy(php_name),
        known.owned(),
        found.owned()
    ))
}

/// Only the types of `types::built` are written so far; the other types follow in later
/// steps (P2R-20).
fn built_yet(ty: RustType) -> Result<RustType, String> {
    if types::built(&ty) {
        return Ok(ty);
    }
    if types::nullable_object(&ty) {
        return Err(types::NULLABLE_OBJECT.to_owned());
    }
    Err(format!(
        "variables of type {} are not built yet (P2R-20)",
        ty.owned()
    ))
}
