//! PHP's `DateTime` read from a format, written back with a format, tested with
//! `instanceof`, and the text of a date read back by `date_parse()` (P2R-330 to P2R-335).

use mago_syntax::cst::*;

use super::Converter;
use super::flow::Var;
use super::helpers::Helper;
use super::lists::unparenthesized;
use crate::printer::RExpr;
use crate::types::{RustType, ShapeField};

/// The keys of a shape that `date_parse()` of a date's text fills (P2R-335).
const PARTS: [&[u8]; 6] = [b"year", b"month", b"day", b"hour", b"minute", b"second"];

/// The Rust name of that shape, its keys in byte order (P2R-200, P2R-335).
pub(super) const DATE_PARTS: &str = "DayHourMinuteMonthSecondYearShape";

/// The first byte of a literal format at which a helper would panic: an ASCII letter it
/// does not read or write, and for `createFromFormat()` also `*` and `+` (P2R-331,
/// P2R-332). `createFromFormat()` reads a format up to its first NUL, `format()` all of it.
fn unsupported(format: &[u8], reading: bool) -> Option<u8> {
    let mut bytes = format.iter().take_while(|&&byte| !reading || byte != 0);
    while let Some(&byte) = bytes.next() {
        match byte {
            b'\\' => {
                bytes.next();
            }
            b'd' | b'j' | b'm' | b'n' | b'Y' | b'y' | b'H' | b'G' | b'i' | b's' => {}
            b'*' | b'+' if reading => return Some(byte),
            letter if letter.is_ascii_alphabetic() => return Some(letter),
            _ => {}
        }
    }
    None
}

/// The side of `binary` that is not the literal `false`, when exactly one side is.
fn beside_false<'e, 'a>(binary: &'e Binary<'a>) -> Option<&'e Expression<'a>> {
    let is_false = |side: &Expression<'_>| {
        matches!(
            unparenthesized(side),
            Expression::Literal(Literal::False(_))
        )
    };
    match (is_false(binary.lhs), is_false(binary.rhs)) {
        (true, false) => Some(binary.rhs),
        (false, true) => Some(binary.lhs),
        _ => None,
    }
}

