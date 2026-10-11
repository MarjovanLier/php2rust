//! Variables: reads, assignments, `++` and `--`, and the `let` and `mut` they need
//! (P2R-20 to P2R-22, P2R-31, P2R-34, P2R-35, P2R-43, P2R-44, P2R-50).
//!
//! A variable is declared by a `let` at its first write in the source. Whether that `let`
//! needs `mut` is known only at the end of the body, so the line carries a mark until
//! [`Converter::finish_variables`] replaces it.
//!
//! Two later steps change this file. A variable first written inside a block, and read
//! after it, needs its declaration moved above that block. A `foreach` binding
//! that reuses a PHP name is a new entry: `lookup` must then find the binding, and the
//! P2R-31 check in `declare` must allow an earlier entry with the same PHP name.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::flow::{Flow, Var};
use super::{Converter, ParameterSignature};
use crate::printer::{self, RExpr};
use crate::types::{self, RustType};

/// Stands in a `let` line for its `mut `. `printer::quote` escapes every control
/// character, so no other output text holds it.
const MARK: char = '\u{1}';

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
}

/// The variables of the body being converted, a function or `fn main()`.
#[derive(Default)]
pub(super) struct Variables {
    /// Indexed by [`Var`] in order of first write, so the output never depends on
    /// hashing (P2R-06).
    entries: Vec<Entry>,
    flow: Flow,
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
            });
        }
        variables
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
        let line = if text.contains(MARK) {
            // A `let` is measured at the end of the body, once its `mut` is known (P2R-51).
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
            Ok(ty) => Some((RExpr::Var(entry.rust_name.clone()), ty)),
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
            self.refuse(
                target,
                "only a plain variable can be assigned to in version 1 (P2R-07)",
            );
            return None;
        };
        // PHP evaluates the value first. Every operator but `=` then reads the variable
        // before it writes it.
        let value = match value {
            Some(value) => self.expression(value),
            None => Some((RExpr::Int(1), RustType::I64)),
        };
        let current = (operator != Some("=")).then(|| self.variable(variable));
        let converted = match (operator, value, current) {
            (Some(operator), Some(value), None) => Some((operator, value, None)),
            (Some(operator), Some(value), Some(Some((_, ty)))) => Some((operator, value, Some(ty))),
            _ => None,
        };
        let Some((operator, (expr, value_type), current_type)) = converted else {
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
        if let Some(current_type) = current_type
            && (current_type != RustType::I64 || value_type != RustType::I64)
        {
            self.record_write(variable);
            self.refuse(
                expression,
                "compound assignment is built for int operands only so far (P2R-43)",
            );
            return None;
        }
        let (var, declares, ty) = self.write(variable, expression)?;
        let name = &self.variables.entries[var].rust_name;
        if operator == "=" && matches!(&expr, RExpr::Var(read) if read == name) {
            self.refuse(
                expression,
                "an assignment of a variable to itself does nothing, and rustc rejects it (P2R-50)",
            );
            return None;
        }
        let rendered = self.render(expression, &expr, false)?;
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
        let checked = match declared {
            // The name problem is recorded.
            Some(false) => return None,
            Some(true) => found,
            None => {
                let known = entry.ty.clone()?;
                found.and_then(|found| same_type(&entry.php_name, &known, found))
            }
        };
        match checked.and_then(int_only) {
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
        (var, declared)
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
        });
        (self.variables.entries.len() - 1, named)
    }

    fn lookup(&self, variable: &DirectVariable<'_>) -> Option<Var> {
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
        let Variables { entries, flow } = std::mem::take(&mut self.variables);
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
            if let Some(var) = resolve_mark(line, &facts.reassigned)
                && line.len() > printer::MAX_WIDTH
                && let Some(span) = entries[var].assigned
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

/// Only int variables are written so far; the other types follow in later steps (P2R-20).
fn int_only(ty: RustType) -> Result<RustType, String> {
    if ty == RustType::I64 {
        return Ok(ty);
    }
    Err(format!(
        "variables of type {} are not built yet (P2R-20)",
        ty.owned()
    ))
}
