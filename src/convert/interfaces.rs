//! The input's own interfaces as Rust traits, and the methods of the classes and enums
//! that implement them as `impl` blocks of those traits (P2R-370 to P2R-372).

use std::collections::HashSet;

use mago_syntax::cst::*;

use super::classes::MethodInfo;
use super::control::emit;
use super::{Converter, Mode, Signature};
use crate::types::RustType;

/// The traits of Rust's prelude. A trait of the output with one of these names hides the
/// prelude's, whose methods the output calls (`.clone()`, `.to_string()`, `.map()`), so
/// such an interface stops (P2R-370).
const PRELUDE: &[&str] = &[
    "AsMut",
    "AsRef",
    "Clone",
    "Copy",
    "Default",
    "DoubleEndedIterator",
    "Drop",
    "Eq",
    "ExactSizeIterator",
    "Extend",
    "Fn",
    "FnMut",
    "FnOnce",
    "From",
    "FromIterator",
    "Future",
    "Into",
    "IntoFuture",
    "IntoIterator",
    "Iterator",
    "Ord",
    "PartialEq",
    "PartialOrd",
    "Send",
    "Sized",
    "Sync",
    "ToString",
    "TryFrom",
    "TryInto",
    "Unpin",
];

/// The methods that prelude traits give the output's types: `to_owned` and `clone_into` of
/// `ToOwned`, `into` and `try_into` of the blanket `Into` and `TryInto`, and `eq` and `ne`
/// of a derived `PartialEq` (P2R-370).
const PRELUDE_METHODS: &[&str] = &["clone_into", "eq", "into", "ne", "to_owned", "try_into"];

/// An interface the input declares (P2R-370).
pub(super) struct InterfaceInfo {
    /// The lowercase PHP name, by which `implements` finds it.
    pub(super) key: Vec<u8>,
    pub(super) rust_name: String,
    pub(super) methods: Vec<MethodInfo>,
}

