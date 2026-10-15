//! Enums: declarations, cases, and the methods PHP code uses on them (P2R-110 to
//! P2R-115).

use mago_syntax::cst::*;

use super::Converter;
use super::classes::ClassInfo;
use crate::names;
use crate::printer::{self, RExpr};
use crate::types::RustType;

/// Type names the output itself uses, which an enum must not take (P2R-111). `None` and
/// `Some` too: a `use` line of a type of that name would sit unused beside the `Option`
/// values the output writes (P2R-174). `PhpException` is the exception struct, and
/// `Result`, `Ok` and `Err` carry it (P2R-210, P2R-211).
const TAKEN: &[&str] = &[
    "Option",
    "String",
    "Vec",
    "ToOwned",
    "None",
    "Some",
    "PhpException",
    "Result",
    "Ok",
    "Err",
    // The map type (P2R-240).
    "IndexMap",
    // The struct of `DateTime` (P2R-330).
    "PhpDateTime",
    // The enum of Spatie's `Optional` (P2R-360).
    "PhpOptional",
];

/// Names PHP reserves, its keywords and type names, which no enum may take; each one fails
/// `php -l` as an enum name on PHP 8.5, while Mago's parser accepts them (P2R-111). `enum`
/// itself is allowed.
pub(super) const RESERVED: &str = "abstract and array as bool break callable case catch class clone \
    const continue declare default die do echo else elseif empty enddeclare endfor endforeach \
    endif endswitch endwhile eval exit extends false final finally float fn for foreach \
    function global goto if implements include include_once instanceof insteadof int \
    interface isset iterable list match mixed namespace never new null object or parent print \
    private protected public readonly require require_once return self static string switch \
    throw trait true try unset use var void while xor yield";

/// An enum of the file (P2R-110).
pub(super) struct EnumInfo {
    /// The lowercase PHP name, by which every use finds it (P2R-111).
    key: Vec<u8>,
    pub(super) rust_name: String,
    /// The backing type: `i64`, `String`, or `None` for a pure enum.
    backing: Option<RustType>,
    cases: Vec<Case>,
    /// The methods the file uses, which the output holds (P2R-115).
    used: Used,
}

#[derive(Default)]
struct Used {
    name: bool,
    value: bool,
    from: bool,
    try_from: bool,
    cases: bool,
}

struct Case {
    php_name: Vec<u8>,
    rust_name: String,
    /// The backing value as a Rust literal, for a backed enum.
    value: Option<String>,
}

