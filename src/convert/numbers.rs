//! Numbers: floats, the operand rules of arithmetic, and ints where a float is declared
//! (P2R-20, P2R-25, P2R-43, P2R-44, P2R-49).
//!
//! Float arithmetic never panics and rustc rejects none of it, so only int arithmetic
//! goes through the P2R-50 checks of expressions.rs.

use crate::printer::{BinOp, RExpr};
use crate::types::RustType;

/// An int value where a float is declared, as a parameter or a return type, becomes a
/// float: a literal by its value (`4` to `4.0`), any other value with `as f64` (P2R-25).
/// Every other value is given back as it is.
pub(super) fn widened(expr: RExpr, ty: &RustType, declared: &RustType) -> RExpr {
    if *ty != RustType::I64 || *declared != RustType::F64 {
        return expr;
    }
    // PHP rounds an int to the nearest float, as `as` does. The value, not the text, also
    // keeps `-0` a positive zero, which `fdiv(1, -0)` can see.
    match int_literal(&expr) {
        Some(value) => RExpr::Float(value as f64),
        None => RExpr::ToFloat(Box::new(expr)),
    }
}

/// The value of an int literal, with any unary minus in front of it.
fn int_literal(expr: &RExpr) -> Option<i64> {
    match expr {
        RExpr::Int(value) => Some(*value),
        RExpr::Neg(inner) => int_literal(inner)?.checked_neg(),
        _ => None,
    }
}

/// Why arithmetic on these operand types does not convert, or `None` when it does: both
/// int, or both float except for `%` (P2R-43, P2R-44).
pub(super) fn arithmetic_refusal(
    op: BinOp,
    left: &RustType,
    right: &RustType,
) -> Option<&'static str> {
    match (left, right) {
        (RustType::I64, RustType::I64) => None,
        (RustType::F64, RustType::F64) if op != BinOp::Rem => None,
        (RustType::F64, RustType::F64) => Some("% on a float is not supported (P2R-43)"),
        (RustType::I64, RustType::F64) | (RustType::F64, RustType::I64) => {
            Some("arithmetic that mixes int and float is not supported (P2R-44)")
        }
        _ => Some("arithmetic needs two int or two float operands (P2R-43)"),
    }
}
