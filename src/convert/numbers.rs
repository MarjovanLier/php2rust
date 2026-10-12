//! Numbers: floats, the operand rules of arithmetic, ints where a float is declared, and
//! `intdiv` and `fdiv` (P2R-20, P2R-25, P2R-43, P2R-44, P2R-48 to P2R-50).
//!
//! Float arithmetic never panics and rustc rejects none of it, so only int arithmetic
//! goes through the P2R-50 checks of expressions.rs.

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::Converter;
use crate::printer::{self, BinOp, RExpr};
use crate::types::RustType;

impl Converter<'_> {
    /// A value passed to a parameter or returned, where `declared` is the declared type. An
    /// int becomes a float, also where a `?float` is declared (P2R-25, P2R-102). A list of
    /// ints where a list of floats is declared has no such conversion, so it stops: Mago
    /// accepts it, but a `Vec<i64>` is no `&[f64]`. Nor has an int that may be null.
    pub(super) fn declared_value(
        &mut self,
        node: &impl HasSpan,
        expr: RExpr,
        ty: &RustType,
        declared: &RustType,
    ) -> Option<RExpr> {
        if ints_for_floats(ty.inner(), declared.inner()) {
            self.refuse(
                node,
                "a list of int where a list of float is declared is not supported (P2R-25)",
            );
            return None;
        }
        if *ty == RustType::Option(Box::new(RustType::I64)) && *declared.inner() == RustType::F64 {
            self.refuse(
                node,
                "an int that may be null where a float is declared is not supported (P2R-25)",
            );
            return None;
        }
        Some(widened(expr, ty, declared.inner()))
    }

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

/// True for a list of ints, at any depth and also inside `Option`, where the same list of
/// floats is declared.
fn ints_for_floats(ty: &RustType, declared: &RustType) -> bool {
    match (ty, declared) {
        (RustType::Vec(found), RustType::Vec(wanted)) => {
            (*found.inner() == RustType::I64 && *wanted.inner() == RustType::F64)
                || ints_for_floats(found.inner(), wanted.inner())
        }
        _ => false,
    }
}

/// An int value where a float is declared, as a parameter or a return type, becomes a
/// float: a literal by its value (`4` to `4.0`), any other value with `as f64` (P2R-25).
/// Every other value is given back as it is.
fn widened(expr: RExpr, ty: &RustType, declared: &RustType) -> RExpr {
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

/// The float PHP reads from an integer literal above PHP_INT_MAX (P2R-49). Decimal text
/// parses as a float literal; hexadecimal, octal and binary digits add up in a float one
/// at a time, as in PHP's zend_hex_strtod, zend_oct_strtod and zend_bin_strtod.
pub(super) fn integer_as_float(raw: &[u8]) -> f64 {
    let text: String = raw
        .iter()
        .filter(|&&byte| byte != b'_')
        .map(|&byte| char::from(byte))
        .collect();
    let (radix, digits) = match text.as_bytes() {
        [b'0', b'x' | b'X', ..] => (16, &text[2..]),
        [b'0', b'o' | b'O', ..] => (8, &text[2..]),
        [b'0', b'b' | b'B', ..] => (2, &text[2..]),
        [b'0', ..] => (8, &text[1..]),
        // Rust parses a decimal that overflows as infinity, as PHP does.
        _ => return text.parse().unwrap_or_default(),
    };
    digits.bytes().fold(0.0, |value, byte| {
        let scaled = value * f64::from(radix);
        if radix == 16 {
            scaled + f64::from(char::from(byte).to_digit(16).unwrap_or_default())
        } else {
            // PHP's octal and binary add the digit's ASCII code, then take away that of
            // `0`. Above 2^53 that rounds twice, and the value must match PHP's.
            scaled + f64::from(byte) - f64::from(b'0')
        }
    })
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
        (RustType::Option(_), _) | (_, RustType::Option(_)) => {
            Some("arithmetic on a value that may be null is not supported (P2R-106)")
        }
        _ => Some("arithmetic needs two int or two float operands (P2R-43)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The values PHP 8.5 gives these literals in var_dump. Adding the digit instead of
    /// its ASCII code gives 2^65 for the 65 binary ones and for 2^65 in octal.
    #[test]
    fn integers_above_the_maximum_read_as_php_reads_them() {
        let max_plus_one = 9_223_372_036_854_775_808.0;
        assert_eq!(integer_as_float(b"9223372036854775808"), max_plus_one);
        assert_eq!(integer_as_float(b"9_223_372_036_854_775_808"), max_plus_one);
        assert_eq!(integer_as_float(b"0x8000000000000000"), max_plus_one);
        let wide = 4.722_366_482_869_645e21;
        assert_eq!(integer_as_float(b"0xFFFFFFFFFFFFFFFFFF"), wide);
        assert_eq!(integer_as_float(b"0777777777777777777777777"), wide);
        assert_eq!(integer_as_float(b"0o777777777777777777777777"), wide);
        let below_2_65 = 3.689_348_814_741_91e19;
        let ones = [b"0b".as_slice(), &[b'1'; 65]].concat();
        assert_eq!(integer_as_float(&ones), below_2_65);
        assert_eq!(integer_as_float(b"0o4000000000000000000000"), below_2_65);
        assert_eq!(integer_as_float(b"04000000000000000000000"), below_2_65);
        assert_eq!(integer_as_float(&[b'1'; 400]), f64::INFINITY);
    }
}
