//! Maps on `IndexMap`: literals, keys, reads, `isset`, `array_key_exists()` and the left
//! side of `??` (P2R-240 to P2R-242). An insert and an `unset` change their variable, so
//! they sit with the other changes in `variables.rs`; a foreach over a map is in
//! `foreach.rs`.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::lists::unparenthesized;
use super::{Converter, nulls};
use crate::printer::RExpr;
use crate::types::RustType;

/// The problem of a key PHP changes before it stores it (P2R-241).
const KEY_CHANGED: &str =
    "a map key must be an int or a string; PHP changes a float, bool or null key (P2R-241)";

/// The problem of `$m[] = v` on a map (P2R-242).
pub(super) const MAP_APPEND: &str = "$m[] = v on a map is not supported, because PHP's next key is one above the highest int key the map ever had (P2R-242)";

/// The problem of a write or `unset` of an element of a map property (P2R-242).
pub(super) const MAP_PROPERTY_CHANGE: &str =
    "a change to an element of a map property is not supported yet (P2R-242)";

/// The problem of an element write or `unset` on a map that may be null (P2R-242).
pub(super) const MAP_MAY_BE_NULL: &str =
    "an element of a map that may be null cannot change (P2R-106, P2R-242)";

/// The problem of `unset` of anything but a map element (P2R-242).
pub(super) const UNSET: &str =
    "unset converts only on an element of a map in a variable or parameter (P2R-242)";

/// The problem of `isset` and `array_key_exists()` on anything but a map (P2R-242).
const KEY_CHECK: &str = "isset and array_key_exists() convert only on a map in a variable, parameter or property (P2R-242)";

/// The problem of a list or shape where a map is needed, or the other way round (P2R-240).
pub(super) const MAP_CLASH: &str = "a list or array shape cannot go where a map is needed, nor a map where a list, an array shape or a map with the other key type is needed (P2R-240)";

/// True for a map in a variable, a parameter or a chain of property reads, which reads no
/// element and calls nothing on the way (P2R-242).
pub(super) fn plain_holder(expression: &Expression<'_>) -> bool {
    match unparenthesized(expression) {
        Expression::Variable(Variable::Direct(_)) => true,
        Expression::Access(Access::Property(access)) => plain_holder(access.object),
        _ => false,
    }
}

/// True where a list or shape value meets a map type, a map value a list or shape type, or
/// a map value a map type with the other key type, at any depth (P2R-240).
pub(super) fn map_clash(found: &RustType, declared: &RustType) -> bool {
    match (found, declared) {
        (RustType::Map(..), RustType::Vec(_) | RustType::Shape(_))
        | (RustType::Vec(_) | RustType::Shape(_), RustType::Map(..)) => true,
        (RustType::Map(found_key, found), RustType::Map(declared_key, declared)) => {
            found_key != declared_key || map_clash(found, declared)
        }
        (RustType::Option(found), declared) => map_clash(found, declared),
        (found, RustType::Option(declared)) => map_clash(found, declared),
        (RustType::Vec(found), RustType::Vec(declared)) => map_clash(found, declared),
        _ => false,
    }
}

/// An int literal, with or without unary minus.
fn int_literal(expr: &RExpr) -> Option<i64> {
    match expr {
        RExpr::Int(value) => Some(*value),
        RExpr::Neg(inner) => match **inner {
            RExpr::Int(value) => value.checked_neg(),
            _ => None,
        },
        _ => None,
    }
}

