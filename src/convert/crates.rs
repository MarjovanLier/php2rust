//! A directory as one crate: the module of each file, the duplicate stops, which items of
//! other modules a module uses, and the order rustfmt gives its `pub mod` and `use` lines
//! (P2R-172 to P2R-174).

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::Converter;
use crate::names;

/// Rust's primitive types. A module of one of these names would take over paths the output
/// writes, such as `usize::from` and `f64::INFINITY` (P2R-172).
const PRIMITIVES: &[&str] = &[
    "bool", "char", "str", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16",
    "u32", "u64", "u128", "usize",
];

impl Converter<'_> {
    /// The module of each input file, with `module_paths` filled: one module for each
    /// namespace that declares an enum, class or function, and for each namespace between
    /// the root and it. The root is the longest namespace that the namespace of every
    /// declaring file starts with; a file that declares nothing goes to the root, where it
    /// adds nothing. One input file has only the root (P2R-172).
    pub(super) fn modules(&mut self, units: &[Program<'_>]) -> Vec<usize> {
        if !self.library {
            return vec![0; units.len()];
        }
        let namespaces: Vec<(Option<&Identifier<'_>>, Vec<&[u8]>)> = units
            .iter()
            .map(|unit| {
                let name = unit
                    .statements
                    .iter()
                    .find_map(|statement| match statement {
                        Statement::Namespace(namespace) => namespace.name.as_ref(),
                        _ => None,
                    });
                let segments = name.map_or_else(Vec::new, |name| {
                    name.value()
                        .split(|byte| *byte == b'\\')
                        .filter(|segment| !segment.is_empty())
                        .collect()
                });
                (name, segments)
            })
            .collect();
        // PHP compares namespace names ignoring letter case, and module names do not.
        for (at, (name, segments)) in namespaces.iter().enumerate() {
            let clash = namespaces[..at].iter().any(|(_, earlier)| {
                segments
                    .iter()
                    .zip(earlier)
                    .take_while(|(a, b)| a.eq_ignore_ascii_case(b))
                    .any(|(a, b)| a != b)
            });
            if let (true, Some(name)) = (clash, name) {
                self.refuse(
                    *name,
                    "an earlier file writes this namespace with other letter case, and PHP reads both as one namespace (P2R-172)",
                );
            }
        }
        let declares: Vec<bool> = units
            .iter()
            .map(|unit| {
                unit.statements.iter().any(|statement| {
                    matches!(
                        statement,
                        Statement::Enum(_) | Statement::Class(_) | Statement::Function(_)
                    )
                })
            })
            .collect();
        let mut root: Option<Vec<&[u8]>> = None;
        for ((_, segments), _) in namespaces.iter().zip(&declares).filter(|(_, d)| **d) {
            root = Some(match root {
                None => segments.clone(),
                Some(root) => root
                    .iter()
                    .zip(segments)
                    .take_while(|(a, b)| a.eq_ignore_ascii_case(b))
                    .map(|(a, _)| *a)
                    .collect(),
            });
        }
        let below = root.map_or(0, |root| root.len());
        // The lowercase namespace each module path was made from (P2R-172).
        let mut sources: BTreeMap<Vec<String>, Vec<u8>> = BTreeMap::new();
        let mut unit_paths: Vec<Vec<String>> = Vec::new();
        for ((name, segments), &declares) in namespaces.iter().zip(&declares) {
            let mut path: Vec<String> = Vec::new();
            if declares {
                let mut named = true;
                for (depth, segment) in segments[below..].iter().enumerate() {
                    let text = String::from_utf8_lossy(segment);
                    match names::rust_name(&text) {
                        None => {
                            named = false;
                            self.refuse_namespace(
                                *name,
                                format!(
                                    "the namespace segment {text} has no Rust module name (P2R-172)"
                                ),
                            )
                        }
                        Some(module) if PRIMITIVES.contains(&module.as_str()) => {
                            named = false;
                            self.refuse_namespace(
                                *name,
                                format!("a module named {module} is not supported, because it would take over the paths of Rust's type {module} (P2R-172)"),
                            )
                        }
                        Some(module)
                            if module == "r#mod"
                                || (depth == 0 && matches!(module.as_str(), "lib" | "main")) =>
                        {
                            named = false;
                            self.refuse_namespace(
                                *name,
                                format!("a module named {} is not supported, because rustc rejects it or warns about it (P2R-172)", module.trim_start_matches("r#")),
                            )
                        }
                        Some(module) => path.push(module),
                    }
                }
                // A refused namespace has no module; its items go to the root, where the
                // conversion stops anyway.
                if !named {
                    unit_paths.push(Vec::new());
                    continue;
                }
                // Each module between the root and this one counts too: `App\FooBar\Deep`
                // and `App\Foo_Bar` would share the module `foo_bar`.
                for depth in 1..=path.len() {
                    let source = segments[below..below + depth]
                        .join(&b'\\')
                        .to_ascii_lowercase();
                    match sources.get(&path[..depth]) {
                        Some(earlier) if *earlier != source => {
                            self.refuse_namespace(
                                *name,
                                "another namespace gives the same module path (P2R-172)".to_owned(),
                            );
                            break;
                        }
                        _ => {
                            sources.insert(path[..depth].to_vec(), source);
                        }
                    }
                }
            }
            unit_paths.push(path);
        }
        let mut paths: BTreeSet<Vec<String>> = BTreeSet::from([Vec::new()]);
        for path in &unit_paths {
            for depth in 1..=path.len() {
                paths.insert(path[..depth].to_vec());
            }
        }
        self.module_paths = paths.into_iter().collect();
        unit_paths
            .iter()
            .map(|path| {
                self.module_paths
                    .iter()
                    .position(|module| module == path)
                    .unwrap_or(0)
            })
            .collect()
    }

    /// Refuses a namespace, or the file start when it has no name.
    fn refuse_namespace(&mut self, name: Option<&Identifier<'_>>, message: String) {
        if let Some(name) = name {
            self.refuse(name, message);
        }
    }

    /// Stops two enums or classes, or two functions, of the crate whose fully qualified
    /// names are equal ignoring letter case, and two declarations whose Rust names are
    /// equal, at each declaration with the span of the whole declaration. Stops a function
    /// named like a child module of its own module, which its `use` line would import too
    /// (P2R-173).
    pub(super) fn duplicates(&mut self, units: &[Program<'_>], module_of: &[usize]) {
        let mut by_name: BTreeMap<String, Vec<Span>> = BTreeMap::new();
        let mut by_key: BTreeMap<(bool, Vec<u8>), Vec<Span>> = BTreeMap::new();
        let mut functions: Vec<(String, usize, Span)> = Vec::new();
        for (unit, &module) in units.iter().zip(module_of) {
            for statement in unit.statements.iter() {
                let (name, function) = match statement {
                    Statement::Function(function) => (&function.name, true),
                    Statement::Enum(declaration) => (&declaration.name, false),
                    Statement::Class(declaration) => (&declaration.name, false),
                    _ => continue,
                };
                let text = String::from_utf8_lossy(name.value);
                let rust = if function {
                    names::rust_name(&text)
                } else {
                    names::type_name(&text)
                };
                let span = statement.span();
                if let Some(rust) = rust {
                    if function {
                        functions.push((rust.clone(), module, name.span()));
                    }
                    by_name.entry(rust).or_default().push(span);
                }
                by_key
                    .entry((function, self.name_key(name, name.value)))
                    .or_default()
                    .push(span);
            }
        }
        let spans: BTreeSet<Span> = by_name
            .values()
            .chain(by_key.values())
            .filter(|spans| spans.len() > 1)
            .flatten()
            .copied()
            .collect();
        for span in spans {
            self.refuse(
                &span,
                "another enum, class or function of the crate has the same Rust name or fully qualified name (P2R-173)",
            );
        }
        for (rust, module, span) in functions {
            let path = &self.module_paths[module];
            let child = self.module_paths.iter().any(|other| {
                other.len() == path.len() + 1
                    && other.starts_with(path)
                    && other[path.len()] == rust
            });
            if child {
                self.refuse(
                    &span,
                    "a function may not take the name of a child module of its own module, which its use line would import too (P2R-173)",
                );
            }
        }
    }

    /// The module files of the crate, each with its path in the crate: the P2R-52 line,
    /// the `pub mod` lines, the `use` lines, the items and the helpers, one blank line
    /// before each group and each item (P2R-172, P2R-174).
    pub(super) fn crate_files(
        &self,
        items: Vec<Vec<Vec<String>>>,
        owners: &BTreeMap<String, (usize, ItemKind)>,
    ) -> Vec<(String, String)> {
        let mut files = Vec::new();
        for (module, (path, items)) in self.module_paths.iter().zip(items).enumerate() {
            let mut groups: Vec<Vec<String>> = Vec::new();
            let mut children: Vec<&String> = self
                .module_paths
                .iter()
                .filter(|other| other.len() == path.len() + 1 && other.starts_with(path))
                .map(|other| &other[path.len()])
                .collect();
            children.sort_by(|a, b| module_order(a, b));
            if !children.is_empty() {
                groups.push(children.iter().map(|c| format!("pub mod {c};")).collect());
            }
            let others: BTreeMap<String, ItemKind> = owners
                .iter()
                .filter(|(_, (owner, _))| *owner != module)
                .map(|(name, (_, kind))| (name.clone(), *kind))
                .collect();
            let text = items
                .iter()
                .flatten()
                .cloned()
                .collect::<Vec<_>>()
                .join("\n");
            let mut uses: Vec<Vec<String>> = used_items(&text, &others)
                .into_iter()
                .map(|name| {
                    let mut path = vec!["crate".to_owned()];
                    path.extend(self.module_paths[owners[name].0].iter().cloned());
                    path.push(name.to_owned());
                    path
                })
                .collect();
            uses.sort_by(|a, b| use_order(a, b));
            if !uses.is_empty() {
                groups.push(
                    uses.iter()
                        .map(|path| format!("use {};", path.join("::")))
                        .collect(),
                );
            }
            groups.extend(items);
            groups.extend(
                self.helpers
                    .iter()
                    .filter(|(owner, _)| *owner == module)
                    .map(|(_, helper)| helper.lines()),
            );
            let file = if path.is_empty() {
                "lib.rs".to_owned()
            } else {
                let parts: Vec<&str> = path.iter().map(|s| s.trim_start_matches("r#")).collect();
                format!("{}.rs", parts.join("/"))
            };
            files.push((file, super::assemble(groups, Vec::new())));
        }
        files
    }
}

/// An enum, class or function of the crate, as a module names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ItemKind {
    /// An enum or class: a use is any mention as a type or the first segment of a path.
    Type,
    /// A function: a use is a call.
    Function,
}

/// The items of other modules that `text`, the Rust lines of a module's items, names:
/// as a type, as the first segment of a path, or as a called function. `items` maps each
/// item's Rust name to its kind. A name after `.` or `::`, before `!` or a single `:`, or
/// inside an enum declaration is no use, so enum cases, fields, methods, macros and
/// bindings named like an item never count (P2R-174).
pub(super) fn used_items<'a>(
    text: &str,
    items: &'a BTreeMap<String, ItemKind>,
) -> BTreeSet<&'a str> {
    let tokens = tokens(text);
    let mut used = BTreeSet::new();
    // The brace depth of an enum body; its identifiers are the cases.
    let mut enum_body: Option<usize> = None;
    let mut depth = 0;
    // The bracket depth inside an attribute (`#[derive(Debug)]`), whose names are macros.
    let mut attribute: Option<usize> = None;
    let mut brackets = 0;
    for (at, token) in tokens.iter().enumerate() {
        match *token {
            "[" => {
                brackets += 1;
                if attribute.is_none() && at >= 1 && tokens[at - 1] == "#" {
                    attribute = Some(brackets);
                }
                continue;
            }
            "]" => {
                if attribute == Some(brackets) {
                    attribute = None;
                }
                brackets -= 1;
                continue;
            }
            _ if attribute.is_some() => continue,
            "{" => {
                depth += 1;
                if enum_body.is_none() && at >= 2 && tokens[at - 2] == "enum" {
                    enum_body = Some(depth);
                }
                continue;
            }
            "}" => {
                if enum_body == Some(depth) {
                    enum_body = None;
                }
                depth = depth.saturating_sub(1);
                continue;
            }
            _ => {}
        }
        if enum_body.is_some() {
            continue;
        }
        let Some((name, kind)) = items.get_key_value(*token) else {
            continue;
        };
        let before = at.checked_sub(1).map(|i| tokens[i]);
        let after = tokens.get(at + 1).copied();
        // `mut` is not here: in `&mut Counter` the name is a type.
        let bound = matches!(before, Some("." | "::" | "fn" | "let" | "const"))
            || matches!(after, Some("!" | ":"));
        let shape = match kind {
            ItemKind::Type => true,
            ItemKind::Function => after == Some("("),
        };
        if !bound && shape {
            used.insert(name.as_str());
        }
    }
    used
}

