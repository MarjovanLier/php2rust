//! PHP types to Rust types (P2R-20 to P2R-22, P2R-24, P2R-100, P2R-101, P2R-112,
//! P2R-132).

use std::collections::HashMap;

use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::list::TList;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;

/// The Rust type of each enum and class the file declares, by its lowercase PHP name: PHP
/// class names ignore letter case, and Mago keeps the case a use wrote (P2R-111, P2R-131).
pub type TypeNames = HashMap<Vec<u8>, RustType>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RustType {
    I64,
    F64,
    Bool,
    String,
    Vec(Box<RustType>),
    /// `?T` and `T|null` (P2R-100).
    Option(Box<RustType>),
    /// An enum the file declares, by its Rust name (P2R-112).
    Enum(String),
    /// A readonly class the file declares, as a struct, by its Rust name (P2R-132).
    Class(String),
}

impl RustType {
    /// The type of an owned value: a return type, a `let` or a list element.
    pub fn owned(&self) -> String {
        match self {
            Self::I64 => "i64".into(),
            Self::F64 => "f64".into(),
            Self::Bool => "bool".into(),
            Self::String => "String".into(),
            Self::Vec(element) => format!("Vec<{}>", element.owned()),
            Self::Option(inner) => format!("Option<{}>", inner.owned()),
            Self::Enum(name) | Self::Class(name) => name.clone(),
        }
    }

    /// The type of a function parameter: strings, lists and objects are borrowed, also
    /// inside an `Option` (P2R-24, P2R-101, P2R-132).
    pub fn parameter(&self) -> String {
        match self {
            Self::String => "&str".into(),
            Self::Vec(element) => format!("&[{}]", element.owned()),
            Self::Class(name) => format!("&{name}"),
            Self::Option(inner) => format!("Option<{}>", inner.parameter()),
            other => other.owned(),
        }
    }

    /// The inner type of an `Option` (P2R-100); any other type is its own.
    pub fn inner(&self) -> &RustType {
        match self {
            Self::Option(inner) => inner,
            other => other,
        }
    }

    /// True when Rust copies the value on assignment, so PHP's copy needs no `.clone()`
    /// (P2R-26).
    pub fn is_copy(&self) -> bool {
        match self {
            Self::I64 | Self::F64 | Self::Bool | Self::Enum(_) => true,
            Self::String | Self::Vec(_) | Self::Class(_) => false,
            Self::Option(inner) => inner.is_copy(),
        }
    }
}

/// True for the types built so far: int, float, bool and string, enums, classes, lists of
/// built types, and an `Option` of a built type that is neither an `Option` nor a class
/// (P2R-20, P2R-100, P2R-112, P2R-132).
pub fn built(ty: &RustType) -> bool {
    match ty {
        RustType::I64 | RustType::F64 | RustType::Bool | RustType::String => true,
        RustType::Enum(_) | RustType::Class(_) => true,
        RustType::Vec(element) => built(element),
        RustType::Option(inner) => {
            !matches!(**inner, RustType::Option(_) | RustType::Class(_)) && built(inner)
        }
    }
}

/// True for `?C`, a nullable object type, which a later slice builds (P2R-132).
pub fn nullable_object(ty: &RustType) -> bool {
    match ty {
        RustType::Option(inner) => matches!(**inner, RustType::Class(_)),
        RustType::Vec(element) => nullable_object(element),
        _ => false,
    }
}

/// The problem of a nullable object type (P2R-132).
pub const NULLABLE_OBJECT: &str = "a nullable object type is not supported yet (P2R-132)";

/// Maps an analysed PHP type to exactly one Rust type, widening literal and refined types
/// to their base type (P2R-21), with `enums` naming the enums and classes of the file. A
/// `null` member makes it an `Option` (P2R-100). The error names the PHP type, for the
/// problem message.
pub fn rust_type(union: &TUnion, enums: &TypeNames) -> Result<RustType, String> {
    let mut found: Option<RustType> = None;
    let mut nullable = false;
    for atomic in union.types.iter() {
        if matches!(atomic, TAtomic::Null) {
            nullable = true;
            continue;
        }
        let this = atomic_type(atomic, enums).ok_or_else(|| union.get_id().to_string())?;
        if found.as_ref().is_some_and(|seen| *seen != this) {
            return Err(union.get_id().to_string());
        }
        found = Some(this);
    }
    match found {
        // `?T` where `T` is itself nullable cannot be written; the union holds one null.
        Some(RustType::Option(_)) if nullable => Err(union.get_id().to_string()),
        Some(ty) if nullable => Ok(RustType::Option(Box::new(ty))),
        Some(ty) => Ok(ty),
        None => Err(union.get_id().to_string()),
    }
}

/// True when every member of `union` is `null`: the type of the literal `null`, and of a
/// variable that may hold null where Mago knows it does (P2R-104).
pub fn only_null(union: &TUnion) -> bool {
    !union.types.is_empty() && union.types.iter().all(|a| matches!(a, TAtomic::Null))
}

fn atomic_type(atomic: &TAtomic, enums: &TypeNames) -> Option<RustType> {
    match atomic {
        TAtomic::Scalar(TScalar::Integer(_)) => Some(RustType::I64),
        TAtomic::Scalar(TScalar::Float(_)) => Some(RustType::F64),
        TAtomic::Scalar(TScalar::Bool(_)) => Some(RustType::Bool),
        TAtomic::Scalar(TScalar::String(_)) => Some(RustType::String),
        TAtomic::Array(TArray::List(list)) => list_type(list, enums),
        // A single case (`enum(Suit::Hearts)`) widens to its enum (P2R-21).
        TAtomic::Object(TObject::Enum(found)) => {
            match enums.get(&found.name.as_bytes().to_ascii_lowercase()) {
                Some(ty @ RustType::Enum(_)) => Some(ty.clone()),
                _ => None,
            }
        }
        // `$this(C)` widens to `C` (P2R-132).
        TAtomic::Object(TObject::Named(found)) => enums
            .get(&found.name.as_bytes().to_ascii_lowercase())
            .cloned(),
        _ => None,
    }
}

