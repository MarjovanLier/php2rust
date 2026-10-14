//! Array shapes as structs: the struct of each shape, shape literals and element reads
//! (P2R-200 to P2R-203).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::control::emit;
use super::lists::unparenthesized;
use super::{Converter, nulls};
use crate::printer::RExpr;
use crate::types::RustType;

/// The problem of an array literal that is not a shape literal (P2R-202).
const NOT_A_SHAPE: &str = "an array literal with keys converts only as an array shape, with a string literal key on every element (P2R-202)";

/// The PHP key of a string literal key, which Mago compares by its bytes.
fn literal_key<'k>(key: &'k Expression<'_>) -> Option<&'k [u8]> {
    match unparenthesized(key) {
        Expression::Literal(Literal::String(string)) => string.value,
        _ => None,
    }
}

/// True for a shape element chain that reads no list element: where PHP's isset of `??`
/// finds an index out of range and gives the default, Rust panics (P2R-203).
pub(super) fn no_list_read(expression: &Expression<'_>) -> bool {
    match unparenthesized(expression) {
        Expression::ArrayAccess(access) => {
            literal_key(access.index).is_some() && no_list_read(access.array)
        }
        Expression::Access(Access::Property(access)) => no_list_read(access.object),
        _ => true,
    }
}

impl Converter<'_> {
    /// The struct of each array shape the output uses, by name in byte order (P2R-200).
    pub(super) fn shape_items(&self) -> Vec<(String, Vec<String>)> {
        self.type_names
            .shapes()
            .into_iter()
            .map(|(name, fields)| {
                let mut lines = vec![
                    "#[derive(Clone, Debug)]".to_owned(),
                    format!("pub struct {name} {{"),
                ];
                for field in fields {
                    lines.push(emit(
                        1,
                        format!("pub {}: {},", field.name, field.ty.owned()),
                    ));
                }
                lines.push("}".to_owned());
                (name, lines)
            })
            .collect()
    }

    /// `['k' => v, ...]` as a struct literal of the shape `expected` names, or of the shape
    /// Mago gives the literal; the fields keep the literal's order, which is PHP's order of
    /// evaluation (P2R-202).
    pub(super) fn shape_literal(
        &mut self,
        array: &Array<'_>,
        expected: Option<&RustType>,
    ) -> Option<(RExpr, RustType)> {
        let mut elements = Vec::new();
        for element in array.elements.iter() {
            match element {
                ArrayElement::KeyValue(pair) => match literal_key(pair.key) {
                    Some(key) if elements.iter().any(|&(seen, _)| seen == key) => {
                        self.refuse_list(
                            array,
                            "a key appears twice in this array literal (P2R-202)",
                        );
                        return None;
                    }
                    Some(key) => elements.push((key, pair.value)),
                    None => {
                        self.refuse_list(array, NOT_A_SHAPE);
                        return None;
                    }
                },
                _ => {
                    self.refuse_list(array, NOT_A_SHAPE);
                    return None;
                }
            }
        }
        let name = match expected {
            Some(RustType::Shape(name)) => name.clone(),
            _ => match self.site_type(array) {
                Ok(RustType::Shape(name)) => name,
                Ok(_) => {
                    self.refuse_list(array, NOT_A_SHAPE);
                    return None;
                }
                Err(message) => {
                    self.refuse_list(array, message);
                    return None;
                }
            },
        };
        let fields = self.type_names.shape(&name).unwrap_or_default();
        let same_keys = fields.len() == elements.len()
            && elements
                .iter()
                .all(|&(key, _)| fields.iter().any(|field| field.key == key));
        if !same_keys {
            self.refuse_list(
                array,
                "the keys of this array literal are not the keys of the array shape it goes to (P2R-202)",
            );
            return None;
        }
        // Every value converts, so the problems in each are reported.
        let values: Vec<Option<(String, RExpr)>> = elements
            .into_iter()
            .map(|(key, value)| {
                let field = fields.iter().find(|field| field.key == key)?;
                let (expr, found) = self.value(value, Some(&field.ty))?;
                if nulls::null_for_plain(&found, &field.ty) {
                    self.refuse(
                        value,
                        "a value that may be null cannot go into a field that cannot be null (P2R-106)",
                    );
                    return None;
                }
                // PHP does not check a docblock shape, so the field keeps the int. Mago's
                // type shows an int that a `?:` arm turned into a float already.
                let int = *found.inner() == RustType::I64
                    || self.expression_type(value).is_some_and(|union| {
                        union
                            .types
                            .iter()
                            .any(|a| matches!(a, TAtomic::Scalar(TScalar::Integer(_))))
                    });
                if int && *field.ty.inner() == RustType::F64 {
                    self.refuse(
                        value,
                        "an int cannot go into a float field of an array shape, because PHP keeps the int (P2R-202)",
                    );
                    return None;
                }
                let expr = self.declared_value(value, expr, &found, &field.ty)?;
                let expr = self.stored(value, expr, &found, array.span())?;
                Some((field.name.clone(), nulls::wrapped(expr, &found, &field.ty)))
            })
            .collect();
        let values = values.into_iter().collect::<Option<Vec<_>>>()?;
        Some((RExpr::Struct(name.clone(), values), RustType::Shape(name)))
    }

    /// `$s['k']` on a value of the shape `name`, converted to `object`: the field read
    /// `s.k` (P2R-203).
    pub(super) fn shape_read(
        &mut self,
        access: &ArrayAccess<'_>,
        object: RExpr,
        name: &str,
    ) -> Option<(RExpr, RustType)> {
        // Rust does not accept a struct literal as the object of a field read everywhere,
        // such as in an `if` condition.
        if matches!(unparenthesized(access.array), Expression::Array(_)) {
            self.refuse(access, "an array literal cannot be read by key (P2R-203)");
            return None;
        }
        let Some(key) = literal_key(access.index) else {
            self.record_refused(access.index);
            self.refuse(
                access.index,
                "an array shape is read only with a string literal key (P2R-203)",
            );
            return None;
        };
        let fields = self.type_names.shape(name).unwrap_or_default();
        let Some(field) = fields.into_iter().find(|field| field.key == key) else {
            self.refuse(access, "this array shape has no such key (P2R-203)");
            return None;
        };
        Some((RExpr::Field(Box::new(object), field.name), field.ty))
    }
}
