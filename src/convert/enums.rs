//! Enums: declarations, cases, and the methods PHP code uses on them (P2R-110 to
//! P2R-115).

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::Converter;
use crate::names;
use crate::printer::{self, RExpr};
use crate::types::RustType;

/// Type names the output itself uses, which an enum must not take (P2R-111).
const TAKEN: &[&str] = &["Option", "String", "Vec", "ToOwned"];

/// Names PHP reserves for types, which no enum may take; Mago does not report them
/// (P2R-111).
const RESERVED: &[&str] = &[
    "array", "bool", "callable", "false", "float", "int", "iterable", "mixed", "never", "null",
    "object", "parent", "self", "static", "string", "true", "void",
];

/// An enum of the file (P2R-110).
pub(super) struct EnumInfo {
    /// The lowercase PHP name, by which every use finds it (P2R-111).
    key: Vec<u8>,
    rust_name: String,
    /// The backing type: `i64`, `String`, or `None` for a pure enum.
    backing: Option<RustType>,
    cases: Vec<Case>,
    span: Span,
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
            self.enum_names
                .insert(info.key.clone(), info.rust_name.clone());
            self.enums.push(info);
        }
    }

    /// The enum an enum declaration registered, when its name converts.
    pub(super) fn enum_index(&self, declaration: &Enum<'_>) -> Option<usize> {
        let key = declaration.name.value.to_ascii_lowercase();
        self.enums.iter().position(|e| e.key == key)
    }

    /// Checks an enum declaration. A problem with a member is recorded and the enum still
    /// registers, so its uses find their type; a name problem leaves it out.
    fn enum_info(&mut self, declaration: &Enum<'_>) -> Option<EnumInfo> {
        let php = String::from_utf8_lossy(declaration.name.value);
        let lower = php.to_ascii_lowercase();
        if RESERVED.contains(&lower.as_str()) || self.built_in_classes.contains(lower.as_bytes()) {
            self.refuse(
                &declaration.name,
                format!("PHP reserves the name {php} or has a class of that name (P2R-111)"),
            );
            return None;
        }
        let Some(rust_name) = names::type_name(&php).filter(|name| !TAKEN.contains(&name.as_str()))
        else {
            self.refuse(
                &declaration.name,
                format!("the enum name {php} has no Rust form, or the output uses it (P2R-111)"),
            );
            return None;
        };
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
            let ClassLikeMember::EnumCase(case) = member else {
                self.refuse(
                    member,
                    "an enum with a method, constant, property or trait is not supported (P2R-110)",
                );
                continue;
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
            key: declaration.name.value.to_ascii_lowercase(),
            rust_name,
            backing,
            cases,
            span: declaration.span(),
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
        let Expression::Identifier(identifier) = class else {
            return None;
        };
        let name = match identifier {
            Identifier::Local(local) => local.value,
            Identifier::Qualified(qualified) => qualified.value,
            Identifier::FullyQualified(full) => full.value,
        };
        let key = name
            .strip_prefix(b"\\")
            .unwrap_or(name)
            .to_ascii_lowercase();
        self.enums.iter().position(|e| e.key == key)
    }

    /// `E::Case`, a case of an enum of the file (P2R-112).
    pub(super) fn enum_case(
        &mut self,
        access: &ClassConstantAccess<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let found = match (&access.constant, self.enum_of(access.class)) {
            (ClassLikeConstantSelector::Identifier(name), Some(index)) => {
                let info = &self.enums[index];
                info.cases
                    .iter()
                    .find(|case| case.php_name == name.value)
                    .map(|case| (info.rust_name.clone(), case.rust_name.clone()))
            }
            _ => None,
        };
        let Some((enum_name, case_name)) = found else {
            self.refuse(
                node,
                "only a case of an enum of this file can be read with :: (P2R-112)",
            );
            return None;
        };
        Some((
            RExpr::Path(format!("{enum_name}::{case_name}")),
            RustType::Enum(enum_name),
        ))
    }

    /// `$e->name` and `$e->value` (P2R-113).
    pub(super) fn enum_property(
        &mut self,
        access: &PropertyAccess<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let (object, ty) = self.expression(access.object)?;
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
            self.refuse(
                node,
                "only ->name of an enum and ->value of a backed enum are supported (P2R-113)",
            );
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
                "named and spread arguments are not supported (P2R-07)",
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

    /// The Rust lines of the enum at `index` and of the methods the file uses, written once
    /// every use is known (P2R-110, P2R-115).
    pub(super) fn enum_item(&mut self, index: usize) -> Vec<String> {
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
        let methods = methods(info);
        if !methods.is_empty() {
            lines.push(String::new());
            lines.push(format!("impl {} {{", info.rust_name));
            for (at, method) in methods.into_iter().enumerate() {
                if at > 0 {
                    lines.push(String::new());
                }
                lines.extend(method);
            }
            lines.push("}".to_owned());
        }
        // rustfmt splits a longer line.
        let span = info.span;
        if lines.iter().any(|line| line.len() > printer::MAX_WIDTH) {
            self.refuse(&span, "this enum has a line too long for one line (P2R-51)");
        }
        lines
    }
}

/// The methods the file uses, in the order `name`, `value`, `from`, `try_from`, `cases`,
/// each as its lines (P2R-115).
fn methods(info: &EnumInfo) -> Vec<Vec<String>> {
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
        let joined = paths.join(", ");
        let mut lines = vec![format!("    pub fn cases() -> Vec<{name}> {{")];
        if paths.is_empty() {
            lines.push("        Vec::new()".to_owned());
        } else if paths.len() == 1 || joined.len() <= printer::ARGS_WIDTH {
            lines.push(format!("        vec![{joined}]"));
        } else {
            // rustfmt's array width is 60: wider elements go one per line, but one element
            // stays on the line.
            lines.push("        vec![".to_owned());
            lines.extend(paths.iter().map(|path| format!("            {path},")));
            lines.push("        ]".to_owned());
        }
        lines.push("    }".to_owned());
        methods.push(lines);
    }
    methods
}

/// A method whose body is one `match` on `subject`, with one arm per pattern and value,
/// then `last`. An arm wider than 100 columns takes rustfmt's block form (P2R-115).
fn matching(
    signature: &str,
    subject: &str,
    arms: impl ExactSizeIterator<Item = (String, String)>,
    last: Option<&str>,
) -> Vec<String> {
    let mut lines = vec![format!("    pub fn {signature} {{")];
    if arms.len() == 0 && last.is_none() {
        // rustfmt writes a match without arms on one line.
        lines.push(format!("        match {subject} {{}}"));
        lines.push("    }".to_owned());
        return lines;
    }
    lines.push(format!("        match {subject} {{"));
    for (pattern, value) in arms {
        let line = format!("            {pattern} => {value},");
        // rustfmt counts columns, not bytes, so a non-ASCII line stays whole and the width
        // check of enum_item refuses it.
        if line.len() <= printer::MAX_WIDTH || !line.is_ascii() {
            lines.push(line);
        } else {
            lines.push(format!("            {pattern} => {{"));
            lines.push(format!("                {value}"));
            lines.push("            }".to_owned());
        }
    }
    lines.extend(last.map(|last| format!("            {last}")));
    lines.push("        }".to_owned());
    lines.push("    }".to_owned());
    lines
}
