//! Numbers: floats and the operand rules of arithmetic (P2R-20, P2R-43, P2R-44, P2R-49).
//!
//! Float arithmetic never panics and rustc rejects none of it, so only int arithmetic
//! goes through the P2R-50 checks of expressions.rs.

use crate::printer::BinOp;
use crate::types::RustType;

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
