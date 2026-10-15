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
    /// The members of each union enum in variant order, by its Rust name (P2R-230).
    unions: RefCell<BTreeMap<String, Vec<RustType>>>,
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

    /// The members of the union enum `name`, in variant order (P2R-230).
    pub fn union(&self, name: &str) -> Option<Vec<RustType>> {
        self.unions.borrow().get(name).cloned()
    }

    /// Every union enum a type has mapped to, by name in byte order (P2R-230).
    pub fn unions(&self) -> Vec<(String, Vec<RustType>)> {
        self.unions
            .borrow()
            .iter()
            .map(|(name, members)| (name.clone(), members.clone()))
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
    /// A declared union, as a generated enum, by its Rust name (P2R-230).
    Union(String),
    /// A map, `IndexMap<K, V>`, keyed by `I64` or by the text of the key, `String`
    /// (P2R-240).
    Map(Box<RustType>, Box<RustType>),
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
            Self::Enum(name) | Self::Class(name) | Self::Shape(name) | Self::Union(name) => {
                name.clone()
            }
            Self::Map(key, value) => format!("IndexMap<{}, {}>", key.owned(), value.owned()),
        }
    }

    /// The type of a function parameter: strings, lists, objects, shapes and unions are
    /// borrowed, also inside an `Option` (P2R-24, P2R-101, P2R-132, P2R-201, P2R-232).
    pub fn parameter(&self) -> String {
        match self {
            Self::String => "&str".into(),
            Self::Vec(element) => format!("&[{}]", element.owned()),
            Self::Class(name) | Self::Shape(name) | Self::Union(name) => format!("&{name}"),
            Self::Map(..) => format!("&{}", self.owned()),
            Self::Option(inner) => format!("Option<{}>", inner.parameter()),
            other => other.owned(),
        }
    }

    /// True for a struct: a class, an array shape, a union enum or a map, which copies
    /// with `.clone()` and is borrowed as `&S` (P2R-132, P2R-201, P2R-232, P2R-240).
    pub fn is_struct(&self) -> bool {
        matches!(
            self,
            Self::Class(_) | Self::Shape(_) | Self::Union(_) | Self::Map(..)
        )
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
            Self::String | Self::Vec(_) | Self::Class(_) | Self::Shape(_) | Self::Union(_) => false,
            Self::Map(..) => false,
            Self::Option(inner) => inner.is_copy(),
        }
    }

    /// True for a class, or a list or `Option` that holds one.
    fn holds_class(&self) -> bool {
        match self {
            Self::Class(_) => true,
            Self::Vec(inner) | Self::Option(inner) | Self::Map(_, inner) => inner.holds_class(),
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
        RustType::Enum(_) | RustType::Class(_) | RustType::Shape(_) | RustType::Union(_) => true,
        RustType::Vec(element) | RustType::Map(_, element) => built(element),
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
            TAtomic::Array(TArray::Keyed(keyed)) if is_map(keyed) => map_type(keyed, enums)?,
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

/// The Rust type of a declared type (a parameter, a return value, a property, or K or V
/// of `ArrayAccess`): `rust_type`, or a union enum gathered into `enums` for a union of
/// members with different Rust types (P2R-230).
pub fn declared_type(union: &TUnion, enums: &TypeNames) -> Result<RustType, String> {
    let error = match rust_type(union, enums) {
        Ok(ty) => return Ok(ty),
        Err(error) => error,
    };
    let mut nullable = false;
    let mut members: Vec<RustType> = Vec::new();
    for atomic in union.types.iter() {
        let found = match atomic {
            TAtomic::Null => {
                nullable = true;
                continue;
            }
            TAtomic::Scalar(TScalar::Generic) => {
                vec![
                    RustType::Bool,
                    RustType::F64,
                    RustType::I64,
                    RustType::String,
                ]
            }
            TAtomic::Scalar(TScalar::ArrayKey) => vec![RustType::I64, RustType::String],
            // A declared map may hold a union (P2R-250).
            TAtomic::Array(TArray::Keyed(keyed)) if is_map(keyed) => {
                vec![declared_map(keyed, enums)?]
            }
            // A list of a union fails here, and a union stays out of lists (P2R-230).
            _ => vec![
                rust_type(&TUnion::from_vec(vec![atomic.clone()]), enums)
                    .map_err(|_| error.clone())?,
            ],
        };
        for ty in found {
            if nullable_object(&ty) {
                return Err(NULLABLE_OBJECT.to_owned());
            }
            if !members.contains(&ty) {
                members.push(ty);
            }
        }
    }
    // A map whose value type is a union, alone or with null (P2R-250).
    if let [map @ RustType::Map(..)] = members.as_slice() {
        let map = map.clone();
        return Ok(if nullable {
            RustType::Option(Box::new(map))
        } else {
            map
        });
    }
    if members.iter().any(|ty| matches!(ty, RustType::Map(..))) {
        return Err("a map inside a union is not supported yet (P2R-240)".to_owned());
    }
    if members.len() < 2 {
        return Err(error);
    }
    members.sort_by_key(variant);
    if let Some(pair) = members
        .windows(2)
        .find(|pair| variant(&pair[0]) == variant(&pair[1]))
    {
        return Err(format!(
            "two members of this union give the variant name {} (P2R-230)",
            variant(&pair[0])
        ));
    }
    let rust_name = members.iter().map(variant).collect::<Vec<_>>().join("Or");
    if enums.shapes.borrow().contains_key(&rust_name)
        || enums
            .values()
            .any(|ty| matches!(ty, RustType::Enum(n) | RustType::Class(n) if *n == rust_name))
    {
        return Err(format!(
            "{rust_name}, the Rust name of this union, is the name of an enum, class or shape (P2R-230)"
        ));
    }
    let mut unions = enums.unions.borrow_mut();
    match unions.get(&rust_name) {
        Some(known) if *known != members => {
            return Err(format!(
                "two different unions give the Rust name {rust_name} (P2R-230)"
            ));
        }
        Some(_) => {}
        None => {
            unions.insert(rust_name.clone(), members);
        }
    }
    let ty = RustType::Union(rust_name);
    Ok(if nullable {
        RustType::Option(Box::new(ty))
    } else {
        ty
    })
}

/// The name of the variant that holds a member of a union enum (P2R-230).
pub fn variant(ty: &RustType) -> String {
    match ty {
        RustType::I64 => "Int".to_owned(),
        RustType::F64 => "Float".to_owned(),
        RustType::Bool => "Bool".to_owned(),
        RustType::String => "String".to_owned(),
        RustType::Vec(_) => "List".to_owned(),
        RustType::Map(..) => "Map".to_owned(),
        RustType::Enum(name)
        | RustType::Class(name)
        | RustType::Shape(name)
        | RustType::Union(name) => name.clone(),
        RustType::Option(inner) => variant(inner),
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

/// True for an array with a key type and a value type, such as `array<string, int>`, also
/// with the keys a write by a literal key adds, `array{'z': int, ...<string, int>}`
/// (P2R-240).
fn is_map(keyed: &TKeyedArray) -> bool {
    keyed.parameters.is_some()
}

/// True when every member of `site`, a type Mago gives, is one of `members`, the members of
/// a union enum, or `null` when `nullable` (P2R-232).
pub fn within(site: &TUnion, members: &[RustType], nullable: bool, enums: &TypeNames) -> bool {
    let scalar = |member: &RustType| {
        matches!(
            member,
            RustType::I64 | RustType::F64 | RustType::Bool | RustType::String
        )
    };
    site.types.iter().all(|atomic| match atomic {
        TAtomic::Null => nullable,
        TAtomic::Scalar(TScalar::Generic) => members.iter().any(scalar),
        TAtomic::Scalar(TScalar::ArrayKey) => members
            .iter()
            .any(|member| matches!(member, RustType::I64 | RustType::String)),
        other => rust_type(&TUnion::from_vec(vec![other.clone()]), enums)
            .is_ok_and(|ty| members.contains(&ty)),
    })
}

/// True when `union`, Mago's type of a variable that holds the map `map`, has only keys and
/// values the map takes. Mago types a map after a write by a literal key with that key, as
/// a shape (`array{'z': int}` after `$m = []; $m['z'] = 2;`) or beside the key and value
/// types, and gives `[]` the type `array{}` (P2R-240).
pub fn fits_map(union: &TUnion, map: &RustType, enums: &TypeNames) -> bool {
    let RustType::Map(key, value) = map else {
        return false;
    };
    let nullable = matches!(**value, RustType::Option(_));
    let value_fits = |union: &TUnion| {
        if only_null(union) {
            return nullable;
        }
        // A union value takes a value of any of its members (P2R-250).
        if let RustType::Union(name) = value.inner() {
            return enums
                .union(name)
                .is_some_and(|members| within(union, &members, nullable, enums));
        }
        rust_type(union, enums).is_ok_and(|found| found == **value || found == *value.inner())
    };
    union.types.iter().all(|atomic| {
        let TAtomic::Array(TArray::Keyed(keyed)) = atomic else {
            return false;
        };
        // An int key is stored as its text in a map keyed by text.
        let mut items = keyed.known_items.iter().flat_map(|items| items.iter());
        let known = items.all(|(known, (_, item))| {
            let key_fits = match known {
                ArrayKey::Integer(_) => true,
                ArrayKey::String(_) => **key == RustType::String,
                _ => false,
            };
            key_fits && value_fits(item)
        });
        let rest = keyed.parameters.as_ref().is_none_or(|(_, rest)| {
            let general = TKeyedArray {
                known_items: None,
                ..keyed.clone()
            };
            // Only the key type is read here; `value_fits` checks the values.
            map_with(&general, |_| Ok(RustType::I64)).is_ok_and(|found| match found {
                RustType::Map(found_key, _) => found_key == *key || *found_key == RustType::I64,
                _ => false,
            }) && value_fits(rest)
        });
        known && rest
    })
}

/// The map of `array<K, V>`: `IndexMap<i64, V>` for an int key, and `IndexMap<String, V>`
/// keyed by the text of the key for a string, `int|string` or `array-key` key, because PHP
/// turns stored and lookup keys the same way. Known keys and their values count as keys
/// and values of the map (P2R-240).
fn map_type(keyed: &TKeyedArray, enums: &TypeNames) -> Result<RustType, String> {
    map_with(keyed, |values| rust_type(values, enums))
}

/// The map of a declared `array<K, V>`, whose value type may be a union (P2R-250).
fn declared_map(keyed: &TKeyedArray, enums: &TypeNames) -> Result<RustType, String> {
    map_with(keyed, |values| declared_type(values, enums))
}

/// The map of `keyed`, with `value` giving the Rust type of the value type.
fn map_with(
    keyed: &TKeyedArray,
    value: impl Fn(&TUnion) -> Result<RustType, String>,
) -> Result<RustType, String> {
    let Some((key, value_type)) = &keyed.parameters else {
        return Err("an array without key and value types has no Rust type (P2R-22)".into());
    };
    let unknown = || {
        format!(
            "the map key type {} has no Rust type (P2R-240)",
            key.get_id()
        )
    };
    let mut int = false;
    let mut text = false;
    for atomic in key.types.iter() {
        match atomic {
            TAtomic::Scalar(TScalar::Integer(_)) => int = true,
            TAtomic::Scalar(TScalar::String(_)) => text = true,
            TAtomic::Scalar(TScalar::ArrayKey) => (int, text) = (true, true),
            _ => return Err(unknown()),
        }
    }
    let mut values = value_type.types.to_vec();
    for (known, (_, known_value)) in keyed.known_items.iter().flat_map(|items| items.iter()) {
        match known {
            ArrayKey::Integer(_) => int = true,
            ArrayKey::String(_) => text = true,
            _ => return Err(unknown()),
        }
        values.extend(known_value.types.iter().cloned());
    }
    let key = match (int, text) {
        (true, false) => RustType::I64,
        (_, true) => RustType::String,
        _ => return Err(unknown()),
    };
    let value = value(&TUnion::from_vec(values))?;
    Ok(RustType::Map(Box::new(key), Box::new(value)))
}

/// The struct of an array shape, gathered into `enums` (P2R-200). Mago keeps the keys in
/// byte order, which is the order of the fields and of the parts of the name.
fn shape_type(keyed: &TKeyedArray, enums: &TypeNames) -> Result<RustType, String> {
    let items = keyed.known_items.as_deref().into_iter().flatten();
    // The name depends on the keys alone, so a shape that Mago narrows finds the struct of
    // the declared shape before its fields map.
    let mut rust_name: String = items
        .clone()
        .filter_map(|(key, _)| match key {
            ArrayKey::String(key) => Some(key.as_str_lossy().to_upper_camel_case()),
            _ => None,
        })
        .collect();
    rust_name.push_str("Shape");
    let known = enums.shapes.borrow().get(&rust_name).cloned();
    let mut fields: Vec<ShapeField> = Vec::new();
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
        // On the null side of a check, Mago types a `?T` field as `null` (P2R-104).
        let declared = known
            .iter()
            .flatten()
            .find(|field| field.key == key.as_bytes() && matches!(field.ty, RustType::Option(_)));
        let ty = match declared {
            Some(field) if only_null(value) => field.ty.clone(),
            _ => rust_type(value, enums)?,
        };
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
        fields.push(ShapeField {
            key: key.as_bytes().to_vec(),
            name,
            ty,
        });
    }
    if enums
        .values()
        .any(|ty| matches!(ty, RustType::Enum(n) | RustType::Class(n) if *n == rust_name))
    {
        return Err(format!(
            "{rust_name}, the Rust name of this array shape, is the name of an enum or class (P2R-200)"
        ));
    }
    // A union enum checks the shapes known when it is made, so a later shape checks it.
    if enums.unions.borrow().contains_key(&rust_name) {
        return Err(format!(
            "{rust_name}, the Rust name of this array shape, is the name of a union enum (P2R-230)"
        ));
    }
    let mut shapes = enums.shapes.borrow_mut();
    match shapes.get(&rust_name) {
        // After a check of a field against null, Mago narrows the whole shape: that is
        // still the declared struct, as a narrowed read keeps its `Option` (P2R-104).
        Some(known) if narrows(known, &fields) => {}
        Some(_) => {
            return Err(format!(
                "two different array shapes give the Rust name {rust_name} (P2R-200)"
            ));
        }
        None => {
            shapes.insert(rust_name.clone(), fields);
        }
    }
    Ok(RustType::Shape(rust_name))
}

/// True when `fields` are the fields of `known`, each of the same type or, for a field
/// that may be null, of its inner type (P2R-104, P2R-200).
fn narrows(known: &[ShapeField], fields: &[ShapeField]) -> bool {
    known.len() == fields.len()
        && known.iter().zip(fields).all(|(known, field)| {
            known.key == field.key
                && (known.ty == field.ty
                    || matches!(&known.ty, RustType::Option(inner) if **inner == field.ty))
        })
}

/// True when every member of `union` is `null`: the type of the literal `null`, and of a
/// variable that may hold null where Mago knows it does (P2R-104).
pub fn only_null(union: &TUnion) -> bool {
    !union.types.is_empty() && union.types.iter().all(|a| matches!(a, TAtomic::Null))
}

/// True when `union` is `mixed`, or `scalar` after `is_scalar()`: what Mago knows of a
/// parameter that the `ArrayAccess` template types (P2R-220).
pub fn mixed_or_scalar(union: &TUnion) -> bool {
    union.is_mixed() || matches!(union.types.as_ref(), [TAtomic::Scalar(TScalar::Generic)])
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
