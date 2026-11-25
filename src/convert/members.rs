//! The form of a class-like member: how a method or property is written, read once in the
//! same way for every kind of member. Each caller turns these facts into its own rule and
//! stop message; the return type, the body and a property's value stay with the caller.

use mago_names::ResolvedNames;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::{AttributeList, Method, Modifier, Property, Sequence};

use super::models::resolved;

/// One modifier keyword.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ModifierKind {
    Static,
    Final,
    Abstract,
    Readonly,
    Public,
    PublicSet,
    Protected,
    ProtectedSet,
    Private,
    PrivateSet,
}

/// A modifier as written, with its span for a stop message.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct WrittenModifier {
    pub(super) kind: ModifierKind,
    pub(super) span: Span,
}

impl HasSpan for WrittenModifier {
    fn span(&self) -> Span {
        self.span
    }
}

/// One `#[...]` group, with its span for a stop message; an empty `#[]` has no attribute.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct AttributeGroup {
    pub(super) span: Span,
    pub(super) attributes: Vec<WrittenAttribute>,
}

impl HasSpan for AttributeGroup {
    fn span(&self) -> Span {
        self.span
    }
}

/// One attribute: its lower-case resolved name, and whether it is given an argument.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct WrittenAttribute {
    pub(super) name: Vec<u8>,
    pub(super) arguments: bool,
}

/// How a method or property is written.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct MemberForm {
    /// A method's name in lower case; a property's name as written, without `$`, because
    /// PHP property names are case sensitive (the first item's for `public $a, $b;`).
    pub(super) name: Vec<u8>,
    /// In the order written.
    pub(super) modifiers: Vec<WrittenModifier>,
    /// `function &name()`; false for a property.
    pub(super) by_reference: bool,
    /// The number of parameters; 0 for a property.
    pub(super) parameters: usize,
    /// In source order.
    pub(super) attribute_groups: Vec<AttributeGroup>,
    /// The span of a property's `var` keyword; none for a method.
    pub(super) var: Option<Span>,
}

impl MemberForm {
    pub(super) fn of_method(method: &Method<'_>, names: &ResolvedNames<'_>) -> Self {
        MemberForm {
            name: method.name.value.to_ascii_lowercase(),
            modifiers: written(&method.modifiers),
            by_reference: method.ampersand.is_some(),
            parameters: method.parameter_list.parameters.len(),
            attribute_groups: groups(&method.attribute_lists, names),
            var: None,
        }
    }

    pub(super) fn of_property(property: &Property<'_>, names: &ResolvedNames<'_>) -> Self {
        let (modifiers, attribute_lists, name, var) = match property {
            Property::Plain(plain) => (
                &plain.modifiers,
                &plain.attribute_lists,
                plain.items.iter().next().map(|item| item.variable().name),
                &plain.var,
            ),
            Property::Hooked(hooked) => (
                &hooked.modifiers,
                &hooked.attribute_lists,
                Some(hooked.item.variable().name),
                &hooked.var,
            ),
        };
        MemberForm {
            name: name
                .map(|name| name.strip_prefix(b"$").unwrap_or(name).to_vec())
                .unwrap_or_default(),
            modifiers: written(modifiers),
            by_reference: false,
            parameters: 0,
            attribute_groups: groups(attribute_lists, names),
            var: var.as_ref().map(HasSpan::span),
        }
    }

    /// True when `kind` is the member's one modifier.
    pub(super) fn only(&self, kind: ModifierKind) -> bool {
        matches!(self.modifiers.as_slice(), [modifier] if modifier.kind == kind)
    }

    /// The first modifier of one of `kinds`.
    pub(super) fn first(&self, kinds: &[ModifierKind]) -> Option<&WrittenModifier> {
        self.modifiers
            .iter()
            .find(|modifier| kinds.contains(&modifier.kind))
    }

    /// The first modifier that repeats an earlier one or is a second visibility, which PHP
    /// rejects when it compiles the file and Mago does not report.
    pub(super) fn doubled(&self) -> Option<&WrittenModifier> {
        doubled_at(self.modifiers.iter().map(|modifier| modifier.kind))
            .map(|at| &self.modifiers[at])
    }

    /// A set visibility wider than the read visibility, which PHP rejects when it compiles
    /// the file and Mago does not report.
    pub(super) fn wider_set(&self) -> Option<&WrittenModifier> {
        let kinds: Vec<ModifierKind> = self.modifiers.iter().map(|m| m.kind).collect();
        wider_set_at(&kinds).map(|at| &self.modifiers[at])
    }

    /// Both `final` and `private`.
    pub(super) fn final_private(&self) -> bool {
        final_private(self.modifiers.iter().map(|modifier| modifier.kind))
    }

