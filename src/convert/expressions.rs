//! Expressions, literals and calls (P2R-43, P2R-44, P2R-48, P2R-49).

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::variables::{self, changes_variable};
use super::{Converter, Mode, ParameterSignature, lists, nulls, numbers, parameter_index, unions};
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
            Expression::Throw(throw) => {
                self.record_refused(throw.exception);
                self.refuse(
                    throw,
                    "a throw converts only as a statement, a match arm or the right side of ?? (P2R-215)",
                );
                None
            }
            Expression::Variable(Variable::Direct(variable))
                if self.caught.iter().any(|(name, ..)| *name == variable.name) =>
            {
                self.refuse(
                    variable,
                    "a caught exception converts only in getMessage(), getCode() and throw (P2R-214)",
                );
                None
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
            Expression::Call(Call::Function(call)) if self.calls_built_in(call, b"strlen") => {
                self.strlen(call, expression)
            }
            Expression::Call(Call::Function(call)) if self.calls_built_in(call, b"is_scalar") => {
                self.type_test(call, expression, true)
            }
            Expression::Call(Call::Function(call)) if self.calls_built_in(call, b"gettype") => {
                self.type_test(call, expression, false)
            }
            Expression::Call(Call::Function(call))
                if let Some(test) = self.member_test_of(call) =>
            {
                self.member_test(call, expression, test)
            }
            Expression::Call(Call::Function(call)) if self.calls_built_in(call, b"count") => {
                self.count(call, expression)
            }
            Expression::Call(Call::Function(call))
                if self.calls_built_in(call, b"array_key_exists") =>
            {
                self.array_key_exists(call, expression)
            }
            Expression::Construct(Construct::Isset(isset)) => self.isset(isset, expression),
            Expression::Call(Call::Function(call)) if self.calls_built_in(call, b"intdiv") => {
                self.intdiv(call, expression)
            }
            Expression::Call(Call::Function(call)) if self.calls_built_in(call, b"fdiv") => {
                self.fdiv(call, expression)
            }
            Expression::Call(Call::Function(call)) if self.calls_built_in(call, b"ctype_digit") => {
                self.ctype_digit(call, expression)
            }
            Expression::Call(Call::Function(call))
                if self.calls_built_in(call, b"str_starts_with") =>
            {
                self.string_search(call, expression, "starts_with")
            }
            Expression::Call(Call::Function(call))
                if self.calls_built_in(call, b"str_ends_with") =>
            {
                self.string_search(call, expression, "ends_with")
            }
            Expression::Call(Call::Function(call))
                if self.calls_built_in(call, b"str_contains") =>
            {
                self.string_search(call, expression, "contains")
            }
            Expression::Call(Call::Function(call)) if self.calls_built_in(call, b"substr") => {
                self.substr(call, expression)
            }
            Expression::Call(Call::Function(call)) if self.calls_built_in(call, b"in_array") => {
                self.in_array(call, expression)
            }
            Expression::Call(Call::Function(call)) if self.string_function_name(call).is_some() => {
                self.string_function(call, expression)
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
            Expression::UnaryPrefix(unary) if unary.operator.is_cast() => self.cast(unary),
            Expression::Literal(Literal::Null(null)) => {
                self.refuse(null, "the type null has no Rust type (P2R-22)");
                None
            }
            Expression::Array(array) => self.list(array, None),
            Expression::ArrayAccess(access) => self.index(access),
            Expression::Literal(Literal::Float(float)) => {
                Some((RExpr::Float(*float.value), RustType::F64))
            }
            Expression::Access(Access::ClassConstant(access)) => {
                self.class_constant(access, expression)
            }
            Expression::Access(Access::Property(access)) => self.property(access, expression),
            Expression::Call(Call::StaticMethod(call)) => {
                match self.static_call(call, expression)? {
                    (call, Some(ty)) => Some((call, ty)),
                    (_, None) => {
                        self.refuse(expression, "a void method has no value (P2R-20)");
                        None
                    }
                }
            }
            Expression::Call(Call::Method(call)) => match self.method_call(call, expression)? {
                (call, Some(ty)) => Some((call, ty)),
                (_, None) => {
                    self.refuse(expression, "a void method has no value (P2R-20)");
                    None
                }
            },
            Expression::Instantiation(instantiation) => {
                self.instantiation(instantiation, expression)
            }
            Expression::Access(Access::NullSafeProperty(_))
            | Expression::Call(Call::NullSafeMethod(_)) => self.nullsafe(expression),
            Expression::Clone(_) => {
                self.record_refused(expression);
                self.refuse(expression, "clone is not supported (P2R-136)");
                None
            }
            Expression::Conditional(node) => self.conditional(node, None),
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
            BinaryOperator::LeftShift(_) => BinOp::Shl,
            BinaryOperator::RightShift(_) => BinOp::Shr,
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
        if let Some(message) = numbers::arithmetic_refusal(op, &left_type, &right_type)
            .or_else(|| numbers::shift_count_refusal(op, &right))
        {
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

    /// `is_scalar($v)` (`scalar`) or `gettype($v)`, folded from the Rust type of the
    /// variable or parameter `$v`; the call does not read it, because the result does not
    /// depend on its value (P2R-222). A union value is read and matched (P2R-233).
    fn type_test(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
        scalar: bool,
    ) -> Option<(RExpr, RustType)> {
        let arguments = self.built_in_arguments(
            call,
            node,
            (1, 1),
            "is_scalar() and gettype() take one argument (P2R-222)",
        )?;
        let ty = match lists::unparenthesized(arguments[0]) {
            Expression::Variable(Variable::Direct(variable)) => self
                .lookup(variable)
                .and_then(|var| self.variables.ty(var).cloned()),
            _ => None,
        };
        let Some(ty) = ty else {
            self.record_refused(arguments[0]);
            self.refuse(
                arguments[0],
                "is_scalar() and gettype() convert only on a variable or parameter whose type is known (P2R-222)",
            );
            return None;
        };
        // A union value holds one of its members, so the call reads it (P2R-233).
        if matches!(ty.inner(), RustType::Union(_)) {
            let test = if scalar {
                unions::Test::Scalar
            } else {
                unions::Test::Type
            };
            return self.union_test(arguments[0], test);
        }
        let name = match &ty {
            RustType::I64 => "integer",
            RustType::F64 => "double",
            RustType::Bool => "boolean",
            RustType::String => "string",
            RustType::Vec(_) | RustType::Shape(_) | RustType::Map(..) => "array",
            RustType::Enum(_) | RustType::Class(_) => "object",
            RustType::Option(_) => {
                self.refuse(arguments[0], nulls::NULL_BUILT_IN);
                return None;
            }
            // Converted above.
            RustType::Union(_) => return None,
        };
        if scalar {
            let is = matches!(name, "integer" | "double" | "boolean" | "string");
            Some((RExpr::Bool(is), RustType::Bool))
        } else {
            Some((RExpr::Str(name.to_owned()), RustType::String))
        }
    }

    /// A call to a function declared in this file, with its return type (`None` for void).
    pub(super) fn call(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, Option<RustType>)> {
        let Expression::Identifier(name) = call.function else {
            self.record_refused(node);
            self.refuse(
                node,
                "only calls by plain function name are supported (P2R-07)",
            );
            return None;
        };
        let key = self.call_key(name);
        let Some(signature) = self.functions.get(&key).cloned() else {
            let text = String::from_utf8_lossy(name.value()).into_owned();
            self.record_refused(node);
            self.refuse(node, format!("calls to {text} are not supported (P2R-48)"));
            return None;
        };
        let arguments = self.arguments(&call.argument_list, &signature.parameters, node)?;
        // rustc counts a call of the function in its own body (P2R-50).
        if self.current_function.as_ref() == Some(&key) {
            self.self_calls.push(call.span());
        }
        let call = RExpr::Call(signature.rust_name, arguments);
        Some((
            self.raised(node, call, signature.throws)?,
            signature.returns,
        ))
    }

    /// The arguments of a call to a function, method or constructor with these
    /// parameters, in parameter order, each converted as for its parameter (P2R-24,
    /// P2R-25, P2R-133). A named argument goes to the parameter of its name, and a
    /// parameter the call leaves out gets its default (P2R-168, P2R-169). `None` when a
    /// problem is recorded.
    pub(super) fn arguments<'arena>(
        &mut self,
        list: &ArgumentList<'arena>,
        parameters: &[ParameterSignature],
        node: &Expression<'_>,
    ) -> Option<Vec<RExpr>> {
        let mut slots: Vec<Option<&'arena Expression<'arena>>> = vec![None; parameters.len()];
        let mut extra = Vec::new();
        let mut converted = true;
        // The first parameter a later argument may go to.
        let mut next = 0;
        let mut named = false;
        for (position, argument) in list.arguments.iter().enumerate() {
            let problem = match argument {
                Argument::Positional(_) if argument.is_unpacked() => {
                    Some("a spread argument is not supported (P2R-169)")
                }
                // PHP rejects the file; Mago does not report it.
                Argument::Positional(_) if named => Some(
                    "a positional argument cannot follow a named one, which PHP rejects (P2R-169)",
                ),
                Argument::Positional(_) => None,
                // Mago reports a name the function does not have (P2R-11).
                Argument::Named(_) => match parameter_index(position, argument, parameters) {
                    Some(index) if index >= next => None,
                    Some(_) => Some(
                        "named arguments must follow the order of the parameters, because PHP runs them in the written order (P2R-169)",
                    ),
                    None => Some("the function has no parameter of this name (P2R-169)"),
                },
            };
            named |= matches!(argument, Argument::Named(_));
            if let Some(message) = problem {
                self.record_refused(argument.value());
                self.refuse(argument, message);
                converted = false;
                continue;
            }
            match parameter_index(position, argument, parameters).filter(|&i| i >= next) {
                Some(index) if index < parameters.len() => {
                    slots[index] = Some(argument.value());
                    next = index + 1;
                }
                _ => extra.push(argument.value()),
            }
        }
        // Named arguments follow the order of the parameters, so this is PHP's order too.
        let mut arguments = Vec::new();
        let mut missing = false;
        for (parameter, slot) in parameters.iter().zip(slots) {
            let passed = match slot {
                Some(value) => self.argument(value, parameter, node),
                None if parameter.default.is_some() => self.default_argument(parameter),
                None => {
                    missing = true;
                    continue;
                }
            };
            match passed {
                Some(expr) => arguments.push(expr),
                None => converted = false,
            }
        }
        // Their problems are reported too. PHP accepts extra arguments to a user function;
        // Rust does not.
        for value in &extra {
            converted &= self.value(value, None).is_some();
        }
        if converted && !extra.is_empty() {
            self.refuse(
                node,
                "a call cannot pass more arguments than the function has parameters (P2R-07)",
            );
            converted = false;
        }
        if converted && missing {
            self.refuse(
                node,
                "a call must pass an argument for each parameter without a default (P2R-07)",
            );
            converted = false;
        }
        converted.then_some(arguments)
    }

    /// The written argument `value` for `parameter`, converted as for it. Mago reports an
    /// argument whose type does not match the parameter (P2R-11).
    fn argument(
        &mut self,
        value: &Expression<'_>,
        parameter: &ParameterSignature,
        node: &Expression<'_>,
    ) -> Option<RExpr> {
        let declared = &parameter.ty;
        let (expr, ty) = self.value(value, Some(declared))?;
        if nulls::null_for_plain(&ty, declared) {
            self.refuse(
                value,
                "a value that may be null cannot go to a parameter that is not nullable (P2R-106)",
            );
            return None;
        }
        let expr = self.declared_value(value, expr, &ty, declared)?;
        // A member value goes into its variant owned, and the variant is lent (P2R-231).
        if let Some(path) = nulls::member_path(&ty, declared) {
            let owned = self.stored(value, expr, &ty, node.span())?;
            return Some(match (parameter.mode, &ty, declared) {
                (Mode::Shared, RustType::Option(_), _) => RExpr::Method(
                    Box::new(RExpr::MethodCall(
                        Box::new(owned),
                        "map".into(),
                        vec![RExpr::Path(path)],
                    )),
                    "as_ref",
                ),
                (Mode::Shared, _, RustType::Option(_)) => RExpr::Some(Box::new(RExpr::Ref(
                    Box::new(RExpr::Call(path, vec![owned])),
                ))),
                (Mode::Shared, _, _) => RExpr::Ref(Box::new(RExpr::Call(path, vec![owned]))),
                _ => nulls::wrapped(owned, &ty, declared),
            });
        }
        let passed = match parameter.mode {
            Mode::Shared => self.borrowed(expr, &ty),
            Mode::Mut => self.lent(value, expr)?,
            Mode::Owned => self.stored(value, expr, &ty, node.span())?,
        };
        Some(nulls::wrapped(passed, &ty, declared))
    }

    /// The default of `parameter`, converted at its declaration, for a call that leaves
    /// it out, passed as an argument for that parameter is: `&[]`, `&mut []`, `Vec::new()`
    /// or `Some(&[])` by the parameter's form (P2R-143, P2R-168).
    fn default_argument(&self, parameter: &ParameterSignature) -> Option<RExpr> {
        let (expr, ty) = parameter.default.clone()?;
        let declared = &parameter.ty;
        // A member literal goes into its variant owned, and the variant is lent (P2R-231).
        if let Some(path) = nulls::member_path(&ty, declared) {
            let variant = RExpr::Call(path, vec![self.owned(expr, &ty)]);
            return Some(match (parameter.mode, declared) {
                (Mode::Shared, RustType::Option(_)) => {
                    RExpr::Some(Box::new(RExpr::Ref(Box::new(variant))))
                }
                (Mode::Shared, _) => RExpr::Ref(Box::new(variant)),
                (_, RustType::Option(_)) => RExpr::Some(Box::new(variant)),
                _ => variant,
            });
        }
        Some(match parameter.mode {
            Mode::Shared => nulls::wrapped(self.borrowed(expr, &ty), &ty, declared),
            Mode::Mut => RExpr::RefMut(Box::new(nulls::wrapped(expr, &ty, declared))),
            Mode::Owned => nulls::wrapped(expr, &ty, declared),
        })
    }

    /// The argument `expr`, converted from `value`, for a parameter that changes its
    /// object: `&mut` before a variable, a chain or `new`; a `&mut` parameter, `self` in a
    /// `&mut self` method and the value of a foreach by `&mut` are references already
    /// (P2R-144).
    fn lent(&mut self, value: &Expression<'_>, expr: RExpr) -> Option<RExpr> {
        if matches!(lists::unparenthesized(value), Expression::Instantiation(_)) {
            return Some(RExpr::RefMut(Box::new(expr)));
        }
        let Some(root) = variables::chain_root(&expr).map(str::to_owned) else {
            self.refuse(
                value,
                "only a variable, a parameter, $this, a foreach value, a chain of property and list reads on one of them, or new can go to a parameter that changes its object (P2R-144)",
            );
            return None;
        };
        if !self.change_root(value, &expr) {
            return None;
        }
        if matches!(expr, RExpr::Var(_)) && self.variables.mode_of(&root) == Mode::Mut {
            Some(expr)
        } else {
            Some(RExpr::RefMut(Box::new(expr)))
        }
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
