//! Lists: literals, reads by index, `count()` and copies (P2R-20, P2R-21, P2R-26, P2R-47,
//! P2R-48). An append is a change of its variable, so it sits with the other changes in
//! `variables.rs`.

use std::borrow::Cow;

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::union::TUnion;
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::Converter;
use crate::printer::RExpr;
use crate::types::RustType;

/// Mago types `[]` as `array{}`, which has no element type of its own (P2R-21).
pub(super) const UNKNOWN_ELEMENT: &str =
    "the element type of this empty list is not known (P2R-21)";

fn is_empty_array(atomic: &TAtomic) -> bool {
    matches!(atomic, TAtomic::Array(array) if array.is_empty())
}

/// True when every member of `union` is the empty array.
pub(super) fn only_empty(union: &TUnion) -> bool {
    union.types.iter().all(is_empty_array)
}

/// `union` without its empty array members, when it has others: a list that may be empty
/// has the type `array{}|list<T>`, and `[]` widens to `list<T>` (P2R-21).
pub(super) fn without_empty(union: &TUnion) -> Cow<'_, TUnion> {
    let rest: Vec<TAtomic> = union
        .types
        .iter()
        .filter(|atomic| !is_empty_array(atomic))
        .cloned()
        .collect();
    if rest.is_empty() || rest.len() == union.types.len() {
        Cow::Borrowed(union)
    } else {
        Cow::Owned(TUnion::from_vec(rest))
    }
}

/// True for the literal `[]`.
pub(super) fn is_empty_literal(expression: &Expression<'_>) -> bool {
    matches!(unparenthesized(expression), Expression::Array(array) if array.elements.is_empty())
}

fn unparenthesized<'e, 'a>(expression: &'e Expression<'a>) -> &'e Expression<'a> {
    match expression {
        Expression::Parenthesized(inner) => unparenthesized(inner.expression),
        other => other,
    }
}

/// True for the list types built so far: lists of int, and lists of such lists (P2R-20).
pub(super) fn list_built(ty: &RustType) -> bool {
    match ty {
        RustType::Vec(element) => **element == RustType::I64 || list_built(element),
        _ => false,
    }
}

impl Converter<'_> {
    /// An expression whose type is `expected` when it is known, so `[]` can take its element
    /// type from the declared parameter or return type it flows into (P2R-21).
    pub(super) fn value(
        &mut self,
        expression: &Expression<'_>,
        expected: Option<&RustType>,
    ) -> Option<(RExpr, RustType)> {
        match unparenthesized(expression) {
            Expression::Array(array) => self.list(array, expected),
            other => self.expression(other),
        }
    }

    /// A list literal `[a, b]` (P2R-47). `[]` has the type `expected` (P2R-21).
    pub(super) fn list(
        &mut self,
        array: &Array<'_>,
        expected: Option<&RustType>,
    ) -> Option<(RExpr, RustType)> {
        let mut values = Vec::new();
        for element in array.elements.iter() {
            match element {
                ArrayElement::Value(element) => values.push(element.value),
                _ => {
                    self.refuse_list(array, "only a list of plain values is supported (P2R-47)");
                    return None;
                }
            }
        }
        let found = match expected {
            Some(expected @ RustType::Vec(_)) if self.empty_list(array) => Ok(expected.clone()),
            _ if self.empty_list(array) => Err(UNKNOWN_ELEMENT.to_owned()),
            _ => self.site_type(array),
        };
        let ty = match found {
            Ok(ty) => ty,
            Err(message) => {
                self.refuse_list(array, message);
                return None;
            }
        };
        // Every element converts, so the problems in each are reported. PHP copies each
        // into the list (P2R-26).
        let items: Vec<Option<RExpr>> = values
            .into_iter()
            .map(|value| {
                let (expr, element) = self.expression(value)?;
                Some(self.stored(value, expr, &element, array.span()))
            })
            .collect();
        let items = items.into_iter().collect::<Option<Vec<RExpr>>>()?;
        Some((RExpr::List(items), ty))
    }

    /// Records the variables of a refused list literal, then refuses it.
    fn refuse_list(&mut self, array: &Array<'_>, message: impl Into<String>) {
        for element in array.elements.iter() {
            match element {
                ArrayElement::KeyValue(element) => {
                    self.record_refused(element.key);
                    self.record_refused(element.value);
                }
                ArrayElement::Value(element) => self.record_refused(element.value),
                ArrayElement::Variadic(element) => self.record_refused(element.value),
                ArrayElement::Missing(_) => {}
            }
        }
        self.refuse(array, message);
    }

    /// `$a[$i]` with an int index (P2R-47).
    pub(super) fn index(&mut self, access: &ArrayAccess<'_>) -> Option<(RExpr, RustType)> {
        let list = self.expression(access.array);
        let index = self.expression(access.index);
        let ((list, list_type), (index, index_type)) = (list?, index?);
        let RustType::Vec(element) = list_type else {
            self.refuse(access, "only a list can be read by index (P2R-47)");
            return None;
        };
        if index_type != RustType::I64 {
            self.refuse(access.index, "a list index must be an int (P2R-47)");
            return None;
        }
        Some((RExpr::Index(Box::new(list), Box::new(index)), *element))
    }

    /// `count($list)`, the length as `i64` (P2R-48).
    pub(super) fn count(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let mut arguments = call.argument_list.arguments.iter();
        let argument = match (arguments.next(), arguments.next()) {
            (Some(Argument::Positional(argument)), None) if argument.ellipsis.is_none() => argument,
            _ => {
                self.record_refused(node);
                self.refuse(
                    node,
                    "count() converts with one list argument only (P2R-48)",
                );
                return None;
            }
        };
        let (list, ty) = self.expression(argument.value)?;
        if !matches!(ty, RustType::Vec(_)) {
            self.refuse(node, "count() converts on a list only (P2R-48)");
            return None;
        }
        Some((RExpr::Len(Box::new(list)), RustType::I64))
    }
}

#[cfg(test)]
mod tests {
    use mago_codex::ttype::atomic::array::TArray;
    use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
    use mago_codex::ttype::{get_int, get_list};

    use super::*;
    use crate::types;

    #[test]
    fn the_empty_array_widens_to_the_list_beside_it() {
        let empty = TAtomic::Array(TArray::Keyed(TKeyedArray::new()));
        let mut members = get_list(get_int()).types.to_vec();
        members.push(empty.clone());
        let maybe_empty = TUnion::from_vec(members);
        assert_eq!(
            types::rust_type(&without_empty(&maybe_empty)),
            Ok(RustType::Vec(Box::new(RustType::I64)))
        );
        let only = TUnion::from_vec(vec![empty]);
        assert!(only_empty(&only));
        assert!(!only_empty(&maybe_empty));
    }
}