/// The tokens of Rust text that matter for [`used_items`]: identifiers (with `r#`), `::`
/// and every other punctuation character on its own. String, byte and char literals and
/// numbers are left out.
fn tokens(text: &str) -> Vec<&str> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        let byte = bytes[at];
        let start = at;
        match byte {
            b'"' => {
                at += 1;
                while at < bytes.len() && bytes[at] != b'"' {
                    at += if bytes[at] == b'\\' { 2 } else { 1 };
                }
                at += 1;
            }
            // A char or byte literal (`'x'`, `'\t'`, `'\x0B'`), or a lifetime
            // (`&'static str`), which is a quote and a name without a closing quote.
            b'\'' => {
                let width = text[at + 1..].chars().next().map_or(1, char::len_utf8);
                let literal =
                    bytes.get(at + 1) == Some(&b'\\') || bytes.get(at + 1 + width) == Some(&b'\'');
                at += 1;
                if literal {
                    while at < bytes.len() && bytes[at] != b'\'' {
                        at += if bytes[at] == b'\\' { 2 } else { 1 };
                    }
                    at += 1;
                } else {
                    while at < bytes.len()
                        && (bytes[at].is_ascii_alphanumeric() || bytes[at] == b'_')
                    {
                        at += 1;
                    }
                }
            }
            b'r' if bytes.get(at + 1) == Some(&b'#') => {
                at += 2;
                while at < bytes.len() && (bytes[at].is_ascii_alphanumeric() || bytes[at] == b'_') {
                    at += 1;
                }
                tokens.push(&text[start..at]);
            }
            b'b' if matches!(bytes.get(at + 1), Some(b'\'' | b'"')) => at += 1,
            b'0'..=b'9' => {
                while at < bytes.len() && (bytes[at].is_ascii_alphanumeric() || bytes[at] == b'_') {
                    at += 1;
                }
            }
            b if b.is_ascii_alphabetic() || b == b'_' => {
                while at < bytes.len() && (bytes[at].is_ascii_alphanumeric() || bytes[at] == b'_') {
                    at += 1;
                }
                tokens.push(&text[start..at]);
            }
            b':' if bytes.get(at + 1) == Some(&b':') => {
                at += 2;
                tokens.push("::");
            }
            b if b.is_ascii_whitespace() => at += 1,
            _ => {
                // One character, which may take several bytes outside a literal only in
                // theory: the output's non-ASCII text is always inside string literals.
                let width = text[at..].chars().next().map_or(1, char::len_utf8);
                at += width;
                tokens.push(&text[start..at]);
            }
        }
    }
    tokens
}

