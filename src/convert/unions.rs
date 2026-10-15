//! Union types as generated enums: the enum items, and `is_scalar()`, `gettype()` and
//! `(string)` on a union value as a `match` over its variants (P2R-230, P2R-233).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::*;

use super::Converter;
use super::control::emit;
use super::flow::Var;
use super::lists::unparenthesized;
use crate::printer::{self, RExpr};
use crate::types::{self, RustType};

/// The problem of a union value as the argument of a built-in function other than
/// `is_scalar()` and `gettype()` (P2R-232).
pub(super) const UNION_BUILT_IN: &str =
    "a value of a union type cannot go to this built-in function (P2R-232)";

/// What a `match` over the variants of a union value gives (P2R-233).
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Test {
    /// `is_scalar()`.
    Scalar,
    /// `gettype()`.
    Type,
    /// `(string)`.
    Text,
}

/// The problem of a union value where another type is needed (P2R-231).
pub(super) const UNION_PLACE: &str =
    "a value of a union type goes only into a place of the same union type (P2R-231)";

/// True for a value of a union type `found` where `wanted` is not that union. Mago accepts
/// a union value where each member fits, also where it narrows the value with no binding,
/// and PHP may convert a member there (P2R-231, P2R-232).
pub(super) fn misplaced(found: &RustType, wanted: &RustType) -> bool {
    matches!(found.inner(), RustType::Union(name)
        if !matches!(wanted.inner(), RustType::Union(other) if other == name))
}

/// The built-in type tests and the member type each accepts (P2R-251).
const MEMBER_TESTS: [(&[u8], RustType); 4] = [
    (b"is_int", RustType::I64),
    (b"is_float", RustType::F64),
    (b"is_bool", RustType::Bool),
    (b"is_string", RustType::String),
];

/// A type test of P2R-252 on a variable that binds its member value.
pub(super) struct MemberCheck<'e, 'a> {
    pub(super) variable: &'e DirectVariable<'a>,
    pub(super) outer: Var,
    pub(super) member: RustType,
    /// `let U::V(v) = S`.
    pub(super) pattern: String,
    /// `matches!(v, U::V(_))`, for a test that binds nothing.
    pub(super) test: String,
    /// `let v = v.as_str()` after the binding of a string member, which binds a `&String`,
    /// so it reads as a `&str` does.
    pub(super) shadow: Option<String>,
}

/// True for a member type that `is_scalar()` accepts.
fn scalar(ty: &RustType) -> bool {
    matches!(
        ty,
        RustType::I64 | RustType::F64 | RustType::Bool | RustType::String
    )
}

