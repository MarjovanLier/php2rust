//! Numbers: floats, the operand rules of arithmetic, ints where a float is declared, and
//! `intdiv` and `fdiv` (P2R-20, P2R-25, P2R-43, P2R-44, P2R-48 to P2R-50).
//!
//! Float arithmetic never panics and rustc rejects none of it, so only int arithmetic
//! goes through the P2R-50 checks of expressions.rs.

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::helpers::Helper;
use super::{Converter, maps, nulls, unions};
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
        // A value of `T` goes into `Present` as it goes where `T` is declared (P2R-361).
        if let RustType::Optional(inner) = declared
            && !matches!(ty, RustType::Optional(_))
        {
            return self.declared_value(node, expr, ty, inner);
        }
        // Mago accepts a list or a shape where a map is declared (P2R-240).
        if maps::map_clash(ty, declared) {
            self.refuse(node, maps::MAP_CLASH);
            return None;
        }
        if ints_for_floats(ty.inner(), declared.inner()) {
            self.refuse(
                node,
                "a list of int where a list of float is declared is not supported (P2R-25)",
            );
            return None;
        }
        // A list literal takes the list of `Option` type it flows into; any other list
        // value has no conversion to it (P2R-102).
        if plain_for_nullable(ty.inner(), declared.inner()) {
            self.refuse(
                node,
                "a list of values that cannot be null where a list of values that may be null is declared is not supported (P2R-102)",
            );
            return None;
        }
        if unions::misplaced(ty, declared) {
            self.refuse(node, unions::UNION_PLACE);
            return None;
        }
        // PHP makes a float of the int for a native union type and keeps the int for a
        // docblock one (P2R-231).
        if let RustType::Union(name) = declared.inner()
            && *ty.inner() == RustType::I64
            && self.type_names.union(name).is_some_and(|members| {
                members.contains(&RustType::F64) && !members.contains(&RustType::I64)
            })
        {
            self.refuse(
                node,
                "an int where a union type has float and no int is not supported (P2R-231)",
            );
            return None;
        }
        // Mago accepts a shape where a union's map member is declared (P2R-270, P2R-271).
        if let RustType::Union(name) = declared.inner()
            && !matches!(ty.inner(), RustType::Union(_))
            && self
                .type_names
                .union(name)
                .is_some_and(|members| !members.contains(ty.inner()))
        {
            self.refuse(
                node,
                "this value is no member of the union type it goes into (P2R-231)",
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

    /// `checkdate($month, $day, $year)` on three ints, as the helper `php_checkdate`
    /// (P2R-292).
    pub(super) fn checkdate(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let arguments = self.built_in_arguments(
            call,
            node,
            (3, 3),
            "checkdate() takes three positional arguments (P2R-292)",
        )?;
        let message = "checkdate() is supported only on ints (P2R-292)";
        // All three convert first, so problems in each are reported.
        let converted: Vec<Option<RExpr>> = arguments
            .iter()
            .map(|&argument| self.typed_argument(argument, &RustType::I64, message))
            .collect();
        let converted = converted.into_iter().collect::<Option<_>>()?;
        Some((
            self.helper_call(Helper::PhpCheckdate, converted),
            RustType::Bool,
        ))
    }

    /// `(int)` of a string, which reads the number at its start as PHP does, and
    /// `(string)` of an int or a string. Every other cast stops (P2R-07, P2R-166).
    pub(super) fn cast(&mut self, unary: &UnaryPrefix<'_>) -> Option<(RExpr, RustType)> {
        let (int, problem) = match unary.operator {
            UnaryPrefixOperator::IntCast(..) => (true, None),
            UnaryPrefixOperator::StringCast(..) => (false, None),
            // PHP 8.5 prints a deprecation for both when it compiles the file; Mago does not
            // report it.
            UnaryPrefixOperator::IntegerCast(..) | UnaryPrefixOperator::BinaryCast(..) => (
                false,
                Some(
                    "PHP 8.5 deprecates (integer) and (binary); write (int) or (string) (P2R-166)",
                ),
            ),
            _ => (false, Some("this cast is not supported (P2R-07)")),
        };
        if let Some(message) = problem {
            self.record_refused(unary.operand);
            self.refuse(unary, message);
            return None;
        }
        // `(string)` of a union value matches its variants (P2R-233).
        if !int && self.union_operand(unary.operand) {
            return self.union_test(unary.operand, unions::Test::Text);
        }
        let (operand, ty) = self.expression(unary.operand)?;
        match (int, &ty) {
            (_, RustType::Option(_)) => {
                self.refuse(
                    unary,
                    "a cast of a value that may be null is not supported (P2R-106)",
                );
                None
            }
            (true, RustType::String) => {
                let text = self.borrowed(operand, &ty);
                Some((self.helper_call(Helper::PhpInt, vec![text]), RustType::I64))
            }
            (false, RustType::I64) => Some((
                RExpr::Method(Box::new(operand), "to_string"),
                RustType::String,
            )),
            // A string cast of a string is the string itself (P2R-166).
            (false, RustType::String) => Some((operand, RustType::String)),
            (true, _) => {
                self.refuse(unary, "(int) is supported only on a string (P2R-166)");
                None
            }
            (false, _) => {
                self.refuse(
                    unary,
                    "(string) is supported only on an int or a string (P2R-166)",
                );
                None
            }
        }
    }

    /// The two arguments of `intdiv` or `fdiv`, converted.
    fn operands(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
        name: &str,
    ) -> Option<((RExpr, RustType), (RExpr, RustType))> {
        let arguments = self.built_in_arguments(
            call,
            node,
            (2, 2),
            &format!("{name} takes two positional arguments (P2R-48)"),
        )?;
        let (left, right) = (arguments[0], arguments[1]);
        // Both convert first, so problems in both are reported.
        let converted = (self.expression(left), self.expression(right));
        let (Some(left_value), Some(right_value)) = converted else {
            return None;
        };
        // Mago may type an argument as a number where it knows more (P2R-104).
        let mut plain = true;
        for (node, (_, ty)) in [(left, &left_value), (right, &right_value)] {
            if matches!(ty.inner(), RustType::Union(_)) {
                self.refuse(node, unions::UNION_BUILT_IN);
                plain = false;
            } else if matches!(ty, RustType::Option(_)) {
                self.refuse(node, nulls::NULL_BUILT_IN);
                plain = false;
            }
        }
        plain.then_some((left_value, right_value))
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

/// True for a list of `T`, at any depth, where a list of `Option<T>` is declared.
fn plain_for_nullable(ty: &RustType, declared: &RustType) -> bool {
    match (ty, declared) {
        (RustType::Vec(found), RustType::Vec(wanted)) => {
            (matches!(**wanted, RustType::Option(_)) && **found == *wanted.inner())
                || plain_for_nullable(found, wanted)
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
/// int, or both float except for `%`, `<<` and `>>` (P2R-43, P2R-44, P2R-167).
pub(super) fn arithmetic_refusal(
    op: BinOp,
    left: &RustType,
    right: &RustType,
) -> Option<&'static str> {
    match (left, right) {
        (RustType::I64, RustType::I64) => None,
        (RustType::Option(_), _) | (_, RustType::Option(_)) => {
            Some("arithmetic on a value that may be null is not supported (P2R-106)")
        }
        _ if op.shifts() => Some("<< and >> need two int operands (P2R-167)"),
        (RustType::F64, RustType::F64) if op != BinOp::Rem => None,
        (RustType::F64, RustType::F64) => Some("% on a float is not supported (P2R-43)"),
        (RustType::I64, RustType::F64) | (RustType::F64, RustType::I64) => {
            Some("arithmetic that mixes int and float is not supported (P2R-44)")
        }
        _ => Some("arithmetic needs two int or two float operands (P2R-43)"),
    }
}

/// Why the count of a shift does not convert, or `None` when it does: an int literal from
/// 0 to 63. PHP gives 0 or -1 for a larger count and throws for a negative one, where Rust
/// panics (P2R-167).
pub(super) fn shift_count_refusal(op: BinOp, count: &RExpr) -> Option<&'static str> {
    (op.shifts() && !matches!(count, RExpr::Int(0..=63))).then_some(
        "a shift count must be an int literal from 0 to 63, where PHP and Rust agree (P2R-167)",
    )
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
