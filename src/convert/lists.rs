//! Lists: literals, reads by index, `count()` and copies (P2R-20, P2R-21, P2R-26, P2R-47,
//! P2R-48). An append is a change of its variable, so it sits with the other changes in
//! `variables.rs`.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::union::TUnion;
use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use super::helpers::Helper;
use super::{Converter, ParameterSignature, nulls, parameter_index};
use crate::printer::RExpr;
use crate::types::{self, RustType};

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

pub(super) fn unparenthesized<'e, 'a>(expression: &'e Expression<'a>) -> &'e Expression<'a> {
    match expression {
        Expression::Parenthesized(inner) => unparenthesized(inner.expression),
        other => other,
    }
}

/// The types the reads and writes of one body give each variable name, for the literals
/// that have no type of their own: `[]` and `null`.
pub(super) struct SiteTypes {
    /// The list type of each name, for a `[]` (P2R-21).
    pub(super) lists: HashMap<Vec<u8>, Option<RustType>>,
    /// The `Option` type of each name that may hold null (P2R-102); `None` when its reads
    /// and writes give different types.
    pub(super) nulls: HashMap<Vec<u8>, Option<RustType>>,
}

/// What the reads and writes of one body say about the type of each variable name.
struct Sites<'c> {
    /// Finds the function or static method a call runs, by its fully qualified name
    /// (P2R-151, P2R-153), and the type Mago gives a node.
    converter: &'c Converter<'c>,
    enums: &'c types::TypeNames,
    /// The declared return type of a function body.
    returns: Option<&'c RustType>,
    /// By PHP name without `$`; `None` for a name with two different list types.
    found: HashMap<Vec<u8>, Option<RustType>>,
    /// By PHP name: the type of every site with the `Option` taken off; `None` for a name
    /// whose sites disagree.
    inner: HashMap<Vec<u8>, Option<RustType>>,
    /// The names that a read or write gives an `Option` type, or that are assigned `null`.
    nullable: HashSet<Vec<u8>>,
}

impl Sites<'_> {
    /// Each variable passed alone as an argument flows into its parameter's type; a named
    /// argument flows into the parameter of its name (P2R-169).
    fn arguments(&mut self, list: &ArgumentList<'_>, parameters: &[ParameterSignature]) {
        for (position, argument) in list.arguments.iter().enumerate() {
            if let Some(parameter) =
                parameter_index(position, argument, parameters).and_then(|at| parameters.get(at))
                && let Expression::Variable(Variable::Direct(variable)) =
                    unparenthesized(argument.value())
            {
                self.add(variable, Some(parameter.ty.clone()), true);
            }
        }
    }

    /// The type Mago gives `node`, unless it is only `[]`.
    fn type_at(&self, node: &impl HasSpan) -> Option<RustType> {
        let union = self.converter.expression_type(node)?;
        types::rust_type(&without_empty(union), self.enums).ok()
    }

    /// A read or write of `variable` with the type `ty`. A declared parameter or return
    /// type that the variable flows into is `flows: true`: it gives the inner type of a
    /// `null`, but never makes the variable nullable, so `return $n;` in a `?int` function
    /// leaves `$n` an int.
    fn add(&mut self, variable: &DirectVariable<'_>, ty: Option<RustType>, flows: bool) {
        let Some(ty) = ty else {
            return;
        };
        let name = variable.name[1..].to_vec();
        if matches!(ty, RustType::Option(_)) && !flows {
            self.nullable.insert(name.clone());
        }
        let inner = ty.inner().clone();
        self.inner
            .entry(name.clone())
            .and_modify(|seen| {
                if seen.as_ref() != Some(&inner) {
                    *seen = None;
                }
            })
            .or_insert(Some(inner));
        if !matches!(ty, RustType::Vec(_)) {
            return;
        }
        self.found
            .entry(name)
            .and_modify(|seen| {
                if seen.as_ref() != Some(&ty) {
                    *seen = None;
                }
            })
            .or_insert(Some(ty));
    }
}

/// Collects [`Sites`]: the type Mago gives each read, append and assignment of a variable,
/// and the declared parameter or return type that a direct read of it flows into.
struct ListSites;

impl<'ast, 'arena, 'c> Walker<'ast, 'arena, Sites<'c>> for ListSites {
    fn walk_in_direct_variable(
        &self,
        variable: &'ast DirectVariable<'arena>,
        sites: &mut Sites<'c>,
    ) {
        let ty = sites.type_at(variable);
        sites.add(variable, ty, false);
    }

