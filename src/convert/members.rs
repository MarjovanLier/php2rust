//! The form of a class-like member: how a method or property is written, read once in the
//! same way for every kind of member. Each caller turns these facts into its own rule and
//! stop message; the return type, the body and a property's value stay with the caller.

use mago_names::ResolvedNames;
use mago_syntax::cst::{AttributeList, Method, Modifier, Property, Sequence};

use super::models::resolved;

/// One modifier keyword, in the order written.
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

/// How a method or property is written.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct MemberForm {
    /// A method's name in lower case; a property's name as written, without `$`, because
    /// PHP property names are case sensitive (the first item's for `public $a, $b;`).
    pub(super) name: Vec<u8>,
    pub(super) modifiers: Vec<ModifierKind>,
    /// `function &name()`; false for a property.
    pub(super) by_reference: bool,
    /// The number of parameters; 0 for a property.
    pub(super) parameters: usize,
    /// Each attribute group in source order, with the lower-case resolved names of its
    /// attributes; an empty `#[]` is an empty group.
    pub(super) attribute_groups: Vec<Vec<Vec<u8>>>,
}

impl MemberForm {
    pub(super) fn of_method(method: &Method<'_>, names: &ResolvedNames<'_>) -> Self {
        MemberForm {
            name: method.name.value.to_ascii_lowercase(),
            modifiers: kinds(&method.modifiers),
            by_reference: method.ampersand.is_some(),
            parameters: method.parameter_list.parameters.len(),
            attribute_groups: groups(&method.attribute_lists, names),
        }
    }

    pub(super) fn of_property(property: &Property<'_>, names: &ResolvedNames<'_>) -> Self {
        let (modifiers, attribute_lists, name) = match property {
            Property::Plain(plain) => (
                &plain.modifiers,
                &plain.attribute_lists,
                plain.items.iter().next().map(|item| item.variable().name),
            ),
            Property::Hooked(hooked) => (
                &hooked.modifiers,
                &hooked.attribute_lists,
                Some(hooked.item.variable().name),
            ),
        };
        MemberForm {
            name: name
                .map(|name| name.strip_prefix(b"$").unwrap_or(name).to_vec())
                .unwrap_or_default(),
            modifiers: kinds(modifiers),
            by_reference: false,
            parameters: 0,
            attribute_groups: groups(attribute_lists, names),
        }
    }

    /// True when `kind` is the member's one modifier.
    pub(super) fn only(&self, kind: ModifierKind) -> bool {
        self.modifiers == [kind]
    }

    /// True for an attribute other than `#[\Override]`.
    pub(super) fn attributed_beyond_override(&self) -> bool {
        self.attribute_groups
            .iter()
            .flatten()
            .any(|name| name != b"override")
    }
}

fn kinds(modifiers: &Sequence<'_, Modifier<'_>>) -> Vec<ModifierKind> {
    modifiers
        .iter()
        .map(|modifier| match modifier {
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
        })
        .collect()
}

fn groups(lists: &Sequence<'_, AttributeList<'_>>, names: &ResolvedNames<'_>) -> Vec<Vec<Vec<u8>>> {
    lists
        .iter()
        .map(|list| {
            list.attributes
                .iter()
                .map(|attribute| resolved(names, &attribute.name, attribute.name.value()))
                .collect()
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

    /// The form of the one member of a class declared with `member` in its body.
    fn form(member: &str) -> MemberForm {
        let source = format!("<?php\nuse Vendor\\Marker;\nclass C\n{{\n    {member}\n}}\n");
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

    #[test]
    fn a_method_reads_its_name_reference_and_parameters() {
        let found = form("public static function &__Make(int $a, $b) {}");
        assert_eq!(found.name, b"__make");
        assert_eq!(
            found.modifiers,
            [ModifierKind::Public, ModifierKind::Static]
        );
        assert!(found.by_reference);
        assert_eq!(found.parameters, 2);
        assert!(!found.only(ModifierKind::Public));
    }

    #[test]
    fn attributes_resolve_by_group_and_an_empty_group_counts() {
        let found = form("#[\\Override, Marker] #[] protected function casts(): array {}");
        assert_eq!(
            found.attribute_groups,
            [
                vec![b"override".to_vec(), b"vendor\\marker".to_vec()],
                vec![]
            ]
        );
        assert!(found.attributed_beyond_override());
        assert!(found.only(ModifierKind::Protected));
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
        assert_eq!(found.attribute_groups, [vec![b"vendor\\marker".to_vec()]]);
    }
}
