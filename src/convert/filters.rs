//! Conditional filters of a store query, `when($value, <callback>)` (P2R-450 to P2R-453).

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::classes::SUPERGLOBALS;
use super::lists::unparenthesized;
use super::models::{CONDITIONABLE, StoreRules, StoreTest, TestValue, steady, variables_read};
use super::{Converter, ParameterSignature};
use crate::names;
use crate::printer::{BinOp, RExpr};
use crate::types::{self, RustType};

/// Eloquent's query builder, which the callback's first parameter is, lowercase.
const BUILDER: &[u8] = b"illuminate\\database\\eloquent\\builder";

/// One `when` link, before the store closure's names are chosen (P2R-452).
pub(super) struct WhenTest {
    /// The value, lent as an argument is.
    value: RExpr,
    /// True for an `Option` value, which converts with `is_none_or`.
    option: bool,
    /// The value's type without null, which the callback's second parameter has.
    inner: RustType,
    /// The name the binding of the value starts from.
    pub(super) base: String,
    /// The tests of the callback's `where` calls, in source order.
    tests: Vec<StoreTest>,
}

impl WhenTest {
    /// The names the value and the tests read.
    pub(super) fn words(&self) -> Vec<String> {
        let mut words = variables_read(&self.value);
        words.extend(self.tests.iter().flat_map(StoreTest::words));
        words
    }

    /// `v.is_none_or(|b| <falsy> || <tests>)` for an `Option` value, and
    /// `<falsy> || <tests>` with the value itself for any other (P2R-452).
    pub(super) fn rendered(self, row: &str, text: &str, binding: &str) -> RExpr {
        // rustc warns about a closure parameter that nothing reads.
        let reads = falsy(&self.inner, &RExpr::Bool(true)).is_some()
            || self.tests.iter().any(StoreTest::binds);
        let binding = if reads { binding } else { "_" };
        let bound = if self.option {
            RExpr::Var(binding.to_owned())
        } else {
            self.value.clone()
        };
        let tests = self
            .tests
            .into_iter()
            .map(|test| test.rendered(row, text, Some(&bound)))
            .reduce(|earlier, test| RExpr::Binary(Box::new(earlier), BinOp::And, Box::new(test)))
            .unwrap_or(RExpr::Bool(true));
        let body = match falsy(&self.inner, &bound) {
            Some(falsy) => RExpr::Binary(Box::new(falsy), BinOp::Or, Box::new(tests)),
            None => tests,
        };
        if self.option {
            RExpr::MethodCall(
                Box::new(self.value),
                "is_none_or".to_owned(),
                vec![RExpr::Closure(vec![binding.to_owned()], Box::new(body))],
            )
        } else {
            body
        }
    }
}

/// The test that a value of the type `inner` is falsy in PHP: `0`, `false`, `""`, `"0"`
/// and an empty list; `None` for a type whose values are all truthy (P2R-452).
fn falsy(inner: &RustType, value: &RExpr) -> Option<RExpr> {
    let value = Box::new(value.clone());
    let equals = |other: RExpr| RExpr::Binary(value.clone(), BinOp::Eq, Box::new(other));
    let empty = || RExpr::MethodCall(value.clone(), "is_empty".to_owned(), Vec::new());
    Some(match inner {
        RustType::I64 => equals(RExpr::Int(0)),
        RustType::Bool => RExpr::Not(value.clone()),
        RustType::String => RExpr::Binary(
            Box::new(empty()),
            BinOp::Or,
            Box::new(equals(RExpr::Str("0".to_owned()))),
        ),
        RustType::Vec(_) => empty(),
        _ => return None,
    })
}

/// The parts of a callback that P2R-451 reads.
struct Callback<'n, 'a> {
    /// The closure or arrow function, whose metadata types its parameters.
    node: &'n Expression<'a>,
    parameters: &'n FunctionLikeParameterList<'a>,
    /// The `where` chains: the statements of a closure, or the body of an arrow function.
    chains: Vec<&'n Expression<'a>>,
}

