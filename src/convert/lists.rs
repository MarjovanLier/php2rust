//! Lists: literals, reads by index, `count()` and copies (P2R-20, P2R-21, P2R-26, P2R-47,
//! P2R-48). An append is a change of its variable, so it sits with the other changes in
//! `variables.rs`.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use mago_analyzer::artifacts::AnalysisArtifacts;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::union::TUnion;
use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use super::{Converter, Signature, nulls};
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
    /// The `Option` type of each name that may hold null, when it is known (P2R-102).
    pub(super) nulls: HashMap<Vec<u8>, RustType>,
}

/// What the reads and writes of one body say about the type of each variable name.
struct Sites<'c> {
    artifacts: &'c AnalysisArtifacts,
    functions: &'c HashMap<Vec<u8>, Signature>,
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
    /// The type Mago gives `node`, unless it is only `[]`.
    fn type_at(&self, node: &impl HasSpan) -> Option<RustType> {
        let union = self.artifacts.get_expression_type(node)?;
        types::rust_type(&without_empty(union)).ok()
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
        let Expression::Identifier(Identifier::Local(name)) = call.function else {
            return;
        };
        let functions = sites.functions;
        let Some(signature) = functions.get(&name.value.to_ascii_lowercase()) else {
            return;
        };
        let arguments = call.argument_list.arguments.iter();
        for (argument, parameter) in arguments.zip(&signature.parameters) {
            if let Argument::Positional(positional) = argument
                && positional.ellipsis.is_none()
                && let Expression::Variable(Variable::Direct(variable)) =
                    unparenthesized(positional.value)
            {
                sites.add(variable, Some(parameter.ty.clone()), true);
            }
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
            artifacts: &self.artifacts,
            functions: &self.functions,
            returns,
            found: HashMap::new(),
            inner: HashMap::new(),
            nullable: HashSet::new(),
        };
        for statement in statements {
            ListSites.walk_statement(statement, &mut sites);
        }
        let nulls = sites
            .nullable
            .iter()
            .filter_map(|name| {
                let inner = sites.inner.get(name)?.clone()?;
                let ty = RustType::Option(Box::new(inner));
                types::built(&ty).then(|| (name.clone(), ty))
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
            (other, _) => self.expression(other),
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
                let expr = self.stored(value, expr, &element, array.span());
                Some(match &wanted {
                    Some(wanted) => nulls::wrapped(expr, &element, wanted),
                    None => expr,
                })
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
