//! The nullsafe operator: `$o?->p` and `$o?->m(args)` as a `map` over the lent value
//! (P2R-234).

use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use super::{Converter, Mode};
use crate::names;
use crate::printer::{self, RExpr};
use crate::types::RustType;

impl Converter<'_> {
    /// `$o?->p` or `$o?->m(args)`, where `$o` is a variable, parameter or property read of
    /// type `Option<S>` for a class, or `Option<E>` for `->name` and `->value` of an enum:
    /// `o.as_ref().map(|o| ..)`, or `and_then` when the member is itself an `Option`
    /// (P2R-234). `None` means a problem was recorded.
    pub(super) fn nullsafe(&mut self, node: &Expression<'_>) -> Option<(RExpr, RustType)> {
        let (object, member, arguments) = match node {
            Expression::Access(Access::NullSafeProperty(access)) => {
                (access.object, &access.property, None)
            }
            Expression::Call(Call::NullSafeMethod(call)) => {
                (call.object, &call.method, Some(&call.argument_list))
            }
            _ => return None,
        };
        let refused = |converter: &mut Self, message: &str| {
            if let Some(list) = arguments {
                converter.record_refused_arguments(list);
            }
            converter.refuse(node, message.to_owned());
            None
        };
        let Some((receiver, ty)) = self.expression(object) else {
            if let Some(list) = arguments {
                self.record_refused_arguments(list);
            }
            return None;
        };
        let RustType::Option(inner) = &ty else {
            return refused(
                self,
                "?-> needs a value that may be null; write -> (P2R-234)",
            );
        };
        let binding = match &receiver {
            RExpr::Var(name) | RExpr::Field(_, name) => name.clone(),
            _ => {
                return refused(
                    self,
                    "?-> converts only on a variable, parameter or property read (P2R-234)",
                );
            }
        };
        if self.is_changing(inner) {
            return refused(
                self,
                "?-> on a class that changes its objects is not supported (P2R-234)",
            );
        }
        let ClassLikeMemberSelector::Identifier(name) = member else {
            return refused(self, "a member name must be a plain identifier (P2R-134)");
        };
        if arguments.is_some_and(|list| reads_rust_name(list, &binding)) {
            return refused(
                self,
                "an argument of ?-> cannot read a variable of the name the closure binds (P2R-234)",
            );
        }
        // An argument runs inside the closure, where a call could borrow what the closure
        // borrows, or a `?` would leave the closure instead of the function.
        if arguments.is_some_and(|list| !list.arguments.iter().all(|a| simple(a.value()))) {
            return refused(
                self,
                "an argument of ?-> must be a literal, a constant, a variable or a property read on one (P2R-234)",
            );
        }
        let bound = RExpr::Var(binding.clone());
        let (body, member_type) = match (&**inner, arguments) {
            (RustType::Class(class), None) => {
                let field = self
                    .class_named(class)
                    .and_then(|index| self.properties(index).find(|f| f.php_name == name.value))
                    .map(|f| (f.rust_name.clone(), f.ty.clone()));
                let Some((field, field_type)) = field else {
                    self.refuse(node, "this class has no property of that name (P2R-134)");
                    return None;
                };
                let read = RExpr::Field(Box::new(bound), field);
                (copied(read, &field_type), field_type)
            }
            (RustType::Enum(_), None) => {
                let (read, value_type) =
                    self.enum_member(name.value, node, (bound, (**inner).clone()))?;
                (copied(read, &value_type), value_type)
            }
            (RustType::Class(class), Some(list)) => {
                let found = self
                    .class_named(class)
                    .and_then(|index| Some((index, self.method_index(index, name.value)?)));
                let Some((index, at)) = found else {
                    return refused(self, "this class has no method of that name (P2R-135)");
                };
                let method = &self.classes[index].methods[at];
                let problem = if method.is_static {
                    Some("a static method is called with ::, not through an object (P2R-153)")
                } else if method.self_mode == Mode::Mut {
                    Some("?-> on a method that changes its object is not supported (P2R-234)")
                } else if method.signature.throws {
                    Some("?-> on a method that throws is not supported (P2R-234)")
                } else if method.signature.returns.is_none() {
                    Some("?-> on a void method is not supported (P2R-234)")
                } else {
                    None
                };
                if let Some(problem) = problem {
                    return refused(self, problem);
                }
                let signature = method.signature.clone();
                let arguments = self.arguments(list, &signature.parameters, node)?;
                let call = RExpr::MethodCall(Box::new(bound), signature.rust_name, arguments);
                (call, signature.returns?)
            }
            _ => {
                return refused(
                    self,
                    "?-> converts only on an object of a class of this file, or ->name and ->value of an enum (P2R-234)",
                );
            }
        };
        // An owned `Option` lends its value; a parameter already holds a reference, and a
        // Copy enum is passed by value.
        let lent = inner.is_copy()
            || matches!(&receiver, RExpr::Var(name) if self.variables.is_borrowed(name));
        let receiver = if lent {
            receiver
        } else {
            RExpr::Method(Box::new(receiver), "as_ref")
        };
        let (combinator, result) = match member_type {
            RustType::Option(_) => ("and_then", member_type),
            other => ("map", RustType::Option(Box::new(other))),
        };
        let closure = RExpr::Raw(format!("|{binding}| {}", printer::expr(&body, false)));
        Some((
            RExpr::MethodCall(Box::new(receiver), combinator.into(), vec![closure]),
            result,
        ))
    }
}

/// True for a literal, a constant, a variable, or a property read on one of them.
fn simple(expression: &Expression<'_>) -> bool {
    match expression {
        Expression::Parenthesized(inner) => simple(inner.expression),
        Expression::Literal(_) | Expression::ConstantAccess(_) => true,
        Expression::Variable(Variable::Direct(_)) => true,
        Expression::Access(Access::ClassConstant(_)) => true,
        Expression::Access(Access::Property(access)) => simple(access.object),
        _ => false,
    }
}

/// The owned value of `read`, of type `ty`, that the closure gives (P2R-26).
fn copied(read: RExpr, ty: &RustType) -> RExpr {
    match ty {
        _ if ty.is_copy() => read,
        RustType::String if matches!(read, RExpr::Method(_, "name" | "value")) => {
            RExpr::Method(Box::new(read), "to_string")
        }
        _ => RExpr::Method(Box::new(read), "clone"),
    }
}

/// True when an argument in `list` reads or writes a variable whose Rust name is `rust`.
fn reads_rust_name(list: &ArgumentList<'_>, rust: &str) -> bool {
    let mut found = Vec::new();
    for argument in list.arguments.iter() {
        Variables.walk_expression(argument.value(), &mut found);
    }
    found.iter().any(|php| {
        std::str::from_utf8(&php[1..])
            .ok()
            .and_then(names::rust_name)
            .is_some_and(|name| name.trim_start_matches("r#") == rust.trim_start_matches("r#"))
    })
}

/// Collects the name of every plain variable an expression holds.
struct Variables;

impl<'ast, 'arena> Walker<'ast, 'arena, Vec<Vec<u8>>> for Variables {
    fn walk_in_direct_variable(
        &self,
        variable: &'ast DirectVariable<'arena>,
        found: &mut Vec<Vec<u8>>,
    ) {
        found.push(variable.name.to_vec());
    }
}
