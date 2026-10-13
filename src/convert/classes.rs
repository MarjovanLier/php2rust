//! Readonly classes: declarations, `new`, property reads, methods and method calls
//! (P2R-130 to P2R-137).

use std::collections::HashSet;

use heck::ToSnakeCase;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::enums::class_key;
use super::recursion::{self, Target};
use super::variables::THIS;
use super::{Context, Converter, Mode, ParameterSignature, Signature, Variables, lists, nulls};
use crate::printer::{self, RExpr};
use crate::types::{self, RustType};

/// rustfmt keeps a struct literal on one line when the text between its braces is at
/// most this wide (P2R-137).
const LITERAL_WIDTH: usize = 18;

/// A class of the file (P2R-130, P2R-140).
pub(super) struct ClassInfo {
    /// The lowercase PHP name, by which every use finds it (P2R-131).
    pub(super) key: Vec<u8>,
    pub(super) rust_name: String,
    /// The promoted constructor parameters, which are the first properties and the
    /// parameters of `new`, in source order (P2R-133).
    pub(super) fields: Vec<ParameterSignature>,
    /// The properties the class body declares, each with the Rust text of its default,
    /// after `fields` in the struct (P2R-140).
    declared: Vec<(ParameterSignature, String)>,
    /// False when a constructor parameter was refused, so `new` cannot check its
    /// arguments against `fields`.
    complete: bool,
    /// True when the class itself has a property that can change (P2R-141).
    writable: bool,
    /// True for a changing type: a value of it is never copied (P2R-141, P2R-145).
    pub(super) changing: bool,
    pub(super) methods: Vec<MethodInfo>,
}

