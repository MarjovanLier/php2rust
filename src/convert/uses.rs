//! `use` imports, which give no Rust: the forms PHP rejects when it compiles the file, or
//! warns about, and Mago does not report (P2R-150).

use mago_syntax::cst::*;

use super::Converter;
use super::enums::RESERVED;

/// What an import names; each kind has its own aliases.
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Class,
    Function,
    Constant,
}

impl Converter<'_> {
    /// Refuses two imports of one kind with one alias, an import whose alias is the name of
    /// a class-like or function the file declares under another name, a class import whose
    /// alias PHP reserves, and a class import of a one-part name outside a namespace
    /// (P2R-150).
    pub(super) fn check_uses(&mut self, statements: &[Statement<'_>]) {
        let namespaced = statements
            .iter()
            .any(|s| matches!(s, Statement::Namespace(_)));
        // The declarations of the file: kind, lowercase short name, key.
        let mut declared: Vec<(Kind, Vec<u8>, Vec<u8>)> = Vec::new();
        for statement in statements {
            let (kind, name) = match statement {
                Statement::Class(d) => (Kind::Class, &d.name),
                Statement::Enum(d) => (Kind::Class, &d.name),
                Statement::Function(d) => (Kind::Function, &d.name),
                _ => continue,
            };
            let key = self.name_key(name, name.value);
            declared.push((kind, name.value.to_ascii_lowercase(), key));
        }
        let mut seen: Vec<(Kind, Vec<u8>)> = Vec::new();
        for statement in statements {
            let Statement::Use(r#use) = statement else {
                continue;
            };
            for (kind, prefix, item) in items(r#use) {
                let name = item
                    .name
                    .value()
                    .strip_prefix(b"\\")
                    .unwrap_or(item.name.value());
                let full = match prefix {
                    Some(prefix) => [prefix, b"\\", name].concat(),
                    None => name.to_vec(),
                };
                let alias = match &item.alias {
                    Some(alias) => alias.identifier.value,
                    None => full.rsplit(|b| *b == b'\\').next().unwrap_or_default(),
                };
                // PHP constant names keep their letter case; the others ignore it.
                let alias = if kind == Kind::Constant {
                    alias.to_vec()
                } else {
                    alias.to_ascii_lowercase()
                };
                let full = full.to_ascii_lowercase();
                let problem = if seen.contains(&(kind, alias.clone())) {
                    Some(
                        "two imports of one kind may not share one name, which PHP rejects (P2R-150)",
                    )
                } else if declared
                    .iter()
                    .any(|(k, short, key)| *k == kind && *short == alias && *key != full)
                {
                    Some(
                        "an import may not take the name of a class, enum or function this file declares, which PHP rejects (P2R-150)",
                    )
                } else if kind == Kind::Class
                    && RESERVED
                        .split_whitespace()
                        .any(|word| word.as_bytes() == alias)
                {
                    Some("PHP reserves the name of this import (P2R-150)")
                } else if kind == Kind::Class && !namespaced && !full.contains(&b'\\') {
                    Some(
                        "a use of a one-part name outside a namespace has no effect, and PHP warns about it (P2R-150)",
                    )
                } else {
                    None
                };
                if let Some(problem) = problem {
                    self.refuse(&item.name, problem);
                }
                seen.push((kind, alias));
            }
        }
    }
}

/// Each item of a `use` statement with its kind and the namespace a grouped list puts
/// before it.
fn items<'u, 'arena>(r#use: &'u Use<'arena>) -> Vec<(Kind, Option<&'u [u8]>, &'u UseItem<'arena>)> {
    let kind = |r#type: Option<&UseType<'_>>| match r#type {
        Some(UseType::Function(_)) => Kind::Function,
        Some(UseType::Const(_)) => Kind::Constant,
        None => Kind::Class,
    };
    let prefix = |namespace: &'u Identifier<'arena>| {
        let value = namespace.value();
        Some(value.strip_prefix(b"\\").unwrap_or(value))
    };
    match &r#use.items {
        UseItems::Sequence(list) => list
            .items
            .iter()
            .map(|item| (Kind::Class, None, item))
            .collect(),
        UseItems::TypedSequence(list) => {
            let kind = kind(Some(&list.r#type));
            list.items.iter().map(|item| (kind, None, item)).collect()
        }
        UseItems::TypedList(list) => {
            let kind = kind(Some(&list.r#type));
            let prefix = prefix(&list.namespace);
            list.items.iter().map(|item| (kind, prefix, item)).collect()
        }
        UseItems::MixedList(list) => {
            let prefix = prefix(&list.namespace);
            list.items
                .iter()
                .map(|typed| (kind(typed.r#type.as_ref()), prefix, &typed.item))
                .collect()
        }
    }
}
