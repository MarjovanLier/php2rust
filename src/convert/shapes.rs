//! Array shapes as structs: the struct of each shape, shape literals and element reads
//! (P2R-200 to P2R-203).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use std::collections::BTreeMap;

use super::control::emit;
use super::crates::{ItemKind, used_items};
use super::lists::unparenthesized;
use super::{Converter, nulls};
use crate::printer::{self, RExpr};
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

/// The shape structs and union enums of `types` that the items of `modules` or `main` name,
/// also through another kept type, in their order. Mago's type of a map variable after
/// writes by literal keys is a shape, which the tool meets but no item uses (P2R-240).
pub(super) fn named(
    types: Vec<(String, Vec<String>)>,
    modules: &[Vec<Vec<String>>],
    main: &[String],
) -> Vec<(String, Vec<String>)> {
    let mut text = modules
        .iter()
        .flatten()
        .flatten()
        .chain(main)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");
    let mut kept = vec![false; types.len()];
    loop {
        let open: BTreeMap<String, ItemKind> = types
            .iter()
            .zip(&kept)
            .filter(|(_, kept)| !**kept)
            .map(|((name, _), _)| (name.clone(), ItemKind::Type))
            .collect();
        let used = used_items(&text, &open);
        if used.is_empty() {
            break;
        }
        for (at, (name, lines)) in types.iter().enumerate() {
            if used.contains(name.as_str()) {
                kept[at] = true;
                text.push('\n');
                text.push_str(&lines.join("\n"));
            }
        }
    }
    types
        .into_iter()
        .zip(kept)
        .filter_map(|(item, kept)| kept.then_some(item))
        .collect()
}

impl Converter<'_> {
    /// The struct of each array shape the output uses, by name in byte order (P2R-200),
    /// followed by `impl From<S> for IndexMap<String, V>` for a shape that a place turns
    /// into a map, in the one key order of the literals that build it (P2R-271).
    pub(super) fn shape_items(&self) -> Vec<(String, Vec<String>)> {
        self.type_names
            .shapes()
            .into_iter()
            .map(|(name, fields)| {
                let mut lines = vec![
                    "#[derive(Clone, Debug)]".to_owned(),
                    format!("pub struct {name} {{"),
                ];
                for field in &fields {
                    lines.push(emit(
                        1,
                        format!("pub {}: {},", field.name, field.ty.owned()),
                    ));
                }
                lines.push("}".to_owned());
                if let Some(order) = self.map_order(&name) {
                    let value = fields
                        .first()
                        .map(|field| field.ty.owned())
                        .unwrap_or_default();
                    lines.push(String::new());
                    lines.push(format!(
                        "impl From<{name}> for IndexMap<String, {value}> {{"
                    ));
                    lines.push(emit(1, format!("fn from(shape: {name}) -> Self {{")));
                    lines.push(emit(2, "IndexMap::from([".to_owned()));
                    for key in order {
                        let field = fields.iter().find(|field| field.key == *key);
                        let text = printer::expr(
                            &RExpr::Str(String::from_utf8_lossy(key).into_owned()),
                            false,
                        );
                        let name = field.map(|field| field.name.as_str()).unwrap_or_default();
                        lines.push(emit(3, format!("(String::from({text}), shape.{name}),")));
                    }
                    lines.push(emit(2, "])".to_owned()));
                    lines.push(emit(1, "}".to_owned()));
                    lines.push("}".to_owned());
                }
                (name, lines)
            })
            .collect()
    }

    /// The one key order of the literals that build the shape struct `name`, when a place
    /// turns a value of it into a map (P2R-271).
    fn map_order(&self, name: &str) -> Option<&Vec<Vec<u8>>> {
        if !self.shape_maps.iter().any(|(_, shape)| shape == name) {
            return None;
        }
        match self.shape_orders.get(name) {
            Some(orders) if orders.len() == 1 => orders.first(),
            _ => None,
        }
    }

    /// Refuses each place that turns a shape into a map when the literals that build the
    /// shape give no key order or more than one (P2R-271). Runs once every body converted.
    pub(super) fn check_shape_maps(&mut self) {
        let refused: Vec<Span> = self
            .shape_maps
            .iter()
            .filter(|(_, shape)| self.shape_orders.get(shape).is_none_or(|o| o.len() != 1))
            .map(|&(span, _)| span)
            .collect();
        for span in refused {
            self.refuse(
                &span,
                "an array shape converts to a map only when every array literal that builds it lists its keys in one order, because PHP keeps the order of the literal (P2R-271)",
            );
        }
    }

    /// The value `expr` of the shape type `ty` where a map of text keys and `value` values
    /// is needed, as `IndexMap::from(s)` or `s.map(IndexMap::from)`, copied first when it
    /// is not owned (P2R-271).
    pub(super) fn shape_as_map(
        &mut self,
        node: &Expression<'_>,
        expr: RExpr,
        ty: &RustType,
        (key, value): (&RustType, &RustType),
    ) -> Option<(RExpr, RustType)> {
        let RustType::Shape(name) = ty.inner() else {
            return Some((expr, ty.clone()));
        };
        if *key != RustType::String {
            self.refuse(
                node,
                "an array shape converts only to a map with text keys (P2R-271)",
            );
            return None;
        }
        let fields = self.type_names.shape(name).unwrap_or_default();
        if fields.iter().any(|field| field.ty != *value) {
            self.refuse(
                node,
                "an array shape converts to a map only when every field has the map's value type (P2R-271)",
            );
            return None;
        }
        self.shape_maps.push((node.span(), name.clone()));
        // Owned before the wrap, as an assigned value is: the place's own copy step sees
        // only the call. The copy mark of a variable needs no statement span.
        let expr = self.stored(node, expr, ty, node.span())?;
        let map = RustType::Map(Box::new(RustType::String), Box::new(value.clone()));
        Some(if matches!(ty, RustType::Option(_)) {
            (
                RExpr::MethodCall(
                    Box::new(expr),
                    "map".into(),
                    vec![RExpr::Path("IndexMap::from".into())],
                ),
                RustType::Option(Box::new(map)),
            )
        } else {
            (RExpr::Call("IndexMap::from".into(), vec![expr]), map)
        })
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
        // PHP's key order of the shape value, which P2R-271 gives a map of it.
        self.shape_orders
            .entry(name.clone())
            .or_default()
            .insert(elements.iter().map(|&(key, _)| key.to_vec()).collect());
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
