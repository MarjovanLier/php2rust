//! A function or method that calls itself on every path (P2R-50, P2R-135).
//!
//! rustc's unconditional_recursion lint, an error under -D warnings, walks a function's
//! control flow from its start. It stays quiet once some path returns, or comes back
//! round a loop, before it calls the function itself; a call cuts the path. Only direct
//! calls count. This check walks the PHP the same way, with the loop shapes of
//! control.rs: `while (true)` and `for (;;)` have no exit at the head, and a `for` step
//! runs before the next pass. A call in the right operand of `&&` or `||` runs on some
//! paths only, so it does not cut them.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use super::control::{branches, statements_of};

/// The ways control leaves statements on a path that has not called the function.
#[derive(Default)]
struct Exits {
    /// Goes on to the next statement.
    falls: bool,
    breaks: bool,
    continues: bool,
    /// Returns, or comes back round a loop, so rustc does not report recursion.
    ends: bool,
}

impl Exits {
    fn add(&mut self, other: Exits) {
        self.falls |= other.falls;
        self.breaks |= other.breaks;
        self.continues |= other.continues;
        self.ends |= other.ends;
    }
}

/// True when every path through a function or method body with these statements calls
/// the function or method itself (P2R-50, P2R-135).
pub(super) fn always_recurses(statements: &[&Statement<'_>], target: &Target<'_>) -> bool {
    let exits = sequence(statements, target);
    !exits.falls && !exits.ends
}

/// What counts as a call of the body itself, and the left-out `default` arms, which
/// never run (P2R-121).
pub(super) struct Target<'t> {
    /// A function's name: a call by that name, in any letter case, counts.
    pub name: Option<&'t [u8]>,
    /// A method's calls of itself, found when the body converted: rustc counts a call
    /// through any object of the class, not only through `self`.
    pub self_calls: &'t [Span],
    pub dropped: &'t [Span],
}