impl Converter<'_> {
    /// Registers every top-level enum before any signature, so a type can name one
    /// (P2R-110, P2R-111).
    pub(super) fn register_enums(&mut self, program: &Program<'_>) {
        for statement in program.statements.iter() {
            let Statement::Enum(declaration) = statement else {
                continue;
            };
            let Some(info) = self.enum_info(declaration) else {
                continue;
            };
            if self.enums.iter().any(|e| e.rust_name == info.rust_name) {
                self.refuse(
                    &declaration.name,
                    format!(
                        "{} is already the Rust name of another enum (P2R-111)",
                        info.rust_name
                    ),
                );
                continue;
            }
            self.type_names
                .insert(info.key.clone(), RustType::Enum(info.rust_name.clone()));
            if info.cases.is_empty() {
                self.type_names.without_cases(&info.rust_name);
            }
            // Its methods and constants live with those of the classes (P2R-155).
            self.classes.push(ClassInfo::new(
                info.key.clone(),
                info.rust_name.clone(),
                true,
            ));
            self.enums.push(info);
        }
    }

    /// The Rust name of an enum or class declaration (`kind`), refused when PHP reserves
    /// the name or has a class-like of that name, when it has no Rust form, or when the
    /// output uses it (P2R-111, P2R-131).
    pub(super) fn declared_type_name(
        &mut self,
        name: &LocalIdentifier<'_>,
        kind: &str,
    ) -> Option<String> {
        let php = String::from_utf8_lossy(name.value);
        let lower = php.to_ascii_lowercase();
        if RESERVED.split_whitespace().any(|word| word == lower)
            || self
                .built_in_classes
                .contains(&self.name_key(name, name.value))
        {
            self.refuse(
                name,
                format!("PHP reserves the name {php} or has a class of that name (P2R-111)"),
            );
            return None;
        }
        let rust_name = names::type_name(&php).filter(|name| !TAKEN.contains(&name.as_str()));
        if rust_name.is_none() {
            self.refuse(
                name,
                format!("the {kind} name {php} has no Rust form, or the output uses it (P2R-111)"),
            );
        }
        rust_name
    }

    /// The enum an enum declaration registered, when its name converts.
    pub(super) fn enum_index(&self, declaration: &Enum<'_>) -> Option<usize> {
        let key = self.name_key(&declaration.name, declaration.name.value);
        self.enums.iter().position(|e| e.key == key)
    }

    /// Checks an enum declaration. A problem with a member is recorded and the enum still
    /// registers, so its uses find their type; a name problem leaves it out.
    fn enum_info(&mut self, declaration: &Enum<'_>) -> Option<EnumInfo> {
        let rust_name = self.declared_type_name(&declaration.name, "enum")?;
        if let Some(attributes) = declaration.attribute_lists.first() {
            self.refuse(attributes, "attributes are not supported (P2R-07)");
        }
        if let Some(implements) = &declaration.implements {
            self.refuse(
                implements,
                "an enum that implements an interface is not supported (P2R-110)",
            );
        }
        let backing = match &declaration.backing_type_hint {
            None => None,
            Some(hint) => match hint.hint {
                Hint::Integer(_) => Some(RustType::I64),
                Hint::String(_) => Some(RustType::String),
                _ => {
                    self.refuse(
                        hint,
                        "an enum can be backed by int or string only (P2R-110)",
                    );
                    None
                }
            },
        };
        let mut cases: Vec<Case> = Vec::new();
        for member in declaration.members.iter() {
            let case = match member {
                ClassLikeMember::EnumCase(case) => case,
                // Section 16 converts methods and constants (P2R-155, P2R-156).
                ClassLikeMember::Method(method) => {
                    self.method_shape(method, false);
                    // PHP rejects a constructor in an enum, and Mago does not report it.
                    if method.name.value.eq_ignore_ascii_case(b"__construct") {
                        self.refuse(
                            &method.name,
                            "a method whose name starts with __ is not supported (P2R-155)",
                        );
                    }
                    continue;
                }
                // A trait of constants adds them where the constants are read (P2R-280).
                ClassLikeMember::Constant(_) | ClassLikeMember::TraitUse(_) => continue,
                _ => {
                    self.refuse(member, "an enum with a property is not supported (P2R-110)");
                    continue;
                }
            };
            let name = case.item.name();
            let php_case = String::from_utf8_lossy(name.value);
            let Some(case_name) = names::type_name(&php_case) else {
                self.refuse(
                    name,
                    format!("the case name {php_case} has no Rust form (P2R-111)"),
                );
                continue;
            };
            if cases.iter().any(|c| c.rust_name == case_name) {
                self.refuse(
                    name,
                    format!("{case_name} is already the Rust name of another case (P2R-111)"),
                );
                continue;
            }
            if let Some(attributes) = case.attribute_lists.first() {
                self.refuse(attributes, "attributes are not supported (P2R-07)");
            }
            // PHP rejects both mismatches when it compiles the file; Mago reports only the
            // second.
            let value = match (&case.item, declaration.backing_type_hint.is_some()) {
                // A refused backing type has no values to check.
                (EnumCaseItem::Backed(backed), true) => backing
                    .as_ref()
                    .and_then(|_| self.backing_value(backed.value)),
                (EnumCaseItem::Unit(_), true) => {
                    self.refuse(name, "a case of a backed enum needs a value (P2R-110)");
                    None
                }
                (EnumCaseItem::Backed(backed), false) => {
                    self.refuse(
                        backed.value,
                        "a case of a pure enum cannot have a value (P2R-110)",
                    );
                    None
                }
                (EnumCaseItem::Unit(_), false) => None,
            };
            cases.push(Case {
                php_name: name.value.to_vec(),
                rust_name: case_name,
                value,
            });
        }
        Some(EnumInfo {
            key: self.name_key(&declaration.name, declaration.name.value),
            rust_name,
            backing,
            cases,
            used: Used::default(),
        })
    }

    /// The Rust literal of a backing value: an int literal, with or without unary minus,
    /// or a string literal, by its value (P2R-49, P2R-110).
    fn backing_value(&mut self, value: &Expression<'_>) -> Option<String> {
        let int = |expression: &Expression<'_>| match expression {
            Expression::Literal(Literal::Integer(integer)) => {
                integer.value.and_then(|v| i64::try_from(v).ok())
            }
            _ => None,
        };
        let literal = match value {
            Expression::Literal(Literal::String(string)) => {
                return self
                    .string_literal(string)
                    .map(|text| printer::quote(&text));
            }
            Expression::UnaryPrefix(unary)
                if matches!(unary.operator, UnaryPrefixOperator::Negation(_)) =>
            {
                int(unary.operand).and_then(i64::checked_neg)
            }
            other => int(other),
        };
        if literal.is_none() {
            self.refuse(
                value,
                "a backing value must be an int or string literal (P2R-110)",
            );
        }
        literal.map(|value| value.to_string())
    }

    /// True when the enum with the key `key` has a case with the PHP name `php`.
    pub(super) fn has_case(&self, key: &[u8], php: &[u8]) -> bool {
        self.enums
            .iter()
            .filter(|e| e.key == key)
            .any(|e| e.cases.iter().any(|case| case.php_name == php))
    }

    /// The paths of the cases of the enum named `rust_name`, as `E::Case`, in source order.
    pub(super) fn case_paths(&self, rust_name: &str) -> Vec<String> {
        self.enums
            .iter()
            .filter(|e| e.rust_name == rust_name)
            .flat_map(|e| {
                e.cases
                    .iter()
                    .map(|case| format!("{rust_name}::{}", case.rust_name))
            })
            .collect()
    }

    /// The enum a class name in a use refers to, whatever its letter case (P2R-111).
    fn enum_of(&self, class: &Expression<'_>) -> Option<usize> {
        let key = self.class_key(class)?;
        self.enums.iter().position(|e| e.key == key)
    }

    /// The Rust names of the enum and case that `E::Case`, `self::Case` or `static::Case`
    /// names, for a case of an enum of the file (P2R-112, P2R-153).
    pub(super) fn enum_case_of(
        &self,
        access: &ClassConstantAccess<'_>,
    ) -> Option<(String, String)> {
        let ClassLikeConstantSelector::Identifier(name) = &access.constant else {
            return None;
        };
        let info = &self.enums[self.enum_of(access.class)?];
        info.cases
            .iter()
            .find(|case| case.php_name == name.value)
            .map(|case| (info.rust_name.clone(), case.rust_name.clone()))
    }

    /// `E::Case`, a case of an enum of the file, or `C::NAME`, a constant of a class or
    /// enum of the file (P2R-112, P2R-156).
    pub(super) fn class_constant(
        &mut self,
        access: &ClassConstantAccess<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        if let Some((enum_name, case_name)) = self.enum_case_of(access) {
            return Some((
                RExpr::Path(format!("{enum_name}::{case_name}")),
                RustType::Enum(enum_name),
            ));
        }
        if self.pairs_constant(access).is_some() {
            self.refuse(
                node,
                "a string map constant converts only as the second argument of strtr (P2R-281)",
            );
            return None;
        }
        if let Some(found) = self.constant_read(access) {
            return Some(found);
        }
        // The refused constant is the problem reported for its reads.
        if self.refused_constant(access) {
            self.record_refused(access.class);
            return None;
        }
        // A class given by a variable (`$o::NAME`) reads it.
        self.record_refused(access.class);
        let outside = self.current_class.is_none()
            && matches!(access.class, Expression::Self_(_) | Expression::Static(_));
        let message = match &access.constant {
            _ if outside => "self:: and static:: work only inside a class or enum (P2R-153)",
            _ if matches!(access.class, Expression::Parent(_)) => {
                "parent:: is not supported (P2R-135)"
            }
            ClassLikeConstantSelector::Identifier(name)
                if name.value.eq_ignore_ascii_case(b"class") =>
            {
                "C::class is not supported (P2R-156)"
            }
            _ => {
                "only a case of an enum or a constant of a class or enum of this file can be read with :: (P2R-156)"
            }
        };
        self.refuse(node, message);
        None
    }

    /// `$e->name` and `$e->value`, on the converted `object` of type `ty` (P2R-113).
    pub(super) fn enum_property(
        &mut self,
        access: &PropertyAccess<'_>,
        node: &Expression<'_>,
        (object, ty): (RExpr, RustType),
    ) -> Option<(RExpr, RustType)> {
        if matches!(ty, RustType::Option(_)) {
            self.refuse(
                access.object,
                "the enum of ->name or ->value cannot be a value that may be null (P2R-106)",
            );
            return None;
        }
        let property = match &access.property {
            ClassLikeMemberSelector::Identifier(property) => property.value,
            _ => b"",
        };
        self.enum_member(property, node, (object, ty))
    }

    /// `->name` or `->value`, by `property`, of the enum value `object` (P2R-113).
    pub(super) fn enum_member(
        &mut self,
        property: &[u8],
        node: &Expression<'_>,
        (object, ty): (RExpr, RustType),
    ) -> Option<(RExpr, RustType)> {
        let index = match &ty {
            RustType::Enum(name) => self.enums.iter().position(|e| e.rust_name == *name),
            _ => None,
        };
        let found = index.and_then(|index| {
            let info = &mut self.enums[index];
            match property {
                b"name" => {
                    info.used.name = true;
                    Some(("name", RustType::String))
                }
                b"value" => {
                    let backing = info.backing.clone()?;
                    info.used.value = true;
                    Some(("value", backing))
                }
                _ => None,
            }
        });
        let Some((method, result)) = found else {
            let message = if ty == RustType::DateTime {
                "of DateTime only DateTime::createFromFormat() and format() convert (P2R-330)"
            } else {
                "only ->name of an enum and ->value of a backed enum are supported (P2R-113)"
            };
            self.refuse(node, message);
            return None;
        };
        Some((RExpr::Method(Box::new(object), method), result))
    }

    /// `E::from($v)`, `E::tryFrom($v)` and `E::cases()` (P2R-114).
    pub(super) fn enum_call(
        &mut self,
        call: &StaticMethodCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let method = match &call.method {
            ClassLikeMemberSelector::Identifier(method) => method.value.to_ascii_lowercase(),
            _ => Vec::new(),
        };
        let index = self.enum_of(call.class);
        let arguments = call.argument_list.arguments.as_slice();
        let named_or_spread = arguments.iter().find(|argument| match argument {
            Argument::Positional(positional) => positional.ellipsis.is_some(),
            Argument::Named(_) => true,
        });
        if let (Some(_), Some(argument)) = (index, named_or_spread) {
            self.record_refused(node);
            self.refuse(
                argument,
                "named and spread arguments of E::from(), E::tryFrom() and E::cases() are not supported (P2R-169)",
            );
            return None;
        }
        let usable = match (index, method.as_slice(), arguments) {
            (Some(_), b"cases", []) => true,
            (Some(index), b"from" | b"tryfrom", [Argument::Positional(_)]) => {
                self.enums[index].backing.is_some()
            }
            _ => false,
        };
        let (Some(index), true) = (index, usable) else {
            self.record_refused(node);
            self.refuse(
                node,
                "only E::cases(), and E::from() and E::tryFrom() with one argument on a backed enum, are supported (P2R-114)",
            );
            return None;
        };
        let name = self.enums[index].rust_name.clone();
        let enum_type = RustType::Enum(name.clone());
        let [Argument::Positional(argument)] = arguments else {
            self.enums[index].used.cases = true;
            let call = RExpr::Call(format!("{name}::cases"), Vec::new());
            return Some((call, RustType::Vec(Box::new(enum_type))));
        };
        let (value, ty) = self.expression(argument.value)?;
        if matches!(ty, RustType::Option(_)) {
            self.refuse(
                argument.value,
                "a value that may be null cannot go to E::from() or E::tryFrom() (P2R-106)",
            );
            return None;
        }
        if Some(&ty) != self.enums[index].backing.as_ref() {
            self.refuse(
                argument.value,
                "the argument of E::from() and E::tryFrom() must have the backing type of the enum (P2R-114)",
            );
            return None;
        }
        if method == b"from" && self.enums[index].cases.is_empty() {
            // The call returns a value of a type with no values, so rustc rejects the code
            // after it as unreachable.
            self.refuse(
                node,
                "E::from() on an enum without cases always fails, and rustc rejects the code after it (P2R-50)",
            );
            return None;
        }
        let value = self.borrowed(value, &ty);
        if method == b"from" {
            self.enums[index].used.from = true;
            Some((RExpr::Call(format!("{name}::from"), vec![value]), enum_type))
        } else {
            self.enums[index].used.try_from = true;
            let call = RExpr::Call(format!("{name}::try_from"), vec![value]);
            Some((call, RustType::Option(Box::new(enum_type))))
        }
    }

    /// The lines of the methods the enum `declaration` declares, in source order, each
    /// converted as a method of its class-like entry (P2R-155).
    pub(super) fn enum_methods(&mut self, declaration: &Enum<'_>) -> Vec<Vec<String>> {
        let key = self.name_key(&declaration.name, declaration.name.value);
        let Some(class) = self.classes.iter().position(|c| c.key == key) else {
            return Vec::new();
        };
        let mut methods = Vec::new();
        for member in declaration.members.iter() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            let Some(at) = self.method_index(class, method.name.value) else {
                continue;
            };
            methods.extend(self.method(method, class, at));
        }
        methods
    }

    /// The Rust lines of the enum at `index`: the enum, then one `impl` block with its
    /// constants, the helpers the file uses and its `methods`, written once every use is
    /// known (P2R-110, P2R-115, P2R-157).
    pub(super) fn enum_item(&mut self, index: usize, methods: Vec<Vec<String>>) -> Vec<String> {
        let info = &self.enums[index];
        let open = format!("pub enum {} {{", info.rust_name);
        let mut lines = vec!["#[derive(Clone, Copy, Debug, PartialEq, Eq)]".to_owned()];
        if info.cases.is_empty() {
            // rustfmt writes an empty enum on one line.
            lines.push(format!("{open}}}"));
        } else {
            lines.push(open);
            lines.extend(
                info.cases
                    .iter()
                    .map(|case| format!("    {},", case.rust_name)),
            );
            lines.push("}".to_owned());
        }
        let helpers = helpers(info);
        let key = info.key.clone();
        let class = self.classes.iter().position(|c| c.key == key);
        let constants = class.map(|c| self.constant_lines(c)).unwrap_or_default();
        let functions: Vec<Vec<String>> = helpers.into_iter().chain(methods).collect();
        if constants.is_empty() && functions.is_empty() {
            return lines;
        }
        lines.push(String::new());
        lines.push(format!("impl {} {{", self.enums[index].rust_name));
        if functions.is_empty() {
            // No blank line before the closing brace.
            lines.extend(constants.iter().take(constants.len() - 1).cloned());
        } else {
            lines.extend(constants);
        }
        for (at, function) in functions.into_iter().enumerate() {
            if at > 0 {
                lines.push(String::new());
            }
            lines.extend(function);
        }
        lines.push("}".to_owned());
        lines
    }
}

