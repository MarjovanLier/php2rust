//! `if`, `while`, `for`, `break` and `continue`, and the statements no path reaches
//! (P2R-34, P2R-40, P2R-50).
//!
//! Each statement adds its blocks and edges to the body's flow graph by the rules in
//! flow.rs, writes a slot line before itself for the typed `let` lines of variables.rs,
//! and keeps the known values of variables.rs right at joins and loops.

use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use super::variables::changes_variable;
use super::{Context, Converter};
use crate::printer::{self, BinOp, RExpr};
use crate::types::RustType;

/// The operands of the top-level `||` chain of a condition, left to right (P2R-334).
fn or_operands<'e, 'a>(condition: &'e Expression<'a>) -> Vec<&'e Expression<'a>> {
    match super::lists::unparenthesized(condition) {
        Expression::Binary(binary) if matches!(binary.operator, BinaryOperator::Or(_)) => {
            let mut operands = or_operands(binary.lhs);
            operands.extend(or_operands(binary.rhs));
            operands
        }
        other => vec![other],
    }
}

/// The known value of every variable, by variable (P2R-50).
type Values = Vec<Option<i64>>;

/// A loop around the statement being converted.
pub(super) struct Loop {
    /// The flow block of the condition, where `continue` goes.
    head: usize,
    /// The flow block after the loop, where `break` goes.
    exit: usize,
    /// The values at each `break`.
    breaks: Vec<Values>,
    /// For a `for` with a step: each `continue`, where the step runs first (P2R-40).
    steps: Option<Vec<Site>>,
    /// The PHP names that a let-else anywhere in the body guards, which the step must not
    /// read or write (P2R-104).
    guarded: Vec<Vec<u8>>,
}

impl Loop {
    /// A loop without a step, whose head may exit: a foreach (P2R-41).
    pub(super) fn new(head: usize, exit: usize) -> Self {
        Self {
            head,
            exit,
            breaks: Vec::new(),
            steps: None,
            guarded: Vec::new(),
        }
    }

    /// The values at each `break`.
    pub(super) fn into_breaks(self) -> Vec<Values> {
        self.breaks
    }
}

/// A `continue` in a `for`. Its copy of the step is converted once the body is done,
/// because only the `for` holds the step.
struct Site {
    /// The index in the body's lines of the `continue;` line.
    line: usize,
    depth: usize,
    /// The flow block between the `continue` and the loop head.
    block: usize,
    /// The Rust block of the `continue`.
    scope: usize,
    values: Values,
}

/// A branch of an `if`: its condition, `None` for `else`, and its statements.
type Branch<'a, 'arena> = (Option<&'a Expression<'arena>>, Vec<&'a Statement<'arena>>);