    /// Every attribute, across the groups.
    pub(super) fn attributes(&self) -> impl Iterator<Item = &WrittenAttribute> {
        self.attribute_groups
            .iter()
            .flat_map(|group| group.attributes.iter())
    }

    /// True for an attribute other than `#[\Override]`.
    pub(super) fn attributed_beyond_override(&self) -> bool {
        self.attributes()
            .any(|attribute| attribute.name != b"override")
    }
}

/// The kind of a modifier keyword.
pub(super) fn kind(modifier: &Modifier<'_>) -> ModifierKind {
    match modifier {
        Modifier::Static(_) => ModifierKind::Static,
        Modifier::Final(_) => ModifierKind::Final,
        Modifier::Abstract(_) => ModifierKind::Abstract,
        Modifier::Readonly(_) => ModifierKind::Readonly,
        Modifier::Public(_) => ModifierKind::Public,
        Modifier::PublicSet(_) => ModifierKind::PublicSet,
        Modifier::Protected(_) => ModifierKind::Protected,
        Modifier::ProtectedSet(_) => ModifierKind::ProtectedSet,
        Modifier::Private(_) => ModifierKind::Private,
        Modifier::PrivateSet(_) => ModifierKind::PrivateSet,
    }
}

/// The position of the first modifier that repeats an earlier one, or that is a second
/// visibility (`public`, `protected`, `private`) or a second set visibility
/// (`private(set)`). Class declarations, constants and parameters share this rule.
pub(super) fn doubled_at(kinds: impl Iterator<Item = ModifierKind>) -> Option<usize> {
    let group = |kind: ModifierKind| match kind {
        ModifierKind::Public | ModifierKind::Protected | ModifierKind::Private => 0,
        ModifierKind::PublicSet | ModifierKind::ProtectedSet | ModifierKind::PrivateSet => 1,
        ModifierKind::Static => 2,
        ModifierKind::Final => 3,
        ModifierKind::Abstract => 4,
        ModifierKind::Readonly => 5,
    };
    let mut seen = [false; 6];
    kinds
        .enumerate()
        .find(|(_, kind)| std::mem::replace(&mut seen[group(*kind)], true))
        .map(|(at, _)| at)
}

/// The position of a set visibility wider than the read visibility, which is public when
/// none is written.
pub(super) fn wider_set_at(kinds: &[ModifierKind]) -> Option<usize> {
    let width = |kind: ModifierKind| match kind {
        ModifierKind::Public | ModifierKind::PublicSet => 3,
        ModifierKind::Protected | ModifierKind::ProtectedSet => 2,
        ModifierKind::Private | ModifierKind::PrivateSet => 1,
        _ => 0,
    };
    let read = kinds
        .iter()
        .find(|kind| {
            matches!(
                kind,
                ModifierKind::Public | ModifierKind::Protected | ModifierKind::Private
            )
        })
        .map_or(3, |kind| width(*kind));
    kinds.iter().position(|kind| {
        matches!(
            kind,
            ModifierKind::PublicSet | ModifierKind::ProtectedSet | ModifierKind::PrivateSet
        ) && width(*kind) > read
    })
}

/// Both `final` and `private`, which PHP warns about or rejects.
pub(super) fn final_private(kinds: impl Iterator<Item = ModifierKind> + Clone) -> bool {
    kinds.clone().any(|kind| kind == ModifierKind::Final)
        && kinds.into_iter().any(|kind| kind == ModifierKind::Private)
}

fn written(modifiers: &Sequence<'_, Modifier<'_>>) -> Vec<WrittenModifier> {
    modifiers
        .iter()
        .map(|modifier| WrittenModifier {
            kind: kind(modifier),
            span: modifier.span(),
        })
        .collect()
}

