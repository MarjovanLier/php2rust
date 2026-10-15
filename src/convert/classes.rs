//! Readonly classes: declarations, `new`, property reads, methods and method calls
//! (P2R-130 to P2R-137).

use std::collections::HashSet;

use heck::ToSnakeCase;
use mago_codex::ttype::union::TUnion;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::control::emit;
use super::recursion::{self, Target};
use super::variables::THIS;
use super::{
    Context, Converter, Mode, ParameterSignature, Signature, Variables, functions, helpers, lists,
    nulls,
};
use crate::names;
use crate::printer::{self, RExpr};
use crate::types::{self, RustType};

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
    /// True for the methods and constants of an enum, which has no struct or `new`
    /// (P2R-155).
    pub(super) is_enum: bool,
    /// The template arguments K and V of `ArrayAccess`, for a class that implements it
    /// (P2R-220).
    pub(super) array_access: Option<(TUnion, TUnion)>,
    pub(super) constants: Vec<ConstantInfo>,
    /// The PHP names of the constants it declares that were refused, whose reads stop
    /// without a second problem.
    pub(super) refused_constants: Vec<Vec<u8>>,
}

impl ClassInfo {
    pub(super) fn new(key: Vec<u8>, rust_name: String, is_enum: bool) -> Self {
        Self {
            key,
            rust_name,
            fields: Vec::new(),
            declared: Vec::new(),
            complete: true,
            writable: false,
            changing: false,
            methods: Vec::new(),
            is_enum,
            array_access: None,
            constants: Vec::new(),
            refused_constants: Vec::new(),
        }
    }

    /// The type of `$this` in a method: the struct, or the enum (P2R-155).
    pub(super) fn own_type(&self) -> RustType {
        if self.is_enum {
            RustType::Enum(self.rust_name.clone())
        } else {
            RustType::Class(self.rust_name.clone())
        }
    }
}

pub(super) struct MethodInfo {
    /// The lowercase PHP name: PHP method names ignore letter case (P2R-131).
    key: Vec<u8>,
    /// The parameters without `$this`.
    pub(super) signature: Signature,
    /// `Mut` for a method that changes its object, `&mut self` (P2R-143).
    pub(super) self_mode: Mode,
    /// A static method has no `self` (P2R-152).
    pub(super) is_static: bool,
    /// The positions of the parameters that K or V of `ArrayAccess` types (P2R-220).
    templated: Vec<usize>,
}

