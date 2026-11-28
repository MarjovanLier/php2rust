//! The form of a parameter: how it is written, read once in the same way for every kind
//! of callable (function, method, constructor, property hook, closure inside a hook, the
//! callback of `when`, the arrow function callback), with the rules PHP applies to every
//! parameter when it compiles the file and Mago does not report. Each caller turns these
//! facts into its own rule and stop message, in its own order; the parameter's type and
//! default value still convert with the caller.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::{
    Expression, FunctionLikeParameter, FunctionLikeParameterList, Hint, Literal, Modifier,
};

use super::classes::names;
use super::members::{self, WrittenModifier};

/// The variables PHP declares in every scope, which a parameter or a catch variable may not
/// take as its name (P2R-31, P2R-214).
pub(super) const SUPERGLOBALS: [&[u8]; 9] = [
    b"$GLOBALS",
    b"$_SERVER",
    b"$_GET",
    b"$_POST",
    b"$_FILES",
    b"$_COOKIE",
    b"$_SESSION",
    b"$_REQUEST",
    b"$_ENV",
];

/// A parameter's name, as PHP's rules for every parameter see it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Name {
    /// `$this`, which PHP rejects as a parameter.
    This,
    /// The name of one of the `SUPERGLOBALS`, which PHP rejects as a parameter.
    Superglobal,
    Plain,
}

/// A parameter's default value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DefaultValue {
    None,
    Null,
    Other,
}

/// A modifier that PHP rejects on a parameter when it compiles the file, and that Mago's
/// parser accepts and its analyser does not report (P2R-130).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ModifierFault {
    /// A modifier written twice, or a second visibility or set visibility.
    Doubled,
    StaticOrAbstract,
    /// A set visibility wider than the visibility the property is read with.
    WiderSet,
}

/// How a parameter is written.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct ParameterForm {
    /// The whole parameter.
    pub(super) span: Span,
    /// The parameter's variable, `$x`.
    pub(super) variable: Span,
    /// The variable's name without `$`, as written.
    pub(super) php_name: Vec<u8>,
    pub(super) name: Name,
    /// `static` as the type, which Mago's parser reads as a modifier when it stands alone,
    /// or inside `?`, `(...)` or a union; PHP allows `static` only as a return type.
    pub(super) static_type: bool,
    /// In the order written.
    pub(super) modifiers: Vec<WrittenModifier>,
    /// A modifier or hooks make a parameter a promoted property, as Mago reads it.
    pub(super) promoted: bool,
    pub(super) hooks: bool,
    pub(super) ampersand: Option<Span>,
    pub(super) ellipsis: Option<Span>,
    pub(super) default: DefaultValue,
    /// The span of each `#[...]` group, in source order.
    pub(super) attribute_groups: Vec<Span>,
    /// A type is written.
    pub(super) typed: bool,
}

impl HasSpan for ParameterForm {
    fn span(&self) -> Span {
        self.span
    }
}

impl ParameterForm {
    pub(super) fn of(parameter: &FunctionLikeParameter<'_>) -> Self {
        let name = parameter.variable.name;
        let only_static = !parameter.modifiers.is_empty()
            && parameter
                .modifiers
                .iter()
                .all(|m| matches!(m, Modifier::Static(_)));
        ParameterForm {
            span: parameter.span(),
            variable: parameter.variable.span(),
            php_name: name[1..].to_vec(),
            name: if name == b"$this" {
                Name::This
            } else if SUPERGLOBALS.contains(&name) {
                Name::Superglobal
            } else {
                Name::Plain
            },
            static_type: only_static
                || parameter
                    .hint
                    .as_ref()
                    .is_some_and(|hint| names(hint, |h| matches!(h, Hint::Static(_)))),
            modifiers: members::written(&parameter.modifiers),
            promoted: parameter.is_promoted_property(),
            hooks: parameter.hooks.is_some(),
            ampersand: parameter.ampersand.as_ref().map(HasSpan::span),
            ellipsis: parameter.ellipsis.as_ref().map(HasSpan::span),
            default: match &parameter.default_value {
                None => DefaultValue::None,
                Some(default) => match default.value {
                    Expression::Literal(Literal::Null(_)) => DefaultValue::Null,
                    _ => DefaultValue::Other,
                },
            },
            attribute_groups: parameter.attribute_lists.iter().map(|l| l.span()).collect(),
            typed: parameter.hint.is_some(),
        }
    }

