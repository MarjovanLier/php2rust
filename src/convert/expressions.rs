//! Expressions, literals and calls (P2R-43, P2R-44, P2R-48, P2R-49).

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::variables::changes_variable;
use super::{Converter, nulls, numbers, strings};
use crate::printer::{self, BinOp, RExpr};
use crate::types::RustType;

impl Converter<'_> {
    /// Converts an expression and gives its Rust type, or records why it cannot.
    pub(super) fn expression(&mut self, expression: &Expression<'_>) -> Option<(RExpr, RustType)> {
        match expression {
            Expression::Parenthesized(inner) => self.expression(inner.expression),
            Expression::Literal(Literal::Integer(integer)) => {
                match integer.value.and_then(|v| i64::try_from(v).ok()) {
                    Some(value) => Some((RExpr::Int(value), RustType::I64)),
                    // PHP reads an integer literal above PHP_INT_MAX as a float (P2R-49).
                    None => Some((
                        RExpr::Float(numbers::integer_as_float(integer.raw)),
                        RustType::F64,
                    )),
                }
            }
            Expression::Literal(Literal::String(string)) => {
                Some((RExpr::Str(self.string_literal(string)?), RustType::String))
            }
            Expression::Variable(Variable::Direct(variable)) => self.variable(variable),
            Expression::UnaryPrefix(unary)
                if matches!(unary.operator, UnaryPrefixOperator::Negation(_)) =>
            {
                let (operand, ty) = self.expression(unary.operand)?;
                match ty {
                    RustType::I64 => self.checked_constant(unary, RExpr::Neg(Box::new(operand))),
                    RustType::F64 => Some((RExpr::Neg(Box::new(operand)), ty)),
                    RustType::Option(_) => {
                        self.refuse(
                            unary,
                            "arithmetic on a value that may be null is not supported (P2R-106)",
                        );
                        None
                    }
                    _ => {
                        self.refuse(unary, "unary minus needs an int or float operand (P2R-43)");
                        None
                    }
                }
            }
            Expression::Binary(binary) => self.binary(binary),
            Expression::CompositeString(CompositeString::Interpolated(_)) => {
                self.joined_string(expression)
            }
            Expression::Call(Call::Function(call)) if strings::calls(call, b"strlen") => {
                self.strlen(call, expression)
            }
            Expression::Call(Call::Function(call)) if strings::calls(call, b"count") => {
                self.count(call, expression)
            }
            Expression::Call(Call::Function(call)) if strings::calls(call, b"intdiv") => {
                self.intdiv(call, expression)
            }
            Expression::Call(Call::Function(call)) if strings::calls(call, b"fdiv") => {
                self.fdiv(call, expression)
            }
            Expression::Call(Call::Function(call)) => match self.call(call, expression)? {
                (call, Some(ty)) => Some((call, ty)),
                (_, None) => {
                    self.refuse(expression, "a void function has no value (P2R-20)");
                    None
                }
            },
            other if changes_variable(other) => self.change_as_value(other),
            Expression::Literal(Literal::True(_)) => Some((RExpr::Bool(true), RustType::Bool)),
            Expression::Literal(Literal::False(_)) => Some((RExpr::Bool(false), RustType::Bool)),
            Expression::UnaryPrefix(unary)
                if matches!(unary.operator, UnaryPrefixOperator::Not(_)) =>
            {
                self.not(unary)
            }
            Expression::Literal(Literal::Null(null)) => {
                self.refuse(null, "the type null has no Rust type (P2R-22)");
                None
            }
            Expression::Array(array) => self.list(array, None),
            Expression::ArrayAccess(access) => self.index(access),
            Expression::Literal(Literal::Float(float)) => {
                Some((RExpr::Float(*float.value), RustType::F64))
            }
            Expression::Access(Access::ClassConstant(access)) => self.enum_case(access, expression),
            Expression::Access(Access::Property(access)) => self.enum_property(access, expression),
            Expression::Call(Call::StaticMethod(call)) => self.enum_call(call, expression),
            Expression::Match(_) => {
                self.record_refused(expression);
                self.refuse(
                    expression,
                    "a match converts only as the whole value of a return or an assignment (P2R-120)",
                );
                None
            }
            other => {
                self.record_refused(other);
                self.refuse(
                    other,
                    "this expression is not supported in version 1 (P2R-07)",
                );
                None
            }
        }
    }

    pub(super) fn binary(&mut self, binary: &Binary<'_>) -> Option<(RExpr, RustType)> {
        let op = match binary.operator {
            BinaryOperator::StringConcat(_) => return self.concatenation(binary),
            BinaryOperator::Addition(_) => BinOp::Add,
            BinaryOperator::Subtraction(_) => BinOp::Sub,
            BinaryOperator::Multiplication(_) => BinOp::Mul,
            BinaryOperator::Modulo(_) => BinOp::Rem,
            BinaryOperator::Division(_) => {
                self.record_refused(binary.lhs);
                self.record_refused(binary.rhs);
                self.refuse(
                    binary,
                    "/ gives an int or a float depending on the values; use intdiv() (P2R-44)",
                );
                return None;
            }
            BinaryOperator::Identical(_) | BinaryOperator::Equal(_) => {
                return self.comparison(binary, BinOp::Eq);
            }
            BinaryOperator::NotIdentical(_)
            | BinaryOperator::NotEqual(_)
            | BinaryOperator::AngledNotEqual(_) => return self.comparison(binary, BinOp::Ne),
            BinaryOperator::LessThan(_) => return self.comparison(binary, BinOp::Lt),
            BinaryOperator::GreaterThan(_) => return self.comparison(binary, BinOp::Gt),
            BinaryOperator::LessThanOrEqual(_) => return self.comparison(binary, BinOp::Le),
            BinaryOperator::GreaterThanOrEqual(_) => return self.comparison(binary, BinOp::Ge),
            BinaryOperator::And(_) => return self.logic(binary, BinOp::And),
            BinaryOperator::Or(_) => return self.logic(binary, BinOp::Or),
            BinaryOperator::Spaceship(_) => {
                self.record_refused(binary.lhs);
                self.record_refused(binary.rhs);
                self.refuse(binary, "<=> is not supported in version 1 (P2R-44)");
                return None;
            }
            BinaryOperator::NullCoalesce(_) => return self.coalesce(binary),
            BinaryOperator::Exponentiation(_) => {
                self.record_refused(binary.lhs);
                self.record_refused(binary.rhs);
                self.refuse(binary, "** is not supported in version 1 (P2R-44)");
                return None;
            }
            _ => {
                self.record_refused(binary.lhs);
                self.record_refused(binary.rhs);
                self.refuse(
                    binary,
                    "this operator is not supported in version 1 (P2R-07)",
                );
                return None;
            }
        };
        // Both sides convert first, so problems on both sides are reported.
        let left = self.expression(binary.lhs);
        let right = self.expression(binary.rhs);
        let ((left, left_type), (right, right_type)) = (left?, right?);
        if let Some(message) = numbers::arithmetic_refusal(op, &left_type, &right_type) {
            self.refuse(binary, message);
            return None;
        }
        // Float arithmetic never panics, so the P2R-50 checks below are for int only.
        if left_type == RustType::F64 {
            let expr = RExpr::Binary(Box::new(left), op, Box::new(right));
            return Some((expr, left_type));
        }
        // P2R-50 stops `%` by a divisor the tool can prove is 0, a literal or a variable
        // that holds 0. rustc rejects many such cases at compile time (unconditional_panic).
        if op == BinOp::Rem && printer::literal_value(&right) == Some(0) {
            self.refuse(binary, "% by zero always fails (P2R-50)");
            return None;
        }
        self.checked_constant(binary, RExpr::Binary(Box::new(left), op, Box::new(right)))
    }

    /// P2R-50 stops int arithmetic that the tool can prove overflows: an operation whose
    /// operands are literals or variables with a known value. rustc rejects many such cases
    /// at compile time (arithmetic_overflow, or unconditional_panic for `%` of i64::MIN by
    /// -1). Each operation is checked when it is built, so an operand that overflowed is
    /// already refused.
    pub(super) fn checked_constant(
        &mut self,
        node: &impl HasSpan,
        expr: RExpr,
    ) -> Option<(RExpr, RustType)> {
        let operands_known = match &expr {
            RExpr::Binary(left, _, right) => {
                printer::literal_value(left).is_some() && printer::literal_value(right).is_some()
            }
            RExpr::Neg(operand) => printer::literal_value(operand).is_some(),
            _ => false,
        };
        if operands_known && printer::literal_value(&expr).is_none() {
            self.refuse(node, "this int arithmetic always overflows (P2R-50)");
            return None;
        }
        Some((expr, RustType::I64))
    }

    /// A call to a function declared in this file, with its return type (`None` for void).
    pub(super) fn call(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, Option<RustType>)> {
        let Expression::Identifier(Identifier::Local(name)) = call.function else {
            self.record_refused(node);
            self.refuse(
                node,
                "only calls by plain function name are supported (P2R-07)",
            );
            return None;
        };
        let Some(signature) = self
            .functions
            .get(&name.value.to_ascii_lowercase())
            .cloned()
        else {
            let text = String::from_utf8_lossy(name.value).into_owned();
            self.record_refused(node);
            self.refuse(node, format!("calls to {text} are not supported (P2R-48)"));
            return None;
        };
        let mut arguments = Vec::new();
        let mut converted = true;
        for (position, argument) in call.argument_list.arguments.iter().enumerate() {
            match argument {
                // Mago reports an argument whose type does not match the parameter (P2R-11).
                Argument::Positional(positional) if positional.ellipsis.is_none() => {
                    let parameter = signature.parameters.get(position).map(|p| &p.ty);
                    match self.value(positional.value, parameter) {
                        Some((_, ty))
                            if parameter.is_some_and(|p| nulls::null_for_plain(&ty, p)) =>
                        {
                            self.refuse(
                                positional.value,
                                "a value that may be null cannot go to a parameter that is not nullable (P2R-106)",
                            );
                            converted = false;
                        }
                        Some((expr, ty)) => {
                            let declared = parameter.unwrap_or(&ty);
                            match self.declared_value(positional.value, expr, &ty, declared) {
                                Some(expr) => arguments.push(nulls::wrapped(
                                    self.borrowed(expr, &ty),
                                    &ty,
                                    declared,
                                )),
                                None => converted = false,
                            }
                        }
                        None => converted = false,
                    }
                }
                other => {
                    let value = match other {
                        Argument::Positional(positional) => positional.value,
                        Argument::Named(named) => named.value,
                    };
                    self.record_refused(value);
                    self.refuse(
                        other,
                        "named and spread arguments are not supported (P2R-07)",
                    );
                    converted = false;
                }
            }
        }
        // PHP accepts extra arguments to a user function; Rust does not.
        if converted && arguments.len() != signature.parameters.len() {
            self.refuse(node, "a call must pass one argument per parameter (P2R-07)");
            converted = false;
        }
        converted.then_some((
            RExpr::Call(signature.rust_name, arguments),
            signature.returns,
        ))
    }

    /// The text of a PHP string literal, decoded by PHP's rules (P2R-49).
    pub(super) fn string_literal(&mut self, string: &LiteralString<'_>) -> Option<String> {
        if matches!(string.kind, LiteralStringKind::DoubleQuoted)
            && printer::octal_overflow(string.raw)
        {
            self.refuse(
                string,
                "an octal escape above \\377 is not supported (P2R-49)",
            );
            return None;
        }
        let Some(bytes) = string.value else {
            self.refuse(string, "this string escape is not supported (P2R-49)");
            return None;
        };
        match std::str::from_utf8(bytes) {
            Ok(text) => Some(text.to_owned()),
            Err(_) => {
                self.refuse(
                    string,
                    "a string that is not valid UTF-8 is not supported (P2R-49)",
                );
                None
            }
        }
    }
}