/// The order of two `pub mod` names, as rustfmt sorts them: byte by byte without `r#`.
pub(super) fn module_order(a: &str, b: &str) -> Ordering {
    bare(a).cmp(bare(b))
}

/// The order of two `use` paths, segment by segment, as rustfmt sorts them: a path that
/// ends first comes first, and segments compare by [`version_order`] (P2R-174).
pub(super) fn use_order(a: &[String], b: &[String]) -> Ordering {
    for (x, y) in a.iter().zip(b) {
        let order = version_order(bare(x), bare(y));
        if order != Ordering::Equal {
            return order;
        }
    }
    a.len().cmp(&b.len())
}

fn bare(name: &str) -> &str {
    name.strip_prefix("r#").unwrap_or(name)
}

/// A run of one kind of character in a name, for [`version_order`].
#[derive(PartialEq, Eq)]
enum Chunk<'a> {
    Underscore,
    /// A run of digits: its value without leading zeros, and the number of zeros.
    Number(&'a str, usize),
    Text(&'a str),
}

fn chunks(name: &str) -> Vec<Chunk<'_>> {
    let bytes = name.as_bytes();
    let mut chunks = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        let start = at;
        if bytes[at] == b'_' {
            at += 1;
            chunks.push(Chunk::Underscore);
        } else if bytes[at].is_ascii_digit() {
            while at < bytes.len() && bytes[at].is_ascii_digit() {
                at += 1;
            }
            let digits = &name[start..at];
            let value = digits.trim_start_matches('0');
            chunks.push(Chunk::Number(value, digits.len() - value.len()));
        } else {
            while at < bytes.len() && bytes[at] != b'_' && !bytes[at].is_ascii_digit() {
                at += 1;
            }
            chunks.push(Chunk::Text(&name[start..at]));
        }
    }
    chunks
}

