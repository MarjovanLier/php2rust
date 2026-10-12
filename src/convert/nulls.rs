//! Values that may be null: `Some` where an `Option` is needed, comparisons with `null`
//! and between `Option` values, and `??` (P2R-102, P2R-103, P2R-105).

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::{Converter, booleans, lists};
use crate::printer::{BinOp, RExpr};
use crate::types::RustType;

/// `expr`, of type `ty`, where a value of type `target` is needed: a `T` value becomes
/// `Some(..)` where an `Option<T>` is needed (P2R-102).
pub(super) fn wrapped(expr: RExpr, ty: &RustType, target: &RustType) -> RExpr {
    if matches!(target, RustType::Option(_)) && !matches!(ty, RustType::Option(_)) {
        RExpr::Some(Box::new(expr))
    } else {
        expr
    }
}

/// True for the literal `null`.
pub(super) fn is_null(expression: &Expression<'_>) -> bool {
    matches!(
        lists::unparenthesized(expression),
        Expression::Literal(Literal::Null(_))
    )
}

/// True when a value that may be null goes where a value that cannot be null is needed
/// (P2R-106).
pub(super) fn null_for_plain(ty: &RustType, target: &RustType) -> bool {
    matches!(ty, RustType::Option(_)) && !matches!(target, RustType::Option(_))
}

/// True for a value with no side effect that cannot panic, so the default of `??` may run
/// whether or not it is needed (P2R-105).
fn simple(expr: &RExpr) -> bool {
    match expr {
        RExpr::Int(_)
        | RExpr::Float(_)
        | RExpr::Bool(_)
        | RExpr::Str(_)
        | RExpr::Var(_)
        | RExpr::Known(..) => true,
        RExpr::Neg(inner) => matches!(**inner, RExpr::Int(_) | RExpr::Float(_)),
        _ => false,
    }
}

/// The side of `binary` that is not the literal `null`, when exactly one side is.
fn beside_null<'e, 'a>(binary: &'e Binary<'a>) -> Option<&'e Expression<'a>> {
    match (is_null(binary.lhs), is_null(binary.rhs)) {
        (true, false) => Some(binary.rhs),
        (false, true) => Some(binary.lhs),
        _ => None,
    }
}

