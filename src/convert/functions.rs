//! Function signatures and bodies (P2R-23, P2R-30 to P2R-33, P2R-35).

use std::collections::HashSet;

use heck::ToSnakeCase;
use mago_codex::ttype::union::TUnion;
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::{Context, Converter, ParameterSignature, Signature, Variables, lists};
use crate::names;
use crate::types::{self, RustType};

impl Converter<'_> {
    /// Registers every top-level function before any body converts, because PHP lets a
    /// call come before the declaration (P2R-33).
    pub(super) fn register(&mut self, program: &Program<'_>) {
        let mut named: Vec<(&Function<'_>, String)> = Vec::new();
        let mut taken: HashSet<String> = HashSet::new();
        for statement in program.statements.iter() {
            let Statement::Function(function) = statement else {
                continue;
            };
            let Some(rust_name) = self.rust_name(function.name.value, &function.name) else {
                continue;
            };
            if rust_name == "main" {
                self.refuse(
                    &function.name,
                    "a function named main is not supported (P2R-31)",
                );
                continue;
            }
            if !taken.insert(rust_name.clone()) {
                let text = String::from_utf8_lossy(function.name.value).into_owned();
                self.refuse(
                    &function.name,
                    format!("{text} gives the same Rust name as another function (P2R-31)"),
                );
                continue;
            }
            named.push((function, rust_name));
        }
        for (function, rust_name) in named {
            if let Some(signature) = self.signature(function, rust_name, &taken) {
                self.functions
                    .insert(function.name.value.to_ascii_lowercase(), signature);
            }
        }
    }

    pub(super) fn signature(
        &mut self,
        function: &Function<'_>,
        rust_name: String,
        taken: &HashSet<String>,
    ) -> Option<Signature> {
        let codebase = self.codebase;
        let Some(metadata) = codebase.get_function(function.name.value) else {
            self.refuse(
                &function.name,
                "Mago has no signature for this function (P2R-23)",
            );
            return None;
        };
        let mut parameters: Vec<ParameterSignature> = Vec::new();
        let mut converted = true;
        for (parameter, meta) in function
            .parameter_list
            .parameters
            .iter()
            .zip(metadata.parameters.iter())
        {
            let declared = meta.get_type_metadata().map(|t| &t.type_union);
            match self.parameter(parameter, declared, taken, &parameters) {
                Some(signature) => parameters.push(signature),
                None => converted = false,
            }
        }
        let returns = self.return_type(
            function,
            metadata
                .return_type_metadata
                .as_ref()
                .map(|t| &t.type_union),
        );
        match returns {
            Some(returns) if converted => Some(Signature {
                rust_name,
                parameters,
                returns,
            }),
            _ => None,
        }
    }

    pub(super) fn parameter(
        &mut self,
        parameter: &FunctionLikeParameter<'_>,
        declared: Option<&TUnion>,
        taken: &HashSet<String>,
        earlier: &[ParameterSignature],
    ) -> Option<ParameterSignature> {
        if parameter.ampersand.is_some()
            || parameter.ellipsis.is_some()
            || parameter.default_value.is_some()
        {
            self.refuse(
                parameter,
                "by-reference, variadic and default parameters are not supported (P2R-07)",
            );
            return None;
        }
        if parameter.hint.is_none() {
            self.refuse(parameter, "a parameter needs a declared type (P2R-23)");
            return None;
        }
        let php_name = &parameter.variable.name[1..];
        let rust_name = self.rust_name(php_name, &parameter.variable)?;
        if taken.contains(&rust_name) || earlier.iter().any(|p| p.rust_name == rust_name) {
            self.refuse(
                &parameter.variable,
                format!("{rust_name} is already the Rust name of a function or parameter (P2R-31)"),
            );
            return None;
        }
        let ty = self.declared_type(parameter, declared)?;
        if !matches!(
            ty,
            RustType::I64 | RustType::F64 | RustType::Bool | RustType::String
        ) && !lists::list_built(&ty)
        {
            self.refuse(
                parameter,
                "only int, float, bool, string and list parameters are built so far (P2R-20)",
            );
            return None;
        }
        let snake = std::str::from_utf8(php_name)
            .unwrap_or_default()
            .to_snake_case();
        Some(ParameterSignature {
            php_name: php_name.to_vec(),
            rust_name,
            snake,
            ty,
        })
    }

    /// `Some(None)` is a `void` function; `None` means a problem was recorded.
    pub(super) fn return_type(
        &mut self,
        function: &Function<'_>,
        declared: Option<&TUnion>,
    ) -> Option<Option<RustType>> {
        let Some(hint) = &function.return_type_hint else {
            self.refuse(
                &function.name,
                "a function needs a declared return type (P2R-23)",
            );
            return None;
        };
        if matches!(hint.hint, Hint::Void(_)) {
            return Some(None);
        }
        let ty = self.declared_type(hint, declared)?;
        if !matches!(
            ty,
            RustType::I64 | RustType::F64 | RustType::Bool | RustType::String
        ) && !lists::list_built(&ty)
        {
            self.refuse(
                hint,
                "only int, float, bool, string, list and void return types are built so far (P2R-20)",
            );
            return None;
        }
        Some(Some(ty))
    }

    pub(super) fn declared_type(
        &mut self,
        node: &impl HasSpan,
        declared: Option<&TUnion>,
    ) -> Option<RustType> {
        let Some(union) = declared else {
            self.refuse(node, "Mago found no type here (P2R-23)");
            return None;
        };
        match types::rust_type(union) {
            Ok(ty) => Some(ty),
            Err(name) => {
                self.refuse(node, format!("the type {name} has no Rust type (P2R-22)"));
                None
            }
        }
    }

    pub(super) fn rust_name(&mut self, php: &[u8], node: &impl HasSpan) -> Option<String> {
        let text = std::str::from_utf8(php)
            .unwrap_or_default()
            .trim_start_matches('$');
        let name = names::rust_name(text);
        if name.is_none() {
            self.refuse(node, format!("the name {text} has no Rust form (P2R-30)"));
        }
        name
    }

    pub(super) fn function(&mut self, function: &Function<'_>) -> Option<Vec<String>> {
        let signature = self
            .functions
            .get(&function.name.value.to_ascii_lowercase())?
            .clone();
        // A function has its own variables; those of `fn main()` wait until it ends.
        let outer = std::mem::replace(
            &mut self.variables,
            Variables::with_parameters(&signature.parameters),
        );
        self.variables.list_types =
            self.list_types(function.body.statements.iter(), signature.returns.as_ref());
        let mut body = Vec::new();
        for statement in function.body.statements.iter() {
            self.statement(
                statement,
                1,
                Context::Function(signature.returns.as_ref()),
                &mut body,
            );
        }
        // rustc rejects a function with a return type whose end a path reaches.
        if signature.returns.is_some() && self.variables.left.is_none() {
            self.refuse(
                &function.name,
                "a function with a return type must return on every path (P2R-50)",
            );
        }
        let parameters = self.finish_variables(&mut body);
        self.variables = outer;
        let returns = signature
            .returns
            .as_ref()
            .map(|t| format!(" -> {}", t.owned()))
            .unwrap_or_default();
        let open = format!(
            "pub fn {}({}){returns} {{",
            signature.rust_name,
            parameters.join(", ")
        );
        if body.is_empty() {
            // rustfmt writes an empty body as `{}` on the same line.
            return Some(vec![self.emit(&function.name, 0, format!("{open}}}"))]);
        }
        let mut lines = vec![self.emit(&function.name, 0, open)];
        lines.extend(body);
        lines.push("}".to_owned());
        Some(lines)
    }
}