impl Converter<'_> {
    /// Refuses a statement that no path reaches, which rustc rejects as unreachable_code
    /// (P2R-40). True when the statement must not convert.
    pub(super) fn unreachable(&mut self, statement: &Statement<'_>) -> bool {
        let Some(reason) = self.variables.left else {
            return false;
        };
        if !matches!(statement, Statement::Noop(_)) {
            self.refuse(
                statement,
                format!("a statement after {reason} never runs, and rustc rejects it (P2R-40)"),
            );
        }
        true
    }

    /// Ends the path: no statement after a `return`, `break` or `continue` runs.
    pub(super) fn leave(&mut self, reason: &'static str) {
        self.variables.left = Some(reason);
        let block = self.variables.flow.block();
        self.variables.flow.enter(block);
    }

    pub(super) fn if_statement(
        &mut self,
        statement: &If<'_>,
        depth: usize,
        context: Context<'_>,
        body: &mut Vec<String>,
    ) {
        let branches = branches(statement);
        // rustfmt keeps an empty block on the opening line only without an else.
        let alone = branches.len() == 1 && empty(&branches[0].1);
        // `if ($v === null)` alone, on a variable that may be null, becomes a let-else when
        // its body always leaves (P2R-104), and so does `if (!is_T($v))` on a union value
        // (P2R-252).
        // `if (X || B)`, where X is such a check of `$v` and the body does not read `$v`,
        // becomes the let-else of X and `if B` after it, each with the body (P2R-334).
        let split = match branches.as_slice() {
            [(Some(condition), statements)] if !alone => self.split_guard(condition, statements),
            _ => None,
        };
        let guarded = match branches.as_slice() {
            [(Some(condition), _)] if !alone => {
                self.guard_check(split.as_ref().map_or(*condition, |(first, _)| *first))
            }
            _ => None,
        };
        let mut copied: Vec<String> = Vec::new();
        let mut guard: Option<usize> = None;
        let mut body_leaves = false;
        let slot = self.slot(depth, body);
        let before = self.variables.values();
        let join = self.variables.flow.block();
        // The flow block where the next condition is tested.
        let mut test = self.variables.flow.current();
        let mut arriving: Vec<Values> = Vec::new();
        for (index, (condition, statements)) in branches.iter().enumerate() {
            self.variables.set_values(&before);
            // The bindings of the null checks in this branch's condition (P2R-104).
            let mut bindings = Vec::new();
            let opening = match condition {
                Some(condition) => {
                    if index > 0 {
                        // An elseif tests its condition only when the ones above are false.
                        let block = self.variables.flow.block();
                        self.variables.flow.edge(test, block);
                        self.variables.flow.enter(block);
                        test = block;
                    }
                    let keyword = if index == 0 { "if" } else { "} else if" };
                    let open = if alone { "{}" } else { "{" };
                    if let Some((_, _, _, pattern)) = &guarded {
                        let head = split.as_ref().map_or(*condition, |(first, _)| *first);
                        let test = self.condition_text(head);
                        let (line, at) = self.variables.guard_line(depth, pattern.clone(), test);
                        guard = Some(at);
                        line
                    } else {
                        match self.checked_condition(condition, keyword, open, depth) {
                            Some((line, checked)) => {
                                bindings = checked;
                                line
                            }
                            None => {
                                let text = self.condition_text(condition);
                                emit(depth, format!("{keyword} {text} {open}"))
                            }
                        }
                    }
                }
                None => format!("{}}} else {{", indent(depth)),
            };
            body.push(opening);
            let start = body.len();
            let block = self.variables.flow.block();
            self.variables.flow.edge(test, block);
            self.variables.flow.enter(block);
            self.variables.left = None;
            let outer = self.variables.open(slot, false);
            for inner in statements {
                self.statement(inner, depth + 1, context, body);
            }
            if split.is_some() {
                copied = body[start..].to_vec();
            }
            self.variables.close(outer);
            self.variables.close_bindings(&bindings);
            body_leaves = self.variables.left.is_some();
            if !body_leaves {
                let end = self.variables.flow.current();
                self.variables.flow.edge(end, join);
                arriving.push(self.variables.values());
            }
        }
        if branches
            .last()
            .is_some_and(|(condition, _)| condition.is_some())
        {
            // Without an else, control goes on after the if when every condition is false.
            self.variables.flow.edge(test, join);
            arriving.push(before);
        }
        if !alone {
            let end = match guard {
                Some(at) => self.variables.guard_end(depth, at),
                None => format!("{}}}", indent(depth)),
            };
            body.push(end);
        }
        self.variables.flow.enter(join);
        self.join(arriving, "an if whose every branch leaves");
        // The statements after a let-else in its block read the binding (P2R-104,
        // P2R-252).
        if let (Some((variable, _, inner, _)), Some(at), true) = (guarded, guard, body_leaves) {
            let owned = self.variables.owned_checks.contains(&statement.span());
            let copy = owned.then(|| self.copy_method(&inner)).flatten();
            let binding = self.bind_null(variable, inner);
            self.variables.guard(binding);
            self.variables.set_guard(at, binding);
            if let Some(copy) = copy {
                let name = self.variables.name_of(binding);
                let line = format!("let mut {name} = {name}{copy}");
                self.variables.own_binding(at, binding, line);
            }
            for frame in &mut self.variables.loops {
                frame.guarded.push(variable.name[1..].to_vec());
            }
        }
        // The rest of the chain reads the binding, so it converts after it (P2R-334).
        if let Some((_, rest)) = split {
            let parts: Vec<Option<RExpr>> = rest.iter().map(|o| self.condition(o)).collect();
            if let Some(test) = parts
                .into_iter()
                .reduce(|left, right| {
                    Some(RExpr::Binary(Box::new(left?), BinOp::Or, Box::new(right?)))
                })
                .flatten()
            {
                let text = printer::expr(&test, false);
                body.push(emit(depth, format!("if {text} {{")));
                body.extend(copied);
                body.push(format!("{}}}", indent(depth)));
            }
        }
    }

    /// The first operand of the top-level `||` chain of `condition` and the rest, when the
    /// first is a check of `$v` that a let-else binds (P2R-104, P2R-333) and `statements`
    /// do not read `$v`, whose Rust name the binding of the copied body would change
    /// (P2R-334).
    fn split_guard<'e, 'a>(
        &self,
        condition: &'e Expression<'a>,
        statements: &[&Statement<'a>],
    ) -> Option<(&'e Expression<'a>, Vec<&'e Expression<'a>>)> {
        let operands = or_operands(condition);
        let [first, rest @ ..] = operands.as_slice() else {
            return None;
        };
        if rest.is_empty() {
            return None;
        }
        let (variable, ..) = self.null_checked(first, false)?;
        // The copy of the body must leave as the let-else does. Inside a `for` with a step,
        // it must hold no `break` or `continue` that leaves it, because the step goes in at
        // each `continue` of the first copy only.
        let ends = match statements.last() {
            Some(Statement::Return(_) | Statement::Break(_) | Statement::Continue(_)) => true,
            Some(Statement::Expression(statement)) => {
                matches!(statement.expression, Expression::Throw(_))
            }
            _ => false,
        };
        let mut leaves = false;
        let mut found = Vec::new();
        for statement in statements {
            super::throws::Leaving.walk_statement(statement, &mut leaves);
            super::nullsafe::Variables.walk_statement(statement, &mut found);
        }
        let stepped = self
            .variables
            .loops
            .iter()
            .any(|frame| frame.steps.is_some());
        if !ends || (leaves && stepped) || found.iter().any(|name| name.as_slice() == variable.name)
        {
            return None;
        }
        Some((*first, rest.to_vec()))
    }

    /// The call that copies the binding of type `inner` on the copy line after its
    /// let-else, empty for a Copy value, or `None` for a type whose binding cannot be owned
    /// (P2R-290).
    fn copy_method(&self, inner: &RustType) -> Option<&'static str> {
        match inner {
            RustType::String => Some(".to_string()"),
            RustType::Vec(element) if !self.is_changing(element) => Some(".to_vec()"),
            // P2R-181 copies a map parameter the same way.
            RustType::Map(..) => Some(".clone()"),
            ty if ty.is_copy() => Some(""),
            _ => None,
        }
    }

    pub(super) fn while_statement(
        &mut self,
        statement: &While<'_>,
        depth: usize,
        context: Context<'_>,
        body: &mut Vec<String>,
    ) {
        let statements = match &statement.body {
            WhileBody::Statement(inner) => statements_of(inner),
            WhileBody::ColonDelimited(inner) => inner.statements.iter().collect(),
        };
        self.forget_writes_in(statement.condition);
        for inner in &statements {
            self.forget_writes(inner);
        }
        let parts = LoopParts {
            condition: Some(statement.condition),
            statements: &statements,
            steps: &[],
        };
        self.run_loop(parts, depth, context, body);
    }

    /// `for` becomes its initialisations, then `while` or `loop` with the step at the end
    /// of the body and before each `continue` (P2R-40).
    pub(super) fn for_statement(
        &mut self,
        statement: &For<'_>,
        depth: usize,
        context: Context<'_>,
        body: &mut Vec<String>,
    ) {
        for &initialization in statement.initializations.iter() {
            self.simple_expression(initialization, depth, body);
        }
        let conditions: Vec<&Expression<'_>> = statement.conditions.iter().copied().collect();
        if let [extra @ .., _] = conditions.as_slice()
            && !extra.is_empty()
        {
            for &condition in extra {
                self.record_refused(condition);
            }
            self.refuse(
                statement,
                "a for with more than one condition is not supported (P2R-07)",
            );
        }
        let steps: Vec<&Expression<'_>> = statement.increments.iter().copied().collect();
        let statements = match &statement.body {
            ForBody::Statement(inner) => statements_of(inner),
            ForBody::ColonDelimited(inner) => inner.statements.iter().collect(),
        };
        // After the initialisations, which run once, so their values stay known.
        for &expression in conditions.iter().chain(&steps) {
            self.forget_writes_in(expression);
        }
        for inner in &statements {
            self.forget_writes(inner);
        }
        let parts = LoopParts {
            condition: conditions.last().copied(),
            statements: &statements,
            steps: &steps,
        };
        self.run_loop(parts, depth, context, body);
    }

    pub(super) fn break_statement(
        &mut self,
        statement: &Break<'_>,
        depth: usize,
        body: &mut Vec<String>,
    ) {
        if !self.one_level(statement.level, statement) {
            return;
        }
        let values = self.variables.values();
        let Some(frame) = self.variables.loops.last_mut() else {
            self.refuse(statement, "break outside a loop is not supported (P2R-40)");
            return;
        };
        frame.breaks.push(values);
        let exit = frame.exit;
        let from = self.variables.flow.current();
        self.variables.flow.edge(from, exit);
        let line = emit(depth, "break;".to_owned());
        body.push(line);
        self.leave("break");
    }

    pub(super) fn continue_statement(
        &mut self,
        statement: &Continue<'_>,
        depth: usize,
        body: &mut Vec<String>,
    ) {
        if !self.one_level(statement.level, statement) {
            return;
        }
        let values = self.variables.values();
        let scope = self.variables.scope;
        let from = self.variables.flow.current();
        let Some(frame) = self.variables.loops.last_mut() else {
            self.refuse(
                statement,
                "continue outside a loop is not supported (P2R-40)",
            );
            return;
        };
        let head = frame.head;
        let to = match &mut frame.steps {
            Some(sites) => {
                let block = self.variables.flow.block();
                sites.push(Site {
                    line: body.len(),
                    depth,
                    block,
                    scope,
                    values,
                });
                self.variables.flow.edge(block, head);
                block
            }
            None => head,
        };
        self.variables.flow.edge(from, to);
        let line = emit(depth, "continue;".to_owned());
        body.push(line);
        self.leave("continue");
    }

    /// A `while`, `for` or `loop`, once the values its parts write are forgotten (P2R-50).
    fn run_loop(
        &mut self,
        parts: LoopParts<'_, '_>,
        depth: usize,
        context: Context<'_>,
        body: &mut Vec<String>,
    ) {
        let slot = self.slot(depth, body);
        let entry = self.variables.values();
        let head = self.variables.flow.block();
        let from = self.variables.flow.current();
        self.variables.flow.edge(from, head);
        self.variables.flow.enter(head);
        // `None` for `while (true)` and `for (;;)`, which become `loop`: rustc's while_true
        // lint rejects `while true`, and the head has no exit edge (flow.rs).
        let test = parts
            .condition
            .and_then(|condition| match self.condition(condition) {
                Some(RExpr::Bool(true)) => None,
                converted => {
                    let text = converted.map(|expr| printer::expr(&expr, false));
                    Some(text.unwrap_or_default())
                }
            });
        let exit = self.variables.flow.block();
        if test.is_some() {
            self.variables.flow.edge(head, exit);
        }
        let start = self.variables.flow.block();
        self.variables.flow.edge(head, start);
        self.variables.flow.enter(start);
        let empty = empty(parts.statements) && parts.steps.is_empty();
        let open = if empty { "{}" } else { "{" };
        let opening = match &test {
            Some(text) => format!("{}while {text} {open}", indent(depth)),
            None => format!("{}loop {open}", indent(depth)),
        };
        body.push(opening);
        self.variables.loops.push(Loop {
            head,
            exit,
            breaks: Vec::new(),
            steps: (!parts.steps.is_empty()).then(Vec::new),
            guarded: Vec::new(),
        });
        self.variables.left = None;
        let outer = self.variables.open(slot, true);
        for inner in parts.statements {
            self.statement(inner, depth + 1, context, body);
        }
        let falls_through = self.variables.left.is_none();
        if falls_through {
            self.variables.step_guards = self
                .variables
                .loops
                .last()
                .map(|frame| frame.guarded.clone())
                .unwrap_or_default();
            for &step in parts.steps {
                self.simple_expression(step, depth + 1, body);
            }
            self.variables.step_guards.clear();
            let end = self.variables.flow.current();
            self.variables.flow.edge(end, head);
        }
        let Some(frame) = self.variables.loops.pop() else {
            return;
        };
        let sites = frame.steps.unwrap_or_default();
        if let [step, ..] = parts.steps
            && !falls_through
            && sites.is_empty()
        {
            self.refuse(*step, "the step of this for loop never runs (P2R-40)");
        }
        // The last site first, so the line indexes of the earlier ones still hold.
        for site in sites.into_iter().rev() {
            self.variables.scope = site.scope;
            self.variables.set_values(&site.values);
            self.variables.flow.enter(site.block);
            let mut lines = Vec::new();
            self.variables.step_guards = frame.guarded.clone();
            for &step in parts.steps {
                self.simple_expression(step, site.depth, &mut lines);
            }
            self.variables.step_guards.clear();
            body.splice(site.line..site.line, lines);
        }
        self.variables.close(outer);
        if !empty {
            body.push(format!("{}}}", indent(depth)));
        }
        let mut arriving = frame.breaks;
        if test.is_some() {
            arriving.push(entry);
        }
        self.variables.flow.enter(exit);
        self.join(arriving, "a loop with no break");
    }

    /// Goes on after an `if` or a loop with the values of every path that arrives. When
    /// none arrives, the next statement is unreachable for `reason` (P2R-40, P2R-50).
    pub(super) fn join(&mut self, arriving: Vec<Values>, reason: &'static str) {
        if arriving.is_empty() {
            self.variables.left = Some(reason);
            return;
        }
        self.variables.left = None;
        self.variables.set_values(&meet(&arriving));
    }

    /// The Rust text of a condition; empty when it does not convert, because a problem is
    /// then recorded.
    fn condition_text(&mut self, condition: &Expression<'_>) -> String {
        self.condition(condition)
            .map(|expr| printer::expr(&expr, false))
            .unwrap_or_default()
    }

    /// A `for` initialisation or step, which converts as an expression statement (P2R-43).
    fn simple_expression(
        &mut self,
        expression: &Expression<'_>,
        depth: usize,
        body: &mut Vec<String>,
    ) {
        if changes_variable(expression) {
            self.assignment_statement(expression, depth, body);
        } else {
            self.expression_statement(expression, depth, body);
        }
    }

    /// True for `break` and `continue` without a level or with level 1. Another level would
    /// need a Rust loop label (P2R-40).
    fn one_level(&mut self, level: Option<&Expression<'_>>, statement: &impl HasSpan) -> bool {
        match level {
            None
            | Some(Expression::Literal(Literal::Integer(LiteralInteger {
                value: Some(1), ..
            }))) => true,
            Some(_) => {
                self.refuse(
                    statement,
                    "break and continue with a level other than 1 are not supported (P2R-40)",
                );
                false
            }
        }
    }
}