/// A constant of a class or enum (P2R-156).
pub(super) struct ConstantInfo {
    /// The PHP name: PHP constant names keep their letter case.
    php_name: Vec<u8>,
    rust_name: String,
    ty: RustType,
    /// The `pub const` line in the `impl` block.
    line: String,
    /// The value of an int constant, which P2R-50 knows as it knows a literal.
    known: Option<i64>,
    /// The value of a string constant, which a pattern reads (P2R-260).
    text: Option<String>,
    /// True for a string map constant, a slice of pairs that only `strtr` reads (P2R-281).
    pairs: bool,
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
            let key = self.name_key(&declaration.name, declaration.name.value);
            self.type_names
                .insert(key.clone(), RustType::Class(rust_name.clone()));
            self.classes.push(ClassInfo::new(key, rust_name, false));
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
            let (name, members) = match statement {
                Statement::Class(declaration) => {
                    if self.class_index(declaration).is_some() {
                        self.class_shape(declaration);
                    }
                    (&declaration.name, &declaration.members)
                }
                Statement::Enum(declaration) => (&declaration.name, &declaration.members),
                _ => continue,
            };
            let key = self.name_key(name, name.value);
            let Some(index) = self.classes.iter().position(|c| c.key == key) else {
                continue;
            };
            if let Statement::Class(declaration) = statement {
                self.fields(declaration, index);
                self.declared_properties(declaration, index);
            }
            self.method_signatures(name, members, index, &taken);
            let mut all = self.trait_members(program, members);
            all.extend(members.iter());
            self.constants(&all, index);
        }
        for index in 0..self.classes.len() {
            self.check_containment(program, index);
        }
        self.mark_changing();
        self.refuse_nullable_changing(program);
    }

    /// Refuses what a trait holds besides constants, which a later slice converts; the
    /// trait itself writes nothing (P2R-280).
    pub(super) fn check_trait(&mut self, declaration: &Trait<'_>) {
        if let Some(attributes) = declaration.attribute_lists.first() {
            self.refuse(attributes, "attributes are not supported (P2R-07)");
        }
        // PHP rejects a trait with the name of a class-like it has built in, or a name it
        // reserves.
        let key = self.name_key(&declaration.name, declaration.name.value);
        let lower = String::from_utf8_lossy(declaration.name.value).to_ascii_lowercase();
        if self.built_in_classes.contains(&key)
            || super::enums::RESERVED
                .split_whitespace()
                .any(|word| word == lower)
        {
            self.refuse(
                &declaration.name,
                "a trait cannot take a name PHP reserves or the name of a class, interface, enum or trait PHP has built in (P2R-280)",
            );
        }
        // Its constants are checked only where a class or enum copies them, and PHP checks
        // them when it reads the file.
        if !self.used_traits.contains(&key) {
            self.refuse(
                &declaration.name,
                "a trait that no class or enum of the input uses is not supported, because the tool checks its constants only where they are copied (P2R-280)",
            );
        }
        for member in declaration.members.iter() {
            if !matches!(member, ClassLikeMember::Constant(_)) {
                self.refuse(
                    member,
                    "a trait may hold only constants; methods, properties and other traits are not supported yet (P2R-280)",
                );
            }
        }
    }

    /// The members of the traits that `members` use, in the order of the `use` statements,
    /// each trait in its own order (P2R-280). A `use` with a block of conflict rules, and a
    /// trait the input does not declare, stop.
    fn trait_members<'p, 'a>(
        &mut self,
        program: &'p Program<'a>,
        members: &Sequence<'_, ClassLikeMember<'_>>,
    ) -> Vec<&'p ClassLikeMember<'a>> {
        let mut found = Vec::new();
        let mut seen: Vec<Vec<u8>> = Vec::new();
        for member in members.iter() {
            let ClassLikeMember::TraitUse(trait_use) = member else {
                continue;
            };
            if let TraitUseSpecification::Concrete(specification) = &trait_use.specification {
                self.refuse(
                    specification,
                    "a use of a trait with a block of conflict rules is not supported (P2R-280)",
                );
                continue;
            }
            for name in trait_use.trait_names.iter() {
                let key = self.identifier_key(name);
                // PHP takes a trait named twice once, which would copy its constants twice.
                if seen.contains(&key) {
                    self.refuse(
                        name,
                        "a trait named twice in one class or enum is not supported (P2R-280)",
                    );
                    continue;
                }
                seen.push(key.clone());
                let declared = program
                    .statements
                    .iter()
                    .find_map(|statement| match statement {
                        Statement::Trait(declaration)
                            if self.name_key(&declaration.name, declaration.name.value) == key =>
                        {
                            Some(declaration)
                        }
                        _ => None,
                    });
                match declared {
                    Some(declaration) => {
                        self.used_traits.insert(key);
                        found.extend(declaration.members.iter());
                    }
                    None => self.refuse(
                        name,
                        "a use of a trait that the input does not declare is not supported (P2R-280)",
                    ),
                }
            }
        }
        found
    }

    /// Refuses each declared parameter, return and property type that is nullable and
    /// holds a changing type, once the changing types are known (P2R-141).
    fn refuse_nullable_changing(&mut self, program: &Program<'_>) {
        let mut found: Vec<Span> = Vec::new();
        for statement in program.statements.iter() {
            match statement {
                Statement::Function(function) => {
                    if let Some(signature) = self.functions.get(&self.function_key(function)) {
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
                // An enum method follows the rules for functions (P2R-155).
                Statement::Enum(declaration) => {
                    let key = self.name_key(&declaration.name, declaration.name.value);
                    let Some(index) = self.classes.iter().position(|c| c.key == key) else {
                        continue;
                    };
                    for member in declaration.members.iter() {
                        let ClassLikeMember::Method(method) = member else {
                            continue;
                        };
                        if let Some(at) = self.method_index(index, method.name.value) {
                            let signature = &self.classes[index].methods[at].signature;
                            self.nullable_in(
                                &method.parameter_list,
                                method.return_type_hint.as_ref(),
                                signature,
                                &mut found,
                            );
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
            RustType::Union(name) => self
                .type_names
                .union(name)
                .is_some_and(|members| members.iter().any(|member| self.is_changing(member))),
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
            // A map clones its values, which such a class does not derive (P2R-240).
            RustType::Map(_, value) => self.is_changing(value) || self.nullable_changing(value),
            // A union enum holds its member inline, as an `Option` does (P2R-230).
            RustType::Union(_) => self.is_changing(ty),
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
    pub(super) fn properties(&self, index: usize) -> impl Iterator<Item = &ParameterSignature> {
        let class = &self.classes[index];
        class
            .fields
            .iter()
            .chain(class.declared.iter().map(|(f, _)| f))
    }

    /// The class a class declaration registered, when its name converts.
    pub(super) fn class_index(&self, declaration: &Class<'_>) -> Option<usize> {
        let key = self.name_key(&declaration.name, declaration.name.value);
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

    /// True when the class extends Spatie's `Data`, which the struct leaves out (P2R-350).
    /// Mago's parser accepts a list of parents, which PHP rejects. A `Data` the input
    /// declares itself is a class of the input, which converts with a Rust type.
    pub(super) fn extends_data(&self, declaration: &Class<'_>) -> bool {
        declaration.extends.as_ref().is_some_and(|extends| {
            extends.types.len() == 1
                && extends
                    .types
                    .iter()
                    .all(|parent| self.identifier_key(parent) == b"spatie\\laraveldata\\data")
        }) && self
            .codebase
            .get_class_like(b"spatie\\laraveldata\\data")
            .is_some_and(|data| {
                !self
                    .files
                    .iter()
                    .any(|analysed| analysed.file == data.span.file_id)
            })
    }

    /// True when the attribute is one of laravel-data's, which only the methods of `Data`
    /// read, with arguments PHP accepts when it compiles the file (P2R-351).
    fn data_attribute(&self, attribute: &Attribute<'_>) -> bool {
        self.identifier_key(&attribute.name)
            .starts_with(b"spatie\\laraveldata\\attributes\\")
            && plain_arguments(attribute)
    }

    /// Refuses everything a class of these slices may not hold (P2R-130, P2R-140).
    fn class_shape(&mut self, declaration: &Class<'_>) {
        let data = self.extends_data(declaration);
        // The first group that holds an attribute this class may not leave out.
        if let Some(attributes) = declaration.attribute_lists.iter().find(|list| {
            !data
                || !list
                    .attributes
                    .iter()
                    .all(|attribute| self.data_attribute(attribute))
        }) {
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
        if let Some(extends) = &declaration.extends
            && !data
        {
            self.refuse(
                extends,
                "a class that extends another is not supported (P2R-130)",
            );
        }
        // The override check below needs every method of Data, also those of its traits.
        if data
            && let Some(extends) = &declaration.extends
            && self
                .codebase
                .get_class_like(b"spatie\\laraveldata\\data")
                .is_some_and(|parent| {
                    parent
                        .used_traits
                        .iter()
                        .chain(parent.all_parent_interfaces.iter())
                        .any(|name| !self.codebase.class_like_exists(name.as_bytes()))
                })
        {
            self.refuse(
                extends,
                "Spatie\\LaravelData\\Data names a trait or interface that no included file declares, so its methods are not all known; include the whole package (P2R-350)",
            );
        }
        // PHP calls these by itself, so a Data that has one is not the 4.18 shape.
        if data
            && let Some(extends) = &declaration.extends
            && let Some(magic) = IMPLICIT_METHODS.iter().find(|name| {
                self.codebase
                    .method_exists(b"Spatie\\LaravelData\\Data", name.as_bytes())
            })
        {
            self.refuse(
                extends,
                format!("Spatie\\LaravelData\\Data has the method {magic}, which PHP calls by itself (P2R-350)"),
            );
        }
        if let Some(implements) = &declaration.implements {
            self.implements(declaration, implements);
        }
        let array_access = self
            .class_index(declaration)
            .is_some_and(|index| self.classes[index].array_access.is_some());
        let mut constructors = 0;
        for member in declaration.members.iter() {
            let method = match member {
                ClassLikeMember::Method(method) => method,
                // Checked where the constants are read (P2R-280).
                ClassLikeMember::Constant(_) | ClassLikeMember::TraitUse(_) => continue,
                ClassLikeMember::Property(_) if !readonly => continue,
                _ if readonly => {
                    self.refuse(
                        member,
                        "a readonly class may hold only a constructor, methods and constants (P2R-130)",
                    );
                    continue;
                }
                _ => {
                    self.refuse(
                        member,
                        "a class may hold only a constructor, methods, properties and constants (P2R-140)",
                    );
                    continue;
                }
            };
            let overrides =
                array_access && ARRAY_ACCESS_METHODS.contains(&lowercase(method).as_slice());
            self.method_shape(method, overrides);
            // PHP checks an override against the parent, which Mago does not always do.
            if data
                && self
                    .codebase
                    .method_exists(b"Spatie\\LaravelData\\Data", method.name.value)
            {
                self.refuse(
                    &method.name,
                    "Spatie\\LaravelData\\Data has a method of this name, and overriding it is not supported (P2R-350)",
                );
            }
            // PHP rejects a parameter that narrows the interface's mixed, and Mago does
            // not report it.
            if overrides {
                let name = lowercase(method);
                for (at, parameter) in method.parameter_list.parameters.iter().enumerate() {
                    let interface = at == 0 || (at == 1 && name == b"offsetset");
                    if interface
                        && parameter
                            .hint
                            .as_ref()
                            .is_some_and(|hint| !matches!(hint, Hint::Mixed(_)))
                    {
                        self.refuse(
                            parameter,
                            "a parameter of an ArrayAccess method must keep the type mixed (P2R-220)",
                        );
                    }
                }
            }
            if is_constructor(method) {
                constructors += 1;
                if constructors > 1 {
                    self.refuse(
                        &method.name,
                        "a class can have one constructor only (P2R-130)",
                    );
                }
                self.constructor_shape(method, data);
            }
        }
    }

    /// `implements ArrayAccess`, whose template arguments K and V the class records when
    /// both have Rust types; any other interface stops (P2R-130, P2R-220).
    fn implements(&mut self, declaration: &Class<'_>, implements: &Implements<'_>) {
        let keys: Vec<Vec<u8>> = implements
            .types
            .iter()
            .map(|identifier| self.identifier_key(identifier))
            .collect();
        if keys != [b"arrayaccess".to_vec()] {
            self.refuse(
                implements,
                "a class that implements an interface other than ArrayAccess alone is not supported (P2R-220)",
            );
            return;
        }
        let Some(index) = self.class_index(declaration) else {
            return;
        };
        let class = self.classes[index].key.clone();
        let arguments = self.codebase.get_class_like(&class).and_then(|meta| {
            let (_, found) = meta
                .template_extended_parameters
                .iter()
                .find(|(interface, _)| interface.as_bytes() == b"arrayaccess")?;
            let argument = |name: &[u8]| {
                found
                    .iter()
                    .find(|(parameter, _)| parameter.as_bytes() == name)
                    .map(|(_, ty)| ty.clone())
            };
            Some((argument(b"K")?, argument(b"V")?))
        });
        // A `mixed` slot keeps an int where P2R-25 would make a float of it, so a float K
        // or V, or a union with a float member, has no faithful form (P2R-230).
        let rust = |union: &TUnion| {
            types::declared_type(union, &self.type_names)
                .ok()
                .filter(|ty| match ty.inner() {
                    RustType::F64 => false,
                    RustType::Union(name) => self
                        .type_names
                        .union(name)
                        .is_some_and(|members| !members.contains(&RustType::F64)),
                    _ => true,
                })
        };
        let typed = arguments.filter(|(key, value)| {
            !key.is_mixed() && !value.is_mixed() && rust(key).is_some() && rust(value).is_some()
        });
        match typed {
            Some(arguments) => self.classes[index].array_access = Some(arguments),
            None => self.refuse(
                implements,
                "ArrayAccess converts only with @implements ArrayAccess<K, V> whose K and V have Rust types other than float (P2R-220)",
            ),
        }
    }

    /// The type Mago declares for parameter `position` of `method`, or for its return when
    /// `position` is `None`, with a `mixed` of an `ArrayAccess` method of the class at
    /// `class` replaced by K or V (P2R-220).
    fn templated<'u>(
        &'u self,
        class: usize,
        method: &Method<'_>,
        position: Option<usize>,
        declared: Option<&'u TUnion>,
    ) -> Option<&'u TUnion> {
        let Some((key, value)) = &self.classes[class].array_access else {
            return declared;
        };
        if !declared.is_some_and(TUnion::is_mixed) {
            return declared;
        }
        let name = lowercase(method);
        match (name.as_slice(), position) {
            (b"offsetexists" | b"offsetget" | b"offsetset" | b"offsetunset", Some(0)) => Some(key),
            (b"offsetset", Some(1)) | (b"offsetget", None) => Some(value),
            _ => declared,
        }
    }

    /// Refuses a method form no class or enum of these slices may hold (P2R-130,
    /// P2R-135, P2R-152, P2R-155). `overrides` allows `#[Override]` on a method that
    /// implements a method of `ArrayAccess` (P2R-221).
    pub(super) fn method_shape(&mut self, method: &Method<'_>, overrides: bool) {
        // PHP rejects a repeated `#[Override]`, and Mago does not report it.
        let attributes: Vec<&Attribute<'_>> = method
            .attribute_lists
            .iter()
            .flat_map(|list| list.attributes.iter())
            .collect();
        let only_override = attributes.len() == 1
            && attributes[0]
                .argument_list
                .as_ref()
                .is_none_or(|list| list.arguments.is_empty())
            && self.identifier_key(&attributes[0].name) == b"override";
        if let Some(attributes) = method.attribute_lists.first()
            && !(overrides && only_override)
        {
            self.refuse(attributes, "attributes are not supported (P2R-07)");
        }
        if let Some(modifier) = method
            .modifiers
            .iter()
            .find(|m| matches!(m, Modifier::Abstract(_)))
        {
            self.refuse(modifier, "an abstract method is not supported (P2R-135)");
        }
        if method.ampersand.is_some() {
            self.refuse(
                &method.name,
                "a method that returns by reference is not supported (P2R-135)",
            );
        }
        if !is_constructor(method) && method.name.value.starts_with(b"__") {
            self.refuse(
                &method.name,
                "a method whose name starts with __ is not supported (P2R-130)",
            );
        }
        // PHP prints a warning for it when it compiles the file; Mago reports nothing.
        if !is_constructor(method) && final_private(&method.modifiers) {
            self.refuse(
                &method.name,
                "a private method cannot be final, and PHP warns about it (P2R-135)",
            );
        }
        // PHP rejects a `static` parameter type; Mago's parser accepts it, alone as a
        // modifier and inside a union as a type.
        for parameter in method.parameter_list.parameters.iter() {
            if static_modifier(parameter)
                || parameter
                    .hint
                    .as_ref()
                    .is_some_and(|hint| names(hint, is_static))
            {
                self.refuse(
                    parameter,
                    "static can only be a return type, which PHP requires (P2R-154)",
                );
            }
        }
    }

    /// A constructor converts only when every parameter promotes to a typed property and
    /// its body is empty (P2R-130). With `data` (the class extends Spatie's `Data`),
    /// laravel-data's attributes are left out (P2R-351).
    fn constructor_shape(&mut self, method: &Method<'_>, data: bool) {
        match &method.body {
            MethodBody::Concrete(block) if block.statements.is_empty() => {}
            _ => self.refuse(
                &method.name,
                "a constructor with a body is not supported (P2R-130)",
            ),
        }
        self.defaults_last(&method.parameter_list);
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
            } else if !parameter.attribute_lists.is_empty()
                && !(data
                    && parameter
                        .attribute_lists
                        .iter()
                        .flat_map(|list| list.attributes.iter())
                        .all(|attribute| self.data_attribute(attribute)))
            {
                Some("attributes are not supported (P2R-07)")
            } else if data && DATA_PROPERTIES.contains(&&parameter.variable.name[1..]) {
                Some(DATA_PROPERTY)
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
            if helpers::is_helper(&rust_name) {
                self.refuse(&parameter.variable, helpers::HELPER_NAME);
                continue;
            }
            if fields.iter().any(|f| f.rust_name == rust_name) {
                self.refuse(
                    &parameter.variable,
                    format!("{rust_name} is already the Rust name of another property (P2R-131)"),
                );
                continue;
            }
            let codebase = self.codebase;
            let class = self.name_key(&declaration.name, declaration.name.value);
            let declared = codebase
                .get_property(&class, parameter.variable.name)
                .and_then(|p| p.type_metadata.as_ref())
                .map(|t| &t.type_union);
            let Some(ty) = self.declared_type(parameter, declared, parameter.hint.as_ref()) else {
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
            // A `self::Case` default names this class (P2R-168).
            let outer = self.current_class.replace(index);
            let default = parameter
                .default_value
                .as_ref()
                .map(|default| self.parameter_default(parameter, default.value, &ty));
            self.current_class = outer;
            let default = match default {
                Some(None) => continue,
                Some(found) => found,
                None => None,
            };
            fields.push(ParameterSignature {
                php_name: php_name.to_vec(),
                rust_name,
                snake: std::str::from_utf8(php_name)
                    .unwrap_or_default()
                    .to_snake_case(),
                ty,
                mode: Mode::Shared,
                default,
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
                    _ if self.extends_data(declaration)
                        && DATA_PROPERTIES.contains(&&item.variable().name[1..]) =>
                    {
                        self.refuse(item, DATA_PROPERTY);
                    }
                    PropertyItem::Concrete(item) if plain.hint.is_some() => {
                        self.declared_property(declaration, index, item, plain.hint.as_ref());
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
        hint: Option<&Hint<'_>>,
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
        let class = self.name_key(&declaration.name, declaration.name.value);
        let declared = codebase
            .get_property(&class, item.variable.name)
            .and_then(|p| p.type_metadata.as_ref())
            .map(|t| &t.type_union);
        let Some(ty) = self.declared_type(&item.variable, declared, hint) else {
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
            default: None,
        };
        self.classes[index].declared.push((signature, default));
        self.classes[index].writable = true;
    }

    /// The Rust text of a declared property's default: a literal of the property's type,
    /// `null`, `[]` or an enum case (P2R-140).
    fn default_value(&mut self, value: &Expression<'_>, ty: &RustType) -> Option<String> {
        let literal = match lists::unparenthesized(value) {
            Expression::Literal(_) => true,
            // A case, never a constant, which P2R-140 does not list.
            Expression::Access(Access::ClassConstant(access)) => {
                self.enum_case_of(access).is_some()
            }
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
            Expression::Array(array) if let RustType::Map(key, map_value) = inner => {
                self.map_literal(array, key, map_value)?
            }
            Expression::Literal(Literal::Null(_)) if matches!(ty, RustType::Option(_)) => {
                (RExpr::None, ty.clone())
            }
            other => self.expression(other)?,
        };
        // An int literal fits a float property (P2R-25), and a member value a union one
        // (P2R-231); every other value must have the property's type, or its inner type
        // for a nullable one.
        if found != *ty
            && found != *inner
            && !(found == RustType::I64 && *inner == RustType::F64)
            && nulls::member_path(&found, ty).is_none()
        {
            self.refuse(
                value,
                "a property default must have the property's type (P2R-140)",
            );
            return None;
        }
        let expr = self.declared_value(value, expr, &found, ty)?;
        let expr = nulls::wrapped(self.owned(expr, &found), &found, ty);
        Some(printer::expr(&expr, false))
    }

    /// The signature of every method of the class at `index`, other than the constructor
    /// (P2R-131, P2R-135).
    /// The signatures of the methods of the class or enum `name` at `index`, from its
    /// `members` (P2R-131, P2R-135, P2R-152, P2R-155).
    pub(super) fn method_signatures(
        &mut self,
        name: &LocalIdentifier<'_>,
        members: &Sequence<'_, ClassLikeMember<'_>>,
        index: usize,
        taken: &HashSet<String>,
    ) {
        let codebase = self.codebase;
        let class = self.name_key(name, name.value);
        let is_enum = self.classes[index].is_enum;
        let mut methods: Vec<MethodInfo> = Vec::new();
        // A `self::Case` default names this class or enum (P2R-168).
        let outer = self.current_class.replace(index);
        for member in members.iter() {
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
            // The helpers of P2R-115 take these names, and PHP itself rejects `cases`,
            // `from` and `tryFrom` in any letter case on a backed enum (P2R-155).
            let helper = matches!(
                rust_name.as_str(),
                "name" | "value" | "from" | "try_from" | "cases"
            ) || [&b"cases"[..], b"from", b"tryfrom"]
                .contains(&method.name.value.to_ascii_lowercase().as_slice());
            if is_enum && helper {
                self.refuse(
                    &method.name,
                    "an enum method may not take the name of name, value, from, tryFrom or cases (P2R-155)",
                );
                continue;
            }
            // PHP method names ignore letter case, so PHP rejects the second one, and Mago
            // does not report it.
            let key = method.name.value.to_ascii_lowercase();
            if methods
                .iter()
                .any(|m| m.signature.rust_name == rust_name || m.key == key)
            {
                self.refuse(
                    &method.name,
                    format!("{rust_name} is already the name of another method (P2R-131)"),
                );
                continue;
            }
            let Some(metadata) = codebase.get_method(&class, method.name.value) else {
                self.refuse(
                    &method.name,
                    "Mago has no signature for this method (P2R-23)",
                );
                continue;
            };
            let mut converted = self.defaults_last(&method.parameter_list);
            let mut parameters: Vec<ParameterSignature> = Vec::new();
            let mut templated = Vec::new();
            for (parameter, meta) in method
                .parameter_list
                .parameters
                .iter()
                .zip(metadata.parameters.iter())
            {
                if parameter.is_promoted_property() {
                    // Mago's parser reads a bare `static` type as a modifier, which
                    // method_shape refuses (P2R-154).
                    if !static_modifier(parameter) {
                        self.refuse(
                            parameter,
                            "only a constructor parameter can be promoted to a property (P2R-130)",
                        );
                    }
                    converted = false;
                    continue;
                }
                if parameter
                    .hint
                    .as_ref()
                    .is_some_and(|hint| names(hint, is_parent))
                {
                    self.refuse(parameter, "the type parent is not supported (P2R-154)");
                    converted = false;
                    continue;
                }
                let declared = meta.get_type_metadata().map(|t| &t.type_union);
                let mixed = declared.is_some_and(TUnion::is_mixed);
                let declared = self
                    .templated(index, method, Some(parameters.len()), declared)
                    .cloned();
                // K and V are never mixed, so a mixed type that is gone was replaced.
                if mixed && !declared.as_ref().is_some_and(TUnion::is_mixed) {
                    templated.push(parameters.len());
                }
                match self.parameter(parameter, declared.as_ref(), taken, &parameters) {
                    Some(signature) => parameters.push(signature),
                    None => converted = false,
                }
            }
            if let Some(hint) = &method.return_type_hint
                && names(&hint.hint, is_parent)
            {
                self.refuse(hint, "the type parent is not supported (P2R-154)");
                continue;
            }
            let declared = metadata
                .return_type_metadata
                .as_ref()
                .map(|t| &t.type_union);
            let declared = self.templated(index, method, None, declared).cloned();
            let returns = self.return_type(
                method.return_type_hint.as_ref(),
                &method.name,
                declared.as_ref(),
            );
            if let (Some(returns), true) = (returns, converted) {
                methods.push(MethodInfo {
                    key,
                    signature: Signature {
                        rust_name,
                        parameters,
                        returns,
                        throws: false,
                    },
                    self_mode: Mode::Shared,
                    is_static: method
                        .modifiers
                        .iter()
                        .any(|m| matches!(m, Modifier::Static(_))),
                    templated,
                });
            }
        }
        self.current_class = outer;
        self.classes[index].methods = methods;
    }

    /// The constants of the class or enum at `index`, from its `members`, which start with
    /// the members of the traits it uses (P2R-156, P2R-280).
    fn constants(&mut self, members: &[&ClassLikeMember<'_>], index: usize) {
        let outer = self.current_class.replace(index);
        let mut constants: Vec<ConstantInfo> = Vec::new();
        for &member in members {
            let ClassLikeMember::Constant(constant) = member else {
                continue;
            };
            if let Some(attributes) = constant.attribute_lists.first() {
                self.refuse(attributes, "attributes are not supported (P2R-07)");
                continue;
            }
            let mut items = constant.items.iter();
            let (Some(item), None) = (items.next(), items.next()) else {
                self.refuse(
                    constant,
                    "a const statement may declare one constant only (P2R-156)",
                );
                continue;
            };
            // PHP rejects both when it compiles the file; Mago reports neither.
            if item.name.value.eq_ignore_ascii_case(b"class") {
                self.refuse(&item.name, "PHP reserves the constant name class (P2R-156)");
                continue;
            }
            if bad_constant_modifiers(&constant.modifiers) {
                self.refuse(
                    constant,
                    "a constant takes one visibility and final at most, which PHP checks (P2R-156)",
                );
                continue;
            }
            if final_private(&constant.modifiers) {
                self.refuse(
                    constant,
                    "a private constant cannot be final, which PHP rejects (P2R-156)",
                );
                continue;
            }
            let php = String::from_utf8_lossy(item.name.value);
            let Some(rust_name) = names::constant_name(&php) else {
                self.refuse(
                    &item.name,
                    format!("the constant name {php} has no Rust form (P2R-156)"),
                );
                continue;
            };
            let class = &self.classes[index];
            // In Rust a case and a constant share the names of the type, and PHP rejects a
            // constant with the name of a case, which Mago does not report. A method name
            // always holds a lowercase letter, so it never takes a constant's name.
            let taken = self.has_case(&class.key, item.name.value)
                || constants.iter().any(|c| c.rust_name == rust_name)
                || (class.is_enum
                    && self
                        .case_paths(&class.rust_name)
                        .iter()
                        .any(|path| path.rsplit("::").next() == Some(rust_name.as_str())));
            if taken {
                self.refuse(
                    &item.name,
                    format!("the constant {rust_name} takes the name of a case or constant of this type (P2R-156)"),
                );
                continue;
            }
            if let Expression::Array(array) = item.value {
                let typed = constant
                    .hint
                    .as_ref()
                    .is_none_or(|hint| matches!(hint, Hint::Array(_)));
                if !typed {
                    self.refuse(
                        item.value,
                        "a string map constant converts only with the type array or none (P2R-281)",
                    );
                    continue;
                }
                let Some(pairs) = self.string_pairs(array) else {
                    continue;
                };
                constants.push(ConstantInfo {
                    php_name: item.name.value.to_vec(),
                    line: format!("    pub const {rust_name}: &[(&str, &str)] = &[{pairs}];"),
                    rust_name,
                    // Its reads stop outside `strtr`, so this type is never read (P2R-281).
                    ty: RustType::String,
                    known: None,
                    text: None,
                    pairs: true,
                });
                continue;
            }
            let Some((value, value_type)) = self.constant_value(item.value) else {
                continue;
            };
            let ty = match &constant.hint {
                None => value_type.clone(),
                Some(hint) => match self.constant_type(hint) {
                    Some(ty) => ty,
                    None => {
                        self.refuse(
                            hint,
                            "a constant's type must be int, float, bool, string or an enum of this file (P2R-156)",
                        );
                        continue;
                    }
                },
            };
            // PHP stores an int as a float in a float constant, so `-0` is `0.0` (P2R-25).
            let value = match (&ty, &value_type, printer::literal_value(&value)) {
                (RustType::F64, RustType::I64, Some(int)) => RExpr::Float(int as f64),
                _ => value,
            };
            let line = format!(
                "    pub const {rust_name}: {} = {};",
                constant_rust_type(&ty),
                printer::expr(&value, false)
            );
            let known = match ty {
                RustType::I64 => printer::literal_value(&value),
                _ => None,
            };
            let text = match value {
                RExpr::Str(text) => Some(text),
                _ => None,
            };
            constants.push(ConstantInfo {
                php_name: item.name.value.to_vec(),
                rust_name,
                ty,
                line,
                known,
                text,
                pairs: false,
            });
        }
        let refused: Vec<Vec<u8>> = members
            .iter()
            .filter_map(|member| match member {
                ClassLikeMember::Constant(constant) => Some(constant),
                _ => None,
            })
            .flat_map(|constant| constant.items.iter().map(|item| item.name.value.to_vec()))
            .filter(|name| !constants.iter().any(|c| c.php_name == *name))
            .collect();
        self.classes[index].refused_constants = refused;
        self.classes[index].constants = constants;
        self.current_class = outer;
    }

    /// The `pub const` lines of the class or enum at `index`, then a blank line, first in
    /// its `impl` block; nothing when it has none (P2R-157).
    pub(super) fn constant_lines(&self, index: usize) -> Vec<String> {
        let constants = &self.classes[index].constants;
        let mut lines: Vec<String> = constants.iter().map(|c| c.line.clone()).collect();
        if !lines.is_empty() {
            lines.push(String::new());
        }
        lines
    }

    /// The value of a constant: an int or float literal, with or without unary minus, a
    /// bool or string literal, or a case of an enum of the file (P2R-156).
    fn constant_value(&mut self, value: &Expression<'_>) -> Option<(RExpr, RustType)> {
        let number = |e: &Expression<'_>| {
            matches!(
                e,
                Expression::Literal(Literal::Integer(_) | Literal::Float(_))
            )
        };
        match value {
            Expression::Literal(
                Literal::Integer(_)
                | Literal::Float(_)
                | Literal::String(_)
                | Literal::True(_)
                | Literal::False(_),
            ) => self.expression(value),
            Expression::UnaryPrefix(unary)
                if matches!(unary.operator, UnaryPrefixOperator::Negation(_))
                    && number(unary.operand) =>
            {
                self.expression(value)
            }
            // PHP rejects `static::` in a constant's value.
            Expression::Access(Access::ClassConstant(access))
                if self.enum_case_of(access).is_some()
                    && !matches!(access.class, Expression::Static(_)) =>
            {
                self.expression(value)
            }
            _ => {
                self.refuse(
                    value,
                    "a constant's value must be a literal or a case of an enum of this file (P2R-156)",
                );
                None
            }
        }
    }

    /// The pairs of a string map constant's array literal, as Rust text in the literal's
    /// order, or `None` with a problem recorded (P2R-281).
    fn string_pairs(&mut self, array: &Array<'_>) -> Option<String> {
        if array.elements.is_empty() {
            self.refuse(
                array,
                "a string map constant needs at least one pair (P2R-281)",
            );
            return None;
        }
        let mut pairs: Vec<(String, String)> = Vec::new();
        for element in array.elements.iter() {
            let ArrayElement::KeyValue(pair) = element else {
                self.refuse(
                    element,
                    "a string map constant needs a string literal key on every element (P2R-281)",
                );
                return None;
            };
            let (
                Expression::Literal(Literal::String(key)),
                Expression::Literal(Literal::String(value)),
            ) = (pair.key, pair.value)
            else {
                self.refuse(
                    pair,
                    "a string map constant needs a string literal key and a string literal value on every element (P2R-281)",
                );
                return None;
            };
            let key = self.string_literal(key)?;
            let value = self.string_literal(value)?;
            // `strtr` ignores an empty key with a warning, which Rust does not print.
            if key.is_empty() {
                self.refuse(
                    pair,
                    "a string map constant cannot hold an empty key (P2R-281)",
                );
                return None;
            }
            if pairs.iter().any(|(seen, _)| *seen == key) {
                self.refuse(
                    pair,
                    "a key appears twice in this string map constant (P2R-281)",
                );
                return None;
            }
            pairs.push((key, value));
        }
        Some(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    format!(
                        "({}, {})",
                        printer::expr(&RExpr::Str(key), false),
                        printer::expr(&RExpr::Str(value), false)
                    )
                })
                .collect::<Vec<_>>()
                .join(", "),
        )
    }

    /// The Rust path of the string map constant that `access` reads, or `None` (P2R-281).
    pub(super) fn pairs_constant(&self, access: &ClassConstantAccess<'_>) -> Option<String> {
        let ClassLikeConstantSelector::Identifier(name) = &access.constant else {
            return None;
        };
        let key = self.class_key(access.class)?;
        let class = self.classes.iter().find(|c| c.key == key)?;
        let constant = class.constants.iter().find(|c| c.php_name == name.value)?;
        constant
            .pairs
            .then(|| format!("{}::{}", class.rust_name, constant.rust_name))
    }

    /// The Rust type of a constant's declared type (P2R-156).
    fn constant_type(&self, hint: &Hint<'_>) -> Option<RustType> {
        match hint {
            Hint::Integer(_) => Some(RustType::I64),
            Hint::Float(_) => Some(RustType::F64),
            Hint::Bool(_) => Some(RustType::Bool),
            Hint::String(_) => Some(RustType::String),
            Hint::Self_(_) | Hint::Static(_) => {
                let class = &self.classes[self.current_class?];
                class.is_enum.then(|| class.own_type())
            }
            Hint::Identifier(identifier) => {
                match self.type_names.get(&self.identifier_key(identifier)) {
                    Some(ty @ RustType::Enum(_)) => Some(ty.clone()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// `C::NAME`, `self::NAME` and `static::NAME` for a constant of a class or enum of the
    /// file, with its declared type. An int constant is a known value, as a literal is
    /// (P2R-50, P2R-156).
    pub(super) fn constant_read(
        &self,
        access: &ClassConstantAccess<'_>,
    ) -> Option<(RExpr, RustType)> {
        let ClassLikeConstantSelector::Identifier(name) = &access.constant else {
            return None;
        };
        let key = self.class_key(access.class)?;
        let class = self.classes.iter().find(|c| c.key == key)?;
        let constant = class.constants.iter().find(|c| c.php_name == name.value)?;
        let path = format!("{}::{}", class.rust_name, constant.rust_name);
        let read = match constant.known {
            Some(value) => RExpr::Known(path, value),
            None => RExpr::Path(path),
        };
        Some((read, constant.ty.clone()))
    }

    /// The value of the string constant that `access` reads, taken from its declaration,
    /// where Mago's literal type would follow a docblock `@var` (P2R-260).
    pub(super) fn constant_text(&self, access: &ClassConstantAccess<'_>) -> Option<String> {
        let ClassLikeConstantSelector::Identifier(name) = &access.constant else {
            return None;
        };
        let key = self.class_key(access.class)?;
        let class = self.classes.iter().find(|c| c.key == key)?;
        let constant = class.constants.iter().find(|c| c.php_name == name.value)?;
        constant.text.clone()
    }

    /// The type of `Name` read through the class or enum at `class`: a case gives the enum,
    /// a constant its declared type (P2R-112, P2R-156).
    pub(super) fn member_type(&self, class: usize, php: &[u8]) -> Option<RustType> {
        let info = &self.classes[class];
        if info.is_enum && self.has_case(&info.key, php) {
            return Some(info.own_type());
        }
        info.constants
            .iter()
            .find(|c| c.php_name == php)
            .map(|c| c.ty.clone())
    }

    /// True when `access` reads a constant its class or enum declares and the tool refused
    /// (P2R-156).
    pub(super) fn refused_constant(&self, access: &ClassConstantAccess<'_>) -> bool {
        let ClassLikeConstantSelector::Identifier(name) = &access.constant else {
            return false;
        };
        self.class_key(access.class)
            .and_then(|key| self.classes.iter().find(|c| c.key == key))
            .is_some_and(|class| class.refused_constants.iter().any(|n| n == name.value))
    }

    /// Refuses a class that holds itself by value, through its properties or those of
    /// other classes, because the struct would have no size; and one that holds an enum
    /// without cases, because no value of the struct could exist (P2R-130).
    fn check_containment(&mut self, program: &Program<'_>, index: usize) {
        let mut seen = vec![false; self.classes.len()];
        let mut pending = vec![index];
        let mut problem = None;
        while let Some(at) = pending.pop() {
            // `?C` and a union enum hold their objects inline too (P2R-204, P2R-230).
            let held: Vec<RustType> = self
                .properties(at)
                .flat_map(|field| match field.ty.inner() {
                    RustType::Union(name) => self.type_names.union(name).unwrap_or_default(),
                    RustType::Class(_) => vec![field.ty.inner().clone()],
                    _ => vec![field.ty.clone()],
                })
                .collect();
            for ty in &held {
                match ty {
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
                Statement::Class(declaration) if self.class_index(declaration) == Some(index) => {
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
                lines.push(emit(1, line));
            }
            lines.push("}".to_owned());
        }
        lines.push(String::new());
        lines.push(format!("impl {name} {{"));
        lines.extend(self.constant_lines(index));
        let parameters: Vec<String> = fields
            .iter()
            .map(|f| format!("{}: {}", f.rust_name, f.mode.parameter(&f.ty)))
            .collect();
        let open = format!("pub fn new({}) -> {name} {{", parameters.join(", "));
        lines.push(emit(1, open));
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
        let literal = if stores.is_empty() {
            format!("{name} {{}}")
        } else {
            format!("{name} {{ {} }}", stores.join(", "))
        };
        lines.push(emit(2, literal));
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
    pub(super) fn method(
        &mut self,
        method: &Method<'_>,
        class: usize,
        at: usize,
    ) -> Option<Vec<String>> {
        let MethodBody::Concrete(block) = &method.body else {
            return None;
        };
        let signature = self.classes[class].methods[at].signature.clone();
        let mut parameters = Vec::new();
        // A static method has no `self` (P2R-152).
        if !self.classes[class].methods[at].is_static {
            parameters.push(ParameterSignature {
                php_name: THIS.to_vec(),
                rust_name: "self".to_owned(),
                snake: "self".to_owned(),
                ty: self.classes[class].own_type(),
                mode: self.classes[class].methods[at].self_mode,
                default: None,
            });
        }
        parameters.extend(signature.parameters.iter().cloned());
        // A method has its own variables, as a function does.
        let statements: Vec<&Statement<'_>> = block.statements.iter().collect();
        let checks = functions::owned_checks(&statements);
        let copied = self.copied_parameters(&statements, &signature.parameters, &checks);
        let outer = std::mem::replace(
            &mut self.variables,
            Variables::with_parameters(&parameters, &copied),
        );
        self.variables.owned_checks = checks.iter().map(|check| check.span).collect();
        // `self` is the first variable of a method that is not static.
        let first = parameters.len() - signature.parameters.len();
        let templated: Vec<usize> = self.classes[class].methods[at]
            .templated
            .iter()
            .map(|at| at + first)
            .collect();
        self.variables.template(&templated);
        let outer_method = self.current_method.replace((class, at));
        let outer_class = self.current_class.replace(class);
        let outer_calls = std::mem::take(&mut self.self_calls);
        let sites = self.site_types(block.statements.iter(), signature.returns.as_ref());
        self.variables.list_types = sites.lists;
        self.variables.null_types = sites.nulls;
        let mut body = Vec::new();
        let outer_body = self.begin_body(signature.throws);
        for statement in block.statements.iter() {
            self.statement(
                statement,
                2,
                Context::Function(signature.returns.as_ref()),
                &mut body,
            );
        }
        self.end_body(signature.returns.is_none(), 2, &mut body, outer_body);
        if signature.returns.is_some() && self.variables.left.is_none() {
            self.refuse(
                &method.name,
                "a function with a return type must return on every path (P2R-50)",
            );
        }
        let statements: Vec<&Statement<'_>> = block.statements.iter().collect();
        let target = Target {
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
        body.splice(
            0..0,
            functions::copy_lines(&signature.parameters, &copied, 2),
        );
        self.variables = outer;
        self.current_method = outer_method;
        self.current_class = outer_class;
        self.self_calls = outer_calls;
        let open = format!(
            "pub fn {}({}){} {{",
            signature.rust_name,
            parameters.join(", "),
            signature.returns_text()
        );
        if body.is_empty() {
            // rustfmt writes an empty body as `{}` on the same line.
            return Some(vec![emit(1, format!("{open}}}"))]);
        }
        let mut lines = vec![emit(1, open)];
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
        let index = self
            .class_key(instantiation.class)
            .and_then(|key| self.classes.iter().position(|c| c.key == key));
        let Some(index) = index.filter(|&index| !self.classes[index].is_enum) else {
            self.record_refused(node);
            let message = if matches!(instantiation.class, Expression::Parent(_)) {
                "parent is not supported (P2R-135)"
            } else {
                "new converts only for a class of this file (P2R-133)"
            };
            self.refuse(node, message);
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
        if matches!(object.1.inner(), RustType::Union(_)) {
            self.refuse(node, UNION_MEMBER);
            return None;
        }
        if matches!(&object.1, RustType::Option(inner) if matches!(**inner, RustType::Class(_))) {
            self.refuse(
                node,
                "a property read on a value that may be null is not supported (P2R-106)",
            );
            return None;
        }
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
        if let Some(converted) = self.caught_call(call, node) {
            return converted;
        }
        let Some((object, ty)) = self.expression(call.object) else {
            self.record_refused_arguments(&call.argument_list);
            return None;
        };
        if ty == RustType::DateTime {
            return self
                .date_method(call, node, object)
                .map(|(call, ty)| (call, Some(ty)));
        }
        // An enum value has the methods of its enum (P2R-155).
        let (RustType::Class(class) | RustType::Enum(class)) = &ty else {
            self.record_refused_arguments(&call.argument_list);
            let message = if matches!(ty.inner(), RustType::Union(_)) {
                UNION_MEMBER
            } else if matches!(ty, RustType::Option(_)) {
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
        if self.classes[index].methods[at].is_static {
            self.record_refused_arguments(&call.argument_list);
            self.refuse(
                node,
                "a static method is called with ::, not through an object (P2R-153)",
            );
            return None;
        }
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
        let call = RExpr::MethodCall(Box::new(object), signature.rust_name, arguments);
        Some((
            self.raised(node, call, signature.throws)?,
            signature.returns,
        ))
    }

    /// `C::m(args)`, `self::m(args)` and `static::m(args)` for a static method of a class
    /// or enum of the file, with its return type, `None` for `void`; on an enum, also
    /// `from`, `tryFrom` and `cases` (P2R-114, P2R-153).
    pub(super) fn static_call(
        &mut self,
        call: &StaticMethodCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, Option<RustType>)> {
        let problem = match call.class {
            Expression::Parent(_) => Some("parent:: is not supported (P2R-135)"),
            Expression::Self_(_) | Expression::Static(_) if self.current_class.is_none() => {
                Some("self:: and static:: work only inside a class or enum (P2R-153)")
            }
            _ => None,
        };
        if let Some(problem) = problem {
            self.record_refused_arguments(&call.argument_list);
            self.refuse(node, problem);
            return None;
        }
        if self.names_date_time(call.class) {
            return self
                .date_create(call, node)
                .map(|(call, ty)| (call, Some(ty)));
        }
        let index = self
            .class_key(call.class)
            .and_then(|key| self.classes.iter().position(|c| c.key == key));
        let found = match (index, &call.method) {
            (Some(index), ClassLikeMemberSelector::Identifier(name)) => {
                Some((index, self.method_index(index, name.value)))
            }
            _ => None,
        };
        let (index, at) = match found {
            Some((index, Some(at))) => (index, at),
            // `from`, `tryFrom`, `cases`, and the problem of any other name (P2R-114).
            Some((index, None)) if self.classes[index].is_enum => {
                return self
                    .enum_call(call, node)
                    .map(|(call, ty)| (call, Some(ty)));
            }
            Some((_, None)) => {
                self.record_refused_arguments(&call.argument_list);
                self.refuse(node, "this class has no method of that name (P2R-153)");
                return None;
            }
            None => {
                // A class given by a variable (`$b::m()`) reads it.
                self.record_refused(call.class);
                self.record_refused_arguments(&call.argument_list);
                self.refuse(
                    node,
                    "a static call needs a static method of a class or enum of this file (P2R-153)",
                );
                return None;
            }
        };
        if !self.classes[index].methods[at].is_static {
            self.record_refused_arguments(&call.argument_list);
            self.refuse(
                node,
                "a method that is not static cannot be called with :: (P2R-153)",
            );
            return None;
        }
        let signature = self.classes[index].methods[at].signature.clone();
        let arguments = self.arguments(&call.argument_list, &signature.parameters, node)?;
        // rustc counts a call of the method in its own body (P2R-50, P2R-152).
        if self.current_method == Some((index, at)) {
            self.self_calls.push(call.span());
        }
        let path = format!("{}::{}", self.classes[index].rust_name, signature.rust_name);
        let call = RExpr::Call(path, arguments);
        Some((
            self.raised(node, call, signature.throws)?,
            signature.returns,
        ))
    }

    pub(super) fn record_refused_arguments(&mut self, list: &ArgumentList<'_>) {
        for argument in list.arguments.iter() {
            let value = match argument {
                Argument::Positional(positional) => positional.value,
                Argument::Named(named) => named.value,
            };
            self.record_refused(value);
        }
    }
}

/// The problem of a property read or method call on a union value (P2R-232).
const UNION_MEMBER: &str =
    "a property read or method call on a value of a union type is not supported (P2R-232)";

/// The problem of a nullable or union type that holds a changing type (P2R-141, P2R-230).
pub(super) const NULLABLE_CHANGING: &str = "a nullable, union or map type that holds objects whose class changes them is not supported (P2R-141, P2R-240)";

/// The problem of a declared property without a type or a default (P2R-140).
const NEEDS_DEFAULT: &str = "a declared property needs a type and a default value (P2R-140)";

/// The properties laravel-data 4.18 declares on `Data` (P2R-350).
const DATA_PROPERTIES: [&[u8]; 2] = [b"_additional", b"_dataContext"];

/// The methods PHP calls by itself that laravel-data 4.18's `Data` lacks (P2R-350).
const IMPLICIT_METHODS: [&str; 8] = [
    "__construct",
    "__destruct",
    "__get",
    "__set",
    "__isset",
    "__unset",
    "__call",
    "__toString",
];

/// The problem of a property that `Data` already declares (P2R-350).
const DATA_PROPERTY: &str =
    "Spatie\\LaravelData\\Data already declares a property of this name (P2R-350)";

/// True when every argument of `attribute` is a string or int literal or `Name::class`,
/// named ones last and each name once. PHP compiles attribute arguments as constant
/// expressions and rejects other forms, such as a call, when it compiles the file; Mago
/// does not report them all (P2R-351).
fn plain_arguments(attribute: &Attribute<'_>) -> bool {
    let Some(list) = &attribute.argument_list else {
        return true;
    };
    let mut names: Vec<&[u8]> = Vec::new();
    list.arguments.iter().all(|argument| {
        let value = match argument {
            PartialArgument::Positional(positional) => {
                if !names.is_empty() || positional.ellipsis.is_some() {
                    return false;
                }
                positional.value
            }
            PartialArgument::Named(named) => {
                if names.contains(&named.name.value) {
                    return false;
                }
                names.push(named.name.value);
                named.value
            }
            _ => return false,
        };
        match value {
            Expression::Literal(Literal::String(_) | Literal::Integer(_)) => true,
            Expression::Access(Access::ClassConstant(access)) => {
                matches!(access.class, Expression::Identifier(_))
                    && matches!(&access.constant, ClassLikeConstantSelector::Identifier(constant)
                        if constant.value.eq_ignore_ascii_case(b"class"))
            }
            _ => false,
        }
    })
}

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

/// The methods of `ArrayAccess`, lowercase (P2R-220).
const ARRAY_ACCESS_METHODS: [&[u8]; 4] =
    [b"offsetexists", b"offsetget", b"offsetset", b"offsetunset"];

/// The lowercase name of `method`: PHP method names ignore letter case.
fn lowercase(method: &Method<'_>) -> Vec<u8> {
    method.name.value.to_ascii_lowercase()
}

/// The Rust type of a constant: a string constant is a `&'static str` (P2R-156).
fn constant_rust_type(ty: &RustType) -> String {
    match ty {
        RustType::String => "&str".to_owned(),
        other => other.owned(),
    }
}

/// True when `hint`, or a hint inside `?` or a union, is one `found` accepts.
pub(super) fn names(hint: &Hint<'_>, found: fn(&Hint<'_>) -> bool) -> bool {
    match hint {
        Hint::Nullable(nullable) => names(nullable.hint, found),
        Hint::Parenthesized(inner) => names(inner.hint, found),
        Hint::Union(union) => names(union.left, found) || names(union.right, found),
        other => found(other),
    }
}

fn is_parent(hint: &Hint<'_>) -> bool {
    matches!(hint, Hint::Parent(_))
}

fn is_static(hint: &Hint<'_>) -> bool {
    matches!(hint, Hint::Static(_))
}

/// `self` or `static`, which name the class whose body holds them (P2R-154).
pub(super) fn is_own_class(hint: &Hint<'_>) -> bool {
    matches!(hint, Hint::Self_(_) | Hint::Static(_))
}

/// True for a parameter whose only modifier is `static` (P2R-154).
fn static_modifier(parameter: &FunctionLikeParameter<'_>) -> bool {
    !parameter.modifiers.is_empty()
        && parameter
            .modifiers
            .iter()
            .all(|m| matches!(m, Modifier::Static(_)))
}

/// True for `final` and `private` together (P2R-130, P2R-156).
fn final_private(modifiers: &Sequence<'_, Modifier<'_>>) -> bool {
    modifiers.iter().any(|m| matches!(m, Modifier::Final(_)))
        && modifiers.iter().any(|m| matches!(m, Modifier::Private(_)))
}

/// True for constant modifiers PHP rejects when it compiles the file, and Mago does not
/// report: anything but one visibility and one `final` (P2R-156).
fn bad_constant_modifiers(modifiers: &Sequence<'_, Modifier<'_>>) -> bool {
    let count = |test: fn(&Modifier<'_>) -> bool| modifiers.iter().filter(|m| test(m)).count();
    let visibility = count(|m| {
        matches!(
            m,
            Modifier::Public(_) | Modifier::Protected(_) | Modifier::Private(_)
        )
    });
    let finals = count(|m| matches!(m, Modifier::Final(_)));
    visibility > 1 || finals > 1 || visibility + finals < modifiers.len()
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
