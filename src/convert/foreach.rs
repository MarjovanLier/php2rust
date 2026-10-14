//! `foreach` over a list (P2R-34, P2R-35, P2R-41).
//!
//! `foreach ($list as $value)` becomes `for value in ...`, and `$key => $value` zips the
//! values with an `i64` count from 0, because a key is an int (P2R-34). A variable or an
//! index that holds a list of Copy values is borrowed, and each value comes out through
//! `&value`. Any other list is iterated by value, as PHP iterates over a copy: a variable
//! moves, or is cloned when it is read again (P2R-26).
//!
//! The list is read once, before the loop head. The head can exit at once, as for an
//! empty list, and the key and the value are new variables written at the start of the
//! body (flow.rs, variables.rs). rustfmt splits a `for` line only past 100 columns, even
//! with `.zip()` in it, so the line is measured once its pattern is known (P2R-51).

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::control::{Loop, empty, indent, statements_of};
use super::flow::Var;
use super::lists::unparenthesized;
use super::{Context, Converter, Mode, variables};
use crate::printer::{self, RExpr};
use crate::types::RustType;

impl Converter<'_> {
    pub(super) fn foreach_statement(
        &mut self,
        statement: &Foreach<'_>,
        depth: usize,
        context: Context<'_>,
        body: &mut Vec<String>,
    ) {
        let statements = match &statement.body {
            ForeachBody::Statement(inner) => statements_of(inner),
            ForeachBody::ColonDelimited(inner) => inner.statements.iter().collect(),
        };
        let (element, by_ref, iter) = self.iteration(statement);
        let lent = element.as_ref().is_some_and(|ty| self.is_changing(ty));
        let lent_mode = if self.loops.by_mut.contains(&statement.span()) {
            Mode::Mut
        } else {
            Mode::Shared
        };
        let list = holder(statement.expression).and_then(|variable| self.lookup(variable));
        self.refuse_target(statement);
        // After the list, which is read once, so the values it saw stay known (P2R-50).
        for inner in &statements {
            self.forget_writes(inner);
        }
        let slot = self.slot(depth, body);
        let entry = self.variables.values();
        let flow = &mut self.variables.flow;
        let head = flow.block();
        let from = flow.current();
        flow.edge(from, head);
        let exit = flow.block();
        flow.edge(head, exit);
        let start = flow.block();
        flow.edge(head, start);
        flow.enter(start);
        let outer = self.variables.open(slot, true);
        let key = statement.target.key();
        let key_var = key.map(|key| self.binding(key, Some(RustType::I64)));
        let value_var = self.binding(statement.target.value(), element);
        if let (true, Some(value)) = (lent, value_var) {
            self.variables.lend(value, lent_mode);
        }
        let iter = match key {
            Some(_) => format!("(0_i64..).zip({iter})"),
            None => iter,
        };
        let open = if empty(&statements) { "{}" } else { "{" };
        let rest = format!(" in {iter} {open}");
        let line = match (key_var, value_var) {
            (None, Some(value)) => self.variables.for_line(depth, None, value, by_ref, rest),
            (Some(Some(key)), Some(value)) => {
                self.variables
                    .for_line(depth, Some(key), value, by_ref, rest)
            }
            // A refused target: the conversion fails, so the line is never printed.
            _ => indent(depth),
        };
        body.push(line);
        self.variables.iterate(list);
        self.variables.loops.push(Loop::new(head, exit));
        self.variables.left = None;
        for inner in &statements {
            self.statement(inner, depth + 1, context, body);
        }
        if self.variables.left.is_none() {
            let end = self.variables.flow.current();
            self.variables.flow.edge(end, head);
        }
        let mut arriving = self
            .variables
            .loops
            .pop()
            .map(Loop::into_breaks)
            .unwrap_or_default();
        self.variables.close(outer);
        let bindings: Vec<Var> = key_var.flatten().into_iter().chain(value_var).collect();
        self.variables.end_foreach(list, &bindings, exit);
        if open == "{" {
            body.push(format!("{}}}", indent(depth)));
        }
        arriving.push(entry);
        self.variables.flow.enter(exit);
        self.join(arriving, "a foreach");
    }

    /// Converts the list of a foreach. Gives its element type, `None` when a problem is
    /// recorded, whether the loop borrows it, and its Rust text (P2R-26, P2R-41).
    fn iteration(&mut self, statement: &Foreach<'_>) -> (Option<RustType>, bool, String) {
        let list = statement.expression;
        let Some((expr, ty)) = self.expression(list) else {
            return (None, false, String::new());
        };
        let RustType::Vec(element) = &ty else {
            let message = if matches!(ty, RustType::Option(_)) {
                "foreach over a value that may be null is not supported (P2R-106)"
            } else {
                "foreach converts over a list only (P2R-41)"
            };
            self.refuse(list, message);
            return (None, false, String::new());
        };
        if self.is_changing(element) {
            let by_mut = self.loops.by_mut.contains(&statement.span());
            return match self.lent_list(list, expr, by_mut) {
                Some(iter) => (Some((**element).clone()), false, iter),
                None => (None, false, String::new()),
            };
        }
        // A Copy value comes out of a borrowed list as a plain value. A list field is
        // borrowed in place, or copied when its elements are not Copy (P2R-134), or when
        // the body changes the object that holds it, as PHP iterates a copy (P2R-146).
        let field = matches!(
            unparenthesized(list),
            Expression::Access(Access::Property(_))
        );
        if field && self.loops.copied.contains(&statement.span()) {
            let copy = RExpr::Method(Box::new(expr), "clone");
            let iter = printer::expr(&copy, true);
            return (Some((**element).clone()), false, iter);
        }
        let by_ref = (holder(list).is_some() || field) && element.is_copy();
        let expr = if by_ref {
            self.borrowed(expr, &ty)
        } else {
            match self.stored(list, expr, &ty, list.span()) {
                Some(expr) => expr,
                None => return (None, false, String::new()),
            }
        };
        let iter = printer::expr(&expr, true);
        (Some((**element).clone()), by_ref, iter)
    }

    /// The Rust text of a list of a changing type that a foreach iterates by reference:
    /// `&xs` or `&mut xs` for a list in a variable, an owned parameter or on a chain, `xs`
    /// for a `&[C]` parameter, and `xs.iter()` or `xs.iter_mut()` for a `&mut [C]`
    /// parameter (P2R-146).
    fn lent_list(&mut self, list: &Expression<'_>, expr: RExpr, by_mut: bool) -> Option<String> {
        let Some(root) = variables::chain_root(&expr).map(str::to_owned) else {
            self.refuse(
                list,
                "a foreach over a list of objects whose class changes them needs the list in a variable, a parameter or a chain of property reads (P2R-146)",
            );
            return None;
        };
        if by_mut && !self.change_root(list, &expr) {
            return None;
        }
        let slice = matches!(expr, RExpr::Var(_)) && self.variables.is_borrowed(&root);
        let iter = match (slice, by_mut) {
            // A `&mut [C]` is not Copy, so iterating it as it is would move it.
            (true, false) if self.variables.mode_of(&root) == Mode::Mut => {
                RExpr::MethodCall(Box::new(expr), "iter".to_owned(), Vec::new())
            }
            (true, false) => expr,
            (true, true) => RExpr::MethodCall(Box::new(expr), "iter_mut".to_owned(), Vec::new()),
            (false, false) => RExpr::Ref(Box::new(expr)),
            (false, true) => RExpr::RefMut(Box::new(expr)),
        };
        Some(printer::expr(&iter, false))
    }

    /// Refuses a by-reference foreach, a key or value that is not a plain variable, and a
    /// key and value with the same name. The problem covers the whole loop, so the
    /// problems its body then has are not printed too (P2R-03).
    fn refuse_target(&mut self, statement: &Foreach<'_>) {
        let (key, value) = (statement.target.key(), statement.target.value());
        if matches!(value, Expression::UnaryPrefix(unary)
            if matches!(unary.operator, UnaryPrefixOperator::Reference(_)))
        {
            self.refuse(
                statement,
                "a by-reference foreach is not supported (P2R-41)",
            );
        } else if key.is_some_and(|key| target(key).is_none()) || target(value).is_none() {
            self.refuse(
                statement,
                "only a plain variable can be a foreach key or value (P2R-07)",
            );
        } else if let (Some(key), Some(value)) = (key.and_then(target), target(value))
            && key.name == value.name
        {
            // PHP writes the key after the value, so the variable holds the key.
            self.refuse(
                statement,
                "a foreach key and value with the same name are not supported (P2R-41)",
            );
        }
    }

    /// The variable of a foreach key or value, declared with the type `ty`, or `None` for
    /// a target that is not a plain variable, whose writes still count (P2R-03).
    fn binding(&mut self, expression: &Expression<'_>, ty: Option<RustType>) -> Option<Var> {
        match target(expression) {
            Some(variable) => Some(self.bind(variable, ty)),
            None => {
                self.record_refused_target(expression);
                None
            }
        }
    }
}

/// The variable a foreach key or value assigns: `$name`, or `&$name`, which is refused.
fn target<'a, 'arena>(expression: &'a Expression<'arena>) -> Option<&'a DirectVariable<'arena>> {
    match expression {
        Expression::Variable(Variable::Direct(variable)) => Some(variable),
        Expression::UnaryPrefix(unary)
            if matches!(unary.operator, UnaryPrefixOperator::Reference(_)) =>
        {
            target(unary.operand)
        }
        _ => None,
    }
}

/// The variable that holds the list a foreach iterates: `$a` for `$a` and `$a[$i]`. `None`
/// for a list that a literal or a call gives (P2R-41).
fn holder<'a, 'arena>(expression: &'a Expression<'arena>) -> Option<&'a DirectVariable<'arena>> {
    match expression {
        Expression::Parenthesized(inner) => holder(inner.expression),
        Expression::Variable(Variable::Direct(variable)) => Some(variable),
        Expression::ArrayAccess(access) => holder(access.array),
        _ => None,
    }
}