    fn walk_in_assignment(&self, assignment: &'ast Assignment<'arena>, sites: &mut Sites<'c>) {
        // Mago types an assignment, not its target.
        if matches!(assignment.operator, AssignmentOperator::Assign(_))
            && let Expression::Variable(Variable::Direct(variable)) = assignment.lhs
        {
            if matches!(
                unparenthesized(assignment.rhs),
                Expression::Literal(Literal::Null(_))
            ) {
                sites.nullable.insert(variable.name[1..].to_vec());
            }
            let ty = sites.type_at(assignment);
            sites.add(variable, ty, false);
        }
    }

    fn walk_in_return(&self, r#return: &'ast Return<'arena>, sites: &mut Sites<'c>) {
        if let Some(value) = r#return.value
            && let Expression::Variable(Variable::Direct(variable)) = unparenthesized(value)
        {
            sites.add(variable, sites.returns.cloned(), true);
        }
    }

    fn walk_in_function_call(&self, call: &'ast FunctionCall<'arena>, sites: &mut Sites<'c>) {
        let Expression::Identifier(name) = call.function else {
            return;
        };
        let converter = sites.converter;
        if let Some(signature) = converter.functions.get(&converter.call_key(name)) {
            sites.arguments(&call.argument_list, &signature.parameters);
        }
    }

    fn walk_in_static_method_call(
        &self,
        call: &'ast StaticMethodCall<'arena>,
        sites: &mut Sites<'c>,
    ) {
        let converter = sites.converter;
        let (Some(key), ClassLikeMemberSelector::Identifier(name)) =
            (converter.class_key(call.class), &call.method)
        else {
            return;
        };
        let Some(class) = converter.classes.iter().position(|c| c.key == key) else {
            return;
        };
        if let Some(at) = converter.method_index(class, name.value) {
            let parameters = &converter.classes[class].methods[at].signature.parameters;
            sites.arguments(&call.argument_list, parameters);
        }
    }

