//! Numbers: floats, the operand rules of arithmetic, ints where a float is declared, and
//! `intdiv` and `fdiv` (P2R-20, P2R-25, P2R-43, P2R-44, P2R-48 to P2R-50).
//!
//! Float arithmetic never panics and rustc rejects none of it, so only int arithmetic
//! goes through the P2R-50 checks of expressions.rs.

use mago_syntax::cst::*;

use super::Converter;
use crate::printer::{self, BinOp, RExpr};
use crate::types::RustType;

impl Converter<'_> {
    /// `intdiv()`: `/` on two ints, which Rust rounds towards zero as PHP does (P2R-48).
    /// Mago reports an argument that is not an int (P2R-11).
    pub(super) fn intdiv(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let ((left, _), (right, _)) = self.operands(call, node, "intdiv")?;
        // P2R-50 stops intdiv by a divisor the tool can prove is 0, as for `%`, and
        // checked_constant stops PHP_INT_MIN by -1. rustc rejects both at compile time
        // (unconditional_panic).
        if printer::literal_value(&right) == Some(0) {
            self.refuse(node, "intdiv by zero always fails (P2R-50)");
            return None;
        }
        self.checked_constant(node, division(left, right))
    }

    /// `fdiv()`: `/` on two floats, which is IEEE division in Rust as in PHP, so a
    /// division by zero gives an infinity or NaN (P2R-48). An int argument becomes a
    /// float (P2R-25).
    pub(super) fn fdiv(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let ((left, left_type), (right, right_type)) = self.operands(call, node, "fdiv")?;
        let left = widened(left, &left_type, &RustType::F64);
        let right = widened(right, &right_type, &RustType::F64);
        Some((division(left, right), RustType::F64))
    }

    /// The two arguments of `intdiv` or `fdiv`, converted.
    fn operands(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
        name: &str,
    ) -> Option<((RExpr, RustType), (RExpr, RustType))> {
        let (left, right) = match call.argument_list.arguments.as_slice() {
            [Argument::Positional(left), Argument::Positional(right)]
                if left.ellipsis.is_none() && right.ellipsis.is_none() =>
            {
                (left.value, right.value)
            }
            _ => {
                self.record_refused(node);
                self.refuse(
                    node,
                    format!("{name} takes two positional arguments (P2R-48)"),
                );
                return None;
            }
        };
        // Both convert first, so problems in both are reported.
        let left = self.expression(left);
        let right = self.expression(right);
        Some((left?, right?))
    }
}

fn division(left: RExpr, right: RExpr) -> RExpr {
    RExpr::Binary(Box::new(left), BinOp::Div, Box::new(right))
}

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