    /// True when every modifier is `static`, as Mago's parser reads a bare `static` type.
    pub(super) fn static_modifier(&self) -> bool {
        !self.modifiers.is_empty()
            && self
                .modifiers
                .iter()
                .all(|m| m.kind == members::ModifierKind::Static)
    }

    /// The first modifier PHP rejects on this parameter, with its fault: one written twice
    /// first, then `static` or `abstract`, then a set visibility wider than the read one.
    pub(super) fn modifier_fault(&self) -> Option<(ModifierFault, Span)> {
        let kinds: Vec<members::ModifierKind> = self.modifiers.iter().map(|m| m.kind).collect();
        let at = |at: usize| self.modifiers[at].span;
        if let Some(found) = members::doubled_at(kinds.iter().copied()) {
            return Some((ModifierFault::Doubled, at(found)));
        }
        if let Some(found) = kinds.iter().position(|kind| {
            matches!(
                kind,
                members::ModifierKind::Static | members::ModifierKind::Abstract
            )
        }) {
            return Some((ModifierFault::StaticOrAbstract, at(found)));
        }
        members::wider_set_at(&kinds).map(|found| (ModifierFault::WiderSet, at(found)))
    }
}

/// How a parameter list is written.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct ParameterList {
    /// In the order written.
    pub(super) parameters: Vec<ParameterForm>,
    /// Per parameter, true when an earlier parameter has its name, which PHP rejects.
    repeats: Vec<bool>,
    /// The first parameter with a default before one without a default that is not
    /// variadic, which PHP deprecates when it compiles the file.
    pub(super) early_default: Option<usize>,
}

impl ParameterList {
    pub(super) fn of(list: &FunctionLikeParameterList<'_>) -> Self {
        let written: Vec<&FunctionLikeParameter<'_>> = list.parameters.iter().collect();
        let repeats = written
            .iter()
            .enumerate()
            .map(|(at, parameter)| {
                written[..at]
                    .iter()
                    .any(|earlier| earlier.variable.name == parameter.variable.name)
            })
            .collect();
        let early_default = written.iter().enumerate().position(|(at, parameter)| {
            parameter.default_value.is_some()
                && written[at + 1..]
                    .iter()
                    .any(|later| later.default_value.is_none() && later.ellipsis.is_none())
        });
        ParameterList {
            parameters: written.into_iter().map(ParameterForm::of).collect(),
            repeats,
            early_default,
        }
    }

