//! `match` as the whole value of a `return` or an assignment (P2R-120 to P2R-124).

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::lists::unparenthesized;
use super::variables::COPY;
use super::{Converter, nulls};
use crate::printer::{self, RExpr};
use crate::types::RustType;

/// rustfmt keeps an arm's pattern on one line while it ends by this column, which leaves
/// room for ` => {` (P2R-157).
const PATTERN_WIDTH: usize = 95;

/// An arm whose value has converted, before the `return` or assignment finishes it.
struct Arm<'a, 'arena> {
    /// The conditions, `["1", "2"]` for `1 | 2`, or `["_"]` for `default`.
    pattern: Vec<String>,
    node: &'a Expression<'arena>,
    expr: RExpr,
    ty: RustType,
}

/// A converted `match`: its subject as Rust text and its arms, `_` last.
struct Converted<'a, 'arena> {
    subject: String,
    arms: Vec<Arm<'a, 'arena>>,
}

impl Converter<'_> {
    /// `return match (...) {...}` in a function that returns `returns` (P2R-120).
    pub(super) fn match_return(
        &mut self,
        node: &Match<'_>,
        returns: &RustType,
        depth: usize,
        body: &mut Vec<String>,
    ) {
        let Some(converted) = self.match_arms(node, Some(returns)) else {
            return;
        };
        let mut lines = Vec::new();
        let mut fine = true;
        for arm in converted.arms {
            if nulls::null_for_plain(&arm.ty, returns) {
                self.refuse(
                    arm.node,
                    "a value that may be null cannot be returned where the return type is not nullable (P2R-106)",
                );
                fine = false;
                continue;
            }
            let rendered = self
                .declared_value(arm.node, arm.expr, &arm.ty, returns)
                .and_then(|expr| self.kept(arm.node, expr, &arm.ty))
                .map(|expr| nulls::wrapped(expr, &arm.ty, returns))
                .and_then(|expr| self.render(arm.node, &expr, false));
            match rendered {
                Some(text) => lines.push((arm.node, arm.pattern, text)),
                None => fine = false,
            }
        }
        if fine {
            let open = format!("return match {} {{", converted.subject);
            let open = self.emit(node, depth, open);
            self.push_match(open, lines, depth, body);
        }
    }

    /// `$v = match (...) {...}` as a statement (P2R-120). False when `expression` is not
    /// one, so it converts as any other assignment.
    pub(super) fn match_assignment(
        &mut self,
        expression: &Expression<'_>,
        depth: usize,
        body: &mut Vec<String>,
    ) -> bool {
        let Expression::Assignment(assignment) = expression else {
            return false;
        };
        let (
            AssignmentOperator::Assign(_),
            Expression::Variable(Variable::Direct(variable)),
            Expression::Match(node),
        ) = (
            &assignment.operator,
            assignment.lhs,
            unparenthesized(assignment.rhs),
        )
        else {
            return false;
        };
        // A `null` or `[]` arm takes the variable's type, or the type Mago gives the whole
        // `match` for a new variable (P2R-21, P2R-102).
        let expected = self
            .lookup(variable)
            .and_then(|var| self.variables.ty(var).cloned())
            .or_else(|| self.site_type(node).ok());
        let Some(converted) = self.match_arms(node, expected.as_ref()) else {
            self.record_write(variable);
            return true;
        };
        // The value's type for the write: an `Option` arm makes every arm one (P2R-102).
        let assigned = converted
            .arms
            .iter()
            .map(|arm| &arm.ty)
            .find(|ty| matches!(ty, RustType::Option(_)))
            .or(converted.arms.first().map(|arm| &arm.ty))
            .cloned();
        let Some(assigned) = assigned else {
            // A `match` without arms, which Mago reports (P2R-11).
            self.record_write(variable);
            return true;
        };
        let other = converted
            .arms
            .iter()
            .find(|arm| arm.ty != assigned && arm.ty != *assigned.inner());
        if let Some(arm) = other {
            self.record_write(variable);
            self.refuse(
                arm.node,
                "the arms of a match must give one Rust type (P2R-122)",
            );
            return true;
        }
        let Some((var, declares)) = self.write(variable, expression, Some(&assigned)) else {
            return true;
        };
        let target = self.variables.ty(var).cloned();
        // rustc works out the value of a `match` with one arm, so the tool keeps it as for a
        // plain assignment, and a later overflow it proves stops (P2R-50).
        if let ([arm], Some(RustType::I64)) = (converted.arms.as_slice(), &target) {
            self.variables
                .set_value(var, printer::literal_value(&arm.expr));
        }
        let mut lines = Vec::new();
        let mut anchored = false;
        for arm in converted.arms {
            let Some(stored) = self.stored(arm.node, arm.expr, &arm.ty, expression.span()) else {
                return true;
            };
            let expr = match &target {
                Some(target) => nulls::wrapped(stored, &arm.ty, target),
                None => stored,
            };
            anchored |= printer::anchored(&expr);
            let Some(text) = self.render(arm.node, &expr, false) else {
                return true;
            };
            lines.push((arm.node, arm.pattern, text));
        }
        let open = format!("match {} {{", converted.subject);
        let open = if declares {
            // No arm fixes the type, so the `let` names it, as for a literal (P2R-49).
            let pending = self.first_write(var, !anchored, open, true);
            format!("{}{pending}", "    ".repeat(depth))
        } else {
            let line = format!("{} = {open}", self.variables.name_of(var));
            self.emit(expression, depth, line)
        };
        self.push_match(open, lines, depth, body);
        true
    }

    /// Writes the opening line, the arms one level deeper and `};` (P2R-123). An arm that
    /// holds a copy mark is measured at the end of the body, once the mark is resolved. A
    /// pattern of several conditions that ends after column 95 goes one condition per line,
    /// as rustfmt writes it (P2R-157).
    fn push_match(
        &mut self,
        open: String,
        arms: Vec<(&Expression<'_>, Vec<String>, String)>,
        depth: usize,
        body: &mut Vec<String>,
    ) {
        body.push(open);
        let indent = "    ".repeat(depth + 1);
        for (node, pattern, value) in arms {
            let joined = pattern.join(" | ");
            let wide = indent.len() + joined.len() > PATTERN_WIDTH;
            if wide && !joined.is_ascii() {
                // rustfmt counts columns and the tool counts bytes, so the layout rustfmt
                // picks is not known.
                self.refuse(
                    node,
                    "a match pattern with non-ASCII text longer than 95 bytes is not supported (P2R-157)",
                );
                continue;
            }
            // rustfmt packs a long pattern of short literals, each at most 20 columns,
            // several to a line, a layout the tool does not write.
            if wide && pattern.iter().all(|p| p.len() <= 20 && !p.contains("::")) {
                self.refuse(
                    node,
                    "a long match pattern of short literals is not supported (P2R-157)",
                );
                continue;
            }
            let (last, earlier) = match pattern.split_last() {
                Some((last, earlier)) if wide && !earlier.is_empty() => (last.clone(), earlier),
                _ => (joined, &[][..]),
            };
            for (at, condition) in earlier.iter().enumerate() {
                let line = if at == 0 {
                    condition.clone()
                } else {
                    format!("| {condition}")
                };
                body.push(self.emit(node, depth + 1, line));
            }
            let lead = if earlier.is_empty() { "" } else { "| " };
            let line = format!("{lead}{last} => {value},");
            let line = if line.contains(COPY) {
                format!("{indent}{line}")
            } else {
                self.emit(node, depth + 1, line)
            };
            body.push(line);
        }
        body.push(format!("{}}};", "    ".repeat(depth)));
    }

    /// Converts the subject and the arms, with `expected` as the type of a `null` or `[]`
    /// arm. Each arm is its own path of the flow graph (P2R-124). `None` when a problem is
    /// recorded.
    fn match_arms<'a, 'arena>(
        &mut self,
        node: &'a Match<'arena>,
        expected: Option<&RustType>,
    ) -> Option<Converted<'a, 'arena>> {
        let subject = self.match_subject(node.expression);
        let mut fine = subject.is_some();
        let mut patterns: Vec<String> = Vec::new();
        let mut results: Vec<(Vec<String>, &'a Expression<'arena>, bool)> = Vec::new();
        let mut default: Option<&'a Expression<'arena>> = None;
        for arm in node.arms.iter() {
            match arm {
                MatchArm::Default(arm) if default.is_some() => {
                    self.record_refused(arm.expression);
                    self.refuse(
                        arm,
                        "a match can have one default arm only, and PHP rejects a second (P2R-120)",
                    );
                    fine = false;
                }
                MatchArm::Default(arm) => default = Some(arm.expression),
                MatchArm::Expression(arm) => {
                    let mut alternatives = Vec::new();
                    let mut converted = true;
                    for &condition in arm.conditions.iter() {
                        let pattern = subject
                            .as_ref()
                            .and_then(|(_, ty)| self.match_pattern(condition, ty));
                        match pattern {
                            Some(pattern) if patterns.contains(&pattern) => {
                                self.refuse(
                                    condition,
                                    "this condition has the value of an earlier one, and Rust rejects it as unreachable (P2R-121)",
                                );
                                converted = false;
                            }
                            Some(pattern) => {
                                patterns.push(pattern.clone());
                                alternatives.push(pattern);
                            }
                            None => converted = false,
                        }
                    }
                    fine &= converted;
                    results.push((alternatives, arm.expression, converted));
                }
            }
        }
        let covered = subject.as_ref().map(|(_, ty)| self.covered(ty, &patterns));
        match (default, covered) {
            // The other arms cover every value, so Rust rejects `_` and the default never
            // runs: it is not converted and its reads do not count (P2R-121), nor its
            // calls (P2R-50).
            (Some(result), Some(Ok(()))) => self.dropped_defaults.push(result.span()),
            (Some(result), _) => results.push((vec!["_".to_owned()], result, true)),
            (None, Some(Err(message))) => {
                self.refuse(node, message);
                fine = false;
            }
            (None, _) => {}
        }
        let start = self.variables.flow.current();
        let join = self.variables.flow.block();
        let mut arms = Vec::new();
        for (pattern, result, converted) in results {
            let block = self.variables.flow.block();
            self.variables.flow.edge(start, block);
            self.variables.flow.enter(block);
            if !converted {
                // The condition's problem is the one reported for this arm.
                self.record_refused(result);
            } else if self.artifacts.get_expression_type(result).is_none() {
                self.record_refused(result);
                self.refuse(
                    result,
                    "Mago finds this arm unreachable and gives its value no type (P2R-120)",
                );
                fine = false;
            } else if let Some((expr, ty)) = self.value(result, expected) {
                arms.push(Arm {
                    pattern,
                    node: result,
                    expr,
                    ty,
                });
            } else {
                fine = false;
            }
            let end = self.variables.flow.current();
            self.variables.flow.edge(end, join);
        }
        self.variables.flow.enter(join);
        let (subject, _) = subject?;
        fine.then_some(Converted { subject, arms })
    }

    /// The subject as Rust text, with its type. A string compares as `&str` (P2R-122).
    fn match_subject(&mut self, subject: &Expression<'_>) -> Option<(String, RustType)> {
        let (expr, ty) = self.expression(subject)?;
        let problem = match &ty {
            RustType::I64 | RustType::Bool | RustType::String => None,
            RustType::Enum(name) if self.case_paths(name).is_empty() => Some(
                "a match on an enum without cases never runs an arm, and rustc rejects the code around it (P2R-120)",
            ),
            RustType::Enum(_) => None,
            RustType::Option(_) => {
                Some("a value that may be null cannot be the subject of a match (P2R-106)")
            }
            _ => Some("the subject of a match must be an int, bool, string or enum (P2R-120)"),
        };
        if let Some(problem) = problem {
            self.refuse(subject, problem);
            return None;
        }
        let expr = if ty == RustType::String && !self.is_reference(&expr) {
            RExpr::Method(Box::new(expr), "as_str")
        } else {
            expr
        };
        // Nothing else types an int subject of literals only, which Rust makes `i32`
        // (P2R-49).
        let anchor = ty == RustType::I64 && !printer::anchored(&expr);
        let text = self.render(subject, &expr, anchor)?;
        Some((text, ty))
    }

    /// The Rust pattern of a condition on a subject of type `subject`: a literal of that
    /// type, or a case of its enum (P2R-120).
    fn match_pattern(&mut self, condition: &Expression<'_>, subject: &RustType) -> Option<String> {
        let int = |expression: &Expression<'_>| match unparenthesized(expression) {
            Expression::Literal(Literal::Integer(integer)) => {
                integer.value.and_then(|v| i64::try_from(v).ok())
            }
            _ => None,
        };
        let pattern = match (unparenthesized(condition), subject) {
            (Expression::UnaryPrefix(unary), RustType::I64)
                if matches!(unary.operator, UnaryPrefixOperator::Negation(_)) =>
            {
                int(unary.operand)
                    .and_then(i64::checked_neg)
                    .map(|value| value.to_string())
            }
            (other, RustType::I64) => int(other).map(|value| value.to_string()),
            (Expression::Literal(Literal::True(_)), RustType::Bool) => Some("true".to_owned()),
            (Expression::Literal(Literal::False(_)), RustType::Bool) => Some("false".to_owned()),
            (Expression::Literal(Literal::String(string)), RustType::String) => {
                // A string escape the tool cannot read records its own problem.
                return self
                    .string_literal(string)
                    .map(|text| printer::quote(&text));
            }
            (Expression::Access(Access::ClassConstant(access)), RustType::Enum(name)) => self
                .enum_case_of(access)
                .filter(|(found, _)| found == name)
                .map(|(found, case)| format!("{found}::{case}")),
            _ => None,
        };
        if pattern.is_none() {
            self.refuse(
                condition,
                "a match condition must be a literal of the subject's type or a case of its enum (P2R-120)",
            );
        }
        pattern
    }

    /// Whether the patterns cover every value of the subject's Rust type, judged from the
    /// type and the enum's declared cases, never from the narrowed type Mago gives the
    /// subject (P2R-121). The error is the problem of a `match` without `default`.
    fn covered(&self, subject: &RustType, patterns: &[String]) -> Result<(), String> {
        let has = |pattern: &str| patterns.iter().any(|p| p == pattern);
        match subject {
            RustType::Bool if has("true") && has("false") => Ok(()),
            RustType::Bool => Err(
                "a match on a bool without default must have both true and false (P2R-121)"
                    .to_owned(),
            ),
            RustType::Enum(name) => match self.case_paths(name).into_iter().find(|path| !has(path))
            {
                None => Ok(()),
                Some(path) => Err(format!(
                    "a match on an enum without default must have every case, and {path} is missing (P2R-121)"
                )),
            },
            _ => Err("a match on an int or string must have a default arm (P2R-121)".to_owned()),
        }
    }
}