impl Converter<'_> {
    /// A comparison with the literal `null`: `=== null` becomes `is_none()` and `!== null`
    /// `is_some()` (P2R-103). `None` when no side is `null`, so the comparison converts as
    /// any other.
    pub(super) fn null_comparison(
        &mut self,
        binary: &Binary<'_>,
        op: BinOp,
        strict: bool,
    ) -> Option<Option<(RExpr, RustType)>> {
        let value = beside_null(binary)?;
        if !strict || !matches!(op, BinOp::Eq | BinOp::Ne) {
            self.record_refused(value);
            self.refuse(
                binary,
                "only === and !== compare with null, because PHP's null == 0 is true (P2R-103)",
            );
            return Some(None);
        }
        // A value that does not convert has its problem recorded already.
        let Some((expr, ty)) = self.expression(value) else {
            return Some(None);
        };
        if !matches!(ty, RustType::Option(_)) {
            self.refuse(
                binary,
                "this value cannot be null, so the comparison with null always gives the same result (P2R-103)",
            );
            return Some(None);
        }
        let test = if op == BinOp::Eq {
            "is_none"
        } else {
            "is_some"
        };
        Some(Some((RExpr::Method(Box::new(expr), test), RustType::Bool)))
    }

    /// `===` and `!==` where one side or both may be null: both sides compare as `Option`
    /// values, a string side as `Option<&str>` (P2R-103).
    pub(super) fn option_comparison(
        &mut self,
        binary: &Binary<'_>,
        op: BinOp,
        strict: bool,
        (left, left_type): (RExpr, RustType),
        (right, right_type): (RExpr, RustType),
    ) -> Option<(RExpr, RustType)> {
        if !strict || !matches!(op, BinOp::Eq | BinOp::Ne) {
            self.refuse(
                binary,
                "only === and !== compare values that may be null (P2R-103)",
            );
            return None;
        }
        let (inner, other) = (left_type.inner(), right_type.inner());
        if let Some(message) = booleans::refusal(op, strict, inner, other) {
            self.refuse(binary, message);
            return None;
        }
        let left = self.compared(left, &left_type);
        let right = self.compared(right, &right_type);
        Some((
            RExpr::Binary(Box::new(left), op, Box::new(right)),
            RustType::Bool,
        ))
    }

    /// `$a ?? $b`, where `$a` is a variable, parameter or call that may be null and `$b`
    /// has its inner type. `$b` runs only when `$a` is null, as in PHP (P2R-105).
    pub(super) fn coalesce(&mut self, binary: &Binary<'_>) -> Option<(RExpr, RustType)> {
        let left_node = lists::unparenthesized(binary.lhs);
        if !matches!(
            left_node,
            Expression::Variable(Variable::Direct(_)) | Expression::Call(Call::Function(_))
        ) {
            self.record_refused(binary.lhs);
            self.record_refused(binary.rhs);
            self.refuse(
                binary,
                "?? converts on a variable, parameter or call only; PHP checks anything else with isset (P2R-105)",
            );
            return None;
        }
        // Both sides convert first, so problems on both sides are reported.
        let left = self.expression(binary.lhs);
        let right = self.expression(binary.rhs);
        let ((left, left_type), (right, right_type)) = (left?, right?);
        let RustType::Option(inner) = &left_type else {
            self.refuse(binary, "the left side of ?? cannot be null here (P2R-105)");
            return None;
        };
        if right_type != **inner {
            self.refuse(
                binary,
                format!(
                    "the right side of ?? must have the type {} here (P2R-105)",
                    inner.owned()
                ),
            );
            return None;
        }
        let lazy = !simple(&right);
        let expr = if inner.is_copy() {
            RExpr::Unwrap {
                option: Box::new(left),
                default: Box::new(right),
                lazy,
            }
        } else if **inner == RustType::String && matches!(right, RExpr::Str(_) | RExpr::Var(_)) {
            // Both sides are borrowed, so neither is copied. The result is a `&str`, which
            // `owned` copies where a `String` is needed.
            RExpr::Unwrap {
                option: Box::new(self.borrowed(left, &left_type)),
                default: Box::new(self.borrowed(right, &right_type)),
                lazy: false,
            }
        } else {
            // `unwrap_or` takes the option, so a variable moves unless it is read again.
            let option = match (left, left_node) {
                (RExpr::Var(name), _) if self.variables.is_parameter(&name) => {
                    RExpr::MapOwned(Box::new(RExpr::Var(name)))
                }
                (RExpr::Var(name), Expression::Variable(Variable::Direct(variable))) => {
                    let mark = self.variables.copy_mark(variable.span, binary.span());
                    RExpr::Var(format!("{name}{mark}"))
                }
                (other, _) => other,
            };
            let default = self.stored(binary.rhs, right, &right_type, binary.span());
            RExpr::Unwrap {
                option: Box::new(option),
                default: Box::new(default),
                lazy,
            }
        };
        Some((expr, (**inner).clone()))
    }

    /// True for the `&str` that `??` gives when both sides are borrowed (P2R-105).
    pub(super) fn lends(&self, expr: &RExpr) -> bool {
        let RExpr::Unwrap { option, .. } = expr else {
            return false;
        };
        match &**option {
            RExpr::Method(_, "as_deref") => true,
            RExpr::Var(name) => self.variables.is_parameter(name),
            _ => false,
        }
    }

    /// One side of an `Option` comparison as an `Option`, with a string inside borrowed.
    fn compared(&self, expr: RExpr, ty: &RustType) -> RExpr {
        match ty {
            RustType::Option(inner) if **inner == RustType::String => self.borrowed(expr, ty),
            RustType::Option(_) => expr,
            RustType::String => {
                let text = match expr {
                    RExpr::Str(_) => expr,
                    RExpr::Var(ref name) if self.variables.is_parameter(name) => expr,
                    other => RExpr::Method(Box::new(other), "as_str"),
                };
                RExpr::Some(Box::new(text))
            }
            _ => RExpr::Some(Box::new(expr)),
        }
    }
}