impl Converter<'_> {
    /// The enum of each union the output uses, by name in byte order (P2R-230).
    pub(super) fn union_items(&self) -> Vec<(String, Vec<String>)> {
        self.type_names
            .unions()
            .into_iter()
            .map(|(name, members)| {
                let mut lines = vec![
                    "#[derive(Clone, Debug)]".to_owned(),
                    format!("pub enum {name} {{"),
                ];
                for member in members {
                    lines.push(emit(
                        1,
                        format!("{}({}),", types::variant(&member), member.owned()),
                    ));
                }
                lines.push("}".to_owned());
                (name, lines)
            })
            .collect()
    }

    /// True when every member of the type Mago gives a read, `site`, is a member of the
    /// union enum `name`, or null for a nullable one (P2R-232).
    pub(super) fn within_union(&self, site: &TUnion, name: &str, nullable: bool) -> bool {
        self.type_names
            .union(name)
            .is_some_and(|members| types::within(site, &members, nullable, &self.type_names))
    }

    /// The type test `call` names and the member type it accepts (P2R-251).
    pub(super) fn member_test_of(
        &self,
        call: &FunctionCall<'_>,
    ) -> Option<(&'static str, RustType)> {
        MEMBER_TESTS
            .iter()
            .find(|(name, _)| self.calls_built_in(call, name))
            .map(|(name, member)| {
                (
                    std::str::from_utf8(name).unwrap_or_default(),
                    member.clone(),
                )
            })
    }

    /// `is_int($v)`, `is_float($v)`, `is_bool($v)` or `is_string($v)` on a value of a union
    /// type, as `matches!` on the variant of `member`; false for null (P2R-251).
    pub(super) fn member_test(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
        (function, member): (&str, RustType),
    ) -> Option<(RExpr, RustType)> {
        let arguments = self.built_in_arguments(
            call,
            node,
            (1, 1),
            &format!("{function}() converts with one argument only (P2R-251)"),
        )?;
        let (expr, ty) = self.expression(arguments[0])?;
        let RustType::Union(name) = ty.inner() else {
            self.refuse(
                node,
                format!("{function}() converts only on a value of a union type (P2R-251)"),
            );
            return None;
        };
        if !self
            .type_names
            .union(name)
            .is_some_and(|members| members.contains(&member))
        {
            self.refuse(
                node,
                format!("the union {name} has no member that {function}() accepts (P2R-251)"),
            );
            return None;
        }
        let mut pattern = format!("{name}::{}(_)", types::variant(&member));
        if matches!(ty, RustType::Option(_)) {
            pattern = format!("Some({pattern})");
        }
        let subject = printer::expr(&expr, false);
        let call = RExpr::Call(
            "matches!".to_owned(),
            vec![RExpr::Path(subject), RExpr::Path(pattern)],
        );
        Some((call, RustType::Bool))
    }

    /// `is_T($v)`, or `!is_T($v)` when not `positive`, on a variable or parameter of a union
    /// type with a member that the test accepts: the check that binds the member value
    /// (P2R-252).
    pub(super) fn member_checked<'e, 'a>(
        &self,
        operand: &'e Expression<'a>,
        positive: bool,
    ) -> Option<MemberCheck<'e, 'a>> {
        let tested = match (unparenthesized(operand), positive) {
            (Expression::Call(Call::Function(call)), true) => call,
            (Expression::UnaryPrefix(unary), false)
                if matches!(unary.operator, UnaryPrefixOperator::Not(_)) =>
            {
                match unparenthesized(unary.operand) {
                    Expression::Call(Call::Function(call)) => call,
                    _ => return None,
                }
            }
            _ => return None,
        };
        let (_, member) = self.member_test_of(tested)?;
        let [Argument::Positional(argument)] = tested.argument_list.arguments.as_slice() else {
            return None;
        };
        let Expression::Variable(Variable::Direct(variable)) = unparenthesized(argument.value)
        else {
            return None;
        };
        let outer = self.lookup(variable)?;
        let ty = self.variables.ty(outer)?;
        let RustType::Union(union) = ty.inner() else {
            return None;
        };
        if !self
            .type_names
            .union(union)
            .is_some_and(|members| members.contains(&member))
        {
            return None;
        }
        let nullable = matches!(ty, RustType::Option(_));
        let name = self.variables.name_of(outer);
        let path = format!("{union}::{}", types::variant(&member));
        // A test never moves the value: a member that is not Copy is lent (P2R-252).
        let (copy, borrowed) = (member.is_copy(), self.variables.is_borrowed(name));
        let (pattern, scrutinee) = match (nullable, borrowed, copy) {
            (false, false, true) | (false, true, false) => {
                (format!("{path}({name})"), name.to_owned())
            }
            (false, false, false) => (format!("{path}({name})"), format!("&{name}")),
            (false, true, true) => (format!("{path}({name})"), format!("*{name}")),
            (true, false, true) | (true, true, false) => {
                (format!("Some({path}({name}))"), name.to_owned())
            }
            (true, false, false) => (format!("Some({path}({name}))"), format!("&{name}")),
            (true, true, true) => (format!("Some(&{path}({name}))"), name.to_owned()),
        };
        let blank = if nullable {
            format!("Some({path}(_))")
        } else {
            format!("{path}(_)")
        };
        let shadow = (member == RustType::String).then(|| format!("let {name} = {name}.as_str()"));
        Some(MemberCheck {
            variable,
            outer,
            member,
            pattern: format!("let {pattern} = {scrutinee}"),
            test: format!("matches!({name}, {blank})"),
            shadow,
        })
    }

    /// The PHP name of `$v` when `condition` is `is_scalar($v)` and `then` is `(string) $v`,
    /// for a variable or parameter `$v` (P2R-233).
    pub(super) fn scalar_checked(
        &self,
        condition: &Expression<'_>,
        then: &Expression<'_>,
    ) -> Option<Vec<u8>> {
        let Expression::Call(Call::Function(call)) = unparenthesized(condition) else {
            return None;
        };
        if !self.calls_built_in(call, b"is_scalar") {
            return None;
        }
        let [argument] = call.argument_list.arguments.as_slice() else {
            return None;
        };
        let Expression::UnaryPrefix(UnaryPrefix {
            operator: UnaryPrefixOperator::StringCast(..),
            operand,
        }) = unparenthesized(then)
        else {
            return None;
        };
        match (unparenthesized(argument.value()), unparenthesized(operand)) {
            (
                Expression::Variable(Variable::Direct(checked)),
                Expression::Variable(Variable::Direct(cast)),
            ) if checked.name == cast.name && matches!(argument, Argument::Positional(_)) => {
                Some(checked.name.to_vec())
            }
            _ => None,
        }
    }

    /// True when `operand` is a variable or parameter of type `U` or `Option<U>` (P2R-233).
    pub(super) fn union_operand(&self, operand: &Expression<'_>) -> bool {
        let Expression::Variable(Variable::Direct(variable)) =
            super::lists::unparenthesized(operand)
        else {
            return false;
        };
        self.lookup(variable)
            .and_then(|var| self.variables.ty(var))
            .is_some_and(|ty| matches!(ty.inner(), RustType::Union(_)))
    }

    /// `is_scalar($v)`, `gettype($v)` or `(string) $v` on a union value as a `match` over its
    /// variants (P2R-233). Only `(string) $v` as the first arm of `is_scalar($v) ? .. : ..`
    /// writes `unreachable!()`, for the variants and the null that the check rules out.
    /// `None` means a problem was recorded.
    pub(super) fn union_test(
        &mut self,
        operand: &Expression<'_>,
        test: Test,
    ) -> Option<(RExpr, RustType)> {
        let guarded = test == Test::Text
            && matches!(unparenthesized(operand), Expression::Variable(Variable::Direct(variable))
                if self.scalar_guard.as_deref() == Some(variable.name));
        let (expr, ty) = self.expression(operand)?;
        let RustType::Union(name) = ty.inner() else {
            return None;
        };
        let members = self.type_names.union(name).unwrap_or_default();
        let nullable = matches!(ty, RustType::Option(_));
        let subject = match &expr {
            RExpr::Var(var) if self.variables.is_borrowed(var) => var.clone(),
            _ => format!("&{}", printer::expr(&expr, false)),
        };
        let pattern = |variant: &str, binding: &str| {
            let inner = format!("{name}::{variant}({binding})");
            if nullable {
                format!("Some({inner})")
            } else {
                inner
            }
        };
        if test == Test::Scalar {
            let patterns: Vec<String> = members
                .iter()
                .filter(|member| scalar(member))
                .map(|member| pattern(&types::variant(member), "_"))
                .collect();
            // A macro call needs no parentheses, wherever it stands (P2R-43).
            let call = RExpr::Call(
                "matches!".to_owned(),
                vec![RExpr::Path(subject), RExpr::Path(patterns.join(" | "))],
            );
            return Some((call, RustType::Bool));
        }
        // `(string)` gives an owned string when an int needs `to_string()`.
        let owned = test == Test::Text && members.contains(&RustType::I64);
        let mut arms: Vec<(Vec<String>, String)> = Vec::new();
        for member in &members {
            let variant = types::variant(member);
            let (binding, result) = match (test, member) {
                (Test::Type, _) => ("_", format!("\"{}\"", php_type(member))),
                (_, RustType::I64) => ("x", "x.to_string()".to_owned()),
                (_, RustType::Bool) if owned => (
                    "x",
                    "String::from(if *x { \"1\" } else { \"\" })".to_owned(),
                ),
                (_, RustType::Bool) => ("x", "if *x { \"1\" } else { \"\" }".to_owned()),
                (_, RustType::String) if owned => ("x", "x.clone()".to_owned()),
                (_, RustType::String) => ("x", "x.as_str()".to_owned()),
                (_, RustType::F64) => {
                    self.refuse(
                        operand,
                        "(string) of a union value that may hold a float is not supported, because PHP prints a float its own way (P2R-233)",
                    );
                    return None;
                }
                _ if guarded => ("_", "unreachable!()".to_owned()),
                _ => {
                    self.refuse(
                        operand,
                        "(string) of a union value that may hold a list, a shape, an enum or an object converts only as the first arm of is_scalar() ? : (P2R-233)",
                    );
                    return None;
                }
            };
            add_arm(&mut arms, pattern(&variant, binding), result);
        }
        if nullable {
            let result = match test {
                Test::Type => "\"NULL\"".to_owned(),
                _ if guarded => "unreachable!()".to_owned(),
                _ if owned => "String::new()".to_owned(),
                _ => "\"\"".to_owned(),
            };
            add_arm(&mut arms, "None".to_owned(), result);
        }
        let mut text = format!("match {subject} {{\n");
        for (patterns, result) in arms {
            text.push_str(&emit(1, format!("{} => {result},\n", patterns.join(" | "))));
        }
        text.push('}');
        Some((RExpr::Variants(text, !owned), RustType::String))
    }
}

