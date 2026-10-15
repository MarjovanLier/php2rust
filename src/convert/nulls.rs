//! Values that may be null: `Some` where an `Option` is needed, comparisons with `null`
//! and between `Option` values, the bindings of null checks, and `??` (P2R-102 to
//! P2R-105).

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::flow::Var;
use super::variables::Part;
use super::{Converter, booleans, lists, shapes};
use crate::printer::{self, BinOp, RExpr, Raise};
use crate::types::{self, RustType};

/// The problem of a value that may be null as an element of a list of plain values
/// (P2R-106).
pub(super) const NULL_ELEMENT: &str =
    "a value that may be null cannot go into a list of values that cannot be null (P2R-106)";

/// The check a let-else binds: the variable, its id, the type it binds, and its pattern
/// with the line after it (P2R-104, P2R-252).
pub(super) type Guard<'e, 'a> = (
    &'e DirectVariable<'a>,
    Var,
    RustType,
    (String, Option<String>),
);

/// The problem of a value that may be null as the argument of a built-in function
/// (P2R-106).
pub(super) const NULL_BUILT_IN: &str =
    "a value that may be null cannot go to a built-in function (P2R-106)";

/// The problem of a value that may be null joined into a string, which PHP joins as an
/// empty string (P2R-106).
pub(super) const NULL_JOINED: &str =
    "a value that may be null cannot be joined into a string (P2R-106)";

/// `expr`, of type `ty`, where a value of type `target` is needed: a `T` value becomes
/// `Some(..)` where an `Option<T>` is needed (P2R-102), and a value of a member of a union
/// enum `U` becomes `U::V(..)`, or `.map(U::V)` for an `Option` (P2R-231).
pub(super) fn wrapped(expr: RExpr, ty: &RustType, target: &RustType) -> RExpr {
    // A value of `T` where `T|Optional` is due is there (P2R-361).
    if let RustType::Optional(inner) = target
        && ty != target
    {
        return RExpr::Call(
            format!("{}::Present", crate::types::OPTIONAL),
            vec![wrapped(expr, ty, inner)],
        );
    }
    // `false` where `DateTime|false` is needed is its `None`; Mago lets no other bool in.
    // Any other value is still read, so a call runs and a variable counts as used (P2R-330).
    if date_or_false(target) && *ty == RustType::Bool {
        return match expr {
            RExpr::Bool(false) => RExpr::None,
            other => RExpr::Raw(format!(
                "{{ let _ = {}; None }}",
                printer::expr(&other, false)
            )),
        };
    }
    let expr = match member_path(ty, target) {
        Some(path) if matches!(ty, RustType::Option(_)) => {
            return RExpr::MethodCall(Box::new(expr), "map".into(), vec![RExpr::Path(path)]);
        }
        Some(path) => RExpr::Call(path, vec![expr]),
        None => expr,
    };
    if matches!(target, RustType::Option(_)) && !matches!(ty, RustType::Option(_)) {
        RExpr::Some(Box::new(expr))
    } else {
        expr
    }
}

/// `U::V`, the variant of the union enum `U` of `target` that holds a value of `ty`, when
/// `ty` is a member type rather than the union itself (P2R-231).
pub(super) fn member_path(ty: &RustType, target: &RustType) -> Option<String> {
    match (target.inner(), ty.inner()) {
        (RustType::Union(_), RustType::Union(_)) => None,
        (RustType::Union(name), member) => Some(format!("{name}::{}", types::variant(member))),
        _ => None,
    }
}

/// True for `DateTime|false`, an `Option` in Rust whose `None` is PHP's `false`, not `null`
/// (P2R-330).
pub(super) fn date_or_false(ty: &RustType) -> bool {
    matches!(ty, RustType::Option(inner) if **inner == RustType::DateTime)
}

/// The problem of a check for null on such a value (P2R-330).
pub(super) const DATE_NOT_NULL: &str =
    "a DateTime|false value holds false, not null, so ?? and ?-> do not apply to it (P2R-330)";

/// True for the literal `null`.
pub(super) fn is_null(expression: &Expression<'_>) -> bool {
    matches!(
        lists::unparenthesized(expression),
        Expression::Literal(Literal::Null(_))
    )
}