/// The helpers the file uses, in the order `name`, `value`, `from`, `try_from`, `cases`,
/// each as its lines (P2R-115).
fn helpers(info: &EnumInfo) -> Vec<Vec<String>> {
    let name = &info.rust_name;
    let path = |case: &Case| format!("{name}::{}", case.rust_name);
    let backing = info
        .backing
        .as_ref()
        .map(|ty| if *ty == RustType::I64 { "i64" } else { "&str" });
    let value = |case: &Case| case.value.clone().unwrap_or_default();
    let mut methods = Vec::new();
    if info.used.name {
        let arms = info.cases.iter().map(|case| {
            (
                path(case),
                printer::quote(&String::from_utf8_lossy(&case.php_name)),
            )
        });
        methods.push(matching("name(self) -> &'static str", "self", arms, None));
    }
    if let (true, Some(backing)) = (info.used.value, backing) {
        let returns = if backing == "i64" {
            "i64"
        } else {
            "&'static str"
        };
        let arms = info.cases.iter().map(|case| (path(case), value(case)));
        methods.push(matching(
            &format!("value(self) -> {returns}"),
            "self",
            arms,
            None,
        ));
    }
    if let (true, Some(backing)) = (info.used.from, backing) {
        let arms = info.cases.iter().map(|case| (value(case), path(case)));
        methods.push(matching(
            &format!("from(value: {backing}) -> {name}"),
            "value",
            arms,
            Some("_ => panic!(\"{:?} is not a valid backing value\", value),"),
        ));
    }
    if let (true, Some(backing)) = (info.used.try_from, backing) {
        let arms = info
            .cases
            .iter()
            .map(|case| (value(case), format!("Some({})", path(case))));
        methods.push(matching(
            &format!("try_from(value: {backing}) -> Option<{name}>"),
            "value",
            arms,
            Some("_ => None,"),
        ));
    }
    if info.used.cases {
        let paths: Vec<String> = info.cases.iter().map(path).collect();
        let value = if paths.is_empty() {
            "Vec::new()".to_owned()
        } else {
            format!("vec![{}]", paths.join(", "))
        };
        methods.push(vec![
            format!("    pub fn cases() -> Vec<{name}> {{"),
            format!("        {value}"),
            "    }".to_owned(),
        ]);
    }
    methods
}

/// A method whose body is one `match` on `subject`, with one arm per pattern and value,
/// then `last` (P2R-115).
fn matching(
    signature: &str,
    subject: &str,
    arms: impl Iterator<Item = (String, String)>,
    last: Option<&str>,
) -> Vec<String> {
    let mut lines = vec![
        format!("    pub fn {signature} {{"),
        format!("        match {subject} {{"),
    ];
    lines.extend(arms.map(|(pattern, value)| format!("            {pattern} => {value},")));
    lines.extend(last.map(|last| format!("            {last}")));
    lines.push("        }".to_owned());
    lines.push("    }".to_owned());
    lines
}
