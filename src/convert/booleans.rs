//! Bools, comparisons and logic (P2R-20, P2R-42 to P2R-44, P2R-50).

use mago_syntax::cst::*;

use super::Converter;
use crate::printer::{BinOp, RExpr};
use crate::types::RustType;

impl Converter<'_> {
    /// A condition of `if`, `while` or `for`, or an operand of `&&`, `||` or `!`. PHP
    /// truthiness has no Rust form, so the value must be a bool (P2R-42).
    pub(super) fn condition(&mut self, expression: &Expression<'_>) -> Option<RExpr> {
        if let Some(call) = self.preg_match_call(expression) {
            return self.preg_match(call, expression);
        }
        let (expr, ty) = self.expression(expression)?;
        if matches!(ty, RustType::Option(_)) {
            self.refuse(
                expression,
                "a value that may be null cannot be a condition (P2R-106)",
            );
            return None;
        }
        if ty != RustType::Bool {
            self.refuse(
                expression,
                format!(
                    "this value is {}, not bool, and PHP truthiness is not supported (P2R-42)",
                    ty.owned()
                ),
            );
            return None;
        }
        Some(expr)
    }

    /// `!` (P2R-42).
    pub(super) fn not(&mut self, unary: &UnaryPrefix<'_>) -> Option<(RExpr, RustType)> {
        // `!$v instanceof DateTime` is `!($v instanceof DateTime)` (P2R-333).
        if let Expression::Binary(binary) = super::lists::unparenthesized(unary.operand)
            && matches!(binary.operator, BinaryOperator::Instanceof(_))
        {
            return self.instance_of(binary, true);
        }
        let operand = self.condition(unary.operand)?;
        Some((RExpr::Not(Box::new(operand)), RustType::Bool))
    }

    /// `&&` and `||` (P2R-42).
    pub(super) fn logic(&mut self, binary: &Binary<'_>, op: BinOp) -> Option<(RExpr, RustType)> {
        // Both sides convert first, so problems on both sides are reported.
        let left = self.condition(binary.lhs);
        let right = self.condition(binary.rhs);
        let expr = RExpr::Binary(Box::new(left?), op, Box::new(right?));
        Some((expr, RustType::Bool))
    }

    /// `===`, `!==`, `==`, `!=`, `<>`, `<`, `>`, `<=` and `>=` (P2R-43, P2R-44, P2R-103).
    pub(super) fn comparison(
        &mut self,
        binary: &Binary<'_>,
        op: BinOp,
    ) -> Option<(RExpr, RustType)> {
        let strict = matches!(
            binary.operator,
            BinaryOperator::Identical(_) | BinaryOperator::NotIdentical(_)
        );
        if let Some(converted) = self.null_comparison(binary, op, strict) {
            return converted;
        }
        if strict && let Some(converted) = self.false_comparison(binary, op == BinOp::Eq) {
            return converted;
        }
        if strict && let Some(converted) = self.preg_match_comparison(binary, op) {
            return converted;
        }
        let left = self.expression(binary.lhs);
        let right = self.expression(binary.rhs);
        let ((left, left_type), (right, right_type)) = (left?, right?);
        if matches!(left_type, RustType::Option(_)) || matches!(right_type, RustType::Option(_)) {
            return self.option_comparison(
                binary,
                op,
                strict,
                (left, left_type),
                (right, right_type),
            );
        }
        if let Some(message) = refusal(op, strict, &left_type, &right_type) {
            self.refuse(binary, message);
            return None;
        }
        if always_the_same(&left, op, &right) {
            self.refuse(
                binary,
                "this comparison gives the same result for every i64, and rustc rejects it (P2R-50)",
            );
            return None;
        }
        let expr = RExpr::Binary(Box::new(left), op, Box::new(right));
        Some((expr, RustType::Bool))
    }
}