impl Converter<'_> {
    /// Registers the name of every top-level interface, before the enums and classes that
    /// implement one read their `implements` (P2R-370).
    pub(super) fn register_interface_names(&mut self, program: &Program<'_>) {
        for statement in program.statements.iter() {
            let Statement::Interface(declaration) = statement else {
                continue;
            };
            let Some(rust_name) = self.declared_type_name(&declaration.name, "interface") else {
                continue;
            };
            if PRELUDE.contains(&rust_name.as_str()) {
                self.refuse(
                    &declaration.name,
                    format!("{rust_name} is the name of a trait of Rust's prelude, which the output uses (P2R-370)"),
                );
                continue;
            }
            if self.interfaces.iter().any(|i| i.rust_name == rust_name) {
                self.refuse(
                    &declaration.name,
                    format!("{rust_name} is already the Rust name of another interface (P2R-370)"),
                );
                continue;
            }
            let key = self.name_key(&declaration.name, declaration.name.value);
            self.type_names.traits.insert(rust_name.clone());
            self.interfaces.push(InterfaceInfo {
                key,
                rust_name,
                methods: Vec::new(),
            });
        }
    }

    /// The input's own interfaces that the keys of an `implements` list name: `None` when
    /// it names none of them, and a problem when it names another interface beside them
    /// (P2R-371).
    pub(super) fn input_interfaces(
        &self,
        keys: &[Vec<u8>],
    ) -> Option<Result<Vec<usize>, &'static str>> {
        let found: Vec<Option<usize>> = keys
            .iter()
            .map(|key| self.interfaces.iter().position(|i| i.key == *key))
            .collect();
        if found.iter().all(Option::is_none) {
            return None;
        }
        let Some(interfaces) = found.into_iter().collect::<Option<Vec<usize>>>() else {
            return Some(Err(
                "a class or enum that implements the input's own interfaces may implement no other interface (P2R-371)",
            ));
        };
        // PHP rejects an interface named twice, and Mago does not report it; rustc would
        // see two impl blocks of one trait.
        if interfaces
            .iter()
            .enumerate()
            .any(|(at, i)| interfaces[..at].contains(i))
        {
            return Some(Err(
                "an implements list may name an interface once (P2R-371)",
            ));
        }
        Some(Ok(interfaces))
    }

    /// Checks every registered interface and reads its method signatures, once the enums
    /// and classes are known (P2R-370).
    pub(super) fn interface_members(&mut self, program: &Program<'_>, taken: &HashSet<String>) {
        for statement in program.statements.iter() {
            let Statement::Interface(declaration) = statement else {
                continue;
            };
            let key = self.name_key(&declaration.name, declaration.name.value);
            let Some(index) = self.interfaces.iter().position(|i| i.key == key) else {
                continue;
            };
            let rust_name = self.interfaces[index].rust_name.clone();
            if self.type_names.values().any(
                |ty| matches!(ty, RustType::Enum(name) | RustType::Class(name) if *name == rust_name),
            ) {
                self.refuse(
                    &declaration.name,
                    format!("{rust_name} is already the Rust name of an enum or class (P2R-370)"),
                );
            }
            self.interface_shape(declaration);
            let methods = self.method_infos(&key, &declaration.members, None, false, taken);
            self.interfaces[index].methods = methods;
        }
    }

    /// Refuses what an interface of this slice may not hold (P2R-370).
    fn interface_shape(&mut self, declaration: &Interface<'_>) {
        if let Some(attributes) = declaration.attribute_lists.first() {
            self.refuse(attributes, "attributes are not supported (P2R-07)");
        }
        if let Some(extends) = &declaration.extends {
            self.refuse(
                extends,
                "an interface that extends another is not supported yet (P2R-370)",
            );
        }
        for member in declaration.members.iter() {
            let ClassLikeMember::Method(method) = member else {
                self.refuse(
                    member,
                    "an interface may hold only methods; its constants are not supported yet (P2R-370)",
                );
                continue;
            };
            if let Some(attributes) = method.attribute_lists.first() {
                self.refuse(attributes, "attributes are not supported (P2R-07)");
            }
            let rust_name = crate::names::rust_name(&String::from_utf8_lossy(method.name.value));
            // PHP rejects a body and every modifier but one `public`, which Mago's analyser
            // does not report.
            let problem = if method
                .modifiers
                .iter()
                .any(|m| matches!(m, Modifier::Static(_)))
            {
                Some("a static method of an interface is not supported yet (P2R-370)")
            } else if method
                .modifiers
                .iter()
                .any(|m| !matches!(m, Modifier::Public(_)))
                || method.modifiers.len() > 1
            {
                Some("an interface method may carry only one public (P2R-370)")
            } else if matches!(method.body, MethodBody::Concrete(_)) {
                Some("an interface method has no body (P2R-370)")
            } else if method.name.value.starts_with(b"__") {
                Some("a method whose name starts with __ is not supported (P2R-370)")
            } else if method.ampersand.is_some() {
                Some("a method that returns by reference is not supported (P2R-370)")
            } else if rust_name
                .as_deref()
                .is_some_and(|name| PRELUDE_METHODS.contains(&name))
            {
                // A trait method of such a name meets the method a prelude trait gives
                // every type, where an inherent method would win.
                Some(
                    "this method takes the name of a method of a trait of Rust's prelude (P2R-370)",
                )
            } else {
                None
            };
            if let Some(message) = problem {
                self.refuse(&method.name, message);
                continue;
            }
            match &method.return_type_hint {
                None => self.refuse(
                    &method.name,
                    "an interface method needs a return type (P2R-370)",
                ),
                Some(hint)
                    if matches!(hint.hint, Hint::Self_(_) | Hint::Static(_) | Hint::Never(_)) =>
                {
                    self.refuse(
                        hint,
                        "an interface method may not return self, static or never (P2R-370)",
                    )
                }
                Some(_) => {}
            }
            for parameter in method.parameter_list.parameters.iter() {
                if let Some(attributes) = parameter.attribute_lists.first() {
                    self.refuse(attributes, "attributes are not supported (P2R-07)");
                } else if parameter.default_value.is_some()
                    || parameter.ampersand.is_some()
                    || parameter.ellipsis.is_some()
                {
                    self.refuse(
                        parameter,
                        "an interface method parameter with a default, by reference or variadic is not supported (P2R-370)",
                    );
                }
            }
        }
    }

    /// The `pub trait` of the interface at `index`, one line per method in source order.
    /// The receiver is `self` when only enums implement it, whose methods take their value
    /// (P2R-155), and `&self` otherwise (P2R-370).
    pub(super) fn interface_item(
        &mut self,
        declaration: &Interface<'_>,
        index: usize,
    ) -> Vec<String> {
        let implementers: Vec<bool> = self
            .classes
            .iter()
            .filter(|class| class.interfaces.contains(&index))
            .map(|class| class.is_enum)
            .collect();
        if implementers.contains(&true) && implementers.contains(&false) {
            self.refuse(
                &declaration.name,
                "an interface that both an enum and a class implement is not supported, because their methods take self in two ways (P2R-370)",
            );
        }
        let receiver = if !implementers.is_empty() && implementers.iter().all(|&e| e) {
            "self"
        } else {
            "&self"
        };
        let interface = &self.interfaces[index];
        let open = format!("pub trait {} {{", interface.rust_name);
        if interface.methods.is_empty() {
            // rustfmt writes an empty trait on one line.
            return vec![format!("{open}}}")];
        }
        let mut lines = vec![open];
        for method in &interface.methods {
            let parameters: Vec<String> = std::iter::once(receiver.to_owned())
                .chain(
                    method
                        .signature
                        .parameters
                        .iter()
                        .map(|p| format!("{}: {}", p.rust_name, p.mode.parameter(&p.ty))),
                )
                .collect();
            lines.push(emit(
                1,
                format!(
                    "fn {}({}){};",
                    method.signature.rust_name,
                    parameters.join(", "),
                    method.signature.returns_text()
                ),
            ));
        }
        lines.push("}".to_owned());
        lines
    }

    /// The interface whose trait holds the method at `at` of the class or enum at `class`,
    /// for a method one of its interfaces declares (P2R-371).
    pub(super) fn trait_of(&self, class: usize, at: usize) -> Option<usize> {
        let key = &self.classes[class].methods[at].key;
        self.classes[class]
            .interfaces
            .iter()
            .copied()
            .find(|&i| self.interfaces[i].methods.iter().any(|m| m.key == *key))
    }

    /// Refuses a method that cannot stand in the trait of `interface`: one that two of its
    /// interfaces declare, one that is static, changes its object or throws, and one whose
    /// Rust signature is not the trait's (P2R-371).
    pub(super) fn check_trait_method(
        &mut self,
        method: &Method<'_>,
        class: usize,
        at: usize,
        interface: usize,
    ) {
        let info = &self.classes[class].methods[at];
        let twice = self.classes[class].interfaces.iter().any(|&i| {
            i != interface && self.interfaces[i].methods.iter().any(|m| m.key == info.key)
        });
        let declared = self.interfaces[interface]
            .methods
            .iter()
            .find(|m| m.key == info.key);
        let problem = if twice {
            Some("a method that two of its interfaces declare is not supported (P2R-371)")
        } else if declared.is_some_and(|d| d.signature.rust_name != info.signature.rust_name) {
            // PHP matches the names ignoring letter case, and the Rust names may differ.
            Some("this method's Rust name is not that of the interface method (P2R-371)")
        } else if info.is_static {
            Some("a static method cannot implement an interface method (P2R-371)")
        } else if info.self_mode != Mode::Shared {
            Some(
                "a method that changes its object cannot implement an interface method, whose trait takes self as it is (P2R-143, P2R-371)",
            )
        } else if info.signature.throws {
            Some(
                "a method that throws cannot implement an interface method, whose trait returns no Result (P2R-211, P2R-371)",
            )
        } else if declared.is_some_and(|d| !same_signature(&d.signature, &info.signature)) {
            Some(
                "this method's Rust signature is not that of the interface method, which the trait needs (P2R-371)",
            )
        } else {
            None
        };
        if let Some(message) = problem {
            self.refuse(&method.name, message);
        }
    }

    /// Refuses a class or enum that declares no method of a name a method of one of its
    /// interfaces has. Mago skips `from`, `tryFrom` and `cases` on an enum, which PHP
    /// rejects, so the check does not rest on Mago (P2R-371).
    pub(super) fn check_missing(
        &mut self,
        name: &LocalIdentifier<'_>,
        members: &Sequence<'_, ClassLikeMember<'_>>,
        class: usize,
    ) {
        let declared: Vec<Vec<u8>> = members
            .iter()
            .filter_map(|member| match member {
                ClassLikeMember::Method(method) => Some(method.name.value.to_ascii_lowercase()),
                _ => None,
            })
            .collect();
        let missing = self.classes[class]
            .interfaces
            .iter()
            .flat_map(|&i| self.interfaces[i].methods.iter())
            .find(|method| !declared.contains(&method.key))
            .map(|method| String::from_utf8_lossy(&method.key).into_owned());
        if let Some(method) = missing {
            self.refuse(
                name,
                format!("this class or enum does not declare {method}(), a method of an interface it implements (P2R-371)"),
            );
        }
    }

    /// The `impl Trait for C` blocks of the class or enum at `class`, one per interface in
    /// the order of its `implements` list, each with the method lines in `traits` at the
    /// same position (P2R-371).
    pub(super) fn trait_impls(&self, class: usize, traits: Vec<Vec<Vec<String>>>) -> Vec<String> {
        let name = &self.classes[class].rust_name;
        let mut lines = Vec::new();
        for (&interface, methods) in self.classes[class].interfaces.iter().zip(traits) {
            lines.push(String::new());
            let open = format!(
                "impl {} for {name} {{",
                self.interfaces[interface].rust_name
            );
            if methods.is_empty() {
                lines.push(format!("{open}}}"));
                continue;
            }
            lines.push(open);
            for (at, method) in methods.into_iter().enumerate() {
                if at > 0 {
                    lines.push(String::new());
                }
                lines.extend(method);
            }
            lines.push("}".to_owned());
        }
        lines
    }

    /// The position in the `implements` list of the class or enum at `class` of the
    /// interface whose trait holds its method at `at` (P2R-371).
    pub(super) fn trait_slot(&self, class: usize, at: usize) -> Option<usize> {
        let interface = self.trait_of(class, at)?;
        self.classes[class]
            .interfaces
            .iter()
            .position(|&i| i == interface)
    }
}

/// True when two signatures have the same parameter and return types and forms.
fn same_signature(trait_method: &Signature, method: &Signature) -> bool {
    trait_method.returns == method.returns
        && trait_method.parameters.len() == method.parameters.len()
        && trait_method
            .parameters
            .iter()
            .zip(&method.parameters)
            .all(|(a, b)| a.ty == b.ty && a.mode == b.mode)
}
