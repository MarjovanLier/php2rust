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
//! [`Converter::finish_variables`] replaces it.
//!
//! A later step changes this file. A `foreach` binding that reuses a PHP name is a new
//! entry: `lookup` must then find the binding, and the P2R-31 check in `declare` must
//! allow an earlier entry with the same PHP name.
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

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use super::control::Loop;
use super::flow::{Flow, Var};
use super::{Converter, ParameterSignature, lists, strings};
use crate::printer::{self, BinOp, RExpr};
use crate::types::{self, RustType};

/// Starts a line whose text is known only at the end of the body, followed by its index
/// in `Variables::pending`. `printer::quote` escapes every control character, so no other
/// output text holds it.
const MARK: char = '\u{1}';

/// Stands after a string variable that an assignment moves, for its `.clone()` when the
/// variable is read again (P2R-26).
const COPY: char = '\u{2}';

/// A line whose text is known only at the end of the body.
enum Pending {
    /// The typed `let` lines of the variables declared before a block statement.
    Slot,
    /// A variable's first write: `let x = v;`, or `x = v;` when its `let` is in a slot.
    First {
        var: Var,
        annotation: String,
        value: String,
    },
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
}

/// The variables of the body being converted, a function or `fn main()`, and the state
/// of its control flow.
pub(super) struct Variables {
    /// Indexed by [`Var`] in order of first write, so the output never depends on
    /// hashing (P2R-06).
    entries: Vec<Entry>,
    pub(super) flow: Flow,
    /// By copy mark: the read the assignment moves, and the assignment.
    copies: Vec<(Span, Span)>,
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
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            flow: Flow::default(),
            copies: Vec::new(),
            scopes: vec![Scope {
                parent: None,
                looped: false,
            }],
            scope: 0,
            pending: Vec::new(),
            read_problems: Vec::new(),
            left: None,
            loops: Vec::new(),
        }
    }
}

impl Variables {
    /// A function body, where each parameter holds its argument from the start.
    pub(super) fn with_parameters(parameters: &[ParameterSignature]) -> Self {
        let mut variables = Self::default();
        for (var, parameter) in parameters.iter().enumerate() {
            variables.flow.write(var, None);
            variables.entries.push(Entry {
                php_name: parameter.php_name.clone(),
                rust_name: parameter.rust_name.clone(),
                ty: Some(parameter.ty.clone()),
                parameter: Some(parameter.snake.clone()),
                assigned: None,
                value: None,
                first: 0,
                common: 0,
            });
        }
        variables
    }