/// The parts of a `while` or `for` that run on every pass.
struct LoopParts<'a, 'arena> {
    /// `None` for `for (;;)`.
    condition: Option<&'a Expression<'arena>>,
    statements: &'a [&'a Statement<'arena>],
    steps: &'a [&'a Expression<'arena>],
}

/// The branches of an `if`. An `else if` written as two words is an else clause that
/// holds an `if`, so its branches join this chain, as Rust writes `else if`.
pub(super) fn branches<'a, 'arena>(statement: &'a If<'arena>) -> Vec<Branch<'a, 'arena>> {
    let mut branches: Vec<Branch<'a, 'arena>> = Vec::new();
    match &statement.body {
        IfBody::Statement(body) => {
            branches.push((Some(statement.condition), statements_of(body.statement)));
            for clause in body.else_if_clauses.iter() {
                branches.push((Some(clause.condition), statements_of(clause.statement)));
            }
            if let Some(clause) = &body.else_clause {
                match clause.statement {
                    Statement::If(inner) => branches.extend(self::branches(inner)),
                    other => branches.push((None, statements_of(other))),
                }
            }
        }
        IfBody::ColonDelimited(body) => {
            branches.push((Some(statement.condition), body.statements.iter().collect()));
            for clause in body.else_if_clauses.iter() {
                branches.push((Some(clause.condition), clause.statements.iter().collect()));
            }
            if let Some(clause) = &body.else_clause {
                branches.push((None, clause.statements.iter().collect()));
            }
        }
    }
    branches
}