/// Why a comparison of these operand types does not convert, or `None` when it does. For
/// `Option` operands, nulls.rs passes the inner types.
pub(super) fn refusal(
    op: BinOp,
    strict: bool,
    left: &RustType,
    right: &RustType,
) -> Option<&'static str> {
    let numbers = |ty: &RustType| matches!(ty, RustType::I64 | RustType::F64);
    let enums = |ty: &RustType| matches!(ty, RustType::Enum(_));
    // Rust has no object identity, and PHP's == compares properties loosely.
    if matches!(left, RustType::Class(_)) || matches!(right, RustType::Class(_)) {
        return Some("an object cannot be compared (P2R-136)");
    }
    // PHP's === on arrays also compares the order of the keys, which a struct loses.
    if matches!(left, RustType::Shape(_)) || matches!(right, RustType::Shape(_)) {
        return Some("an array shape cannot be compared (P2R-201)");
    }
    if matches!(left.inner(), RustType::Union(_)) || matches!(right.inner(), RustType::Union(_)) {
        return Some(UNION_COMPARED);
    }
    if left != right {
        return Some(if numbers(left) && numbers(right) {
            "a comparison that mixes int and float is not supported (P2R-44)"
        } else if enums(left) || enums(right) {
            "an enum compares only with a value of the same enum (P2R-112)"
        } else {
            "a comparison needs two operands of the same type (P2R-43)"
        });
    }
    let ordering = matches!(op, BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge);
    match left {
        RustType::I64 | RustType::F64 => None,
        RustType::String if ordering => {
            Some("<, >, <= and >= on strings are not supported in version 1 (P2R-44)")
        }
        // PHP compares two numeric strings as numbers: "1e3" == "1000" is true.
        RustType::String if !strict => {
            Some("== and != on strings are not supported in version 1; use === or !== (P2R-44)")
        }
        RustType::Bool if ordering => {
            Some("<, >, <= and >= are built for int and float only (P2R-43)")
        }
        RustType::String | RustType::Bool => None,
        RustType::Vec(_) => Some("a comparison of lists is not supported in version 1 (P2R-07)"),
        // Only nulls.rs compares `Option` values, by their inner types.
        RustType::Option(_) => Some("only === and !== compare values that may be null (P2R-103)"),
        RustType::Optional(_) => Some(super::optionals::OPTIONAL_USE),
        // PHP compares enum cases by identity with ==, != and their strict forms.
        RustType::Enum(_) if ordering => {
            Some("<, >, <= and >= on enums are not supported (P2R-112)")
        }
        RustType::Enum(_) => None,
        RustType::Class(_) => Some("an object cannot be compared (P2R-136)"),
        RustType::Shape(_) => Some("an array shape cannot be compared (P2R-201)"),
        RustType::Union(_) => Some(UNION_COMPARED),
        RustType::Map(..) => Some("a comparison of maps is not supported (P2R-242)"),
        RustType::DateTime => Some("a DateTime cannot be compared (P2R-330)"),
    }
}

/// The problem of a comparison with a union value (P2R-232).
const UNION_COMPARED: &str =
    "a value of a union type compares only with the literal null (P2R-232)";

/// True when rustc's unused_comparisons lint, an error under -D warnings, fires: the
/// comparison holds for every i64 or for none (P2R-50). The lint looks at the left
/// operand when it is an int literal, otherwise at the right one; a literal here is never
/// negative, so only `i64::MAX` with `<=` or `>` on the literal's other side fires.
fn always_the_same(left: &RExpr, op: BinOp, right: &RExpr) -> bool {
    match (left, right) {
        // A negative value prints as unary minus, which the lint does not read.
        (RExpr::Int(value), _) if *value >= 0 => {
            *value == i64::MAX && matches!(op, BinOp::Ge | BinOp::Lt)
        }
        (_, RExpr::Int(value)) if *value >= 0 => {
            *value == i64::MAX && matches!(op, BinOp::Le | BinOp::Gt)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The cases probed with rustc 1.99: only the literal rustc reads, at `i64::MAX`.
    #[test]
    fn useless_comparisons_match_rustc() {
        let max = || RExpr::Int(i64::MAX);
        let n = || RExpr::Var("n".into());
        assert!(always_the_same(&n(), BinOp::Le, &max()));
        assert!(always_the_same(&n(), BinOp::Gt, &max()));
        assert!(always_the_same(&max(), BinOp::Lt, &n()));
        assert!(always_the_same(&max(), BinOp::Ge, &RExpr::Int(5)));
        assert!(!always_the_same(&RExpr::Int(5), BinOp::Le, &max()));
        assert!(!always_the_same(&max(), BinOp::Gt, &max()));
        assert!(!always_the_same(&n(), BinOp::Lt, &max()));
        assert!(!always_the_same(&n(), BinOp::Eq, &max()));
    }
}