    fn read(&mut self, var: Var, span: Span) {
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

    pub(super) fn close(&mut self, outer: usize) {
        self.scope = outer;
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

    /// The copy mark for an assignment that moves the variable read at `read`.
    fn copy_mark(&mut self, read: Span, assignment: Span) -> String {
        self.copies.push((read, assignment));
        format!("{COPY}{}{COPY}", self.copies.len() - 1)
    }

    /// True when `rust_name` is a parameter: a string parameter is a `&str` (P2R-24).
    pub(super) fn is_parameter(&self, rust_name: &str) -> bool {
        self.entries
            .iter()
            .any(|e| e.parameter.is_some() && e.rust_name == rust_name)
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
        let Some(text) = self.change(expression) else {
            return;
        };
        let line = if text.contains([MARK, COPY]) {
            // A first write or a move is measured at the end of the body, once its `let`
            // and its `.clone()` are known (P2R-51).
            format!("{}{text}", "    ".repeat(depth))
        } else {
            self.emit(expression, depth, text)
        };
        body.push(line);
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
        let Some(var) = self.lookup(variable) else {
            // No write comes before this read in the source, so a path reaches it unassigned.
            self.refuse(variable, unassigned(&variable.name[1..]));
            return None;
        };
        self.variables.read(var, variable.span);
        let entry = &self.variables.entries[var];
        let known = entry.ty.clone()?;
        let checked = self
            .site_type(variable)
            .and_then(|found| same_type(&entry.php_name, &known, found));
        match checked {
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
        let Expression::Variable(Variable::Direct(variable)) = target else {
            // The value's reads and the target's writes still count, so no false P2R-34 or
            // P2R-35 line follows. The refusal covers the whole assignment, so it is the one
            // line printed for it (P2R-03).
            if let Some(value) = value {
                self.expression(value);
            }
            self.record_refused_target(target);
            let message = if matches!(
                target,
                Expression::ArrayAccess(_) | Expression::ArrayAppend(_)
            ) {
                "writing to a list index is not supported in version 1 (P2R-47)"
            } else {
                "only a plain variable can be assigned to in version 1 (P2R-07)"
            };
            self.refuse(expression, message);
            return None;
        };
        // PHP evaluates the value first. Every operator but `=` then reads the variable
        // before it writes it.
        let source = value;
        let value = match value {
            Some(value) if operator == Some(".=") => self.joined_string(value),
            Some(value) => self.expression(value),
            None => Some((RExpr::Int(1), RustType::I64)),
        };
        let current = (operator != Some("=")).then(|| self.variable(variable));
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
        if operator != ".="
            && let Some((_, current_type)) = &current
            && (*current_type != RustType::I64 || value_type != RustType::I64)
        {
            self.record_write(variable);
            self.refuse(
                expression,
                "compound assignment is built for int operands only so far (P2R-43)",
            );
            return None;
        }
        let (var, declares, ty) = self.write(variable, expression)?;
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
        self.variables.set_value(var, value);
        let name = &self.variables.entries[var].rust_name;
        let read_itself = matches!(&expr, RExpr::Var(read) | RExpr::Known(read, _) if read == name);
        if operator == "=" && read_itself {
            self.refuse(
                expression,
                "an assignment of a variable to itself does nothing, and rustc rejects it (P2R-50)",
            );
            return None;
        }
        let mut moved = None;
        let (operator, expr) = if operator == ".=" {
            // `s += &s` borrows `s` twice, which rustc rejects (E0502).
            let expr = if read_itself {
                RExpr::Method(Box::new(expr), "clone")
            } else {
                expr
            };
            ("+=", self.borrowed(expr, &value_type))
        } else if ty == RustType::String {
            let expr = self.owned(expr, &ty);
            // A string variable assigned alone moves; whether it needs a copy is known
            // at the end of the body (P2R-26).
            if matches!(expr, RExpr::Var(_)) {
                moved = source.and_then(strings::sole_read);
            }
            (operator, expr)
        } else {
            (operator, expr)
        };
        let mut rendered = self.render(expression, &expr, false)?;
        if let Some(read) = moved {
            rendered.push_str(&self.variables.copy_mark(read, expression.span()));
        }
        let name = &self.variables.entries[var].rust_name;
        if !declares {
            return Some(format!("{name} {operator} {rendered};"));
        }
        // A literal alone would make Rust infer i32 (P2R-49).
        let annotation = if printer::anchored(&expr) {
            String::new()
        } else {
            format!(": {}", ty.owned())
        };
        self.variables.pending.push(Pending::First {
            var,
            annotation,
            value: rendered,
        });
        Some(format!("{MARK}{}", self.variables.pending.len() - 1))
    }

    /// `$a[] = x`, a change of the list in `$a`: PHP evaluates the value, then reads the list
    /// and writes it back (P2R-47).
    fn append(
        &mut self,
        expression: &Expression<'_>,
        list: &DirectVariable<'_>,
        value: &Expression<'_>,
    ) -> Option<String> {
        let value = self.expression(value);
        let current = self.variable(list);
        let (Some((value, _)), Some((_, current_type))) = (value, current) else {
            // The write still counts, so later reads see it.
            self.record_write(list);
            return None;
        };
        if !matches!(current_type, RustType::Vec(_)) {
            self.record_write(list);
            self.refuse(expression, "only a list can be appended to (P2R-47)");
            return None;
        }
        // Mago types the `$a` of `$a[]` as the list after the append, so a value of another
        // type shows as a type change.
        let (var, _, _) = self.write(list, list)?;
        let name = &self.variables.entries[var].rust_name;
        // rustfmt lays out the arguments of a method call as those of a call (P2R-51).
        let push = RExpr::Call(format!("{name}.push"), vec![value]);
        Some(format!("{};", self.render(expression, &push, false)?))
    }

    /// Records a write of `variable` by `node` and checks the type Mago gives the write.
    /// Gives the variable, whether this write declares it, and its type.
    fn write(
        &mut self,
        variable: &DirectVariable<'_>,
        node: &impl HasSpan,
    ) -> Option<(Var, bool, RustType)> {
        let found = self.site_type(node);
        let (var, declared) = self.record_write(variable);
        let entry = &self.variables.entries[var];
        // PHP passes the function a copy, but Rust borrows a string or list (P2R-24).
        let borrowed = entry
            .ty
            .as_ref()
            .is_some_and(|ty| ty.parameter() != ty.owned());
        if entry.parameter.is_some() && borrowed {
            self.refuse(
                node,
                "a string or list parameter cannot be changed, because PHP passes a copy (P2R-24)",
            );
            return None;
        }
        let checked = match declared {
            // The name problem is recorded.
            Some(false) => return None,
            Some(true) => found,
            None => {
                let known = entry.ty.clone()?;
                found.and_then(|found| same_type(&entry.php_name, &known, found))
            }
        };
        match checked.and_then(built_yet) {
            Ok(ty) => {
                self.variables.entries[var].ty = Some(ty.clone());
                Some((var, declared.is_some(), ty))
            }
            Err(message) => {
                self.fail(var, node, message);
                None
            }
        }
    }

    /// Records a write of `variable`, and declares the variable at its first write. The
    /// flag is `Some` for that first write, and `Some(false)` when the name does not
    /// convert. The write is placed at the variable, not the whole assignment, so a
    /// problem with it does not hide one in the value (P2R-03).
    fn record_write(&mut self, variable: &DirectVariable<'_>) -> (Var, Option<bool>) {
        let (var, declared) = match self.lookup(variable) {
            Some(var) => (var, None),
            None => {
                let (var, named) = self.declare(variable);
                (var, Some(named))
            }
        };
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
    fn record_refused_target(&mut self, target: &Expression<'_>) {
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

    /// Adds a variable at its first write. The flag is `false` when the name has no Rust
    /// form, or gives the Rust name of a function or of another variable (P2R-30,
    /// P2R-31).
    fn declare(&mut self, variable: &DirectVariable<'_>) -> (Var, bool) {
        let php_name = &variable.name[1..];
        let rust_name = self.rust_name(php_name, variable);
        let named = match &rust_name {
            None => false,
            Some(name)
                if self.functions.values().any(|f| f.rust_name == *name)
                    || self.variables.entries.iter().any(|e| e.rust_name == *name) =>
            {
                self.refuse(
                    variable,
                    format!("{name} is already the Rust name of a function or variable (P2R-31)"),
                );
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
        });
        (self.variables.entries.len() - 1, named)
    }

    /// The variable a PHP name refers to, or `None` before its first write.
    pub(super) fn lookup(&self, variable: &DirectVariable<'_>) -> Option<Var> {
        let php_name = &variable.name[1..];
        self.variables
            .entries
            .iter()
            .position(|e| e.php_name == php_name)
    }

    /// The Rust type of the value Mago found at `node` (P2R-21, P2R-22).
    pub(super) fn site_type(&self, node: &impl HasSpan) -> Result<RustType, String> {
        let union = self
            .artifacts
            .get_expression_type(node)
            .ok_or_else(|| "Mago found no type here (P2R-34)".to_owned())?;
        types::rust_type(union).map_err(|name| format!("the type {name} has no Rust type (P2R-22)"))
    }

    /// Records a problem with a variable's type. Later uses of the variable convert to
    /// nothing, so one problem is reported per variable.
    fn fail(&mut self, var: Var, node: &impl HasSpan, message: String) {
        self.variables.entries[var].ty = None;
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
            scopes,
            pending,
            read_problems,
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
        let mut unused = vec![false; entries.len()];
        for (var, span) in facts.unread {
            let entry = &entries[var];
            let name = String::from_utf8_lossy(&entry.php_name);
            match (span, entry.assigned) {
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
        let slots: Vec<Option<usize>> = entries
            .iter()
            .map(|entry| slot(&scopes, entry.first, entry.common))
            .collect();
        let let_keyword = |var: Var| {
            if facts.reassigned[var] {
                "let mut "
            } else {
                "let "
            }
        };
        let mut lines = Vec::with_capacity(body.len());
        for line in body.drain(..) {
            // Each line, with the first write it declares, whose span a width problem uses.
            let mut finished: Vec<(Option<Span>, String)> = Vec::new();
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
                            finished.push((entry.assigned, text));
                        }
                    }
                }
                Some((
                    at,
                    _,
                    Pending::First {
                        var,
                        annotation,
                        value,
                    },
                )) => {
                    let indent = &line[..at];
                    let name = &entries[*var].rust_name;
                    let text = match slots[*var] {
                        Some(_) => format!("{indent}{name} = {value};"),
                        None => {
                            format!("{indent}{}{name}{annotation} = {value};", let_keyword(*var))
                        }
                    };
                    finished.push((entries[*var].assigned, text));
                }
                None => finished.push((None, line)),
            }
            for (declared, mut text) in finished {
                let moved = resolve_copy(&mut text, &copies, &facts.read_again);
                if text.len() > printer::MAX_WIDTH
                    && let Some(span) = declared.or(moved)
                {
                    self.refuse(&span, "this statement is too long for one line (P2R-51)");
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
                let name = if unused[var] {
                    format!("_{snake}")
                } else if facts.reassigned[var] {
                    format!("mut {}", entry.rust_name)
                } else {
                    entry.rust_name.clone()
                };
                Some(format!("{name}: {}", entry.ty.as_ref()?.parameter()))
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

/// Replaces the copy mark of an assignment with `.clone()` when the variable it moves is
/// read again, or with nothing, and gives the assignment (P2R-26).
fn resolve_copy(
    line: &mut String,
    copies: &[(Span, Span)],
    read_again: &[(Var, Span)],
) -> Option<Span> {
    let start = line.find(COPY)?;
    let end = start + 1 + line[start + 1..].find(COPY)?;
    let (read, assignment) = *copies.get(line[start + 1..end].parse::<usize>().ok()?)?;
    let again = read_again.iter().any(|&(_, span)| span == read);
    line.replace_range(start..=end, if again { ".clone()" } else { "" });
    Some(assignment)
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
fn operation(operator: &str) -> Option<BinOp> {
    match operator {
        "+=" => Some(BinOp::Add),
        "-=" => Some(BinOp::Sub),
        "*=" => Some(BinOp::Mul),
        _ => None,
    }
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

/// Only int, bool, string and list variables are written so far; the other types follow
/// in later steps (P2R-20).
fn built_yet(ty: RustType) -> Result<RustType, String> {
    if matches!(ty, RustType::I64 | RustType::Bool | RustType::String) || lists::list_built(&ty) {
        return Ok(ty);
    }
    Err(format!(
        "variables of type {} are not built yet (P2R-20)",
        ty.owned()
    ))
}