/// True when a value that may be null goes where a value that cannot be null is needed
/// (P2R-106).
pub(super) fn null_for_plain(ty: &RustType, target: &RustType) -> bool {
    // A value of `?T` goes into `Present` of `T|Optional|null` (P2R-361).
    let target = match target {
        RustType::Optional(inner) => inner,
        other => other,
    };
    matches!(ty, RustType::Option(_)) && !matches!(target, RustType::Option(_))
}

/// True for a value with no side effect that cannot panic, so the default of `??` may run
/// whether or not it is needed (P2R-105).
fn simple(expr: &RExpr) -> bool {
    match expr {
        RExpr::Int(_)
        | RExpr::Float(_)
        | RExpr::Bool(_)
        | RExpr::Str(_)
        | RExpr::Var(_)
        | RExpr::Known(..)
        | RExpr::Path(_) => true,
        RExpr::Neg(inner) => matches!(**inner, RExpr::Int(_) | RExpr::Float(_)),
        _ => false,
    }
}

/// The operands of the top-level `&&` chain of a condition, left to right.
fn and_operands<'e, 'a>(condition: &'e Expression<'a>) -> Vec<&'e Expression<'a>> {
    match condition {
        Expression::Binary(binary) if matches!(binary.operator, BinaryOperator::And(_)) => {
            let mut operands = and_operands(binary.lhs);
            operands.extend(and_operands(binary.rhs));
            operands
        }
        other => vec![other],
    }
}

/// The side of `binary` that is not the literal `null`, when exactly one side is.
fn beside_null<'e, 'a>(binary: &'e Binary<'a>) -> Option<&'e Expression<'a>> {
    match (is_null(binary.lhs), is_null(binary.rhs)) {
        (true, false) => Some(binary.rhs),
        (false, true) => Some(binary.lhs),
        _ => None,
    }
}

