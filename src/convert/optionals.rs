//! Spatie's `Optional`, the marker of a value a request left out, as `PhpOptional<T>`
//! (P2R-360 to P2R-362).

use mago_syntax::cst::*;

use super::Converter;
use crate::printer::{self, RExpr};
use crate::types::{self, RustType};

/// The problem of any use of a `PhpOptional` value other than `instanceof Optional`
/// (P2R-362).
pub(super) const OPTIONAL_USE: &str = "a value that may be Spatie's Optional converts only in instanceof Optional until a later slice (P2R-362)";

/// The enum of Spatie's `Optional`, which the output holds once, where it holds shape
/// structs (P2R-360).
pub(super) fn optional_lines() -> Vec<String> {
    [
        "#[derive(Clone, Debug, PartialEq, Eq)]",
        "pub enum PhpOptional<T> {",
        "    Absent,",
        "    Present(T),",
        "}",
    ]
    .map(str::to_owned)
    .to_vec()
}

impl Converter<'_> {
    /// True when an included file declares Spatie's `Optional` with the shape of
    /// laravel-data 4.18: no parent, interface, trait, property or constant, and none of
    /// the methods PHP calls by itself (P2R-360).
    pub(super) fn included_optional(&self) -> bool {
        let Some(optional) = self
            .codebase
            .get_class_like(b"spatie\\laraveldata\\optional")
        else {
            return false;
        };
        !self
            .files
            .iter()
            .any(|analysed| analysed.file == optional.span.file_id)
            && optional.kind.is_class()
            && !optional.flags.is_abstract()
            // `Optional::create()` becomes `Absent`, so it must be callable without anything.
            && self
                .codebase
                .get_method(b"Spatie\\LaravelData\\Optional", b"create")
                .is_some_and(|create| {
                    create.parameters.is_empty()
                        && create
                            .method_metadata
                            .as_ref()
                            .is_some_and(|method| method.is_static && method.visibility.is_public())
                })
            && optional.direct_parent_class.is_none()
            && optional.all_parent_interfaces.is_empty()
            && optional.used_traits.is_empty()
            && optional.properties.is_empty()
            && optional.constants.is_empty()
            && !super::classes::IMPLICIT_METHODS.iter().any(|name| {
                self.codebase
                    .method_exists(b"Spatie\\LaravelData\\Optional", name.as_bytes())
            })
    }

    /// True when `name` is Spatie's `Optional` from an included file (P2R-360).
    pub(super) fn names_optional(&self, name: &Identifier<'_>) -> bool {
        let key = self.identifier_key(name);
        self.type_names.optional
            && key == b"spatie\\laraveldata\\optional"
            && !self.type_names.contains_key(&key)
    }

    /// `PhpOptional::Absent` for `new Optional`, `new Optional()` and `Optional::create()`
    /// where a `PhpOptional` is due (P2R-361).
    pub(super) fn absent(&self, expression: &Expression<'_>, due: &RustType) -> Option<RExpr> {
        if !matches!(due, RustType::Optional(_)) {
            return None;
        }
        let made = match expression {
            Expression::Instantiation(new) => {
                matches!(new.class, Expression::Identifier(name) if self.names_optional(name))
                    && new
                        .argument_list
                        .as_ref()
                        .is_none_or(|list| list.arguments.is_empty())
            }
            Expression::Call(Call::StaticMethod(call)) => {
                matches!(call.class, Expression::Identifier(name) if self.names_optional(name))
                    && matches!(&call.method, ClassLikeMemberSelector::Identifier(method)
                        if method.value.eq_ignore_ascii_case(b"create"))
                    && call.argument_list.arguments.is_empty()
                    // The declared return type of `create()` decides what PHP returns.
                    && self.expression_type(expression).is_some_and(|union| {
                        matches!(&union.types[..], [only] if types::is_optional(only, &self.type_names))
                    })
            }
            _ => false,
        };
        made.then(|| RExpr::Path(format!("{}::Absent", types::OPTIONAL)))
    }

    /// `$v instanceof Optional` on a `PhpOptional` value as `matches!(v,
    /// PhpOptional::Absent)`, negated when `negated` (P2R-362).
    pub(super) fn instance_of_optional(
        &mut self,
        binary: &Binary<'_>,
        negated: bool,
    ) -> Option<(RExpr, RustType)> {
        let (expr, ty) = self.expression(binary.lhs)?;
        if !matches!(ty, RustType::Optional(_)) {
            self.refuse(
                binary,
                "instanceof Optional converts only on a value of type T|Optional (P2R-362)",
            );
            return None;
        }
        let test = format!(
            "matches!({}, {}::Absent)",
            printer::expr(&expr, false),
            types::OPTIONAL
        );
        let test = if negated { format!("!{test}") } else { test };
        Some((RExpr::Raw(test), RustType::Bool))
    }
}
