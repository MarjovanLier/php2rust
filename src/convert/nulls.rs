//! Values that may be null: `Some` where an `Option` is needed, and comparisons with
//! `null` and between `Option` values (P2R-102, P2R-103).

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

/// True when a value that may be null goes where a value that cannot be null is needed
/// (P2R-106).
pub(super) fn null_for_plain(ty: &RustType, target: &RustType) -> bool {
    matches!(ty, RustType::Option(_)) && !matches!(target, RustType::Option(_))
}

/// The side of `binary` that is not the literal `null`, when exactly one side is.
fn beside_null<'e, 'a>(binary: &'e Binary<'a>) -> Option<&'e Expression<'a>> {
    let null = |e: &Expression<'_>| {
        matches!(
            lists::unparenthesized(e),
            Expression::Literal(Literal::Null(_))
        )
    };
    match (null(binary.lhs), null(binary.rhs)) {
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
        let (expr, ty) = self.expression(value)?;
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