fn groups(
    lists: &Sequence<'_, AttributeList<'_>>,
    names: &ResolvedNames<'_>,
) -> Vec<AttributeGroup> {
    lists
        .iter()
        .map(|list| AttributeGroup {
            span: list.span(),
            attributes: list
                .attributes
                .iter()
                .map(|attribute| WrittenAttribute {
                    name: resolved(names, &attribute.name, attribute.name.value()),
                    arguments: attribute
                        .argument_list
                        .as_ref()
                        .is_some_and(|list| !list.arguments.is_empty()),
                })
                .collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_syntax::cst::{ClassLikeMember, Statement};
    use mago_syntax::parser::parse_file;

    use super::*;

    /// The source before the member, so a test can find a span's offset.
    const BEFORE: &str = "<?php\nuse Vendor\\Marker;\nclass C\n{\n    ";

    /// The form of the one member of a class declared with `member` in its body.
    fn form(member: &str) -> MemberForm {
        let source = format!("{BEFORE}{member}\n}}\n");
        let file = File::ephemeral(Cow::Borrowed(b"test.php"), Cow::Owned(source.into_bytes()));
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        let names = NameResolver::new(&arena).resolve(program);
        let class = program
            .statements
            .iter()
            .find_map(|statement| match statement {
                Statement::Class(class) => Some(class),
                _ => None,
            })
            .expect("a class");
        match class.members.iter().next().expect("a member") {
            ClassLikeMember::Method(method) => MemberForm::of_method(method, &names),
            ClassLikeMember::Property(property) => MemberForm::of_property(property, &names),
            _ => panic!("a method or property"),
        }
    }

    fn kinds(form: &MemberForm) -> Vec<ModifierKind> {
        form.modifiers
            .iter()
            .map(|modifier| modifier.kind)
            .collect()
    }

    fn names(form: &MemberForm) -> Vec<Vec<&[u8]>> {
        form.attribute_groups
            .iter()
            .map(|group| {
                group
                    .attributes
                    .iter()
                    .map(|attribute| attribute.name.as_slice())
                    .collect()
            })
            .collect()
    }

    /// The offset of a span within the member's own source.
    fn offset(span: Span) -> usize {
        span.start.offset as usize - BEFORE.len()
    }

    #[test]
    fn a_method_reads_its_name_reference_and_parameters() {
        let found = form("public static function &__Make(int $a, $b) {}");
        assert_eq!(found.name, b"__make");
        assert_eq!(kinds(&found), [ModifierKind::Public, ModifierKind::Static]);
        assert!(found.by_reference);
        assert_eq!(found.parameters, 2);
        assert!(!found.only(ModifierKind::Public));
    }

    #[test]
    fn attributes_resolve_by_group_and_an_empty_group_counts() {
        let found = form("#[\\Override, Marker(1)] #[] protected function casts(): array {}");
        assert_eq!(
            names(&found),
            [vec![&b"override"[..], b"vendor\\marker"], vec![]]
        );
        let arguments: Vec<bool> = found.attributes().map(|a| a.arguments).collect();
        assert_eq!(arguments, [false, true]);
        assert_eq!(offset(found.attribute_groups[1].span), 24);
        assert!(found.attributed_beyond_override());
        assert!(found.only(ModifierKind::Protected));
    }

    #[test]
    fn two_visibility_modifiers_are_not_one() {
        let found = form("public private function f() {}");
        assert_eq!(kinds(&found), [ModifierKind::Public, ModifierKind::Private]);
        assert!(!found.only(ModifierKind::Public));
        assert_eq!(
            found.doubled().map(|modifier| offset(modifier.span)),
            Some(7)
        );
    }

    #[test]
    fn a_repeated_modifier_is_found_where_it_repeats() {
        let found = form("final public static static function f() {}");
        assert_eq!(
            found.doubled().map(|modifier| offset(modifier.span)),
            Some(20)
        );
        assert_eq!(
            found.first(&[ModifierKind::Static]).map(|m| offset(m.span)),
            Some(13)
        );
        assert!(form("final private function f() {}").final_private());
        assert!(!form("final public function f() {}").final_private());
    }

    #[test]
    fn a_property_reads_var_and_a_wider_set_visibility() {
        assert_eq!(form("var $a;").var.map(offset), Some(0));
        assert_eq!(form("public $a;").var, None);
        let found = form("private public(set) string $a;");
        assert_eq!(
            found.wider_set().map(|modifier| offset(modifier.span)),
            Some(8)
        );
    }

    #[test]
    fn a_set_visibility_wider_than_the_read_one_is_found() {
        use ModifierKind::*;
        assert_eq!(wider_set_at(&[Private, PublicSet]), Some(1));
        assert_eq!(wider_set_at(&[Public, PrivateSet]), None);
        assert_eq!(wider_set_at(&[ProtectedSet]), None);
    }

    #[test]
    fn override_alone_is_not_another_attribute() {
        assert!(
            !form("#[\\Override] public function getRouteKeyName(): string {}")
                .attributed_beyond_override()
        );
    }

    #[test]
    fn a_property_keeps_the_case_of_its_name() {
        let found = form("#[Marker] protected $Table = 'users', $other;");
        assert_eq!(found.name, b"Table");
        assert!(found.only(ModifierKind::Protected));
        assert!(!found.by_reference);
        assert_eq!(found.parameters, 0);
        assert_eq!(names(&found), [vec![&b"vendor\\marker"[..]]]);
    }
}