    // A nested function or closure has variables of its own.
    fn walk_function(&self, _: &'ast Function<'arena>, _: &mut Sites<'c>) {}

    fn walk_closure(&self, _: &'ast Closure<'arena>, _: &mut Sites<'c>) {}

    fn walk_arrow_function(&self, _: &'ast ArrowFunction<'arena>, _: &mut Sites<'c>) {}
}

impl Converter<'_> {
    /// The types that the reads and writes of a body give each variable name, so a `[]`
    /// takes its element type from them, also when it is read while still empty (P2R-21),
    /// and a `null` its `Option` type (P2R-102). `returns` is the declared return type of a
    /// function body.
    pub(super) fn site_types<'ast, 'arena: 'ast>(
        &self,
        statements: impl IntoIterator<Item = &'ast Statement<'arena>>,
        returns: Option<&RustType>,
    ) -> SiteTypes {
        let mut sites = Sites {
            converter: self,
            enums: &self.type_names,
            returns,
            found: HashMap::new(),
            inner: HashMap::new(),
            nullable: HashSet::new(),
        };
        for statement in statements {
            ListSites.walk_statement(statement, &mut sites);
        }
        // A name with no typed site at all is left out: its null has the type null.
        let nulls = sites
            .nullable
            .iter()
            .filter_map(|name| match sites.inner.get(name)? {
                Some(inner) => {
                    let ty = RustType::Option(Box::new(inner.clone()));
                    types::built(&ty).then(|| (name.clone(), Some(ty)))
                }
                None => Some((name.clone(), None)),
            })
            .collect();
        SiteTypes {
            lists: sites.found,
            nulls,
        }
    }

    /// An expression whose type is `expected` when it is known, so `[]` and `null` can take
    /// their type from the declared parameter or return type they flow into (P2R-21,
    /// P2R-102).
    pub(super) fn value(
        &mut self,
        expression: &Expression<'_>,
        expected: Option<&RustType>,
    ) -> Option<(RExpr, RustType)> {
        match (unparenthesized(expression), expected) {
            (Expression::Array(array), _) => self.list(array, expected),
            (Expression::Literal(Literal::Null(_)), Some(ty @ RustType::Option(_))) => {
                Some((RExpr::None, ty.clone()))
            }
            (Expression::Conditional(node), _) => self.conditional(node, expected),
            (other, _) => self.expression(other),
        }
    }

    /// A list literal `[a, b]` (P2R-47). `[]` has the type `expected` (P2R-21).
    pub(super) fn list(
        &mut self,
        array: &Array<'_>,
        expected: Option<&RustType>,
    ) -> Option<(RExpr, RustType)> {
        if array
            .elements
            .iter()
            .any(|element| matches!(element, ArrayElement::KeyValue(_)))
        {
            return self.shape_literal(array, expected.map(RustType::inner));
        }
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
        // A nullable list type gives `[]` its list type too (P2R-100).
        let expected = expected.map(RustType::inner);
        let found = match expected {
            Some(expected @ RustType::Vec(_)) if self.empty_list(array) => Ok(expected.clone()),
            _ if self.empty_list(array) => Err(UNKNOWN_ELEMENT.to_owned()),
            // Where a list of `Option` is expected, a literal of plain values, or of nulls
            // only, takes that type: its elements become `Some(..)` and `None` (P2R-102).
            Some(expected @ RustType::Vec(element))
                if matches!(**element, RustType::Option(_))
                    && self.elements_fit(array, element) =>
            {
                Ok(expected.clone())
            }
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
        // into the list (P2R-26). In a list of `Option`, `null` is `None` and every other
        // element is `Some(..)` (P2R-102).
        let wanted = match &ty {
            RustType::Vec(element) if matches!(**element, RustType::Option(_)) => {
                Some((**element).clone())
            }
            _ => None,
        };
        let items: Vec<Option<RExpr>> = values
            .into_iter()
            .map(|value| {
                let (expr, element) = self.value(value, wanted.as_ref())?;
                // Mago may type the element as `T` where it knows more (P2R-104).
                if let RustType::Vec(target) = &ty
                    && nulls::null_for_plain(&element, target)
                {
                    self.refuse(value, nulls::NULL_ELEMENT);
                    return None;
                }
                let expr = self.stored(value, expr, &element, array.span())?;
                Some(match &wanted {
                    Some(wanted) => nulls::wrapped(expr, &element, wanted),
                    None => expr,
                })
            })
            .collect();
        let items = items.into_iter().collect::<Option<Vec<RExpr>>>()?;
        Some((RExpr::List(items), ty))
    }

    /// True when every element Mago finds in the list literal `array` is null, `element`
    /// or its inner type (P2R-102).
    fn elements_fit(&self, array: &Array<'_>, element: &RustType) -> bool {
        let Some(elements) = self.expression_type(array).and_then(types::list_elements) else {
            return false;
        };
        types::only_null(&elements)
            || types::rust_type(&elements, &self.type_names)
                .is_ok_and(|found| found == *element || found == *element.inner())
    }

    /// Records the variables of a refused list literal, then refuses it.
    pub(super) fn refuse_list(&mut self, array: &Array<'_>, message: impl Into<String>) {
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

    /// `$a[$i]` with an int index (P2R-47), `$s[$i]`, the byte at an offset of a string
    /// (P2R-165), and `$s['k']`, a field of an array shape (P2R-203).
    pub(super) fn index(&mut self, access: &ArrayAccess<'_>) -> Option<(RExpr, RustType)> {
        let list = self.expression(access.array);
        if let Some((object, RustType::Shape(name))) = list {
            return self.shape_read(access, object, &name);
        }
        let index = self.expression(access.index);
        let ((list, list_type), (index, index_type)) = (list?, index?);
        let element = match list_type {
            RustType::Vec(element) => Some(*element),
            RustType::String => None,
            RustType::Option(_) => {
                self.refuse(
                    access,
                    "a value that may be null cannot be read by index (P2R-106)",
                );
                return None;
            }
            RustType::Class(_) => {
                self.refuse(
                    access,
                    "an element read on an object is not supported yet; call offsetGet() (P2R-220)",
                );
                return None;
            }
            _ => {
                self.refuse(
                    access,
                    "only a list or a string can be read by index (P2R-47, P2R-165)",
                );
                return None;
            }
        };
        if index_type != RustType::I64 {
            let message = match (&index_type, &element) {
                (RustType::Option(_), Some(_)) => {
                    "a value that may be null cannot be a list index (P2R-106)"
                }
                (RustType::Option(_), None) => {
                    "a value that may be null cannot be a string offset (P2R-106)"
                }
                (_, Some(_)) => "a list index must be an int (P2R-47)",
                (_, None) => "a string offset must be an int (P2R-165)",
            };
            self.refuse(access.index, message);
            return None;
        }
        let Some(element) = element else {
            let text = self.borrowed(list, &RustType::String);
            let call = self.helper_call(Helper::PhpOffset, vec![text, index]);
            return Some((call, RustType::String));
        };
        Some((RExpr::Index(Box::new(list), Box::new(index)), element))
    }

    /// `in_array($v, [...], true)` on a list literal of int, string or enum case literals
    /// of the needle's type, as `[a, b].contains(&v)`. Without `true` PHP compares loosely
    /// (P2R-163).
    pub(super) fn in_array(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let arguments = self.built_in_arguments(
            call,
            node,
            (3, 3),
            "in_array() converts only with three positional arguments, the last true, because PHP otherwise compares loosely (P2R-163)",
        )?;
        let (value, list, strict) = (arguments[0], arguments[1], arguments[2]);
        if !matches!(
            unparenthesized(strict),
            Expression::Literal(Literal::True(_))
        ) {
            self.record_refused(node);
            self.refuse(
                strict,
                "in_array() converts only with true as its third argument, because PHP otherwise compares loosely (P2R-163)",
            );
            return None;
        }
        let items = match unparenthesized(list) {
            Expression::Array(array) if !array.elements.is_empty() => array
                .elements
                .iter()
                .map(|element| match element {
                    ArrayElement::Value(element) if self.literal_element(element.value) => {
                        Some(element.value)
                    }
                    _ => None,
                })
                .collect::<Option<Vec<_>>>(),
            _ => None,
        };
        let Some(items) = items else {
            self.record_refused(node);
            self.refuse(
                list,
                "in_array() converts only on a list literal with elements that are all int, string or enum case literals (P2R-163)",
            );
            return None;
        };
        let needle = self.expression(value);
        let converted: Option<Vec<(RExpr, RustType)>> =
            items.iter().map(|item| self.expression(item)).collect();
        let ((needle, ty), converted) = (needle?, converted?);
        if matches!(ty, RustType::Option(_)) {
            self.refuse(value, nulls::NULL_BUILT_IN);
            return None;
        }
        if !matches!(ty, RustType::I64 | RustType::String | RustType::Enum(_)) {
            self.refuse(
                value,
                "in_array() converts only an int, string or enum needle (P2R-163)",
            );
            return None;
        }
        let mut elements = Vec::new();
        for (item, (expr, element)) in items.iter().zip(converted) {
            if element != ty {
                // PHP reads an integer literal above PHP_INT_MAX as a float (P2R-49).
                let message = if element == RustType::F64 {
                    "an integer literal above PHP_INT_MAX is a float in PHP, which in_array() does not convert (P2R-163)"
                } else {
                    "every element of the list must have the needle's type (P2R-163)"
                };
                self.refuse(*item, message);
                return None;
            }
            elements.push(expr);
        }
        // An owned string lends a `&str`, which the elements are (P2R-163).
        let needle = if ty == RustType::String && !self.is_reference(&needle) {
            RExpr::Method(Box::new(needle), "as_str")
        } else {
            needle
        };
        Some((
            RExpr::Contains(elements, Box::new(RExpr::Ref(Box::new(needle)))),
            RustType::Bool,
        ))
    }

    /// True for an element `in_array()` takes: an int literal with or without unary minus,
    /// a string literal without interpolation, or a case of an enum of the file (P2R-163).
    fn literal_element(&self, value: &Expression<'_>) -> bool {
        match unparenthesized(value) {
            Expression::Literal(Literal::Integer(_) | Literal::String(_)) => true,
            Expression::UnaryPrefix(unary) => {
                matches!(unary.operator, UnaryPrefixOperator::Negation(_))
                    && matches!(
                        unparenthesized(unary.operand),
                        Expression::Literal(Literal::Integer(_))
                    )
            }
            Expression::Access(Access::ClassConstant(access)) => {
                self.enum_case_of(access).is_some()
            }
            _ => false,
        }
    }

    /// `count($list)`, the length as `i64` (P2R-48).
    pub(super) fn count(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let arguments = self.built_in_arguments(
            call,
            node,
            (1, 1),
            "count() converts with one list argument only (P2R-48)",
        )?;
        let (list, ty) = self.expression(arguments[0])?;
        if matches!(ty, RustType::Option(_)) {
            self.refuse(arguments[0], nulls::NULL_BUILT_IN);
            return None;
        }
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
            types::rust_type(&without_empty(&maybe_empty), &types::TypeNames::default()),
            Ok(RustType::Vec(Box::new(RustType::I64)))
        );
        let only = TUnion::from_vec(vec![empty]);
        assert!(only_empty(&only));
        assert!(!only_empty(&maybe_empty));
    }
}