    /// True when a parameter before the one at `at` has its name.
    pub(super) fn repeats(&self, at: usize) -> bool {
        self.repeats[at]
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_database::file::File;
    use mago_syntax::cst::{ClassLikeMember, Statement};
    use mago_syntax::parser::parse_file;

    use super::*;
    use crate::convert::members::ModifierKind;

    /// The source before the parameters, so a test can find a span's offset.
    const BEFORE: &str = "<?php\nclass C\n{\n    public function __construct(";

    /// The form of the parameter list `parameters` of a constructor; Mago's parser accepts
    /// every form below there, PHP's rules aside.
    fn list(parameters: &str) -> ParameterList {
        let source = format!("{BEFORE}{parameters}) {{}}\n}}\n");
        let file = File::ephemeral(Cow::Borrowed(b"test.php"), Cow::Owned(source.into_bytes()));
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        let class = program
            .statements
            .iter()
            .find_map(|statement| match statement {
                Statement::Class(class) => Some(class),
                _ => None,
            })
            .expect("a class");
        match class.members.iter().next().expect("a member") {
            ClassLikeMember::Method(method) => ParameterList::of(&method.parameter_list),
            _ => panic!("a method"),
        }
    }

    /// The form of the one parameter `parameter`.
    fn form(parameter: &str) -> ParameterForm {
        list(parameter).parameters.remove(0)
    }

    /// The offset of a span within the parameters' own source.
    fn offset(span: Span) -> usize {
        span.start.offset as usize - BEFORE.len()
    }

    #[test]
    fn a_name_is_this_a_superglobal_or_plain() {
        assert_eq!(form("int $this").name, Name::This);
        assert_eq!(form("int $_GET").name, Name::Superglobal);
        assert_eq!(form("int $GLOBALS").name, Name::Superglobal);
        assert_eq!(form("int $_get").name, Name::Plain);
        assert_eq!(form("int $x").name, Name::Plain);
        assert_eq!(offset(form("int $x").variable), 4);
    }

    #[test]
    fn static_is_a_type_alone_and_inside_a_union() {
        let alone = form("static $x");
        assert!(alone.static_type);
        assert!(alone.static_modifier());
        assert!(alone.promoted);
        let union = form("int|static $x");
        assert!(union.static_type);
        assert!(!union.static_modifier());
        assert!(form("?static $x").static_type);
        assert!(!form("self $x").static_type);
        // Beside another modifier, `static` is a modifier, not the type.
        let public = form("public static int $x");
        assert!(!public.static_type);
        assert!(!public.static_modifier());
    }

    #[test]
    fn a_promoted_parameter_reads_its_modifiers_and_hooks() {
        let found = form("public readonly int $x");
        assert!(found.promoted);
        assert!(!found.hooks);
        let kinds: Vec<ModifierKind> = found.modifiers.iter().map(|m| m.kind).collect();
        assert_eq!(kinds, [ModifierKind::Public, ModifierKind::Readonly]);
        assert_eq!(offset(found.modifiers[1].span), 7);
        let hooked = form("int $x { get => 1; }");
        assert!(hooked.promoted);
        assert!(hooked.hooks);
        assert!(!form("int $x").promoted);
    }

    #[test]
    fn a_reference_and_a_variadic_keep_their_spans() {
        let reference = form("int &$x");
        assert_eq!(reference.ampersand.map(offset), Some(4));
        assert_eq!(reference.ellipsis, None);
        let variadic = form("int ...$x");
        assert_eq!(variadic.ellipsis.map(offset), Some(4));
        assert_eq!(variadic.ampersand, None);
    }

    #[test]
    fn a_default_is_none_null_or_another_value() {
        assert_eq!(form("int $x").default, DefaultValue::None);
        assert_eq!(form("?int $x = null").default, DefaultValue::Null);
        assert_eq!(form("?int $x = NULL").default, DefaultValue::Null);
        assert_eq!(form("int $x = 1").default, DefaultValue::Other);
    }

    #[test]
    fn attributes_and_a_type_are_read() {
        let found = form("#[A] #[B, C] $x");
        assert_eq!(
            found
                .attribute_groups
                .iter()
                .map(|s| offset(*s))
                .collect::<Vec<_>>(),
            [0, 5]
        );
        assert!(!found.typed);
        assert!(form("int $x").attribute_groups.is_empty());
        assert!(form("int $x").typed);
    }

    #[test]
    fn the_name_is_read_without_its_dollar() {
        assert_eq!(form("int $message").php_name, b"message");
        assert_eq!(form("public int $fooBar").php_name, b"fooBar");
    }

    #[test]
    fn a_modifier_fault_names_its_kind_and_modifier() {
        let fault = |parameter: &str| {
            form(parameter)
                .modifier_fault()
                .map(|(fault, span)| (fault, offset(span)))
        };
        assert_eq!(
            fault("public public int $x"),
            Some((ModifierFault::Doubled, 7))
        );
        assert_eq!(
            fault("public private int $x"),
            Some((ModifierFault::Doubled, 7))
        );
        assert_eq!(
            fault("public static int $x"),
            Some((ModifierFault::StaticOrAbstract, 7))
        );
        assert_eq!(
            fault("abstract int $x"),
            Some((ModifierFault::StaticOrAbstract, 0))
        );
        assert_eq!(
            fault("private public(set) int $x"),
            Some((ModifierFault::WiderSet, 8))
        );
        assert_eq!(fault("public private(set) readonly int $x"), None);
        assert_eq!(fault("int $x"), None);
    }

    #[test]
    fn a_doubled_modifier_comes_before_static_and_static_before_a_wider_set() {
        let fault = |parameter: &str| {
            form(parameter)
                .modifier_fault()
                .map(|(fault, span)| (fault, offset(span)))
        };
        assert_eq!(
            fault("static public public int $x"),
            Some((ModifierFault::Doubled, 14))
        );
        assert_eq!(
            fault("private public(set) static int $x"),
            Some((ModifierFault::StaticOrAbstract, 20))
        );
    }

    #[test]
    fn a_repeated_name_is_found_at_each_repeat() {
        let found = list("int $a, int $b, int $a, int $a");
        let repeats: Vec<bool> = (0..4).map(|at| found.repeats(at)).collect();
        assert_eq!(repeats, [false, false, true, true]);
    }

    #[test]
    fn a_default_before_a_required_parameter_is_found() {
        assert_eq!(list("int $a = 1, int $b").early_default, Some(0));
        assert_eq!(list("int $a, int $b = 1, int $c").early_default, Some(1));
        assert_eq!(list("int $a = 1, int ...$b").early_default, None);
        assert_eq!(list("int $a, int $b = 1").early_default, None);
    }
}