impl Converter<'_> {
    /// An array literal where the map type `IndexMap<key, value>` is needed, in source order
    /// (P2R-241).
    pub(super) fn map_literal(
        &mut self,
        array: &Array<'_>,
        key: &RustType,
        value: &RustType,
    ) -> Option<(RExpr, RustType)> {
        let mut elements = Vec::new();
        for element in array.elements.iter() {
            let ArrayElement::KeyValue(pair) = element else {
                self.refuse_list(
                    array,
                    "an array literal where a map is needed converts only with a key and a value on every element, never a plain value or a spread (P2R-241)",
                );
                return None;
            };
            elements.push((pair.key, pair.value));
        }
        // Every key and value converts, so the problems in each are reported.
        let statement = array.span();
        let pairs: Vec<Option<(RExpr, RExpr)>> = elements
            .into_iter()
            .map(|(k, v)| {
                let k = self.map_key(k, key, Some(statement));
                let v = self.map_value(v, value, statement);
                Some((k?, v?))
            })
            .collect();
        let pairs = pairs.into_iter().collect::<Option<Vec<_>>>()?;
        let ty = RustType::Map(Box::new(key.clone()), Box::new(value.clone()));
        Some((RExpr::MapLiteral(pairs), ty))
    }

    /// A value stored in a map of `wanted` values by `statement`: converted, copied and
    /// wrapped as a value assigned to a variable of that type (P2R-241, P2R-242).
    pub(super) fn map_value(
        &mut self,
        value: &Expression<'_>,
        wanted: &RustType,
        statement: Span,
    ) -> Option<RExpr> {
        let (expr, found) = self.value(value, Some(wanted))?;
        if nulls::null_for_plain(&found, wanted) {
            self.refuse(
                value,
                "a value that may be null cannot go into a map of values that cannot be null (P2R-106)",
            );
            return None;
        }
        // PHP does not check a docblock map, so the value keeps the int.
        if *found.inner() == RustType::I64 && *wanted.inner() == RustType::F64 {
            self.refuse(
                value,
                "an int cannot go into a map of float, because PHP keeps the int (P2R-241)",
            );
            return None;
        }
        let expr = self.declared_value(value, expr, &found, wanted)?;
        let expr = self.stored(value, expr, &found, statement)?;
        Some(nulls::wrapped(expr, &found, wanted))
    }

    /// The key `node` of a map keyed by `wanted`, owned for an insert by `owned`, the
    /// statement, and borrowed for a lookup otherwise. An int key of a map keyed by text
    /// becomes its decimal text, as PHP stores it (P2R-241, P2R-242).
    pub(super) fn map_key(
        &mut self,
        node: &Expression<'_>,
        wanted: &RustType,
        owned: Option<Span>,
    ) -> Option<RExpr> {
        if nulls::is_null(node) {
            self.refuse(node, KEY_CHANGED);
            return None;
        }
        let (expr, found) = self.expression(node)?;
        let problem = match (wanted, &found) {
            (_, RustType::F64 | RustType::Bool | RustType::Option(_)) => KEY_CHANGED,
            (_, RustType::Union(_)) => "a map key of a union type is not supported (P2R-241)",
            (RustType::I64, RustType::String) => {
                "a string key of a map with int keys is not supported, because PHP makes a decimal string an int (P2R-241)"
            }
            (RustType::I64, RustType::I64) => {
                return Some(match owned {
                    Some(_) => expr,
                    None => RExpr::Ref(Box::new(expr)),
                });
            }
            (RustType::String, RustType::String) => {
                return match owned {
                    Some(statement) => self.stored(node, expr, &found, statement),
                    None => Some(self.borrowed(expr, &found)),
                };
            }
            (RustType::String, RustType::I64) => {
                let text = match int_literal(&expr) {
                    Some(value) => RExpr::Str(value.to_string()),
                    None => RExpr::Method(Box::new(expr), "to_string"),
                };
                return Some(match (owned, text) {
                    (Some(_), RExpr::Str(text)) => {
                        RExpr::Call("String::from".into(), vec![RExpr::Str(text)])
                    }
                    (None, text @ RExpr::Str(_)) | (Some(_), text) => text,
                    (None, text) => RExpr::Ref(Box::new(text)),
                });
            }
            _ => "a map key must be an int or a string (P2R-241)",
        };
        self.refuse(node, problem);
        None
    }

    /// `$m[k]` on the map `object` keyed by `key`: `m[&k]`, never a position (P2R-242).
    pub(super) fn map_read(
        &mut self,
        access: &ArrayAccess<'_>,
        object: RExpr,
        key: &RustType,
        value: RustType,
    ) -> Option<(RExpr, RustType)> {
        let key = self.map_key(access.index, key, None)?;
        Some((RExpr::Key(Box::new(object), Box::new(key)), value))
    }

    /// The map and the borrowed key of `$m[k]` in a key check or a `??`, with the map's
    /// value type. `message` is the problem when `$m` is not a map in a variable,
    /// parameter or property (P2R-242).
    fn map_lookup(
        &mut self,
        access: &ArrayAccess<'_>,
        message: &str,
    ) -> Option<(RExpr, RExpr, RustType)> {
        if !plain_holder(access.array) {
            self.record_refused(access.array);
            self.record_refused(access.index);
            self.refuse(access, message.to_owned());
            return None;
        }
        let (map, ty) = self.expression(access.array)?;
        let (key, value) = match ty {
            RustType::Map(key, value) => (key, value),
            RustType::Option(_) => {
                self.record_refused(access.index);
                self.refuse(
                    access,
                    "a value that may be null cannot be read by key (P2R-106)",
                );
                return None;
            }
            _ => {
                self.record_refused(access.index);
                self.refuse(access, message.to_owned());
                return None;
            }
        };
        let key = self.map_key(access.index, &key, None)?;
        Some((map, key, *value))
    }

    /// `isset($m[k])`: `m.contains_key(&k)`, or for a map of `Option` values
    /// `m.get(&k).is_some_and(Option::is_some)`, because isset is false for null (P2R-242).
    pub(super) fn isset(
        &mut self,
        isset: &IssetConstruct<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let access = match isset.values.iter().collect::<Vec<_>>()[..] {
            [value] => match unparenthesized(value) {
                Expression::ArrayAccess(access) => Some(access),
                _ => None,
            },
            _ => None,
        };
        let Some(access) = access else {
            for value in isset.values.iter() {
                self.record_refused(value);
            }
            self.refuse(node, "isset converts only on one map element (P2R-242)");
            return None;
        };
        let (map, key, value) = self.map_lookup(access, KEY_CHECK)?;
        let check = if matches!(value, RustType::Option(_)) {
            let get = RExpr::MethodCall(Box::new(map), "get".to_owned(), vec![key]);
            let some = RExpr::Path("Option::is_some".to_owned());
            RExpr::MethodCall(Box::new(get), "is_some_and".to_owned(), vec![some])
        } else {
            RExpr::MethodCall(Box::new(map), "contains_key".to_owned(), vec![key])
        };
        Some((check, RustType::Bool))
    }

    /// `array_key_exists(k, $m)`: `m.contains_key(&k)`, true for a null value too
    /// (P2R-242).
    pub(super) fn array_key_exists(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let arguments = self.built_in_arguments(
            call,
            node,
            (2, 2),
            "array_key_exists() converts with two arguments only (P2R-242)",
        )?;
        let (key, map) = (arguments[0], arguments[1]);
        if !plain_holder(map) {
            self.record_refused(key);
            self.record_refused(map);
            self.refuse(map, KEY_CHECK);
            return None;
        }
        let (map_expr, ty) = self.expression(map)?;
        let RustType::Map(key_type, _) = ty else {
            self.record_refused(key);
            let message = match ty {
                RustType::Option(_) => nulls::NULL_BUILT_IN,
                _ => KEY_CHECK,
            };
            self.refuse(map, message);
            return None;
        };
        let key = self.map_key(key, &key_type, None)?;
        let check = RExpr::MethodCall(Box::new(map_expr), "contains_key".to_owned(), vec![key]);
        Some((check, RustType::Bool))
    }

    /// The left side of `??`: a map element as the owned option of its value,
    /// `m.get(&k).copied()` or `.cloned()`, flattened for a map of `Option` values
    /// (P2R-242); anything else as it converts.
    pub(super) fn coalesce_left(&mut self, left: &Expression<'_>) -> Option<(RExpr, RustType)> {
        let Expression::ArrayAccess(access) = unparenthesized(left) else {
            return self.expression(left);
        };
        if !matches!(self.site_type(access.array), Ok(RustType::Map(..))) {
            return self.expression(left);
        }
        let (map, key, value) = self.map_lookup(
            access,
            "?? on a map element converts only on a map in a variable, parameter or property (P2R-242)",
        )?;
        let get = RExpr::MethodCall(Box::new(map), "get".to_owned(), vec![key]);
        let owned = if value.is_copy() { "copied" } else { "cloned" };
        let option = RExpr::Method(Box::new(get), owned);
        Some(match value {
            RustType::Option(_) => (RExpr::Method(Box::new(option), "flatten"), value),
            other => (option, RustType::Option(Box::new(other))),
        })
    }
}