/// The problem of a declared type whose PHP type `hint` and Mago type `union` disagree on
/// int and float, or that PHP rejects while Mago accepts it (P2R-25, P2R-230).
/// `resolve` gives the lowercase fully qualified name of a class name in the hint, or of
/// the class whose body holds it for `self` (`None`).
pub(super) fn native_problem(
    hint: &Hint<'_>,
    union: &TUnion,
    resolve: &dyn Fn(Option<&Identifier<'_>>) -> Option<Vec<u8>>,
) -> Option<&'static str> {
    let mut written = Vec::new();
    hint_names(hint, resolve, &mut written);
    let has = |name: &str| written.iter().any(|w| w == name);
    if written
        .iter()
        .enumerate()
        .any(|(at, name)| written[..at].contains(name))
    {
        return Some(
            "PHP rejects a type written twice in one declaration, and Mago does not report it (P2R-230)",
        );
    }
    if (has("true") && has("false")) || (has("bool") && (has("true") || has("false"))) {
        return Some(
            "PHP rejects true|false and bool beside true or false, and Mago does not report it (P2R-230)",
        );
    }
    let (mut int, mut float) = (false, false);
    for atomic in union.types.iter() {
        match atomic {
            TAtomic::Scalar(TScalar::Integer(_) | TScalar::ArrayKey) => int = true,
            TAtomic::Scalar(TScalar::Float(_)) => float = true,
            TAtomic::Scalar(TScalar::Generic | TScalar::Numeric) => (int, float) = (true, true),
            _ => {}
        }
    }
    if int && has("float") && !has("int") {
        return Some(
            "PHP stores an int as a float for this declared type, while its docblock type holds int (P2R-230)",
        );
    }
    if float && (!has("float") || (!int && has("int"))) {
        return Some(
            "PHP keeps an int where only a docblock gives float, which Rust would turn into a float (P2R-25)",
        );
    }
    None
}

