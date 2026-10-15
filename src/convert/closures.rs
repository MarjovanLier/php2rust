//! Arrow functions and first-class callables as callbacks of `array_map` and
//! `preg_replace_callback`, which run them at once, before the enclosing body goes on, so
//! a closure that copies the Copy values it reads sees what PHP's copy holds (P2R-300 to
//! P2R-302).

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::Converter;
use super::helpers::Helper;
use super::lists::unparenthesized;
use super::variables::mentions;
use crate::printer::RExpr;
use crate::types::RustType;

impl Converter<'_> {
    /// `arrow` as a Rust closure of one parameter of type `wanted`, with the Rust type of
    /// its body. With `matches`, the parameter must be declared `array` and takes the
    /// matches of `preg_replace_callback` (P2R-302); otherwise its declared type must be
    /// `wanted` (P2R-300).
    pub(super) fn arrow_closure(
        &mut self,
        arrow: &ArrowFunction<'_>,
        wanted: &RustType,
        matches: bool,
    ) -> Option<(RExpr, RustType)> {
        let parameter = match arrow.parameter_list.parameters.iter().collect::<Vec<_>>()[..] {
            [parameter] => parameter,
            _ => {
                self.record_refused(arrow.expression);
                self.refuse(
                    &arrow.parameter_list,
                    "an arrow function as a callback takes one parameter here (P2R-300)",
                );
                return None;
            }
        };
        if let Some(ampersand) = arrow.ampersand {
            self.record_refused(arrow.expression);
            self.refuse(
                &ampersand,
                "an arrow function that returns by reference is not supported (P2R-300)",
            );
            return None;
        }
        let ty = self.closure_parameter(arrow, parameter, wanted, matches);
        let returns = match &arrow.return_type_hint {
            Some(hint) => {
                let codebase = self.codebase;
                let declared = codebase
                    .get_closure_at(self.analysed(arrow).source, arrow.span())
                    .and_then(|metadata| metadata.return_type_metadata.as_ref())
                    .map(|t| &t.type_union);
                self.declared_type(&hint.hint, declared, Some(&hint.hint))
            }
            // Without a declared type, the body is returned as the type Mago gives it.
            None => match self.site_type(arrow.expression) {
                Ok(ty) => Some(ty),
                Err(message) => {
                    self.refuse(arrow.expression, message);
                    None
                }
            },
        };
        let (Some(ty), Some(returns)) = (ty, returns) else {
            self.record_refused(arrow.expression);
            return None;
        };
        let start = self.variables.next_var();
        let var = self.bind_parameter(&parameter.variable, ty.clone());
        let outer = self.variables.closure_start.replace(start);
        // A `?` in the body would leave the closure, not the function (P2R-300).
        let raise = self.raise.take();
        let body = self.returned_value(arrow.expression, &returns);
        self.raise = raise;
        self.variables.closure_start = outer;
        self.variables.close_bindings(&[var]);
        let (body, result) = (body?, returns);
        let name = self.variables.name_of(var);
        // rustc warns about a closure parameter that nothing reads.
        let name = if mentions(arrow.expression, parameter.variable.name) {
            name.to_owned()
        } else {
            format!("_{name}")
        };
        let parameters = vec![format!("{name}: {}", ty.parameter())];
        Some((RExpr::Closure(parameters, Box::new(body)), result))
    }

    /// The Rust type of the parameter of an arrow function callback, or `None` with a
    /// problem recorded (P2R-300, P2R-302).
    fn closure_parameter(
        &mut self,
        arrow: &ArrowFunction<'_>,
        parameter: &FunctionLikeParameter<'_>,
        wanted: &RustType,
        matches: bool,
    ) -> Option<RustType> {
        if parameter.hint.is_none()
            || parameter.ampersand.is_some()
            || parameter.ellipsis.is_some()
            || parameter.default_value.is_some()
        {
            self.refuse(
                parameter,
                "an arrow function parameter needs a declared type, and cannot be by reference, variadic or have a default (P2R-300)",
            );
            return None;
        }
        if matches {
            if !matches!(parameter.hint, Some(Hint::Array(_))) {
                self.refuse(
                    parameter,
                    "the parameter of a preg_replace_callback() callback must be declared array (P2R-302)",
                );
                return None;
            }
            return Some(wanted.clone());
        }
        let codebase = self.codebase;
        let declared = codebase
            .get_closure_at(self.analysed(arrow).source, arrow.span())
            .and_then(|metadata| metadata.parameters.first())
            .and_then(|meta| meta.get_type_metadata())
            .map(|t| &t.type_union);
        let ty = self.declared_type(parameter, declared, parameter.hint.as_ref())?;
        if ty != *wanted {
            self.refuse(
                parameter,
                format!(
                    "this parameter must have the type of the list's elements, {} (P2R-301)",
                    wanted.owned()
                ),
            );
            return None;
        }
        Some(ty)
    }

    /// `array_map(CB, $list)` on a list of Copy values or strings, as
    /// `list.iter().copied().map(CB)` or `list.iter().map(String::as_str).map(CB)`, then
    /// `.collect::<Vec<_>>()` (P2R-301).
    pub(super) fn array_map(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let arguments = self.built_in_arguments(
            call,
            node,
            (2, 2),
            "array_map() converts with a callback and one list only (P2R-301)",
        )?;
        let list = self.expression(arguments[1]);
        let element = match &list {
            Some((_, RustType::Vec(element)))
                if element.is_copy() || **element == RustType::String =>
            {
                Some((**element).clone())
            }
            Some(_) => {
                self.refuse(
                    arguments[1],
                    "array_map() converts only on a list of ints, floats, bools, enums or strings (P2R-301)",
                );
                None
            }
            None => None,
        };
        let Some(element) = element else {
            self.record_refused(arguments[0]);
            return None;
        };
        let callback = match unparenthesized(arguments[0]) {
            Expression::ArrowFunction(arrow) => self.arrow_closure(arrow, &element, false),
            Expression::PartialApplication(PartialApplication::Function(partial))
                if partial.argument_list.is_first_class_callable() =>
            {
                self.callable_closure(partial, &element)
            }
            other => {
                self.record_refused(other);
                self.refuse(
                    other,
                    "array_map() converts only with an arrow function or a first-class callable such as trim(...) (P2R-301)",
                );
                None
            }
        };
        let ((list, _), (callback, result)) = (list?, callback?);
        let items = RExpr::MethodCall(Box::new(list), "iter".to_owned(), Vec::new());
        let items = if element.is_copy() {
            RExpr::MethodCall(Box::new(items), "copied".to_owned(), Vec::new())
        } else {
            let lend = RExpr::Path("String::as_str".to_owned());
            RExpr::MethodCall(Box::new(items), "map".to_owned(), vec![lend])
        };
        let mapped = RExpr::MethodCall(Box::new(items), "map".to_owned(), vec![callback]);
        let collected =
            RExpr::MethodCall(Box::new(mapped), "collect::<Vec<_>>".to_owned(), Vec::new());
        Some((collected, RustType::Vec(Box::new(result))))
    }

    /// A first-class callable `f(...)` as the closure `|x: T| f(x)` on an element of type
    /// `element`: `f` a function of the input or a string built-in (P2R-301).
    fn callable_closure(
        &mut self,
        partial: &FunctionPartialApplication<'_>,
        element: &RustType,
    ) -> Option<(RExpr, RustType)> {
        if let Expression::Identifier(identifier) = partial.function {
            let key = self.call_key(identifier);
            if let Some(signature) = self.functions.get(&key) {
                if let ([parameter], Some(returns), false) = (
                    signature.parameters.as_slice(),
                    &signature.returns,
                    signature.throws,
                ) && parameter.ty == *element
                {
                    let name = if signature.rust_name == "x" { "y" } else { "x" };
                    let call = RExpr::Call(
                        signature.rust_name.clone(),
                        vec![RExpr::Var(name.to_owned())],
                    );
                    let parameters = vec![format!("{name}: {}", element.parameter())];
                    return Some((RExpr::Closure(parameters, Box::new(call)), returns.clone()));
                }
            } else if *element == RustType::String
                && let Some(body) = self.built_in_callable(identifier)
            {
                let parameters = vec!["x: &str".to_owned()];
                return Some((RExpr::Closure(parameters, Box::new(body)), RustType::String));
            }
        }
        self.refuse(
            partial,
            "a first-class callable converts as an array_map() callback only for a function of the input with one parameter of the element type that returns a value and does not throw, or for strtolower, strtoupper, ucfirst, lcfirst, ucwords, trim, ltrim and rtrim on strings (P2R-301)",
        );
        None
    }

    /// The call of the string built-in that `identifier` names on `x`, when it is one of
    /// those P2R-301 lists.
    fn built_in_callable(&mut self, identifier: &Identifier<'_>) -> Option<RExpr> {
        let x = || RExpr::Var("x".to_owned());
        let helpers = [
            (&b"ucfirst"[..], Helper::PhpUcfirst),
            (b"lcfirst", Helper::PhpLcfirst),
            (b"ucwords", Helper::PhpUcwords),
            (b"trim", Helper::PhpTrim),
            (b"ltrim", Helper::PhpLtrim),
            (b"rtrim", Helper::PhpRtrim),
        ];
        if let Some(&(_, helper)) = helpers
            .iter()
            .find(|(name, _)| self.names_built_in(identifier, name))
        {
            return Some(self.helper_call(helper, vec![x()]));
        }
        let method = if self.names_built_in(identifier, b"strtolower") {
            "to_ascii_lowercase"
        } else if self.names_built_in(identifier, b"strtoupper") {
            "to_ascii_uppercase"
        } else {
            return None;
        };
        Some(RExpr::MethodCall(
            Box::new(x()),
            method.to_owned(),
            Vec::new(),
        ))
    }
}
