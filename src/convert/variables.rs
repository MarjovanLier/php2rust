//! Variables: reads, assignments, `++` and `--`, and the `let` and `mut` they need
//! (P2R-20 to P2R-22, P2R-31, P2R-34, P2R-35, P2R-43, P2R-44, P2R-50).
//!
//! A variable is declared by a `let` at its first write in the source. Whether that `let`
//! needs `mut` is known only at the end of the body, so the line carries a mark until
//! [`Converter::finish_variables`] replaces it.
//!
//! Two later steps change this file. A variable first written inside a block, and read
//! after it, needs its declaration moved above that block. A variable first
//! written inside a loop needs a typed `let` above the outermost loop that holds that
//! write, even when only the loop reads it: `reassigned` counts back edges, so an inline
//! `let` in the loop body would get rustc's unused_mut (see flow.rs). A `foreach` binding
//! that reuses a PHP name is a new entry: `lookup` must then find the binding, and the
//! P2R-31 check in `declare` must allow an earlier entry with the same PHP name.
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
//! change it. the control flow code keeps the values right at loops and joins with `lookup`,
//! `Variables::value` and `Variables::set_value`:
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

use super::flow::{Flow, Var};
use super::{Converter, ParameterSignature, strings};
use crate::printer::{self, BinOp, RExpr};
use crate::types::{self, RustType};

/// Stands in a `let` line for its `mut `. `printer::quote` escapes every control
/// character, so no other output text holds it.
const MARK: char = '\u{1}';

/// Stands after a string variable that an assignment moves, for its `.clone()` when the
/// variable is read again (P2R-26).
const COPY: char = '\u{2}';

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
}

/// The variables of the body being converted, a function or `fn main()`.
#[derive(Default)]
pub(super) struct Variables {
    /// Indexed by [`Var`] in order of first write, so the output never depends on
    /// hashing (P2R-06).
    entries: Vec<Entry>,
    flow: Flow,
    /// By copy mark: the read the assignment moves, and the assignment.
    copies: Vec<(Span, Span)>,
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
            });
        }
        variables
    }

    /// The value `var` holds here, when it is known (P2R-50).
    pub(super) fn value(&self, var: Var) -> Option<i64> {
        self.entries[var].value
    }

    /// Sets the value `var` holds here; `None` forgets it. the control flow code clears and meets values
    /// with this at loops and joins (see the module doc).
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
    /// An assignment, `++` or `--` as a statement (P2R-43).
    pub(super) fn assignment_statement(
        &mut self,
        statement: &ExpressionStatement<'_>,
        depth: usize,
        body: &mut Vec<String>,
    ) {
        let Some(text) = self.change(statement.expression) else {
            return;
        };
        let line = if text.contains([MARK, COPY]) {
            // A `let` or a move is measured at the end of the body, once its `mut` and its
            // `.clone()` are known (P2R-51).
            format!("{}{text}", "    ".repeat(depth))
        } else {
            self.emit(statement, depth, text)
        };
        body.push(line);
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
        self.variables.flow.read(var, variable.span);
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
                self.fail(var, variable, message);
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
        let Expression::Variable(Variable::Direct(variable)) = target else {
            // The value's reads and the target's writes still count, so no false P2R-34 or
            // P2R-35 line follows. The refusal covers the whole assignment, so it is the one
            // line printed for it (P2R-03).
            if let Some(value) = value {
                self.expression(value);
            }
            self.record_refused_target(target);
            self.refuse(
                expression,
                "only a plain variable can be assigned to in version 1 (P2R-07)",
            );
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
        Some(format!(
            "let {MARK}{var}{MARK}{name}{annotation} = {rendered};"
        ))
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
        self.variables.flow.write(var, Some(variable.span));
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
                        self.variables.flow.read(var, variable.span);
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
    fn site_type(&self, node: &impl HasSpan) -> Result<RustType, String> {
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
    pub(super) fn finish_variables(&mut self, body: &mut [String]) -> Vec<String> {
        let Variables {
            entries,
            flow,
            copies,
        } = std::mem::take(&mut self.variables);
        let facts = flow.finish(entries.len());
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
        for line in body.iter_mut() {
            let moved = resolve_copy(line, &copies, &facts.read_again);
            let declared =
                resolve_mark(line, &facts.reassigned).and_then(|var| entries[var].assigned);
            if line.len() > printer::MAX_WIDTH
                && let Some(span) = declared.or(moved)
            {
                self.refuse(&span, "this statement is too long for one line (P2R-51)");
            }
        }
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

/// Replaces the mark of a `let` line with `mut ` or with nothing, and gives the variable
/// the line declares.
fn resolve_mark(line: &mut String, reassigned: &[bool]) -> Option<Var> {
    let start = line.find(MARK)?;
    let end = start + 1 + line[start + 1..].find(MARK)?;
    let var: Var = line[start + 1..end].parse().ok()?;
    line.replace_range(start..=end, if reassigned[var] { "mut " } else { "" });
    Some(var)
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

/// Only int and string variables are written so far; the other types follow in later
/// steps (P2R-20).
fn built_yet(ty: RustType) -> Result<RustType, String> {
    if matches!(ty, RustType::I64 | RustType::String) {
        return Ok(ty);
    }
    Err(format!(
        "variables of type {} are not built yet (P2R-20)",
        ty.owned()
    ))
}