/// The lowercase names a PHP type hint writes, through `?`, `( )` and `|`, a class name
/// resolved as PHP resolves it, so `Foo|\App\Foo` names one class twice.
fn hint_names(
    hint: &Hint<'_>,
    resolve: &dyn Fn(Option<&Identifier<'_>>) -> Option<Vec<u8>>,
    written: &mut Vec<String>,
) {
    let name = match hint {
        Hint::Nullable(nullable) => return hint_names(nullable.hint, resolve, written),
        Hint::Parenthesized(inner) => return hint_names(inner.hint, resolve, written),
        Hint::Union(union) => {
            hint_names(union.left, resolve, written);
            return hint_names(union.right, resolve, written);
        }
        Hint::Identifier(identifier) => match resolve(Some(identifier)) {
            Some(name) => String::from_utf8_lossy(&name).into_owned(),
            None => return,
        },
        // `self` names the class whose body holds it (P2R-154).
        Hint::Self_(_) => match resolve(None) {
            Some(name) => String::from_utf8_lossy(&name).into_owned(),
            None => return,
        },
        Hint::Integer(_) => "int".to_owned(),
        Hint::Float(_) => "float".to_owned(),
        Hint::Bool(_) => "bool".to_owned(),
        Hint::String(_) => "string".to_owned(),
        Hint::True(_) => "true".to_owned(),
        Hint::False(_) => "false".to_owned(),
        Hint::Null(_) => "null".to_owned(),
        Hint::Array(_) => "array".to_owned(),
        Hint::Mixed(_) => "mixed".to_owned(),
        Hint::Object(_) => "object".to_owned(),
        Hint::Iterable(_) => "iterable".to_owned(),
        Hint::Callable(_) => "callable".to_owned(),
        _ => return,
    };
    written.push(name);
}

/// Adds the arm `pattern => result`, joined to an earlier arm of the same result when
/// neither binds a value.
fn add_arm(arms: &mut Vec<(Vec<String>, String)>, pattern: String, result: String) {
    let binds = |pattern: &str| !pattern.contains("(_)") && pattern != "None";
    if !binds(&pattern)
        && let Some((patterns, _)) = arms
            .iter_mut()
            .find(|(patterns, known)| *known == result && !patterns.iter().any(|p| binds(p)))
    {
        patterns.push(pattern);
        return;
    }
    arms.push((vec![pattern], result));
}

/// The name `gettype()` gives a value of a member type (P2R-233).
fn php_type(member: &RustType) -> &'static str {
    match member {
        RustType::I64 => "integer",
        RustType::F64 => "double",
        RustType::Bool => "boolean",
        RustType::String => "string",
        RustType::Vec(_) | RustType::Shape(_) | RustType::Map(..) => "array",
        RustType::Enum(_) | RustType::Class(_) | RustType::Union(_) => "object",
        RustType::Option(inner) => php_type(inner),
    }
}
