//! PHP types to Rust types (P2R-20 to P2R-22, P2R-24, P2R-100, P2R-101, P2R-112,
//! P2R-132, P2R-200).

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::{Deref, DerefMut};

use heck::ToUpperCamelCase;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
use mago_codex::ttype::atomic::array::list::TList;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;

use crate::names;

/// The Rust type of each enum and class the file declares, by its lowercase PHP name: PHP
/// class names ignore letter case, and Mago keeps the case a use wrote (P2R-111, P2R-131).
/// It also gathers the array shapes that types map to, whose structs the output holds
/// (P2R-200).
#[derive(Default)]
pub struct TypeNames {
    names: HashMap<Vec<u8>, RustType>,
    /// The fields of each array shape, by the Rust name of its struct.
    shapes: RefCell<BTreeMap<String, Vec<ShapeField>>>,
    /// The Rust names of the enums without cases, which no shape may hold (P2R-200).
    empty: HashSet<String>,
}

impl TypeNames {
    /// Records that the enum `name` has no cases (P2R-200).
    pub fn without_cases(&mut self, name: &str) {
        self.empty.insert(name.to_owned());
    }

    /// The fields of the array shape whose struct is `name`, in struct order (P2R-200).
    pub fn shape(&self, name: &str) -> Option<Vec<ShapeField>> {
        self.shapes.borrow().get(name).cloned()
    }

    /// Every array shape a type has mapped to, by struct name in byte order (P2R-200).
    pub fn shapes(&self) -> Vec<(String, Vec<ShapeField>)> {
        self.shapes
            .borrow()
            .iter()
            .map(|(name, fields)| (name.clone(), fields.clone()))
            .collect()
    }
}

impl Deref for TypeNames {
    type Target = HashMap<Vec<u8>, RustType>;

    fn deref(&self) -> &Self::Target {
        &self.names
    }
}

impl DerefMut for TypeNames {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.names
    }
}

/// A field of the struct of an array shape (P2R-200).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShapeField {
    /// The PHP key.
    pub key: Vec<u8>,
    pub name: String,
    pub ty: RustType,
}

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
    /// An array shape, as a struct, by its Rust name (P2R-200).
    Shape(String),
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
            Self::Enum(name) | Self::Class(name) | Self::Shape(name) => name.clone(),
        }
    }

    /// The type of a function parameter: strings, lists, objects and shapes are borrowed,
    /// also inside an `Option` (P2R-24, P2R-101, P2R-132, P2R-201).
    pub fn parameter(&self) -> String {
        match self {
            Self::String => "&str".into(),
            Self::Vec(element) => format!("&[{}]", element.owned()),
            Self::Class(name) | Self::Shape(name) => format!("&{name}"),
            Self::Option(inner) => format!("Option<{}>", inner.parameter()),
            other => other.owned(),
        }
    }

    /// True for a struct: a class or an array shape, which copies with `.clone()` and is
    /// borrowed as `&S` (P2R-132, P2R-201).
    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Class(_) | Self::Shape(_))
    }

    /// The method that lends the value inside an owned `Option` of this inner type:
    /// `as_deref` gives `Option<&str>` and `Option<&[T]>`, `as_ref` gives `Option<&S>`
    /// (P2R-101, P2R-204).
    pub fn lending(&self) -> &'static str {
        if self.is_struct() {
            "as_ref"
        } else {
            "as_deref"
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
            Self::String | Self::Vec(_) | Self::Class(_) | Self::Shape(_) => false,
            Self::Option(inner) => inner.is_copy(),
        }
    }

    /// True for a class, or a list or `Option` that holds one.
    fn holds_class(&self) -> bool {
        match self {
            Self::Class(_) => true,
            Self::Vec(inner) | Self::Option(inner) => inner.holds_class(),
            _ => false,
        }
    }
}

/// True for the types built so far: int, float, bool and string, enums, classes, shapes,
/// lists of built types, and an `Option` of a built type that is not an `Option` (P2R-20,
/// P2R-100, P2R-112, P2R-132, P2R-200, P2R-204).
pub fn built(ty: &RustType) -> bool {
    match ty {
        RustType::I64 | RustType::F64 | RustType::Bool | RustType::String => true,
        RustType::Enum(_) | RustType::Class(_) | RustType::Shape(_) => true,
        RustType::Vec(element) => built(element),
        RustType::Option(inner) => !matches!(**inner, RustType::Option(_)) && built(inner),
    }
}

/// True for a list of a nullable struct type, `list<?C>` (P2R-204).
pub fn nullable_object(ty: &RustType) -> bool {
    match ty {
        RustType::Vec(element) => {
            matches!(&**element, RustType::Option(inner) if inner.is_struct())
                || nullable_object(element)
        }
        _ => false,
    }
}

/// The problem of a list of a nullable struct type (P2R-204).
pub const NULLABLE_OBJECT: &str = "a list of nullable objects is not supported yet (P2R-204)";