fn sequence(statements: &[&Statement<'_>], target: &Target<'_>) -> Exits {
    let mut exits = Exits::default();
    for statement in statements {
        let next = one(statement, target);
        let falls = next.falls;
        exits.add(Exits {
            falls: false,
            ..next
        });
        if !falls {
            return exits;
        }
    }
    exits.falls = true;
    exits
}

fn one(statement: &Statement<'_>, target: &Target<'_>) -> Exits {
    let falls = Exits {
        falls: true,
        ..Exits::default()
    };
    match statement {
        Statement::Echo(echo) if echo.values.iter().any(|value| calls(value, target)) => {
            Exits::default()
        }
        Statement::Expression(expression) if calls(expression.expression, target) => {
            Exits::default()
        }
        Statement::Return(ret) => Exits {
            ends: !ret.value.is_some_and(|value| calls(value, target)),
            ..Exits::default()
        },
        Statement::Break(_) => Exits {
            breaks: true,
            ..Exits::default()
        },
        Statement::Continue(_) => Exits {
            continues: true,
            ..Exits::default()
        },
        Statement::If(statement) => {
            let branches = branches(statement);
            let mut exits = Exits::default();
            for (condition, statements) in &branches {
                // A condition that calls cuts its branch and every later one.
                if condition.is_some_and(|condition| calls(condition, target)) {
                    return exits;
                }
                exits.add(sequence(statements, target));
            }
            // Without an else, control goes on when every condition is false.
            exits.falls |= branches
                .last()
                .is_some_and(|(condition, _)| condition.is_some());
            exits
        }
        Statement::While(statement) => {
            let statements = match &statement.body {
                WhileBody::Statement(inner) => statements_of(inner),
                WhileBody::ColonDelimited(inner) => inner.statements.iter().collect(),
            };
            let condition = Some(statement.condition);
            looped(condition, &statements, &[], target)
        }
        Statement::For(statement) => {
            if statement
                .initializations
                .iter()
                .any(|&init| calls(init, target))
            {
                return Exits::default();
            }
            let conditions: Vec<&Expression<'_>> = statement.conditions.iter().copied().collect();
            if conditions.iter().any(|&condition| calls(condition, target)) {
                return Exits::default();
            }
            let statements = match &statement.body {
                ForBody::Statement(inner) => statements_of(inner),
                ForBody::ColonDelimited(inner) => inner.statements.iter().collect(),
            };
            let steps: Vec<&Expression<'_>> = statement.increments.iter().copied().collect();
            looped(conditions.last().copied(), &statements, &steps, target)
        }
        Statement::Foreach(statement) => {
            if calls(statement.expression, target) {
                return Exits::default();
            }
            let statements = match &statement.body {
                ForeachBody::Statement(inner) => statements_of(inner),
                ForeachBody::ColonDelimited(inner) => inner.statements.iter().collect(),
            };
            // The head exits at the end of the list.
            let body = sequence(&statements, target);
            Exits {
                falls: true,
                ends: body.ends || body.falls || body.continues,
                ..Exits::default()
            }
        }
        _ => falls,
    }
}

/// A `while` or `for` loop once its initialisations ran. `condition` is `None` for
/// `for (;;)`.
fn looped(
    condition: Option<&Expression<'_>>,
    statements: &[&Statement<'_>],
    steps: &[&Expression<'_>],
    target: &Target<'_>,
) -> Exits {
    if condition.is_some_and(|condition| calls(condition, target)) {
        return Exits::default();
    }
    // `while (true)` and `for (;;)` become `loop`, whose head has no exit.
    let exits_at_head = condition.is_some_and(|condition| !always_true(condition));
    let body = sequence(statements, target);
    let stepped = steps.iter().any(|&step| calls(step, target));
    Exits {
        falls: exits_at_head || body.breaks,
        ends: body.ends || ((body.falls || body.continues) && !stepped),
        ..Exits::default()
    }
}

/// True for the literal `true`, which control.rs turns into `loop`.
fn always_true(expression: &Expression<'_>) -> bool {
    match expression {
        Expression::Parenthesized(inner) => always_true(inner.expression),
        Expression::Literal(Literal::True(_)) => true,
        _ => false,
    }
}

/// True when the expression calls the function `target` names whenever it runs.
fn calls(expression: &Expression<'_>, target: &Target<'_>) -> bool {
    let mut found = false;
    Calls(target).walk_expression(expression, &mut found);
    found
}

struct Calls<'c, 't>(&'c Target<'t>);

impl<'ast, 'arena> Walker<'ast, 'arena, bool> for Calls<'_, '_> {
    fn walk_in_function_call(&self, call: &'ast FunctionCall<'arena>, found: &mut bool) {
        if let (Expression::Identifier(Identifier::Local(local)), Some(name)) =
            (call.function, self.0.name)
            && local.value.eq_ignore_ascii_case(name)
        {
            *found = true;
        }
    }

    fn walk_in_method_call(&self, call: &'ast MethodCall<'arena>, found: &mut bool) {
        if self.0.self_calls.contains(&call.span()) {
            *found = true;
        }
    }

    fn walk_match(&self, r#match: &'ast Match<'arena>, found: &mut bool) {
        self.walk_expression(r#match.expression, found);
        // Only one arm runs, so the arms call on every path only when each Rust arm does;
        // a left-out default is no Rust arm (P2R-121).
        let arms: Vec<&Expression<'_>> = r#match
            .arms
            .iter()
            .map(|arm| match arm {
                MatchArm::Expression(arm) => arm.expression,
                MatchArm::Default(arm) => arm.expression,
            })
            .filter(|result| !self.0.dropped.contains(&result.span()))
            .collect();
        if !arms.is_empty() && arms.iter().all(|result| calls(result, self.0)) {
            *found = true;
        }
    }

    fn walk_binary(&self, binary: &'ast Binary<'arena>, found: &mut bool) {
        self.walk_expression(binary.lhs, found);
        // The right operand of `&&` and `||` runs on some paths only.
        if !matches!(
            binary.operator,
            BinaryOperator::And(_) | BinaryOperator::Or(_)
        ) {
            self.walk_expression(binary.rhs, found);
        }
    }
}
