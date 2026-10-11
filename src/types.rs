//! PHP types to Rust types (P2R-20 to P2R-22, P2R-24).

use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::list::TList;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RustType {
    I64,
    F64,
    Bool,
    String,
    Vec(Box<RustType>),
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
        }
    }

    /// The type of a function parameter: strings and lists are borrowed (P2R-24).
    pub fn parameter(&self) -> String {
        match self {
            Self::String => "&str".into(),
            Self::Vec(element) => format!("&[{}]", element.owned()),
            other => other.owned(),
        }
    }
}

/// Maps an analysed PHP type to exactly one Rust type, widening literal and refined types
/// to their base type (P2R-21). The error names the PHP type, for the problem message.
pub fn rust_type(union: &TUnion) -> Result<RustType, String> {
    let mut found: Option<RustType> = None;
    for atomic in union.types.iter() {
        let this = atomic_type(atomic).ok_or_else(|| union.get_id().to_string())?;
        if found.as_ref().is_some_and(|seen| *seen != this) {
            return Err(union.get_id().to_string());
        }
        found = Some(this);
    }
    found.ok_or_else(|| union.get_id().to_string())
}

fn atomic_type(atomic: &TAtomic) -> Option<RustType> {
    match atomic {
        TAtomic::Scalar(TScalar::Integer(_)) => Some(RustType::I64),
        TAtomic::Scalar(TScalar::Float(_)) => Some(RustType::F64),
        TAtomic::Scalar(TScalar::Bool(_)) => Some(RustType::Bool),
        TAtomic::Scalar(TScalar::String(_)) => Some(RustType::String),
        TAtomic::Array(TArray::List(list)) => list_type(list),
        _ => None,
    }
}

/// `list<T>`, `non-empty-list<T>` and `list{T, T}` all give `Vec<T>`. A shape like
/// `list{int, int}` keeps its element types in `known_elements` and `never` as its
/// general element type, so both are folded together.
fn list_type(list: &TList) -> Option<RustType> {
    let mut members: Vec<&TUnion> = Vec::new();
    if !list
        .element_type
        .types
        .iter()
        .all(|a| matches!(a, TAtomic::Never))
    {
        members.push(&list.element_type);
    }
    if let Some(known) = &list.known_elements {
        members.extend(known.values().map(|(_, element)| element));
    }
    let mut found: Option<RustType> = None;
    for member in members {
        let this = rust_type(member).ok()?;
        if found.as_ref().is_some_and(|seen| *seen != this) {
            return None;
        }
        found = Some(this);
    }
    found.map(|element| RustType::Vec(Box::new(element)))
}

#[cfg(test)]
mod tests {
    use mago_codex::ttype::*;

    use super::*;

    #[test]
    fn narrowed_ints_and_strings_widen() {
        assert_eq!(rust_type(&get_literal_int(5)), Ok(RustType::I64));
        assert_eq!(rust_type(&get_int_range(Some(0), None)), Ok(RustType::I64));
        assert_eq!(rust_type(&get_positive_int()), Ok(RustType::I64));
        assert_eq!(rust_type(&get_non_empty_string()), Ok(RustType::String));
    }

    #[test]
    fn lists_become_vectors() {
        assert_eq!(
            rust_type(&get_list(get_float())),
            Ok(RustType::Vec(Box::new(RustType::F64)))
        );
    }

    #[test]
    fn mixed_null_and_mixed_unions_have_no_rust_type() {
        assert!(rust_type(&get_mixed()).is_err());
        assert!(rust_type(&get_null()).is_err());
        let int_or_string = TUnion::from_vec(
            get_int()
                .types
                .iter()
                .chain(get_string().types.iter())
                .cloned()
                .collect(),
        );
        assert!(rust_type(&int_or_string).is_err());
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