/// The statements of a body: a braced block, or one statement without braces.
pub(super) fn statements_of<'a, 'arena>(
    statement: &'a Statement<'arena>,
) -> Vec<&'a Statement<'arena>> {
    match statement {
        Statement::Block(block) => block.statements.iter().collect(),
        other => vec![other],
    }
}

pub(super) fn empty(statements: &[&Statement<'_>]) -> bool {
    statements
        .iter()
        .all(|statement| matches!(statement, Statement::Noop(_)))
}

pub(super) fn indent(depth: usize) -> String {
    "    ".repeat(depth)
}

/// One output line: `text` at `depth` levels of indentation.
pub(super) fn emit(depth: usize, text: String) -> String {
    format!("{}{text}", indent(depth))
}

/// A value stays known only when every path agrees on it (P2R-50).
fn meet(paths: &[Values]) -> Values {
    let longest = paths.iter().map(Vec::len).max().unwrap_or(0);
    (0..longest)
        .map(|var| {
            let value = |path: &Values| path.get(var).copied().flatten();
            let first = value(&paths[0]);
            paths
                .iter()
                .all(|path| value(path) == first)
                .then_some(first)
                .flatten()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meet_keeps_only_agreed_values() {
        let paths = vec![vec![Some(1), Some(2), None], vec![Some(1), Some(3)]];
        assert_eq!(meet(&paths), vec![Some(1), None, None]);
    }
}