impl Converter<'_> {
    /// True when `class` names PHP's `DateTime` rather than a class of the input (P2R-330).
    pub(super) fn names_date_time(&self, class: &Expression<'_>) -> bool {
        self.class_key(class).is_some_and(|key| {
            key == b"datetime" && !self.classes.iter().any(|class| class.key == key)
        })
    }

    /// `DateTime::createFromFormat($format, $text)` as
    /// `php_date_create_from_format(format, text)` (P2R-331).
    pub(super) fn date_create(
        &mut self,
        call: &StaticMethodCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let creates = matches!(&call.method, ClassLikeMemberSelector::Identifier(name)
            if name.value.eq_ignore_ascii_case(b"createFromFormat"));
        if !creates {
            self.record_refused_arguments(&call.argument_list);
            self.refuse(
                node,
                "of DateTime only DateTime::createFromFormat() and format() convert (P2R-330)",
            );
            return None;
        }
        let arguments = self.list_arguments(
            &call.argument_list,
            node,
            (2, 2),
            "DateTime::createFromFormat() converts with two arguments only (P2R-331)",
        )?;
        let message = "DateTime::createFromFormat() converts on string arguments only (P2R-331)";
        let format = self.typed_argument(arguments[0], &RustType::String, message);
        let text = self.typed_argument(arguments[1], &RustType::String, message);
        let checked = self.date_format_checked(arguments[0], true);
        let (format, text) = (format?, text?);
        if !checked {
            return None;
        }
        let format = self.borrowed(format, &RustType::String);
        let text = self.borrowed(text, &RustType::String);
        Some((
            self.helper_call(Helper::PhpDateCreateFromFormat, vec![format, text]),
            RustType::Option(Box::new(RustType::DateTime)),
        ))
    }

    /// `$d->format($f)` on a `DateTime` as `php_date_format(d, f)` (P2R-332).
    pub(super) fn date_method(
        &mut self,
        call: &MethodCall<'_>,
        node: &Expression<'_>,
        object: RExpr,
    ) -> Option<(RExpr, RustType)> {
        let formats = matches!(&call.method, ClassLikeMemberSelector::Identifier(name)
            if name.value.eq_ignore_ascii_case(b"format"));
        if !formats {
            self.record_refused_arguments(&call.argument_list);
            self.refuse(
                node,
                "of DateTime only DateTime::createFromFormat() and format() convert (P2R-330)",
            );
            return None;
        }
        let arguments = self.list_arguments(
            &call.argument_list,
            node,
            (1, 1),
            "format() converts with one argument only (P2R-332)",
        )?;
        let format = self.typed_argument(
            arguments[0],
            &RustType::String,
            "format() converts on a string argument only (P2R-332)",
        );
        let checked = self.date_format_checked(arguments[0], false);
        let format = format?;
        if !checked {
            return None;
        }
        let format = self.borrowed(format, &RustType::String);
        Some((
            self.helper_call(Helper::PhpDateFormat, vec![object, format]),
            RustType::String,
        ))
    }

    /// False, with a problem recorded, for a literal format that holds a byte the helper
    /// panics at (P2R-331, P2R-332). A format read at run time passes.
    fn date_format_checked(&mut self, node: &Expression<'_>, reading: bool) -> bool {
        let Some(format) = self.literal_text(node) else {
            return true;
        };
        let Some(byte) = unsupported(format.as_bytes(), reading) else {
            return true;
        };
        let id = if reading { "P2R-331" } else { "P2R-332" };
        self.refuse(
            node,
            format!(
                "the date format character {} is not supported ({id})",
                char::from(byte)
            ),
        );
        false
    }

    /// `$v instanceof DateTime` on a `DateTime|false` value as `v.is_some()`, and as
    /// `v.is_none()` when `negated` (P2R-333).
    pub(super) fn instance_of(
        &mut self,
        binary: &Binary<'_>,
        negated: bool,
    ) -> Option<(RExpr, RustType)> {
        if let Expression::Identifier(name) = binary.rhs
            && self.names_optional(name)
        {
            return self.instance_of_optional(binary, negated);
        }
        let date = RustType::Option(Box::new(RustType::DateTime));
        let value = self.expression(binary.lhs);
        let class = self.names_date_time(binary.rhs);
        let (expr, ty) = value?;
        if !class || ty != date {
            self.refuse(
                binary,
                "instanceof converts only with the class DateTime, on a value of type DateTime|false (P2R-333)",
            );
            return None;
        }
        let test = if negated { "is_none" } else { "is_some" };
        Some((RExpr::Method(Box::new(expr), test), RustType::Bool))
    }

    /// `$v === false` and `$v !== false` on a `DateTime|false` value, as `is_none()` and
    /// `is_some()` (P2R-330). `None` when no side is `false` or the other side has another
    /// type, so the comparison converts as any other.
    pub(super) fn false_comparison(
        &mut self,
        binary: &Binary<'_>,
        equal: bool,
    ) -> Option<Option<(RExpr, RustType)>> {
        let value = beside_false(binary)?;
        let date = RustType::Option(Box::new(RustType::DateTime));
        if self.site_type(value).ok()? != date {
            return None;
        }
        let converted = self.expression(value).map(|(expr, _)| {
            let test = if equal { "is_none" } else { "is_some" };
            (RExpr::Method(Box::new(expr), test), RustType::Bool)
        });
        Some(converted)
    }

    /// The variable of a check on a `DateTime|false` value that P2R-104 binds: `$v
    /// instanceof DateTime` and `$v !== false` when `negated`, `!$v instanceof DateTime`
    /// and `$v === false` otherwise, with its id and the inner type (P2R-333).
    pub(super) fn date_checked<'e, 'a>(
        &self,
        operand: &'e Expression<'a>,
        negated: bool,
    ) -> Option<(&'e DirectVariable<'a>, Var, RustType)> {
        let (operand, negated) = match unparenthesized(operand) {
            Expression::UnaryPrefix(unary)
                if matches!(unary.operator, UnaryPrefixOperator::Not(_)) =>
            {
                (unparenthesized(unary.operand), !negated)
            }
            other => (other, negated),
        };
        let Expression::Binary(binary) = operand else {
            return None;
        };
        let value = match binary.operator {
            BinaryOperator::Instanceof(_) if negated && self.names_date_time(binary.rhs) => {
                binary.lhs
            }
            BinaryOperator::NotIdentical(_) if negated => beside_false(binary)?,
            BinaryOperator::Identical(_) if !negated => beside_false(binary)?,
            _ => return None,
        };
        let Expression::Variable(Variable::Direct(variable)) = unparenthesized(value) else {
            return None;
        };
        let outer = self.lookup(variable)?;
        match self.variables.ty(outer)? {
            RustType::Option(inner) if **inner == RustType::DateTime => {
                Some((variable, outer, RustType::DateTime))
            }
            _ => None,
        }
    }

    /// `date_parse($d->format('Y-m-d H:i:s'))` as a struct literal of the shape of its six
    /// int parts, each field read from `d`; a place that needs another type stops by the
    /// rule that checks it (P2R-335).
    pub(super) fn date_parse(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let arguments = self.built_in_arguments(
            call,
            node,
            (1, 1),
            "date_parse() converts with one argument only (P2R-335)",
        )?;
        let date = match unparenthesized(arguments[0]) {
            Expression::Call(Call::Method(format)) => {
                let full = matches!(&format.method, ClassLikeMemberSelector::Identifier(name)
                    if name.value.eq_ignore_ascii_case(b"format"))
                    && format.argument_list.arguments.len() == 1
                    && matches!(
                        &format.argument_list.arguments[0],
                        Argument::Positional(argument)
                            if self.literal_text(argument.value).as_deref()
                                == Some("Y-m-d H:i:s")
                    );
                full.then_some(format.object)
            }
            _ => None,
        };
        let Some(date) = date else {
            self.record_refused(node);
            self.refuse(
                node,
                "date_parse() converts only of $d->format('Y-m-d H:i:s') (P2R-335)",
            );
            return None;
        };
        let (expr, ty) = self.expression(date)?;
        if ty != RustType::DateTime {
            self.refuse(
                date,
                "date_parse() converts only of the text of a DateTime that is not false (P2R-335)",
            );
            return None;
        }
        // The fields in byte order of their keys, as P2R-200 orders a shape.
        let mut parts = PARTS;
        parts.sort_unstable();
        let fields: Vec<ShapeField> = parts
            .iter()
            .map(|&key| ShapeField {
                key: key.to_vec(),
                name: String::from_utf8_lossy(key).into_owned(),
                ty: RustType::I64,
            })
            .collect();
        let name = DATE_PARTS.to_owned();
        if !self.type_names.add_shape(&name, fields.clone()) {
            self.refuse(
                node,
                format!("{name}, the struct of the parts of a date, is the name of another shape, enum or class (P2R-335)"),
            );
            return None;
        }
        let values = fields
            .into_iter()
            .map(|field| {
                let read = RExpr::Field(Box::new(expr.clone()), field.name.clone());
                (field.name, read)
            })
            .collect();
        Some((RExpr::Struct(name.clone(), values), RustType::Shape(name)))
    }
}