impl Converter<'_> {
    /// The condition of `->when($value, <callback>)` in a store chain, or `None` with a
    /// problem recorded (P2R-451 to P2R-453).
    pub(super) fn when_test(
        &mut self,
        node: &Expression<'_>,
        list: &ArgumentList<'_>,
        fields: &[ParameterSignature],
        store: &StoreRules,
    ) -> Option<WhenTest> {
        if !self.when_told() {
            self.refuse(
                node,
                "when converts only as Laravel's, whose type the tool tells Mago when an included file declares its trait Conditionable (P2R-450, P2R-451)",
            );
            return None;
        }
        let [Argument::Positional(value), Argument::Positional(callback)] =
            list.arguments.as_slice()
        else {
            self.refuse(
                node,
                "when converts only with a value and a callback, written as two positional arguments (P2R-451, P2R-453)",
            );
            return None;
        };
        if value.ellipsis.is_some() || callback.ellipsis.is_some() {
            self.refuse(
                node,
                "when converts only with a value and a callback, written as two positional arguments (P2R-451, P2R-453)",
            );
            return None;
        }
        let value = value.value;
        if !read_place(value) {
            self.refuse(
                value,
                "the value of when must be a variable, a parameter or a property read of one, which reads the same each time the store compares a row (P2R-451)",
            );
            return None;
        }
        let (expr, ty) = self.expression(value)?;
        let inner = ty.inner().clone();
        // A float stops: PHP 8.5 warns when it tests NAN for truth (P2R-453).
        if !matches!(
            inner,
            RustType::I64
                | RustType::Bool
                | RustType::String
                | RustType::Vec(_)
                | RustType::Class(_)
                | RustType::Enum(_)
        ) {
            self.refuse(
                value,
                "the value of when must be an int, a bool, a string, a list, an object or an enum case, whose truthiness P2R-452 gives (P2R-453)",
            );
            return None;
        }
        let option = matches!(ty, RustType::Option(_));
        let value_expr = if option {
            self.borrowed(expr, &ty)
        } else {
            expr
        };
        let callback = self.callback(callback.value)?;
        let (query, second) = self.callback_parameters(&callback, &inner)?;
        let mut tests = Vec::new();
        for chain in callback.chains {
            for list in self.where_chain(chain, &query)? {
                let (field, value) = self.where_field(node, list, fields, store)?;
                tests.push(self.bound_test(value, &field, second.as_deref(), &inner)?);
            }
        }
        // Rust would read such a value nowhere, and rustc warns about a parameter it is.
        if !option
            && falsy(&inner, &RExpr::Bool(true)).is_none()
            && !tests.iter().any(StoreTest::binds)
        {
            self.refuse(
                value,
                "a when whose value is an object or an enum case without null, which is always truthy, needs a where on the callback's second parameter (P2R-453)",
            );
            return None;
        }
        let base = second
            .as_deref()
            .and_then(names::rust_name)
            .unwrap_or_else(|| "value".to_owned());
        Some(WhenTest {
            value: value_expr,
            option,
            inner,
            base,
            tests,
        })
    }

    /// True when the tool told Mago the type of Laravel's `when()`: an included file, never
    /// the input, declares its trait with the method (P2R-450).
    fn when_told(&self) -> bool {
        self.codebase
            .get_class_like(CONDITIONABLE)
            .is_some_and(|conditionable| {
                !self
                    .files
                    .iter()
                    .any(|analysed| analysed.file == conditionable.span.file_id)
                    && conditionable
                        .methods
                        .iter()
                        .any(|method| method.as_bytes().eq_ignore_ascii_case(b"when"))
            })
    }

    /// The parameters and the `where` chains of a callback of P2R-451, or `None` with a
    /// problem recorded (P2R-453).
    fn callback<'n, 'a>(&mut self, callback: &'n Expression<'a>) -> Option<Callback<'n, 'a>> {
        let refuse = |this: &mut Self, at: Span| {
            this.refuse(
                &at,
                "the callback of when converts only as a closure without use, attributes or a return type other than void, whose statements are where calls on its first parameter, or an arrow function whose body is such a call (P2R-451, P2R-453)",
            );
            None
        };
        match unparenthesized(callback) {
            Expression::Closure(closure) => {
                let void = closure
                    .return_type_hint
                    .as_ref()
                    .is_none_or(|hint| matches!(hint.hint, Hint::Void(_)));
                if !closure.attribute_lists.is_empty()
                    || closure.ampersand.is_some()
                    || closure.use_clause.is_some()
                    || !void
                {
                    return refuse(self, closure.span());
                }
                let mut chains = Vec::new();
                for statement in closure.body.statements.iter() {
                    let Statement::Expression(statement) = statement else {
                        return refuse(self, statement.span());
                    };
                    chains.push(statement.expression);
                }
                if chains.is_empty() {
                    return refuse(self, closure.span());
                }
                Some(Callback {
                    node: unparenthesized(callback),
                    parameters: &closure.parameter_list,
                    chains,
                })
            }
            Expression::ArrowFunction(arrow) => {
                if !arrow.attribute_lists.is_empty()
                    || arrow.ampersand.is_some()
                    || arrow.return_type_hint.is_some()
                {
                    return refuse(self, arrow.span());
                }
                Some(Callback {
                    node: unparenthesized(callback),
                    parameters: &arrow.parameter_list,
                    chains: vec![arrow.expression],
                })
            }
            other => refuse(self, other.span()),
        }
    }

    /// The PHP names, without `$`, of the callback's first parameter and of its second when
    /// it has one, or `None` with a problem recorded: one or two plain parameters, the
    /// first untyped or Eloquent's `Builder`, the second untyped or of the type `inner`
    /// (P2R-451).
    fn callback_parameters(
        &mut self,
        callback: &Callback<'_, '_>,
        inner: &RustType,
    ) -> Option<(String, Option<String>)> {
        let parameters = callback.parameters;
        let list: Vec<&FunctionLikeParameter<'_>> = parameters.parameters.iter().collect();
        if !(1..=2).contains(&list.len()) {
            self.refuse(
                parameters,
                "the callback of when takes the query and, optionally, the value (P2R-451)",
            );
            return None;
        }
        for parameter in &list {
            if parameter.ampersand.is_some()
                || parameter.ellipsis.is_some()
                || parameter.default_value.is_some()
                || !parameter.modifiers.is_empty()
                || !parameter.attribute_lists.is_empty()
            {
                self.refuse(
                    *parameter,
                    "a parameter of the callback of when cannot be by reference, variadic, with a default, a modifier or an attribute (P2R-451)",
                );
                return None;
            }
            // Mago does not report these, but PHP rejects the file.
            let name = parameter.variable.name;
            let message = if name == b"$this" {
                "a parameter cannot be named $this, which PHP rejects (P2R-135)"
            } else if SUPERGLOBALS.contains(&name) {
                "PHP rejects a parameter named like a superglobal (P2R-31)"
            } else if list[0].variable.name == name && !std::ptr::eq(list[0], *parameter) {
                "PHP rejects two parameters of one name (P2R-451)"
            } else {
                continue;
            };
            self.refuse(&parameter.variable, message);
            return None;
        }
        let query = list[0];
        if query.hint.is_some() {
            let builder = self.parameter_type(callback.node, 0).is_some_and(|union| {
                union.types.iter().all(|atomic| {
                    matches!(atomic, mago_codex::ttype::atomic::TAtomic::Object(
                        mago_codex::ttype::atomic::object::TObject::Named(named))
                        if named.name.as_bytes().eq_ignore_ascii_case(BUILDER))
                })
            });
            if !builder {
                self.refuse(
                    query,
                    "the first parameter of the callback of when is untyped or Eloquent's Builder (P2R-451)",
                );
                return None;
            }
        }
        let second = list.get(1).copied();
        if let Some(second) = second
            && let Some(hint) = &second.hint
        {
            // PHP checks the hint against the value it passes: `false` or `true` would throw
            // a TypeError for the other truthy bool.
            let plain = matches!(
                hint,
                Hint::Integer(_) | Hint::String(_) | Hint::Bool(_) | Hint::Identifier(_)
            );
            let declared = self
                .parameter_type(callback.node, 1)
                .and_then(|union| types::rust_type(&union, &self.type_names).ok());
            if !plain || declared.as_ref() != Some(inner) {
                self.refuse(
                    second,
                    "the second parameter of the callback of when is untyped or written as the PHP type of the value without null: int, string, bool, the class or the enum (P2R-451)",
                );
                return None;
            }
        }
        let name = |parameter: &FunctionLikeParameter<'_>| {
            String::from_utf8_lossy(&parameter.variable.name[1..]).into_owned()
        };
        Some((name(query), second.map(name)))
    }

    /// The declared type Mago gives the parameter at `position` of the callback `node`.
    fn parameter_type(
        &self,
        node: &Expression<'_>,
        position: usize,
    ) -> Option<mago_codex::ttype::union::TUnion> {
        let codebase = self.codebase;
        codebase
            .get_closure_at(self.analysed(node).source, node.span())
            .and_then(|metadata| metadata.parameters.get(position))
            .and_then(|parameter| parameter.get_type_metadata())
            .map(|declared| declared.type_union.clone())
    }

    /// The argument lists of a chain `$query->where(..)->where(..)` on the callback's
    /// first parameter, in source order, or `None` with a problem recorded (P2R-451).
    fn where_chain<'n, 'a>(
        &mut self,
        chain: &'n Expression<'a>,
        query: &str,
    ) -> Option<Vec<&'n ArgumentList<'a>>> {
        let mut lists = Vec::new();
        let mut at = unparenthesized(chain);
        loop {
            match at {
                Expression::Call(Call::Method(call))
                    if matches!(&call.method, ClassLikeMemberSelector::Identifier(name)
                        if name.value.eq_ignore_ascii_case(b"where")) =>
                {
                    lists.push(&call.argument_list);
                    at = unparenthesized(call.object);
                }
                Expression::Variable(Variable::Direct(variable))
                    if !lists.is_empty() && variable.name[1..] == *query.as_bytes() =>
                {
                    lists.reverse();
                    return Some(lists);
                }
                _ => {
                    self.refuse(
                        chain,
                        "the callback of when converts only with where calls on its first parameter (P2R-451, P2R-453)",
                    );
                    return None;
                }
            }
        }
    }

    /// The test of a `where` inside a callback: a literal, the callback's second parameter
    /// `second`, or a property read of it, which stand for the value (P2R-451).
    fn bound_test(
        &mut self,
        value: &Expression<'_>,
        field: &ParameterSignature,
        second: Option<&str>,
        inner: &RustType,
    ) -> Option<StoreTest> {
        let is_second = |expression: &Expression<'_>| {
            matches!(unparenthesized(expression), Expression::Variable(Variable::Direct(variable))
                if second.is_some_and(|second| variable.name[1..] == *second.as_bytes()))
        };
        if is_second(value) {
            return self.compared_test(value, field, TestValue::Bound(None), inner);
        }
        if let Expression::Access(Access::Property(access)) = unparenthesized(value)
            && is_second(access.object)
            && let ClassLikeMemberSelector::Identifier(name) = &access.property
            && let RustType::Class(class) = inner
        {
            // A model that takes no struct has no properties (P2R-401).
            let Some(property) = self.class_named(class).and_then(|index| {
                self.properties(index)
                    .find(|property| property.php_name == name.value)
                    .cloned()
            }) else {
                self.refuse(value, "this class has no property of that name (P2R-134)");
                return None;
            };
            return self.compared_test(
                value,
                field,
                TestValue::Bound(Some(property.rust_name.clone())),
                &property.ty,
            );
        }
        let literal = !matches!(
            unparenthesized(value),
            Expression::Variable(_) | Expression::Access(_)
        ) && steady(value);
        if !literal {
            self.refuse(
                value,
                "the value of a where in the callback of when must be a literal, the callback's second parameter or a property read of it (P2R-451, P2R-453)",
            );
            return None;
        }
        if let Some(test) = self.null_test(value, field) {
            return test;
        }
        let (expr, found) = self.expression(value)?;
        self.compared_test(value, field, TestValue::Expr(expr), &found)
    }
}

/// True for a variable, a parameter or a property read of one (P2R-451).
fn read_place(expression: &Expression<'_>) -> bool {
    match unparenthesized(expression) {
        Expression::Variable(Variable::Direct(_)) => true,
        Expression::Access(Access::Property(access)) => {
            matches!(&access.property, ClassLikeMemberSelector::Identifier(_))
                && read_place(access.object)
        }
        _ => false,
    }
}
