//! Function signatures and bodies (P2R-23, P2R-30 to P2R-33, P2R-35).

use std::collections::HashSet;

use heck::ToSnakeCase;
use mago_codex::ttype::union::TUnion;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use super::control::{self, emit};
use super::{
    Context, Converter, Mode, ParameterSignature, Signature, Variables, classes, helpers, lists,
    nulls, recursion, unions,
};
use crate::names;
use crate::printer::RExpr;
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
            if helpers::is_helper(&rust_name) {
                self.refuse(&function.name, helpers::HELPER_NAME);
                continue;
            }
            if self
                .built_in_functions
                .contains(&function.name.value.to_ascii_lowercase())
            {
                self.refuse(
                    &function.name,
                    "a function may not take the name of a function PHP has built in (P2R-31)",
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
            if let Some(attributes) = function.attribute_lists.first() {
                self.refuse(attributes, "attributes are not supported (P2R-07)");
            }
            named.push((function, rust_name));
        }
        // A parameter may not hide the function of an exception constructor either (P2R-391).
        let hidden: HashSet<String> = taken
            .iter()
            .cloned()
            .chain(self.exceptions.iter().filter_map(|c| c.rust_name.clone()))
            .collect();
        for (function, rust_name) in named {
            if let Some(signature) = self.signature(function, rust_name, &hidden) {
                self.functions
                    .insert(self.function_key(function), signature);
            }
        }
    }

    pub(super) fn signature(
        &mut self,
        function: &Function<'_>,
        rust_name: String,
        taken: &HashSet<String>,
    ) -> Option<Signature> {
        // PHP rejects `self` and `static` outside a class; Mago does not report it (P2R-154).
        let own_class = function
            .parameter_list
            .parameters
            .iter()
            .filter_map(|p| p.hint.as_ref())
            .chain(function.return_type_hint.as_ref().map(|r| &r.hint))
            .find(|hint| classes::names(hint, classes::is_own_class));
        if let Some(hint) = own_class {
            self.refuse(
                hint,
                "self and static name a class, so a function outside one cannot use them (P2R-154)",
            );
            return None;
        }
        let codebase = self.codebase;
        let Some(metadata) = codebase.get_function(&self.function_key(function)) else {
            self.refuse(
                &function.name,
                "Mago has no signature for this function (P2R-23)",
            );
            return None;
        };
        let mut parameters: Vec<ParameterSignature> = Vec::new();
        let mut converted = self.defaults_last(&function.parameter_list);
        for (parameter, meta) in function
            .parameter_list
            .parameters
            .iter()
            .zip(metadata.parameters.iter())
        {
            // Mago's parser accepts a modifier on any parameter; PHP only on a constructor's.
            if let Some(modifier) = parameter.modifiers.first() {
                self.refuse(
                    modifier,
                    "PHP rejects a modifier on the parameter of a function (P2R-31)",
                );
                converted = false;
                continue;
            }
            let declared = meta.get_type_metadata().map(|t| &t.type_union);
            match self.parameter(parameter, declared, taken, &parameters) {
                Some(signature) => parameters.push(signature),
                None => converted = false,
            }
        }
        let returns = self.return_type(
            function.return_type_hint.as_ref(),
            &function.name,
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
                throws: false,
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
        if parameter.ampersand.is_some() || parameter.ellipsis.is_some() {
            self.refuse(
                parameter,
                "by-reference and variadic parameters are not supported (P2R-07)",
            );
            return None;
        }
        if let Some(attributes) = parameter.attribute_lists.first()
            && !self.only_sensitive(parameter)
        {
            self.refuse(attributes, "attributes are not supported (P2R-07)");
            return None;
        }
        if parameter.hint.is_none() {
            self.refuse(parameter, "a parameter needs a declared type (P2R-23)");
            return None;
        }
        // Mago does not report it, but PHP rejects the file.
        if parameter.variable.name == b"$this" {
            self.refuse(
                &parameter.variable,
                "a parameter cannot be named $this, which PHP rejects (P2R-135)",
            );
            return None;
        }
        if classes::SUPERGLOBALS.contains(&parameter.variable.name) {
            self.refuse(
                &parameter.variable,
                "PHP rejects a parameter named like a superglobal (P2R-31)",
            );
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
        if helpers::is_helper(&rust_name) {
            self.refuse(&parameter.variable, helpers::HELPER_NAME);
            return None;
        }
        let ty = self.declared_type(parameter, declared, parameter.hint.as_ref())?;
        if types::nullable_object(&ty) {
            self.refuse(parameter, types::NULLABLE_OBJECT);
            return None;
        }
        if !types::built(&ty) {
            self.refuse(
                parameter,
                "only int, float, bool, string, list and nullable parameters are built so far (P2R-20)",
            );
            return None;
        }
        let default = match &parameter.default_value {
            Some(default) => Some(self.parameter_default(parameter, default.value, &ty)?),
            None => None,
        };
        let snake = std::str::from_utf8(php_name)
            .unwrap_or_default()
            .to_snake_case();
        Some(ParameterSignature {
            php_name: php_name.to_vec(),
            rust_name,
            snake,
            ty,
            mode: Mode::Shared,
            default,
        })
    }

    /// True when the attributes of `parameter` are one `#[SensitiveParameter]` with no
    /// arguments, which PHP uses only to hide the value in a stack trace (P2R-341).
    pub(super) fn only_sensitive(&self, parameter: &FunctionLikeParameter<'_>) -> bool {
        let attributes: Vec<&Attribute<'_>> = parameter
            .attribute_lists
            .iter()
            .flat_map(|list| list.attributes.iter())
            .collect();
        attributes.len() == 1
            && attributes[0]
                .argument_list
                .as_ref()
                .is_none_or(|list| list.arguments.is_empty())
            && self.identifier_key(&attributes[0].name) == b"sensitiveparameter"
    }

    /// The default value of `parameter`, whose type is `ty`, converted at its
    /// declaration, with its Rust type: an int or float literal with or without unary
    /// minus, a bool or string literal, `null` for a nullable declared type, `[]` for a
    /// list type, or a case of an enum of the file (P2R-168).
    pub(super) fn parameter_default(
        &mut self,
        parameter: &FunctionLikeParameter<'_>,
        value: &Expression<'_>,
        ty: &RustType,
    ) -> Option<(RExpr, RustType)> {
        let value = lists::unparenthesized(value);
        let form = match value {
            Expression::Literal(_) => true,
            Expression::UnaryPrefix(unary) => {
                matches!(unary.operator, UnaryPrefixOperator::Negation(_))
                    && matches!(
                        lists::unparenthesized(unary.operand),
                        Expression::Literal(Literal::Integer(_) | Literal::Float(_))
                    )
            }
            // A map literal too (P2R-241).
            Expression::Array(array) => {
                array.elements.is_empty() || matches!(ty.inner(), RustType::Map(..))
            }
            // PHP rejects `static::` in a default when it compiles the file; Mago does not
            // report it.
            Expression::Access(Access::ClassConstant(access)) => {
                !matches!(access.class, Expression::Static(_))
                    && self.enum_case_of(access).is_some()
            }
            _ => false,
        };
        if !form {
            self.record_refused(value);
            self.refuse(
                value,
                "a parameter default must be a literal, null, [] or an enum case (P2R-168)",
            );
            return None;
        }
        let inner = ty.inner();
        let (expr, found) = match value {
            // PHP deprecates a null default for a type that is not nullable, and rejects it
            // on a promoted parameter; Mago does not report it as an error.
            Expression::Literal(Literal::Null(_))
                if !parameter.hint.as_ref().is_some_and(nullable) =>
            {
                self.refuse(
                    value,
                    "a null default needs a nullable type, ?T or T|null (P2R-168)",
                );
                return None;
            }
            // `null` for `T|Optional|null` is a value that is there (P2R-168, P2R-361).
            Expression::Literal(Literal::Null(_)) if matches!(ty, RustType::Optional(_)) => (
                RExpr::Call(
                    format!("{}::Present", crate::types::OPTIONAL),
                    vec![RExpr::None],
                ),
                ty.clone(),
            ),
            Expression::Literal(Literal::Null(_)) => (RExpr::None, ty.clone()),
            Expression::Array(_) if matches!(inner, RustType::Vec(_)) => {
                (RExpr::List(Vec::new()), inner.clone())
            }
            Expression::Array(array) if let RustType::Map(key, map_value) = inner => {
                self.map_literal(array, key, map_value)?
            }
            Expression::Array(_) => {
                self.refuse(
                    value,
                    "a parameter default must have the parameter's type (P2R-168)",
                );
                return None;
            }
            other => self.expression(other)?,
        };
        // An int fits a float parameter (P2R-25), and a member value a union one
        // (P2R-231); every other value must have the parameter's type, or its inner type
        // for a nullable one.
        let widens = found == RustType::I64 && *inner == RustType::F64;
        if found != *ty && found != *inner && !widens && nulls::member_path(&found, ty).is_none() {
            self.refuse(
                value,
                "a parameter default must have the parameter's type (P2R-168)",
            );
            return None;
        }
        let expr = self.declared_value(value, expr, &found, ty)?;
        Some((expr, if widens { RustType::F64 } else { found }))
    }

    /// Refuses a parameter with a default before one without, which PHP deprecates with a
    /// message when it compiles the file (P2R-168). False when it refuses one.
    pub(super) fn defaults_last(&mut self, list: &FunctionLikeParameterList<'_>) -> bool {
        let parameters: Vec<&FunctionLikeParameter<'_>> = list.parameters.iter().collect();
        let early = parameters.iter().enumerate().find(|(at, parameter)| {
            parameter.default_value.is_some()
                && parameters[at + 1..]
                    .iter()
                    .any(|later| later.default_value.is_none() && later.ellipsis.is_none())
        });
        if let Some((_, parameter)) = early {
            self.refuse(
                *parameter,
                "a parameter with a default before one without is deprecated in PHP (P2R-168)",
            );
            return false;
        }
        true
    }

    /// The return type of the function or method `name`. `Some(None)` is `void`; `None`
    /// means a problem was recorded.
    pub(super) fn return_type(
        &mut self,
        hint: Option<&FunctionLikeReturnTypeHint<'_>>,
        name: &LocalIdentifier<'_>,
        declared: Option<&TUnion>,
    ) -> Option<Option<RustType>> {
        let Some(hint) = hint else {
            self.refuse(name, "a function needs a declared return type (P2R-23)");
            return None;
        };
        if matches!(hint.hint, Hint::Void(_)) {
            return Some(None);
        }
        let ty = self.declared_type(hint, declared, Some(&hint.hint))?;
        if types::nullable_object(&ty) {
            self.refuse(hint, types::NULLABLE_OBJECT);
            return None;
        }
        if !types::built(&ty) {
            self.refuse(
                hint,
                "only int, float, bool, string, list, nullable and void return types are built so far (P2R-20)",
            );
            return None;
        }
        Some(Some(ty))
    }

    /// The Rust type of a declared type, from Mago's type `declared` and the PHP type
    /// `hint` (P2R-20, P2R-230). `None` means a problem was recorded.
    pub(super) fn declared_type(
        &mut self,
        node: &impl HasSpan,
        declared: Option<&TUnion>,
        hint: Option<&Hint<'_>>,
    ) -> Option<RustType> {
        let Some(union) = declared else {
            self.refuse(node, "Mago found no type here (P2R-23)");
            return None;
        };
        let resolve = |identifier: Option<&Identifier<'_>>| match identifier {
            Some(identifier) => Some(self.name_key(identifier, identifier.value())),
            None => self
                .current_class
                .map(|index| self.classes[index].key.clone()),
        };
        if let Some(problem) = hint.and_then(|hint| unions::native_problem(hint, union, &resolve)) {
            self.refuse(node, problem);
            return None;
        }
        match types::declared_type(union, &self.type_names) {
            Ok(ty) => Some(ty),
            Err(message) => {
                self.refuse(node, message);
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
        let key = self.function_key(function);
        let signature = self.functions.get(&key)?.clone();
        let statements: Vec<&Statement<'_>> = function.body.statements.iter().collect();
        Some(self.function_lines(
            Some(key),
            &function.name,
            &statements,
            &signature,
            |_, _| {},
        ))
    }

    /// The lines of a function with `signature` and the body `statements`, after which
    /// `tail` adds its own lines in the same scope (P2R-30, P2R-391).
    pub(super) fn function_lines(
        &mut self,
        key: Option<Vec<u8>>,
        name: &LocalIdentifier<'_>,
        statements: &[&Statement<'_>],
        signature: &Signature,
        tail: impl FnOnce(&mut Self, &mut Vec<String>),
    ) -> Vec<String> {
        // A function has its own variables; those of `fn main()` wait until it ends.
        let checks = owned_checks(statements);
        let copied = self.copied_parameters(statements, &signature.parameters, &checks);
        let outer = std::mem::replace(
            &mut self.variables,
            Variables::with_parameters(&signature.parameters, &copied),
        );
        self.variables.owned_checks = checks.iter().map(|check| check.span).collect();
        let outer_function = std::mem::replace(&mut self.current_function, key);
        let outer_calls = std::mem::take(&mut self.self_calls);
        let sites = self.site_types(statements.iter().copied(), signature.returns.as_ref());
        self.variables.list_types = sites.lists;
        self.variables.null_types = sites.nulls;
        let mut body = Vec::new();
        let outer_body = self.begin_body(signature.throws);
        for statement in statements {
            self.statement(
                statement,
                1,
                Context::Function(signature.returns.as_ref()),
                &mut body,
            );
        }
        tail(self, &mut body);
        self.end_body(signature.returns.is_none(), 1, &mut body, outer_body);
        // rustc rejects a function with a return type whose end a path reaches.
        if signature.returns.is_some() && self.variables.left.is_none() {
            self.refuse(
                name,
                "a function with a return type must return on every path (P2R-50)",
            );
        }
        let target = recursion::Target {
            self_calls: &self.self_calls,
            dropped: &self.dropped_defaults,
        };
        if recursion::always_recurses(statements, &target) {
            self.refuse(
                name,
                "this function calls itself on every path, and rustc rejects it (P2R-50)",
            );
        }
        let parameters = self.finish_variables(&mut body);
        body.splice(0..0, copy_lines(&signature.parameters, &copied, 1));
        self.variables = outer;
        self.current_function = outer_function;
        self.self_calls = outer_calls;
        let open = format!(
            "pub fn {}({}){} {{",
            signature.rust_name,
            parameters.join(", "),
            signature.returns_text()
        );
        if body.is_empty() {
            // rustfmt writes an empty body as `{}` on the same line.
            return vec![emit(0, format!("{open}}}"))];
        }
        let mut lines = vec![emit(0, open)];
        lines.extend(body);
        lines.push("}".to_owned());
        lines
    }
}

impl Converter<'_> {
    /// The PHP names of the string and list parameters that `statements` change by an
    /// assignment, `.=` or an append, which work on a copy (P2R-181). A list of a type that
    /// changes its objects is left out: its change still stops (P2R-143). A write after one
    /// of `checks` assigns its owned binding, not the parameter (P2R-290).
    pub(super) fn copied_parameters(
        &self,
        statements: &[&Statement<'_>],
        parameters: &[ParameterSignature],
        checks: &[OwnedCheck],
    ) -> HashSet<Vec<u8>> {
        let mut written = HashSet::new();
        for (at, statement) in statements.iter().enumerate() {
            let mut names = HashSet::new();
            Written.walk_statement(statement, &mut names);
            names.retain(|name| {
                !checks
                    .iter()
                    .any(|check| check.name == *name && check.at < at)
            });
            written.extend(names);
        }
        parameters
            .iter()
            .filter(|parameter| written.contains(&parameter.php_name))
            .filter(|parameter| match parameter.ty.inner() {
                RustType::String | RustType::Map(..) | RustType::Collection(_) => true,
                RustType::Vec(element) => !self.is_changing(element),
                _ => false,
            })
            .map(|parameter| parameter.php_name.clone())
            .collect()
    }
}

/// A top-level `if ($v === null)` whose binding is owned where the type of `$v` allows it
/// (P2R-290).
pub(super) struct OwnedCheck {
    pub(super) span: Span,
    /// The PHP name of `$v`, without `$`.
    pub(super) name: Vec<u8>,
    /// The position of the `if` among the top-level statements.
    pub(super) at: usize,
}

/// The checks among a body's top-level `statements` that give an owned binding (P2R-290):
/// `if ($v === null)` or `if (null === $v)`, also opening an `||` chain (P2R-334), with
/// no `elseif` or `else`, whose body ends
/// with a `return` or a `throw`, and with a write of `$v` in a later statement. Whether
/// P2R-104 makes the `if` a let-else, and whether the type allows an owned binding, is
/// decided when it converts.
pub(super) fn owned_checks(statements: &[&Statement<'_>]) -> Vec<OwnedCheck> {
    let mut checks = Vec::new();
    for (at, statement) in statements.iter().enumerate() {
        let Statement::If(check) = statement else {
            continue;
        };
        let branches = control::branches(check);
        let [(Some(condition), body)] = branches.as_slice() else {
            continue;
        };
        let leaves = match body.last() {
            Some(Statement::Return(_)) => true,
            Some(Statement::Expression(statement)) => {
                matches!(statement.expression, Expression::Throw(_))
            }
            _ => false,
        };
        let Some(variable) = null_test(condition).filter(|_| leaves) else {
            continue;
        };
        let name = variable.name[1..].to_vec();
        let mut written = HashSet::new();
        for later in &statements[at + 1..] {
            Written.walk_statement(later, &mut written);
        }
        if written.contains(&name) {
            checks.push(OwnedCheck {
                span: check.span(),
                name,
                at,
            });
        }
    }
    checks
}

/// The variable of `$v === null` or `null === $v`, also as the first operand of a
/// top-level `||` chain, which P2R-334 turns into a let-else too.
fn null_test<'a, 'arena>(condition: &'a Expression<'arena>) -> Option<&'a DirectVariable<'arena>> {
    let Expression::Binary(binary) = lists::unparenthesized(condition) else {
        return None;
    };
    if matches!(binary.operator, BinaryOperator::Or(_)) {
        return null_test(binary.lhs);
    }
    if !matches!(binary.operator, BinaryOperator::Identical(_)) {
        return None;
    }
    match (
        lists::unparenthesized(binary.lhs),
        lists::unparenthesized(binary.rhs),
    ) {
        (Expression::Variable(Variable::Direct(variable)), other)
        | (other, Expression::Variable(Variable::Direct(variable)))
            if nulls::is_null(other) =>
        {
            Some(variable)
        }
        _ => None,
    }
}

/// The copy line of each parameter in `copied`, in parameter order, at `depth` (P2R-181).
pub(super) fn copy_lines(
    parameters: &[ParameterSignature],
    copied: &HashSet<Vec<u8>>,
    depth: usize,
) -> Vec<String> {
    parameters
        .iter()
        .filter(|parameter| copied.contains(&parameter.php_name))
        .map(|parameter| {
            let name = &parameter.rust_name;
            let copy = match &parameter.ty {
                RustType::Option(_) => "map(ToOwned::to_owned)",
                RustType::Vec(_) | RustType::Collection(_) => "to_vec()",
                RustType::Map(..) => "clone()",
                _ => "to_string()",
            };
            emit(depth, format!("let mut {name} = {name}.{copy};"))
        })
        .collect()
}

/// Collects the PHP names, without `$`, of the variables that an assignment, an append,
/// `++` or `--` writes, outside nested functions, closures and classes.
struct Written;

impl<'ast, 'arena> Walker<'ast, 'arena, HashSet<Vec<u8>>> for Written {
    fn walk_in_assignment(
        &self,
        assignment: &'ast Assignment<'arena>,
        names: &mut HashSet<Vec<u8>>,
    ) {
        let variable = match lists::unparenthesized(assignment.lhs) {
            Expression::Variable(Variable::Direct(variable)) => variable,
            // A map insert changes the map (P2R-242).
            Expression::ArrayAppend(ArrayAppend { array, .. })
            | Expression::ArrayAccess(ArrayAccess { array, .. }) => {
                match lists::unparenthesized(array) {
                    Expression::Variable(Variable::Direct(variable)) => variable,
                    _ => return,
                }
            }
            _ => return,
        };
        names.insert(variable.name[1..].to_vec());
    }

    fn walk_in_unary_prefix(&self, unary: &'ast UnaryPrefix<'arena>, names: &mut HashSet<Vec<u8>>) {
        if let UnaryPrefixOperator::PreIncrement(_) | UnaryPrefixOperator::PreDecrement(_) =
            unary.operator
            && let Expression::Variable(Variable::Direct(variable)) = unary.operand
        {
            names.insert(variable.name[1..].to_vec());
        }
    }

    fn walk_in_unary_postfix(
        &self,
        unary: &'ast UnaryPostfix<'arena>,
        names: &mut HashSet<Vec<u8>>,
    ) {
        if let Expression::Variable(Variable::Direct(variable)) = unary.operand {
            names.insert(variable.name[1..].to_vec());
        }
    }

    // So does a removal (P2R-242).
    fn walk_in_unset(&self, unset: &'ast Unset<'arena>, names: &mut HashSet<Vec<u8>>) {
        for value in unset.values.iter() {
            if let Expression::ArrayAccess(access) = lists::unparenthesized(value)
                && let Expression::Variable(Variable::Direct(variable)) =
                    lists::unparenthesized(access.array)
            {
                names.insert(variable.name[1..].to_vec());
            }
        }
    }

    fn walk_function(&self, _: &'ast Function<'arena>, _: &mut HashSet<Vec<u8>>) {}

    fn walk_closure(&self, _: &'ast Closure<'arena>, _: &mut HashSet<Vec<u8>>) {}

    fn walk_arrow_function(&self, _: &'ast ArrowFunction<'arena>, _: &mut HashSet<Vec<u8>>) {}

    fn walk_class(&self, _: &'ast Class<'arena>, _: &mut HashSet<Vec<u8>>) {}
}

/// True for a declared type that names null: `?T` or a union with `null` (P2R-168).
fn nullable(hint: &Hint<'_>) -> bool {
    match hint {
        Hint::Nullable(_) | Hint::Null(_) => true,
        Hint::Union(union) => nullable(union.left) || nullable(union.right),
        Hint::Parenthesized(inner) => nullable(inner.hint),
        _ => false,
    }
}