pub(super) struct MethodInfo {
    /// The lowercase PHP name: PHP method names ignore letter case (P2R-131).
    key: Vec<u8>,
    /// The parameters without `$this`.
    pub(super) signature: Signature,
    /// `Mut` for a method that changes its object, `&mut self` (P2R-143).
    pub(super) self_mode: Mode,
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
                declared: Vec::new(),
                complete: true,
                writable: false,
                changing: false,
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
                self.declared_properties(declaration, index);
                self.method_signatures(declaration, index, &taken);
            }
        }
        for index in 0..self.classes.len() {
            self.check_containment(program, index);
        }
        self.mark_changing();
        self.refuse_nullable_changing(program);
    }

    /// Refuses each declared parameter, return and property type that is nullable and
    /// holds a changing type, once the changing types are known (P2R-141).
    fn refuse_nullable_changing(&mut self, program: &Program<'_>) {
        let mut found: Vec<Span> = Vec::new();
        for statement in program.statements.iter() {
            match statement {
                Statement::Function(function) => {
                    let key = function.name.value.to_ascii_lowercase();
                    if let Some(signature) = self.functions.get(&key) {
                        self.nullable_in(
                            &function.parameter_list,
                            function.return_type_hint.as_ref(),
                            signature,
                            &mut found,
                        );
                    }
                }
                Statement::Class(declaration) => {
                    let Some(index) = self.class_index(declaration) else {
                        continue;
                    };
                    for member in declaration.members.iter() {
                        match member {
                            ClassLikeMember::Method(method) => {
                                if is_constructor(method) {
                                    for parameter in method.parameter_list.parameters.iter() {
                                        let name = &parameter.variable.name[1..];
                                        let ty = self
                                            .properties(index)
                                            .find(|f| f.php_name == name)
                                            .map(|f| &f.ty);
                                        if ty.is_some_and(|ty| self.nullable_changing(ty)) {
                                            found.push(parameter.span());
                                        }
                                    }
                                } else if let Some(at) = self.method_index(index, method.name.value)
                                {
                                    let signature = &self.classes[index].methods[at].signature;
                                    self.nullable_in(
                                        &method.parameter_list,
                                        method.return_type_hint.as_ref(),
                                        signature,
                                        &mut found,
                                    );
                                }
                            }
                            ClassLikeMember::Property(Property::Plain(plain)) => {
                                for item in plain.items.iter() {
                                    let variable = item.variable();
                                    let name = &variable.name[1..];
                                    let ty = self
                                        .properties(index)
                                        .find(|f| f.php_name == name)
                                        .map(|f| &f.ty);
                                    if ty.is_some_and(|ty| self.nullable_changing(ty)) {
                                        found.push(variable.span());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        for span in found {
            self.refuse(&span, NULLABLE_CHANGING);
        }
    }

    /// Adds to `found` each parameter and the return type of `signature` that is nullable
    /// and holds a changing type.
    fn nullable_in(
        &self,
        parameters: &FunctionLikeParameterList<'_>,
        returns: Option<&FunctionLikeReturnTypeHint<'_>>,
        signature: &Signature,
        found: &mut Vec<Span>,
    ) {
        for (parameter, converted) in parameters.parameters.iter().zip(&signature.parameters) {
            if self.nullable_changing(&converted.ty) {
                found.push(parameter.span());
            }
        }
        if let (Some(hint), Some(ty)) = (returns, &signature.returns)
            && self.nullable_changing(ty)
        {
            found.push(hint.span());
        }
    }

    /// Finds the changing types: a class holding one is one too, so this repeats until no
    /// class is added. `new` then takes their values by value (P2R-141, P2R-144).
    fn mark_changing(&mut self) {
        let mut grew = true;
        while grew {
            grew = false;
            for index in 0..self.classes.len() {
                let changing = self.classes[index].writable
                    || self.properties(index).any(|f| self.is_changing(&f.ty));
                if changing && !self.classes[index].changing {
                    self.classes[index].changing = true;
                    grew = true;
                }
            }
        }
        for index in 0..self.classes.len() {
            for at in 0..self.classes[index].fields.len() {
                if self.is_changing(&self.classes[index].fields[at].ty) {
                    self.classes[index].fields[at].mode = Mode::Owned;
                }
            }
        }
    }

    /// True for a changing type: a changing class, or a list or `Option` of one (P2R-141).
    pub(super) fn is_changing(&self, ty: &RustType) -> bool {
        match ty {
            RustType::Class(name) => self
                .class_named(name)
                .is_some_and(|index| self.classes[index].changing),
            RustType::Vec(inner) | RustType::Option(inner) => self.is_changing(inner),
            _ => false,
        }
    }

    /// True for a nullable type that holds a changing type (`?list<Cell>`), also as a list
    /// element: none of the `Option` forms of P2R-101 to P2R-105 moves without a copy
    /// (P2R-141).
    pub(super) fn nullable_changing(&self, ty: &RustType) -> bool {
        match ty {
            RustType::Option(inner) => self.is_changing(inner),
            RustType::Vec(element) => self.nullable_changing(element),
            _ => false,
        }
    }

    /// The type of the property with the PHP name `php` of the class `rust_name`.
    pub(super) fn property_type(&self, rust_name: &str, php: &[u8]) -> Option<RustType> {
        let index = self.class_named(rust_name)?;
        self.properties(index)
            .find(|f| f.php_name == php)
            .map(|f| f.ty.clone())
    }

    /// Every property of the class at `index`, in struct order.
    fn properties(&self, index: usize) -> impl Iterator<Item = &ParameterSignature> {
        let class = &self.classes[index];
        class
            .fields
            .iter()
            .chain(class.declared.iter().map(|(f, _)| f))
    }

    /// The class a class declaration registered, when its name converts.
    pub(super) fn class_index(&self, declaration: &Class<'_>) -> Option<usize> {
        let key = declaration.name.value.to_ascii_lowercase();
        self.classes.iter().position(|c| c.key == key)
    }

    pub(super) fn class_named(&self, rust_name: &str) -> Option<usize> {
        self.classes.iter().position(|c| c.rust_name == rust_name)
    }

    /// The method of the class at `index` with the PHP name `php`, in any letter case.
    pub(super) fn method_index(&self, index: usize, php: &[u8]) -> Option<usize> {
        let key = php.to_ascii_lowercase();
        self.classes[index]
            .methods
            .iter()
            .position(|m| m.key == key)
    }

    /// Refuses everything a class of these slices may not hold (P2R-130, P2R-140).
    fn class_shape(&mut self, declaration: &Class<'_>) {
        if let Some(attributes) = declaration.attribute_lists.first() {
            self.refuse(attributes, "attributes are not supported (P2R-07)");
        }
        let readonly = is_readonly(declaration);
        if let Some(modifier) = declaration
            .modifiers
            .iter()
            .find(|m| !matches!(m, Modifier::Readonly(_) | Modifier::Final(_)))
        {
            self.refuse(
                modifier,
                "only readonly and final may stand on a class (P2R-130)",
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
            let method = match member {
                ClassLikeMember::Method(method) => method,
                ClassLikeMember::Property(_) if !readonly => continue,
                _ if readonly => {
                    self.refuse(
                        member,
                        "a readonly class may hold only a constructor and methods (P2R-130)",
                    );
                    continue;
                }
                _ => {
                    self.refuse(
                        member,
                        "a class may hold only a constructor, methods and properties (P2R-140)",
                    );
                    continue;
                }
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
            if !is_readonly(declaration)
                && !parameter
                    .modifiers
                    .iter()
                    .any(|m| matches!(m, Modifier::Readonly(_)))
            {
                self.classes[index].writable = true;
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
                mode: Mode::Shared,
            });
        }
        self.classes[index].complete = fields.len() == constructor.parameter_list.parameters.len();
        self.classes[index].fields = fields;
    }

    /// The properties a class that is not readonly declares in its body, each with a type
    /// and a literal default (P2R-140). Each one can change.
    fn declared_properties(&mut self, declaration: &Class<'_>, index: usize) {
        if is_readonly(declaration) {
            return;
        }
        for member in declaration.members.iter() {
            let ClassLikeMember::Property(property) = member else {
                continue;
            };
            let Property::Plain(plain) = property else {
                self.refuse(property, "a property hook is not supported (P2R-140)");
                continue;
            };
            let problem = if let Some(var) = &plain.var {
                Some((var.span(), "var is not supported (P2R-140)"))
            } else if let Some(attributes) = plain.attribute_lists.first() {
                Some((attributes.span(), "attributes are not supported (P2R-07)"))
            } else {
                plain
                    .modifiers
                    .iter()
                    .find(|m| {
                        matches!(
                            m,
                            Modifier::Static(_) | Modifier::Readonly(_) | Modifier::Abstract(_)
                        )
                    })
                    .map(|modifier| {
                        (
                            modifier.span(),
                            "a static, readonly or abstract declared property is not supported; PHP rejects a default on a readonly one (P2R-140)",
                        )
                    })
            };
            if let Some((span, message)) = problem {
                self.refuse(&span, message);
                continue;
            }
            for item in plain.items.iter() {
                match item {
                    PropertyItem::Concrete(item) if plain.hint.is_some() => {
                        self.declared_property(declaration, index, item);
                    }
                    _ => self.refuse(item, NEEDS_DEFAULT),
                }
            }
        }
    }

    /// One declared property with a type and a default, added to the class at `index`
    /// when it converts (P2R-140).
    fn declared_property(
        &mut self,
        declaration: &Class<'_>,
        index: usize,
        item: &PropertyConcreteItem<'_>,
    ) {
        let php_name = &item.variable.name[1..];
        let Some(rust_name) = self.rust_name(php_name, &item.variable) else {
            return;
        };
        if self.properties(index).any(|f| f.rust_name == rust_name) {
            self.refuse(
                &item.variable,
                format!("{rust_name} is already the Rust name of another property (P2R-131)"),
            );
            return;
        }
        let codebase = self.codebase;
        let declared = codebase
            .get_property(declaration.name.value, item.variable.name)
            .and_then(|p| p.type_metadata.as_ref())
            .map(|t| &t.type_union);
        let Some(ty) = self.declared_type(&item.variable, declared) else {
            return;
        };
        if types::nullable_object(&ty) {
            self.refuse(&item.variable, types::NULLABLE_OBJECT);
            return;
        }
        if !types::built(&ty) {
            self.refuse(
                &item.variable,
                "only int, float, bool, string, list, nullable, enum and object properties are built so far (P2R-20)",
            );
            return;
        }
        let Some(default) = self.default_value(item.value, &ty) else {
            return;
        };
        let signature = ParameterSignature {
            php_name: php_name.to_vec(),
            rust_name,
            snake: std::str::from_utf8(php_name)
                .unwrap_or_default()
                .to_snake_case(),
            ty,
            mode: Mode::Shared,
        };
        self.classes[index].declared.push((signature, default));
        self.classes[index].writable = true;
    }

    /// The Rust text of a declared property's default: a literal of the property's type,
    /// `null`, `[]` or an enum case (P2R-140).
    fn default_value(&mut self, value: &Expression<'_>, ty: &RustType) -> Option<String> {
        let literal = match lists::unparenthesized(value) {
            Expression::Literal(_) | Expression::Access(Access::ClassConstant(_)) => true,
            Expression::UnaryPrefix(unary) => {
                matches!(unary.operator, UnaryPrefixOperator::Negation(_))
                    && matches!(
                        lists::unparenthesized(unary.operand),
                        Expression::Literal(Literal::Integer(_) | Literal::Float(_))
                    )
            }
            Expression::Array(array) => array.elements.is_empty(),
            _ => false,
        };
        if !literal {
            self.refuse(
                value,
                "a property default must be a literal of the property's type, null, [] or an enum case (P2R-140)",
            );
            return None;
        }
        let inner = ty.inner();
        let (expr, found) = match lists::unparenthesized(value) {
            Expression::Array(_) if matches!(inner, RustType::Vec(_)) => {
                (RExpr::List(Vec::new()), inner.clone())
            }
            Expression::Literal(Literal::Null(_)) if matches!(ty, RustType::Option(_)) => {
                (RExpr::None, ty.clone())
            }
            other => self.expression(other)?,
        };
        // An int literal fits a float property (P2R-25); every other value must have the
        // property's type, or its inner type for a nullable one.
        if found != *ty && found != *inner && !(found == RustType::I64 && *inner == RustType::F64) {
            self.refuse(
                value,
                "a property default must have the property's type (P2R-140)",
            );
            return None;
        }
        let expr = self.declared_value(value, expr, &found, ty)?;
        let expr = nulls::wrapped(self.owned(expr, &found), &found, ty);
        match printer::expr(&expr, false) {
            Ok(text) => Some(text),
            Err(_) => {
                self.refuse(
                    value,
                    "this property default is too wide for one line (P2R-140)",
                );
                None
            }
        }
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
                    self_mode: Mode::Shared,
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
            for field in self.properties(at) {
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
        let properties: Vec<ParameterSignature> = self.properties(index).cloned().collect();
        // A value of a changing type is never copied (P2R-141).
        let derive = if self.classes[index].changing {
            "#[derive(Debug)]"
        } else {
            "#[derive(Clone, Debug)]"
        };
        let mut lines = vec![derive.to_owned()];
        if properties.is_empty() {
            lines.push(format!("pub struct {name} {{}}"));
        } else {
            lines.push(format!("pub struct {name} {{"));
            for field in &properties {
                let line = format!("pub {}: {},", field.rust_name, field.ty.owned());
                lines.push(self.emit(&declaration.name, 1, line));
            }
            lines.push("}".to_owned());
        }
        lines.push(String::new());
        lines.push(format!("impl {name} {{"));
        let parameters: Vec<String> = fields
            .iter()
            .map(|f| format!("{}: {}", f.rust_name, f.mode.parameter(&f.ty)))
            .collect();
        let open = format!("pub fn new({}) -> {name} {{", parameters.join(", "));
        lines.push(self.emit(&declaration.name, 1, open));
        let stores: Vec<String> = fields
            .iter()
            .map(store)
            .chain(
                self.classes[index]
                    .declared
                    .iter()
                    .map(|(f, default)| format!("{}: {default}", f.rust_name)),
            )
            .collect();
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
            mode: self.classes[class].methods[at].self_mode,
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
        // The refused constructor parameter is the problem reported for the class.
        if !self.classes[index].complete {
            self.record_refused(node);
            return None;
        }
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
            .and_then(|index| self.properties(index).find(|f| f.php_name == name.value))
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
        // A `&mut self` method changes the object its receiver chain starts from (P2R-144).
        if self.classes[index].methods[at].self_mode == Mode::Mut
            && !self.change_root(node, &object)
        {
            return None;
        }
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

/// The problem of a nullable type that holds a changing type (P2R-141).
pub(super) const NULLABLE_CHANGING: &str =
    "a nullable type that holds objects whose class changes them is not supported (P2R-141)";

/// The problem of a declared property without a type or a default (P2R-140).
const NEEDS_DEFAULT: &str = "a declared property needs a type and a default value (P2R-140)";

/// True for a class declared `readonly` (P2R-130).
fn is_readonly(declaration: &Class<'_>) -> bool {
    declaration
        .modifiers
        .iter()
        .any(|m| matches!(m, Modifier::Readonly(_)))
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

/// How `new` stores a parameter in its field: a Copy value or a value it takes by value as
/// it is, with the shorthand, and any other value as an owned copy (P2R-133, P2R-137,
/// P2R-144).
fn store(field: &ParameterSignature) -> String {
    let name = &field.rust_name;
    match &field.ty {
        ty if ty.is_copy() || field.mode == Mode::Owned => name.clone(),
        RustType::String => format!("{name}: {name}.to_string()"),
        RustType::Vec(_) => format!("{name}: {name}.to_vec()"),
        RustType::Option(_) => format!("{name}: {name}.map(ToOwned::to_owned)"),
        _ => format!("{name}: {name}.clone()"),
    }
}
