//! Readonly classes: declarations, `new`, property reads, methods and method calls
//! (P2R-130 to P2R-137).

use std::collections::HashSet;

use heck::ToSnakeCase;
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::enums::class_key;
use super::recursion::{self, Target};
use super::variables::THIS;
use super::{Context, Converter, ParameterSignature, Signature, Variables};
use crate::printer::RExpr;
use crate::types::{self, RustType};

/// rustfmt keeps a struct literal on one line when the text between its braces is at
/// most this wide (P2R-137).
const LITERAL_WIDTH: usize = 18;

/// A readonly class of the file (P2R-130).
pub(super) struct ClassInfo {
    /// The lowercase PHP name, by which every use finds it (P2R-131).
    key: Vec<u8>,
    rust_name: String,
    /// The promoted constructor parameters, which are the properties and the parameters
    /// of `new`, in source order (P2R-133).
    fields: Vec<ParameterSignature>,
    methods: Vec<MethodInfo>,
}

struct MethodInfo {
    /// The lowercase PHP name: PHP method names ignore letter case (P2R-131).
    key: Vec<u8>,
    /// The parameters without `$this`.
    signature: Signature,
}

impl Converter<'_> {
    /// Registers the name of every top-level readonly class, so a type can name one before
    /// its members are read (P2R-131).
    pub(super) fn register_class_names(&mut self, program: &Program<'_>) {
        for statement in program.statements.iter() {
            let Statement::Class(declaration) = statement else {
                continue;
            };
            let Some(rust_name) = self.declared_type_name(&declaration.name, "class") else {
                continue;
            };
            if self
                .type_names
                .values()
                .any(|ty| matches!(ty, RustType::Enum(n) | RustType::Class(n) if *n == rust_name))
            {
                self.refuse(
                    &declaration.name,
                    format!("{rust_name} is already the Rust name of an enum or class (P2R-131)"),
                );
                continue;
            }
            let key = declaration.name.value.to_ascii_lowercase();
            self.type_names
                .insert(key.clone(), RustType::Class(rust_name.clone()));
            self.classes.push(ClassInfo {
                key,
                rust_name,
                fields: Vec::new(),
                methods: Vec::new(),
            });
        }
    }

    /// Checks every registered class and reads its properties and method signatures,
    /// once the functions are known (P2R-130, P2R-131, P2R-135).
    pub(super) fn register_class_members(&mut self, program: &Program<'_>) {
        let taken: HashSet<String> = self
            .functions
            .values()
            .map(|f| f.rust_name.clone())
            .collect();
        for statement in program.statements.iter() {
            let Statement::Class(declaration) = statement else {
                continue;
            };
            if let Some(index) = self.class_index(declaration) {
                self.class_shape(declaration);
                self.fields(declaration, index);
                self.method_signatures(declaration, index, &taken);
            }
        }
        for index in 0..self.classes.len() {
            self.check_containment(program, index);
        }
    }

    /// The class a class declaration registered, when its name converts.
    pub(super) fn class_index(&self, declaration: &Class<'_>) -> Option<usize> {
        let key = declaration.name.value.to_ascii_lowercase();
        self.classes.iter().position(|c| c.key == key)
    }

    fn class_named(&self, rust_name: &str) -> Option<usize> {
        self.classes.iter().position(|c| c.rust_name == rust_name)
    }

    /// The method of the class at `index` with the PHP name `php`, in any letter case.
    fn method_index(&self, index: usize, php: &[u8]) -> Option<usize> {
        let key = php.to_ascii_lowercase();
        self.classes[index]
            .methods
            .iter()
            .position(|m| m.key == key)
    }

    /// Refuses everything a readonly class of this slice may not hold (P2R-130).
    fn class_shape(&mut self, declaration: &Class<'_>) {
        if let Some(attributes) = declaration.attribute_lists.first() {
            self.refuse(attributes, "attributes are not supported (P2R-07)");
        }
        let readonly = declaration
            .modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Readonly(_)));
        if !readonly {
            self.refuse(
                &declaration.name,
                "a class that is not readonly is not supported yet (P2R-130)",
            );
        }
        if let Some(modifier) = declaration
            .modifiers
            .iter()
            .find(|m| !matches!(m, Modifier::Readonly(_) | Modifier::Final(_)))
        {
            self.refuse(
                modifier,
                "only final may stand beside readonly on a class (P2R-130)",
            );
        }
        if let Some(extends) = &declaration.extends {
            self.refuse(
                extends,
                "a class that extends another is not supported (P2R-130)",
            );
        }
        if let Some(implements) = &declaration.implements {
            self.refuse(
                implements,
                "a class that implements an interface is not supported (P2R-130)",
            );
        }
        let mut constructors = 0;
        for member in declaration.members.iter() {
            let ClassLikeMember::Method(method) = member else {
                self.refuse(
                    member,
                    "a readonly class may hold only a constructor and methods (P2R-130)",
                );
                continue;
            };
            if let Some(attributes) = method.attribute_lists.first() {
                self.refuse(attributes, "attributes are not supported (P2R-07)");
            }
            if let Some(modifier) = method
                .modifiers
                .iter()
                .find(|m| matches!(m, Modifier::Static(_) | Modifier::Abstract(_)))
            {
                self.refuse(
                    modifier,
                    "a static or abstract method is not supported (P2R-135)",
                );
            }
            if method.ampersand.is_some() {
                self.refuse(
                    &method.name,
                    "a method that returns by reference is not supported (P2R-135)",
                );
            }
            if is_constructor(method) {
                constructors += 1;
                if constructors > 1 {
                    self.refuse(
                        &method.name,
                        "a class can have one constructor only (P2R-130)",
                    );
                }
                self.constructor_shape(method);
            } else if method.name.value.starts_with(b"__") {
                self.refuse(
                    &method.name,
                    "a method whose name starts with __ is not supported (P2R-130)",
                );
            }
        }
    }

    /// A constructor converts only when every parameter promotes to a typed property and
    /// its body is empty (P2R-130).
    fn constructor_shape(&mut self, method: &Method<'_>) {
        match &method.body {
            MethodBody::Concrete(block) if block.statements.is_empty() => {}
            _ => self.refuse(
                &method.name,
                "a constructor with a body is not supported (P2R-130)",
            ),
        }
        for parameter in method.parameter_list.parameters.iter() {
            let problem = if !parameter.is_promoted_property() {
                Some(
                    "a constructor parameter that is not promoted to a property is not supported (P2R-130)",
                )
            } else if parameter.hint.is_none() {
                Some("a promoted property needs a declared type, as PHP requires (P2R-130)")
            } else if parameter.hooks.is_some() {
                Some(
                    "a property hook is not supported, and PHP rejects one on a readonly property (P2R-130)",
                )
            } else if parameter.ampersand.is_some() || parameter.ellipsis.is_some() {
                Some("a promoted property cannot be by reference or variadic (P2R-130)")
            } else if parameter.default_value.is_some() {
                Some("a parameter default value is not supported (P2R-130)")
            } else if !parameter.attribute_lists.is_empty() {
                Some("attributes are not supported (P2R-07)")
            } else {
                None
            };
            if let Some(problem) = problem {
                self.refuse(parameter, problem);
            }
        }
    }

    /// The properties of the class at `index`: its promoted constructor parameters, each
    /// with the type Mago gives the property, which a docblock may refine (P2R-130).
    fn fields(&mut self, declaration: &Class<'_>, index: usize) {
        let Some(constructor) = declaration.members.iter().find_map(|member| match member {
            ClassLikeMember::Method(method) if is_constructor(method) => Some(method),
            _ => None,
        }) else {
            return;
        };
        let mut fields: Vec<ParameterSignature> = Vec::new();
        for parameter in constructor.parameter_list.parameters.iter() {
            if !parameter.is_promoted_property() || parameter.hint.is_none() {
                continue;
            }
            let php_name = &parameter.variable.name[1..];
            let Some(rust_name) = self.rust_name(php_name, &parameter.variable) else {
                continue;
            };
            if fields.iter().any(|f| f.rust_name == rust_name) {
                self.refuse(
                    &parameter.variable,
                    format!("{rust_name} is already the Rust name of another property (P2R-131)"),
                );
                continue;
            }
            let codebase = self.codebase;
            let declared = codebase
                .get_property(declaration.name.value, parameter.variable.name)
                .and_then(|p| p.type_metadata.as_ref())
                .map(|t| &t.type_union);
            let Some(ty) = self.declared_type(parameter, declared) else {
                continue;
            };
            if types::nullable_object(&ty) {
                self.refuse(parameter, types::NULLABLE_OBJECT);
                continue;
            }
            if !types::built(&ty) {
                self.refuse(
                    parameter,
                    "only int, float, bool, string, list, nullable, enum and object properties are built so far (P2R-20)",
                );
                continue;
            }
            fields.push(ParameterSignature {
                php_name: php_name.to_vec(),
                rust_name,
                snake: std::str::from_utf8(php_name)
                    .unwrap_or_default()
                    .to_snake_case(),
                ty,
            });
        }
        self.classes[index].fields = fields;
    }

    /// The signature of every method of the class at `index`, other than the constructor
    /// (P2R-131, P2R-135).
    fn method_signatures(
        &mut self,
        declaration: &Class<'_>,
        index: usize,
        taken: &HashSet<String>,
    ) {
        let codebase = self.codebase;
        let mut methods: Vec<MethodInfo> = Vec::new();
        for member in declaration.members.iter() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            if is_constructor(method) || method.name.value.starts_with(b"__") {
                continue;
            }
            let Some(rust_name) = self.rust_name(method.name.value, &method.name) else {
                continue;
            };
            // `new` is the constructor's name, and an own `clone` would replace the copy
            // of P2R-132 at every `.clone()`.
            if matches!(rust_name.as_str(), "new" | "clone") {
                self.refuse(
                    &method.name,
                    format!("a method named {rust_name} is not supported, because Rust uses that name (P2R-131)"),
                );
                continue;
            }
            if methods.iter().any(|m| m.signature.rust_name == rust_name) {
                self.refuse(
                    &method.name,
                    format!("{rust_name} is already the Rust name of another method (P2R-131)"),
                );
                continue;
            }
            let Some(metadata) = codebase.get_method(declaration.name.value, method.name.value)
            else {
                self.refuse(
                    &method.name,
                    "Mago has no signature for this method (P2R-23)",
                );
                continue;
            };
            let mut converted = true;
            let mut parameters: Vec<ParameterSignature> = Vec::new();
            for (parameter, meta) in method
                .parameter_list
                .parameters
                .iter()
                .zip(metadata.parameters.iter())
            {
                if parameter.is_promoted_property() {
                    self.refuse(
                        parameter,
                        "only a constructor parameter can be promoted to a property (P2R-130)",
                    );
                    converted = false;
                    continue;
                }
                if parameter.hint.as_ref().is_some_and(names_own_class) {
                    self.refuse(
                        parameter,
                        "the types self, static and parent are not supported yet (P2R-135)",
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
            if let Some(hint) = &method.return_type_hint
                && names_own_class(&hint.hint)
            {
                self.refuse(
                    hint,
                    "the types self, static and parent are not supported yet (P2R-135)",
                );
                continue;
            }
            let returns = self.return_type(
                method.return_type_hint.as_ref(),
                &method.name,
                metadata
                    .return_type_metadata
                    .as_ref()
                    .map(|t| &t.type_union),
            );
            if let (Some(returns), true) = (returns, converted) {
                methods.push(MethodInfo {
                    key: method.name.value.to_ascii_lowercase(),
                    signature: Signature {
                        rust_name,
                        parameters,
                        returns,
                    },
                });
            }
        }
        self.classes[index].methods = methods;
    }

    /// Refuses a class that holds itself by value, through its properties or those of
    /// other classes, because the struct would have no size; and one that holds an enum
    /// without cases, because no value of the struct could exist (P2R-130).
    fn check_containment(&mut self, program: &Program<'_>, index: usize) {
        let mut seen = vec![false; self.classes.len()];
        let mut pending = vec![index];
        let mut problem = None;
        while let Some(at) = pending.pop() {
            for field in &self.classes[at].fields {
                match &field.ty {
                    RustType::Class(name) => match self.class_named(name) {
                        Some(found) if found == index => {
                            problem = Some(
                                "this class holds itself, so the Rust struct would have no size (P2R-130)",
                            );
                        }
                        Some(found) if !seen[found] => {
                            seen[found] = true;
                            pending.push(found);
                        }
                        _ => {}
                    },
                    RustType::Enum(name) if self.case_paths(name).is_empty() => {
                        problem = Some(
                            "this class holds an enum without cases, so no object of it can exist (P2R-130)",
                        );
                    }
                    _ => {}
                }
            }
        }
        let Some(problem) = problem else {
            return;
        };
        let declaration = program
            .statements
            .iter()
            .find_map(|statement| match statement {
                Statement::Class(declaration)
                    if declaration.name.value.to_ascii_lowercase() == self.classes[index].key =>
                {
                    Some(declaration)
                }
                _ => None,
            });
        if let Some(declaration) = declaration {
            self.refuse(&declaration.name, problem);
        }
    }

    /// The Rust lines of the class at `index`: the struct and its `impl` block with `new`
    /// and the converted methods (P2R-130, P2R-135, P2R-137).
    pub(super) fn class_item(&mut self, declaration: &Class<'_>, index: usize) -> Vec<String> {
        let name = self.classes[index].rust_name.clone();
        let fields = self.classes[index].fields.clone();
        let mut lines = vec!["#[derive(Clone, Debug)]".to_owned()];
        if fields.is_empty() {
            lines.push(format!("pub struct {name} {{}}"));
        } else {
            lines.push(format!("pub struct {name} {{"));
            for field in &fields {
                let line = format!("pub {}: {},", field.rust_name, field.ty.owned());
                lines.push(self.emit(&declaration.name, 1, line));
            }
            lines.push("}".to_owned());
        }
        lines.push(String::new());
        lines.push(format!("impl {name} {{"));
        let parameters: Vec<String> = fields
            .iter()
            .map(|f| format!("{}: {}", f.rust_name, f.ty.parameter()))
            .collect();
        let open = format!("pub fn new({}) -> {name} {{", parameters.join(", "));
        lines.push(self.emit(&declaration.name, 1, open));
        let stores: Vec<String> = fields.iter().map(store).collect();
        let inner = stores.join(", ");
        if inner.is_empty() {
            lines.push(format!("        {name} {{}}"));
        } else if inner.len() <= LITERAL_WIDTH {
            let line = format!("{name} {{ {inner} }}");
            lines.push(self.emit(&declaration.name, 2, line));
        } else {
            lines.push(format!("        {name} {{"));
            for store in stores {
                lines.push(self.emit(&declaration.name, 3, format!("{store},")));
            }
            lines.push("        }".to_owned());
        }
        lines.push("    }".to_owned());
        for member in declaration.members.iter() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            let Some(at) = self.method_index(index, method.name.value) else {
                continue;
            };
            if let Some(method_lines) = self.method(method, index, at) {
                lines.push(String::new());
                lines.extend(method_lines);
            }
        }
        lines.push("}".to_owned());
        lines
    }

    /// The lines of a method, `pub fn name(&self, ...)` with its body, one level deep in
    /// the `impl` block (P2R-135).
    fn method(&mut self, method: &Method<'_>, class: usize, at: usize) -> Option<Vec<String>> {
        let MethodBody::Concrete(block) = &method.body else {
            return None;
        };
        let signature = self.classes[class].methods[at].signature.clone();
        let this = ParameterSignature {
            php_name: THIS.to_vec(),
            rust_name: "self".to_owned(),
            snake: "self".to_owned(),
            ty: RustType::Class(self.classes[class].rust_name.clone()),
        };
        let mut parameters = vec![this];
        parameters.extend(signature.parameters.iter().cloned());
        // A method has its own variables, as a function does.
        let outer = std::mem::replace(&mut self.variables, Variables::with_parameters(&parameters));
        let outer_method = self.current_method.replace((class, at));
        let outer_calls = std::mem::take(&mut self.self_calls);
        let sites = self.site_types(block.statements.iter(), signature.returns.as_ref());
        self.variables.list_types = sites.lists;
        self.variables.null_types = sites.nulls;
        let mut body = Vec::new();
        for statement in block.statements.iter() {
            self.statement(
                statement,
                2,
                Context::Function(signature.returns.as_ref()),
                &mut body,
            );
        }
        if signature.returns.is_some() && self.variables.left.is_none() {
            self.refuse(
                &method.name,
                "a function with a return type must return on every path (P2R-50)",
            );
        }
        let statements: Vec<&Statement<'_>> = block.statements.iter().collect();
        let target = Target {
            name: None,
            self_calls: &self.self_calls,
            dropped: &self.dropped_defaults,
        };
        if recursion::always_recurses(&statements, &target) {
            self.refuse(
                &method.name,
                "this method calls itself on every path, and rustc rejects it (P2R-50)",
            );
        }
        let parameters = self.finish_variables(&mut body);
        self.variables = outer;
        self.current_method = outer_method;
        self.self_calls = outer_calls;
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
            return Some(vec![self.emit(&method.name, 1, format!("{open}}}"))]);
        }
        let mut lines = vec![self.emit(&method.name, 1, open)];
        lines.extend(body);
        lines.push("    }".to_owned());
        Some(lines)
    }

    /// `new C(args)` and `new C` for a readonly class of the file (P2R-133).
    pub(super) fn instantiation(
        &mut self,
        instantiation: &Instantiation<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let index = class_key(instantiation.class)
            .and_then(|key| self.classes.iter().position(|c| c.key == key));
        let Some(index) = index else {
            self.record_refused(node);
            self.refuse(
                node,
                "new converts only for a readonly class of this file (P2R-133)",
            );
            return None;
        };
        let name = self.classes[index].rust_name.clone();
        let parameters = self.classes[index].fields.clone();
        let arguments = match &instantiation.argument_list {
            Some(list) => self.arguments(list, &parameters, node)?,
            None if parameters.is_empty() => Vec::new(),
            None => {
                self.refuse(node, "a call must pass one argument per parameter (P2R-07)");
                return None;
            }
        };
        Some((
            RExpr::Call(format!("{name}::new"), arguments),
            RustType::Class(name),
        ))
    }

    /// `$o->p`: a property read with the type of the property's declaration, or `->name`
    /// and `->value` of an enum (P2R-113, P2R-134).
    pub(super) fn property(
        &mut self,
        access: &PropertyAccess<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let object = self.expression(access.object)?;
        let RustType::Class(class) = &object.1 else {
            return self.enum_property(access, node, object);
        };
        let ClassLikeMemberSelector::Identifier(name) = &access.property else {
            self.refuse(node, "a property name must be a plain identifier (P2R-134)");
            return None;
        };
        // PHP property names keep their letter case.
        let field = self
            .class_named(class)
            .and_then(|index| {
                self.classes[index]
                    .fields
                    .iter()
                    .find(|f| f.php_name == name.value)
            })
            .map(|f| (f.rust_name.clone(), f.ty.clone()));
        let Some((field, ty)) = field else {
            self.refuse(node, "this class has no property of that name (P2R-134)");
            return None;
        };
        Some((RExpr::Field(Box::new(object.0), field), ty))
    }

    /// `$o->m(args)`, with the method's return type, `None` for `void` (P2R-135).
    pub(super) fn method_call(
        &mut self,
        call: &MethodCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, Option<RustType>)> {
        let Some((object, ty)) = self.expression(call.object) else {
            self.record_refused_arguments(&call.argument_list);
            return None;
        };
        let RustType::Class(class) = &ty else {
            self.record_refused_arguments(&call.argument_list);
            let message = if matches!(ty, RustType::Option(_)) {
                "a method call on a value that may be null is not supported (P2R-106)"
            } else {
                "a method call needs an object of a class of this file (P2R-135)"
            };
            self.refuse(node, message);
            return None;
        };
        let ClassLikeMemberSelector::Identifier(name) = &call.method else {
            self.record_refused_arguments(&call.argument_list);
            self.refuse(node, "a method name must be a plain identifier (P2R-135)");
            return None;
        };
        if name.value.starts_with(b"__") {
            self.record_refused_arguments(&call.argument_list);
            self.refuse(
                node,
                "a method whose name starts with __ cannot be called (P2R-133)",
            );
            return None;
        }
        let found = self
            .class_named(class)
            .and_then(|index| Some((index, self.method_index(index, name.value)?)));
        let Some((index, at)) = found else {
            self.record_refused_arguments(&call.argument_list);
            self.refuse(node, "this class has no method of that name (P2R-135)");
            return None;
        };
        let signature = self.classes[index].methods[at].signature.clone();
        let arguments = self.arguments(&call.argument_list, &signature.parameters, node)?;
        // rustc counts a call of the method through any object of its class (P2R-50).
        if self.current_method == Some((index, at)) {
            self.self_calls.push(call.span());
        }
        Some((
            RExpr::MethodCall(Box::new(object), signature.rust_name, arguments),
            signature.returns,
        ))
    }

    /// `C::m()` and the other static forms on a class of the file stop; on an enum they
    /// are the enum's methods (P2R-114, P2R-135).
    pub(super) fn static_call(
        &mut self,
        call: &StaticMethodCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let own =
            class_key(call.class).is_some_and(|key| self.classes.iter().any(|c| c.key == key));
        let relative = matches!(
            call.class,
            Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)
        );
        if own || relative {
            self.record_refused(node);
            self.refuse(node, "a static method call is not supported (P2R-135)");
            return None;
        }
        self.enum_call(call, node)
    }

    fn record_refused_arguments(&mut self, list: &ArgumentList<'_>) {
        for argument in list.arguments.iter() {
            let value = match argument {
                Argument::Positional(positional) => positional.value,
                Argument::Named(named) => named.value,
            };
            self.record_refused(value);
        }
    }
}