/// `list<T>`, `non-empty-list<T>` and `list{T, T}` all give `Vec<T>`. A shape like
/// `list{int, int}` keeps its element types in `known_elements` and `never` as its
/// general element type, so both are folded together. They fold as one union, so
/// `list{int(1), null}` gives `Vec<Option<i64>>` (P2R-100).
fn list_type(list: &TList, enums: &TypeNames) -> Option<RustType> {
    let members = element_members(list);
    if members.is_empty() {
        return None;
    }
    let element = rust_type(&TUnion::from_vec(members), enums).ok()?;
    Some(RustType::Vec(Box::new(element)))
}

/// The element types of a list: its general element type unless it is `never`, and the
/// type of each known element.
fn element_members(list: &TList) -> Vec<TAtomic> {
    let mut members: Vec<TAtomic> = Vec::new();
    if !list
        .element_type
        .types
        .iter()
        .all(|a| matches!(a, TAtomic::Never))
    {
        members.extend(list.element_type.types.iter().cloned());
    }
    if let Some(known) = &list.known_elements {
        for (_, element) in known.values() {
            members.extend(element.types.iter().cloned());
        }
    }
    members
}

/// The element types of every list in `union` as one union, when every member is a list.
/// After an append, Mago types a list that holds null as a union of list shapes, such as
/// `list{int(1), null}|list{2: int(4)}`, which has no one Rust type (P2R-102).
pub fn list_elements(union: &TUnion) -> Option<TUnion> {
    let mut members: Vec<TAtomic> = Vec::new();
    for atomic in union.types.iter() {
        let TAtomic::Array(TArray::List(list)) = atomic else {
            return None;
        };
        members.extend(element_members(list));
    }
    (!members.is_empty()).then(|| TUnion::from_vec(members))
}

#[cfg(test)]
mod tests {
    use mago_codex::ttype::*;

    use super::*;

    /// The Rust type of `union` in a file that declares no enum.
    fn rust(union: &TUnion) -> Result<RustType, String> {
        rust_type(union, &TypeNames::new())
    }

    #[test]
    fn narrowed_ints_and_strings_widen() {
        assert_eq!(rust(&get_literal_int(5)), Ok(RustType::I64));
        assert_eq!(rust(&get_int_range(Some(0), None)), Ok(RustType::I64));
        assert_eq!(rust(&get_positive_int()), Ok(RustType::I64));
        assert_eq!(rust(&get_non_empty_string()), Ok(RustType::String));
    }

    #[test]
    fn lists_become_vectors() {
        assert_eq!(
            rust(&get_list(get_float())),
            Ok(RustType::Vec(Box::new(RustType::F64)))
        );
    }

    #[test]
    fn enums_take_their_declared_rust_name() {
        let enums = TypeNames::from([(b"suit".to_vec(), RustType::Enum("Suit".to_owned()))]);
        let case = TUnion::from_vec(vec![TAtomic::Object(TObject::new_enum_case(
            "Suit".into(),
            "Hearts".into(),
        ))]);
        assert_eq!(rust_type(&case, &enums), Ok(RustType::Enum("Suit".into())));
        assert!(rust(&case).is_err());
        assert!(RustType::Enum("Suit".into()).is_copy());
    }

    #[test]
    fn mixed_null_and_mixed_unions_have_no_rust_type() {
        assert!(rust(&get_mixed()).is_err());
        assert!(rust(&get_null()).is_err());
        let int_or_string = TUnion::from_vec(
            get_int()
                .types
                .iter()
                .chain(get_string().types.iter())
                .cloned()
                .collect(),
        );
        assert!(rust(&int_or_string).is_err());
    }

    #[test]
    fn a_null_member_gives_an_option() {
        let with_null = |union: TUnion| {
            let mut members = union.types.to_vec();
            members.extend(get_null().types.iter().cloned());
            TUnion::from_vec(members)
        };
        assert_eq!(
            rust(&with_null(get_int())),
            Ok(RustType::Option(Box::new(RustType::I64)))
        );
        assert_eq!(
            rust(&get_list(with_null(get_string()))),
            Ok(RustType::Vec(Box::new(RustType::Option(Box::new(
                RustType::String
            )))))
        );
        assert!(only_null(&get_null()));
        assert!(!only_null(&with_null(get_int())));
        let option = RustType::Option(Box::new(RustType::String));
        assert_eq!(option.parameter(), "Option<&str>");
        assert_eq!(option.owned(), "Option<String>");
        assert!(!option.is_copy());
        assert!(built(&option));
        assert!(!built(&RustType::Option(Box::new(option))));
    }

    #[test]
    fn classes_are_borrowed_structs_and_never_nullable() {
        let class = RustType::Class("Point".into());
        assert_eq!(class.owned(), "Point");
        assert_eq!(class.parameter(), "&Point");
        assert!(!class.is_copy());
        assert!(built(&RustType::Vec(Box::new(class.clone()))));
        assert!(!built(&RustType::Option(Box::new(class))));
    }

    #[test]
    fn parameters_borrow_strings_and_lists() {
        assert_eq!(RustType::String.parameter(), "&str");
        assert_eq!(RustType::Vec(Box::new(RustType::I64)).parameter(), "&[i64]");
        assert_eq!(
            RustType::Vec(Box::new(RustType::String)).owned(),
            "Vec<String>"
        );
    }
}