/// rustfmt's version sorting of two names: `_` before every other character, text by its
/// bytes (upper case before lower case), and a run of digits by its number; where two
/// names differ only in leading zeros, the one with more zeros comes first.
fn version_order(a: &str, b: &str) -> Ordering {
    let (a, b) = (chunks(a), chunks(b));
    let mut zeros = Ordering::Equal;
    for (x, y) in a.iter().zip(&b) {
        let order = match (x, y) {
            (Chunk::Underscore, Chunk::Underscore) => Ordering::Equal,
            (Chunk::Underscore, _) => Ordering::Less,
            (_, Chunk::Underscore) => Ordering::Greater,
            (Chunk::Number(x, x_zeros), Chunk::Number(y, y_zeros)) => {
                let order = x.len().cmp(&y.len()).then_with(|| x.cmp(y));
                if order == Ordering::Equal && zeros == Ordering::Equal {
                    zeros = y_zeros.cmp(x_zeros);
                }
                order
            }
            // Digits sort before letters, as their bytes do.
            (Chunk::Number(..), Chunk::Text(_)) => Ordering::Less,
            (Chunk::Text(_), Chunk::Number(..)) => Ordering::Greater,
            (Chunk::Text(x), Chunk::Text(y)) => x.cmp(y),
        };
        if order != Ordering::Equal {
            return order;
        }
    }
    a.len().cmp(&b.len()).then(zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The order rustfmt 1.10 gave these `use` lines (crate:: left out).
    #[test]
    fn use_lines_sort_as_rustfmt_sorts_them() {
        let expected = [
            "Root",
            "a::Dto02",
            "a::Dto2",
            "a::Dto2X",
            "a::Dto2a",
            "a::Dto002b",
            "a::Dto2x",
            "a::Dto10",
            "a::K_b",
            "a::KB",
            "a::Kb",
            "a::Z",
            "a::a_1",
            "a::a_B",
            "a::a_b",
            "a::a1",
            "a::aZ",
            "a::ab",
            "a::b::Y",
            "a_b::X",
            "a1::X",
            "ab::X",
            "en::X",
            "r#enum::X",
            "enum_x::X",
            "root_fn",
            "r#type::Y",
        ];
        let path = |text: &str| text.split("::").map(str::to_owned).collect::<Vec<_>>();
        let mut paths: Vec<Vec<String>> = expected.iter().rev().map(|p| path(p)).collect();
        paths.sort_by(|a, b| use_order(a, b));
        let sorted: Vec<String> = paths.iter().map(|p| p.join("::")).collect();
        assert_eq!(sorted, expected);
    }

    #[test]
    fn module_lines_sort_by_bytes_without_raw_prefix() {
        let mut names = vec!["r#enum", "ab", "a_1", "a2", "a10", "a1", "enum_x", "en"];
        names.sort_by(|a, b| module_order(a, b));
        assert_eq!(
            names,
            ["a1", "a10", "a2", "a_1", "ab", "en", "r#enum", "enum_x"]
        );
    }

    fn items() -> BTreeMap<String, ItemKind> {
        BTreeMap::from([
            ("Male".to_owned(), ItemKind::Type),
            ("Gender".to_owned(), ItemKind::Type),
            ("total".to_owned(), ItemKind::Function),
            ("i64".to_owned(), ItemKind::Function),
            ("r#type".to_owned(), ItemKind::Function),
        ])
    }

    #[test]
    fn types_paths_and_calls_count() {
        let text = "pub fn f(g: Gender) -> Option<Male> {\n    let m: Male;\n    \
                    return Gender::from(total(1) + r#type(2));\n}\n";
        let items = items();
        let used: Vec<&str> = used_items(text, &items).into_iter().collect();
        assert_eq!(used, ["Gender", "Male", "r#type", "total"]);
    }

    #[test]
    fn cases_fields_methods_and_bindings_do_not_count() {
        let text = "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum Sex {\n    Male,\n    \
                    Female = 2,\n}\n\npub struct C {\n    pub total: i64,\n}\n\nimpl C {\n    \
                    pub fn new(total: i64) -> C {\n        C { total }\n    }\n\n    \
                    pub fn total(&self) -> i64 {\n        print!(\"total(Male)\");\n        \
                    return self.total + c.total() + Sex::Male as i64;\n    }\n}\n";
        let items = items();
        assert!(used_items(text, &items).is_empty());
    }

    #[test]
    fn literals_hide_their_text() {
        let text = "let a = 'M'; let b = b'\\''; let c = \"Male \\\" total(\"; let d = 1_i64;";
        let items = items();
        assert!(used_items(text, &items).is_empty());
    }
}