/// True for `__construct`, in any letter case.
fn is_constructor(method: &Method<'_>) -> bool {
    method.name.value.eq_ignore_ascii_case(b"__construct")
}

/// True when a hint names `self`, `static` or `parent`, also inside `?` or a union.
fn names_own_class(hint: &Hint<'_>) -> bool {
    match hint {
        Hint::Self_(_) | Hint::Static(_) | Hint::Parent(_) => true,
        Hint::Nullable(nullable) => names_own_class(nullable.hint),
        Hint::Parenthesized(inner) => names_own_class(inner.hint),
        Hint::Union(union) => names_own_class(union.left) || names_own_class(union.right),
        _ => false,
    }
}

/// How `new` stores a parameter in its field: a Copy value as it is, with the shorthand,
/// and any other value as an owned copy (P2R-133, P2R-137).
fn store(field: &ParameterSignature) -> String {
    let name = &field.rust_name;
    match &field.ty {
        ty if ty.is_copy() => name.clone(),
        RustType::String => format!("{name}: {name}.to_string()"),
        RustType::Vec(_) => format!("{name}: {name}.to_vec()"),
        RustType::Option(_) => format!("{name}: {name}.map(ToOwned::to_owned)"),
        _ => format!("{name}: {name}.clone()"),
    }
}