impl Converter<'_> {
    /// A comparison with the literal `null`: `=== null` becomes `is_none()` and `!== null`
    /// `is_some()` (P2R-103). `None` when no side is `null`, so the comparison converts as
    /// any other.
    pub(super) fn null_comparison(
        &mut self,
        binary: &Binary<'_>,
        op: BinOp,
        strict: bool,
    ) -> Option<Option<(RExpr, RustType)>> {
        let value = beside_null(binary)?;
        if !strict || !matches!(op, BinOp::Eq | BinOp::Ne) {
            self.record_refused(value);
            self.refuse(
                binary,
                "only === and !== compare with null, because PHP's null == 0 is true (P2R-103)",
            );
            return Some(None);
        }
        // A value that does not convert has its problem recorded already.
        let Some((expr, ty)) = self.expression(value) else {
            return Some(None);
        };
        if matches!(ty, RustType::Optional(_)) {
            self.refuse(binary, super::optionals::OPTIONAL_USE);
            return Some(None);
        }
        if !matches!(ty, RustType::Option(_)) || date_or_false(&ty) {
            self.refuse(
                binary,
                "this value cannot be null, so the comparison with null always gives the same result (P2R-103)",
            );
            return Some(None);
        }
        let test = if op == BinOp::Eq {
            "is_none"
        } else {
            "is_some"
        };
        Some(Some((RExpr::Method(Box::new(expr), test), RustType::Bool)))
    }

    /// `===` and `!==` where one side or both may be null: both sides compare as `Option`
    /// values, a string side as `Option<&str>` (P2R-103).
    pub(super) fn option_comparison(
        &mut self,
        binary: &Binary<'_>,
        op: BinOp,
        strict: bool,
        (left, left_type): (RExpr, RustType),
        (right, right_type): (RExpr, RustType),
    ) -> Option<(RExpr, RustType)> {
        if !strict || !matches!(op, BinOp::Eq | BinOp::Ne) {
            self.refuse(
                binary,
                "only === and !== compare values that may be null (P2R-103)",
            );
            return None;
        }
        let (inner, other) = (left_type.inner(), right_type.inner());
        if let Some(message) = booleans::refusal(op, strict, inner, other) {
            self.refuse(binary, message);
            return None;
        }
        let left = self.compared(left, &left_type);
        let right = self.compared(right, &right_type);
        Some((
            RExpr::Binary(Box::new(left), op, Box::new(right)),
            RustType::Bool,
        ))
    }

    /// The opening line of an `if` or `elseif` whose condition has `$v !== null`, on a
    /// variable or parameter that may be null, as an operand of its top-level `&&` chain.
    /// Each such check binds the inner value for the later operands and the body
    /// (P2R-104). Gives the pending line and the bindings, which end with the branch;
    /// `None` when the condition has no such check.
    pub(super) fn checked_condition(
        &mut self,
        condition: &Expression<'_>,
        keyword: &'static str,
        open: &'static str,
        depth: usize,
    ) -> Option<(String, Vec<Var>)> {
        let operands = and_operands(condition);
        if !operands
            .iter()
            .any(|o| self.null_checked(o, true).is_some() || self.member_checked(o, true).is_some())
        {
            return None;
        }
        let mut parts = Vec::new();
        let mut bindings = Vec::new();
        let mut converted = true;
        for operand in operands {
            // `is_T($v)` on a union value binds its member as `$v !== null` binds (P2R-252).
            let check = self
                .null_checked(operand, true)
                .map(|(variable, outer, inner)| {
                    let pattern = self.pattern(outer, &inner);
                    let test = format!("{}.is_some()", self.variables.name_of(outer));
                    (variable, outer, inner, pattern, test)
                })
                .or_else(|| {
                    self.member_checked(operand, true).map(|check| {
                        let pattern = match check.shadow {
                            Some(shadow) => format!("{} && {shadow}", check.pattern),
                            None => check.pattern,
                        };
                        (
                            check.variable,
                            check.outer,
                            check.member,
                            pattern,
                            check.test,
                        )
                    })
                });
            if let Some((variable, outer, inner, pattern, test)) = check {
                self.variables.read(outer, variable.span);
                let binding = self.bind_null(variable, inner);
                bindings.push(binding);
                parts.push(Part::Let {
                    binding,
                    pattern,
                    test,
                });
                continue;
            }
            let Some(expr) = self.condition(operand) else {
                converted = false;
                continue;
            };
            let text = printer::expr(&expr, false);
            let text = if matches!(expr, RExpr::Binary(_, BinOp::Or, _)) {
                format!("({text})")
            } else {
                text
            };
            parts.push(Part::Plain(text));
        }
        // A problem is recorded for every operand that did not convert.
        let line = if converted {
            self.variables.condition_line(depth, keyword, parts, open)
        } else {
            String::new()
        };
        Some((line, bindings))
    }

    /// The check of `if ($v === null)` or `if (!is_T($v))` that a let-else binds when its
    /// body always leaves, with its pattern and the line after it (P2R-104, P2R-252).
    pub(super) fn guard_check<'e, 'a>(
        &self,
        condition: &'e Expression<'a>,
    ) -> Option<Guard<'e, 'a>> {
        if let Some((variable, outer, inner)) = self.null_checked(condition, false) {
            let pattern = self.pattern(outer, &inner);
            return Some((variable, outer, inner, (pattern, None)));
        }
        self.member_checked(condition, false).map(|check| {
            let lines = (check.pattern, check.shadow);
            (check.variable, check.outer, check.member, lines)
        })
    }

    /// `let Some(v) = v`, the pattern that binds the inner value of `outer` (P2R-104). An
    /// owned `Option<String>`, `Option<Vec<T>>` or `Option<C>` lends its value, so a later
    /// read of the variable still has it (P2R-204).
    pub(super) fn pattern(&self, outer: Var, inner: &RustType) -> String {
        let name = self.variables.name_of(outer);
        if inner.is_copy() || self.variables.is_borrowed(name) {
            format!("let Some({name}) = {name}")
        } else {
            format!("let Some({name}) = {name}.{}()", inner.lending())
        }
    }

    /// The variable or parameter of a `$v !== null` operand (`$v === null` when not
    /// `negated`), with its id and inner type, when its Rust type is `Option` (P2R-104).
    pub(super) fn null_checked<'e, 'a>(
        &self,
        operand: &'e Expression<'a>,
        negated: bool,
    ) -> Option<(&'e DirectVariable<'a>, Var, RustType)> {
        // A check of a `DateTime|false` value binds as a null check does (P2R-333).
        if let Some(found) = self.date_checked(operand, negated) {
            return Some(found);
        }
        let Expression::Binary(binary) = lists::unparenthesized(operand) else {
            return None;
        };
        let operator = if negated {
            matches!(binary.operator, BinaryOperator::NotIdentical(_))
        } else {
            matches!(binary.operator, BinaryOperator::Identical(_))
        };
        if !operator {
            return None;
        }
        let Expression::Variable(Variable::Direct(variable)) =
            lists::unparenthesized(beside_null(binary)?)
        else {
            return None;
        };
        let outer = self.lookup(variable)?;
        let ty = self.variables.ty(outer)?;
        let RustType::Option(inner) = ty else {
            return None;
        };
        if date_or_false(ty) {
            return None;
        }
        Some((variable, outer, (**inner).clone()))
    }

    /// `$a ?? $b`, where `$a` is a variable, parameter, call or shape element that may be
    /// null and `$b` has its inner type. `$b` runs only when `$a` is null, as in PHP
    /// (P2R-105). A shape always has its keys, so PHP's isset of an element is a check
    /// against null (P2R-203).
    pub(super) fn coalesce(&mut self, binary: &Binary<'_>) -> Option<(RExpr, RustType)> {
        let left_node = lists::unparenthesized(binary.lhs);
        if let Expression::Call(Call::Function(call)) = left_node
            && self.calls_built_in(call, b"preg_replace")
        {
            return self.preg_replace(call, left_node, binary.rhs);
        }
        if let Expression::Call(Call::Function(call)) = left_node
            && self.calls_built_in(call, b"preg_replace_callback")
        {
            return self.preg_replace_callback(call, left_node, binary.rhs);
        }
        // A map variable keeps its type where Mago gives it a shape (P2R-240).
        let holder = match left_node {
            Expression::ArrayAccess(access) => match self.chain_type(access.array) {
                Some(map @ RustType::Map(..)) => Some(map),
                _ => self.site_type(access.array).ok(),
            },
            _ => None,
        };
        let shape = matches!(holder, Some(RustType::Shape(_)));
        // `coalesce_left` refuses a map that is not in a variable, parameter or property.
        let map = matches!(holder, Some(RustType::Map(..)));
        let element = (shape && shapes::no_list_read(left_node)) || map;
        if !element
            && !matches!(
                left_node,
                Expression::Variable(Variable::Direct(_))
                    | Expression::Call(
                        Call::Function(_) | Call::StaticMethod(_) | Call::NullSafeMethod(_)
                    )
                    // `?->` gives an owned `Option`, as a call does (P2R-234).
                    | Expression::Access(Access::NullSafeProperty(_))
            )
        {
            self.record_refused(binary.lhs);
            self.record_refused(binary.rhs);
            let message = if matches!(
                left_node,
                Expression::Access(Access::Property(_)) | Expression::Call(Call::Method(_))
            ) {
                "?? on a property read or a method call is not supported yet (P2R-134)"
            } else if shape {
                "?? on an array shape element converts only when no list element is read on the way to it (P2R-203)"
            } else if matches!(&holder, Some(RustType::Option(inner)) if inner.is_struct()) {
                "a value that may be null cannot be read by key (P2R-106)"
            } else {
                "?? converts on a variable, parameter, call or array shape element only; PHP checks anything else with isset (P2R-105)"
            };
            self.refuse(binary, message);
            return None;
        }
        if let Expression::Throw(throw) = lists::unparenthesized(binary.rhs) {
            return self.coalesce_throw(binary, left_node, throw);
        }
        // Both sides convert first, so problems on both sides are reported. A lazy default
        // runs in a closure, where a call that throws has nowhere to go (P2R-213).
        let left = self.coalesce_left(binary.lhs);
        let outer = self.raise.take();
        let right = self.expression(binary.rhs);
        self.raise = outer;
        let ((left, left_type), (right, right_type)) = (left?, right?);
        let RustType::Option(inner) = &left_type else {
            self.refuse(binary, "the left side of ?? cannot be null here (P2R-105)");
            return None;
        };
        if date_or_false(&left_type) {
            self.refuse(binary, DATE_NOT_NULL);
            return None;
        }
        if right_type != **inner {
            self.refuse(
                binary,
                format!(
                    "the right side of ?? must have the type {} here (P2R-105)",
                    inner.owned()
                ),
            );
            return None;
        }
        let lazy = !simple(&right);
        let expr = if inner.is_copy() {
            RExpr::Unwrap {
                option: Box::new(left),
                default: Box::new(right),
                lazy,
            }
        } else if **inner == RustType::String
            && (matches!(right, RExpr::Str(_) | RExpr::Var(_)) || self.lends(&right))
        {
            // Both sides are borrowed, so neither is copied. The result is a `&str`, which
            // `owned` copies where a `String` is needed. A default that is itself such a
            // `??` (`$p ?? $q ?? 'z'`) is borrowed too.
            RExpr::Unwrap {
                option: Box::new(self.borrowed(left, &left_type)),
                default: Box::new(self.borrowed(right, &right_type)),
                lazy,
            }
        } else {
            let option = self.owned_option(left, left_node, binary.span());
            let default = self.stored(binary.rhs, right, &right_type, binary.span())?;
            RExpr::Unwrap {
                option: Box::new(option),
                default: Box::new(default),
                lazy,
            }
        };
        Some((expr, (**inner).clone()))
    }

    /// `$a ?? throw new C(..)` as `a.ok_or_else(|| PhpException::new(..))?` (P2R-215).
    fn coalesce_throw(
        &mut self,
        binary: &Binary<'_>,
        left_node: &Expression<'_>,
        throw: &Throw<'_>,
    ) -> Option<(RExpr, RustType)> {
        if self.variables.closure_start.is_some() {
            self.record_refused(binary.lhs);
            self.record_refused(binary.rhs);
            self.refuse(
                throw,
                "a throw cannot stand in an arrow function, whose ? would leave the closure (P2R-300)",
            );
            return None;
        }
        let left = self.coalesce_left(binary.lhs);
        let exception = self.thrown_lazily(throw);
        let ((left, left_type), exception) = (left?, exception?);
        let RustType::Option(inner) = &left_type else {
            self.refuse(binary, "the left side of ?? cannot be null here (P2R-105)");
            return None;
        };
        let option = if inner.is_copy() {
            left
        } else {
            self.owned_option(left, left_node, binary.span())
        };
        let closure = RExpr::Raw(format!("|| {}", printer::expr(&exception, false)));
        let call = RExpr::MethodCall(Box::new(option), "ok_or_else".to_owned(), vec![closure]);
        Some((
            RExpr::Raised(Box::new(call), Raise::Propagate),
            (**inner).clone(),
        ))
    }

    /// The owned option that a method taking it by value needs: a variable moves unless it
    /// is read again, a parameter `Option<&T>` is copied, and a field, which Rust cannot
    /// move out of its struct, is copied (P2R-26, P2R-105).
    fn owned_option(&mut self, left: RExpr, left_node: &Expression<'_>, statement: Span) -> RExpr {
        match (left, left_node) {
            (field @ RExpr::Field(..), _) => RExpr::Method(Box::new(field), "clone"),
            (RExpr::Var(name), _) if self.variables.is_borrowed(&name) => {
                RExpr::MapOwned(Box::new(RExpr::Var(name)))
            }
            (RExpr::Var(name), Expression::Variable(Variable::Direct(variable))) => {
                let mark = self.variables.copy_mark(variable.span, statement, false);
                RExpr::Var(format!("{name}{mark}"))
            }
            (other, _) => other,
        }
    }

    /// True for a string value that Rust holds as a `&str`: the result of `??` when both
    /// sides are borrowed (P2R-105), of `?:` (P2R-180), and an enum's `name()` or string
    /// `value()`, which return `&'static str` (P2R-113). Callers ask only about string
    /// values.
    pub(super) fn lends(&self, expr: &RExpr) -> bool {
        match expr {
            RExpr::Method(_, "name" | "value") | RExpr::If(..) => true,
            // A `match` over a union value that gives literals or borrowed text (P2R-233).
            RExpr::Variants(_, lends) => *lends,
            // A string constant is a `&'static str`; any other path is a Copy enum case
            // (P2R-156).
            RExpr::Path(_) => true,
            RExpr::Unwrap { option, .. } => match &**option {
                RExpr::Method(_, "as_deref") => true,
                RExpr::Var(name) => self.variables.is_borrowed(name),
                _ => false,
            },
            _ => false,
        }
    }

    /// One side of an `Option` comparison as an `Option`, with a string inside borrowed.
    fn compared(&self, expr: RExpr, ty: &RustType) -> RExpr {
        match ty {
            RustType::Option(inner) if **inner == RustType::String => self.borrowed(expr, ty),
            RustType::Option(_) => expr,
            RustType::String => {
                let text = match expr {
                    RExpr::Str(_) => expr,
                    RExpr::Var(ref name) if self.variables.is_borrowed(name) => expr,
                    // The `&str` that `??` gives is borrowed already (P2R-105).
                    lent if self.lends(&lent) => lent,
                    other => RExpr::Method(Box::new(other), "as_str"),
                };
                RExpr::Some(Box::new(text))
            }
            _ => RExpr::Some(Box::new(expr)),
        }
    }
}
