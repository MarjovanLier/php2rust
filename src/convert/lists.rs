//! Lists: literals, reads by index, `count()` and copies (P2R-20, P2R-26, P2R-47, P2R-48).
//! An append is a change of its variable, so it sits with the other changes in
//! `variables.rs`.

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::Converter;
use crate::printer::RExpr;
use crate::types::RustType;

/// True for the list types built so far: lists of int, and lists of such lists (P2R-20).
pub(super) fn list_built(ty: &RustType) -> bool {
    match ty {
        RustType::Vec(element) => **element == RustType::I64 || list_built(element),
        _ => false,
    }
}

impl Converter<'_> {
    /// A list literal `[a, b]` (P2R-47).
    pub(super) fn list(&mut self, array: &Array<'_>) -> Option<(RExpr, RustType)> {
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
        let ty = match self.site_type(array) {
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