/// Maps an analysed PHP type to exactly one Rust type, widening literal and refined types
/// to their base type (P2R-21), with `enums` naming the enums and classes of the file and
/// gathering the array shapes (P2R-200). A `null` member makes it an `Option` (P2R-100).
/// The error is the problem message.
pub fn rust_type(union: &TUnion, enums: &TypeNames) -> Result<RustType, String> {
    let unknown = || format!("the type {} has no Rust type (P2R-22)", union.get_id());
    let mut found: Option<RustType> = None;
    let mut nullable = false;
    for atomic in union.types.iter() {
        if matches!(atomic, TAtomic::Null) {
            nullable = true;
            continue;
        }
        let this = match atomic {
            TAtomic::Array(TArray::Keyed(keyed)) if is_shape(keyed) => shape_type(keyed, enums)?,
            _ => atomic_type(atomic, enums).ok_or_else(unknown)?,
        };
        if found.as_ref().is_some_and(|seen| *seen != this) {
            return Err(unknown());
        }
        found = Some(this);
    }
    match found {
        // `?T` where `T` is itself nullable cannot be written; the union holds one null.
        Some(RustType::Option(_)) if nullable => Err(unknown()),
        Some(ty) if nullable => Ok(RustType::Option(Box::new(ty))),
        Some(ty) => Ok(ty),
        None => Err(unknown()),
    }
}

/// True for an array with known keys and no other elements, such as
/// `array{year: string}`; `array{}`, whose `[]` widens to a list (P2R-21), has none.
fn is_shape(keyed: &TKeyedArray) -> bool {
    keyed.parameters.is_none()
        && keyed
            .known_items
            .as_ref()
            .is_some_and(|items| !items.is_empty())
}

/// The struct of an array shape, gathered into `enums` (P2R-200). Mago keeps the keys in
/// byte order, which is the order of the fields and of the parts of the name.
fn shape_type(keyed: &TKeyedArray, enums: &TypeNames) -> Result<RustType, String> {
    let items = keyed.known_items.as_deref().into_iter().flatten();
    let mut fields: Vec<ShapeField> = Vec::new();
    let mut rust_name = String::new();
    for (key, (optional, value)) in items {
        let ArrayKey::String(key) = key else {
            return Err("an array shape converts only with string keys (P2R-200)".to_owned());
        };
        if *optional {
            return Err("an array shape with an optional key is not supported (P2R-200)".into());
        }
        let text = key.as_str_lossy();
        let Some(name) = names::rust_name(&text) else {
            return Err(format!(
                "the array shape key '{text}' has no Rust form (P2R-200)"
            ));
        };
        if fields.iter().any(|field| field.name == name) {
            return Err(format!(
                "two keys of this array shape give the field name {name} (P2R-200)"
            ));
        }
        let ty = rust_type(value, enums)?;
        if ty.holds_class() {
            return Err(
                "an array shape that holds an object is not supported yet (P2R-200)".into(),
            );
        }
        // No value of such a shape could exist, so rustc rejects the code after it.
        if matches!(&ty, RustType::Enum(name) if enums.empty.contains(name)) {
            return Err(
                "an array shape that holds an enum without cases cannot have a value (P2R-200)"
                    .into(),
            );
        }
        rust_name.push_str(&text.to_upper_camel_case());
        fields.push(ShapeField {
            key: key.as_bytes().to_vec(),
            name,
            ty,
        });
    }
    rust_name.push_str("Shape");
    if enums
        .values()
        .any(|ty| matches!(ty, RustType::Enum(n) | RustType::Class(n) if *n == rust_name))
    {
        return Err(format!(
            "{rust_name}, the Rust name of this array shape, is the name of an enum or class (P2R-200)"
        ));
    }
    let mut shapes = enums.shapes.borrow_mut();
    match shapes.get(&rust_name) {
        Some(known) if *known != fields => {
            return Err(format!(
                "two different array shapes give the Rust name {rust_name} (P2R-200)"
            ));
        }
        Some(_) => {}
        None => {
            shapes.insert(rust_name.clone(), fields);
        }
    }
    Ok(RustType::Shape(rust_name))
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
        rust_type(union, &TypeNames::default())
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
        let mut enums = TypeNames::default();
        enums.insert(b"suit".to_vec(), RustType::Enum("Suit".to_owned()));
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
    fn classes_are_borrowed_structs_and_may_be_nullable() {
        let class = RustType::Class("Point".into());
        assert_eq!(class.owned(), "Point");
        assert_eq!(class.parameter(), "&Point");
        assert!(!class.is_copy());
        assert!(built(&RustType::Vec(Box::new(class.clone()))));
        let option = RustType::Option(Box::new(class.clone()));
        assert!(built(&option));
        assert_eq!(option.parameter(), "Option<&Point>");
        assert_eq!(class.lending(), "as_ref");
        assert!(!nullable_object(&option));
        assert!(nullable_object(&RustType::Vec(Box::new(option))));
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
