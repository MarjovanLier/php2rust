//! Changes of a property: `$x->p = v`, the compound operators, `++`, `--` and
//! `$x->p[] = v`, where `$x` is a chain of property and list reads from a variable
//! (P2R-142, P2R-148).
//!
//! PHP evaluates the value first, so it converts first: a variable it moves is then read
//! again by the target, and that read stops the move (P2R-145).

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::lists::unparenthesized;
use super::variables::{chain_root, operation};
use super::{Converter, nulls, numbers};
use crate::printer::RExpr;
use crate::types::RustType;

impl Converter<'_> {
    /// The Rust statement for a change whose target is a property or an append to a list
    /// property, or `None` when the target is neither. `Some(None)` means a problem is
    /// recorded.
    pub(super) fn property_change(
        &mut self,
        expression: &Expression<'_>,
        target: &Expression<'_>,
        operator: Option<&str>,
        value: Option<&Expression<'_>>,
    ) -> Option<Option<String>> {
        let (chain, append) = match unparenthesized(target) {
            Expression::Access(Access::Property(_)) => (target, false),
            Expression::ArrayAppend(append)
                if matches!(
                    unparenthesized(append.array),
                    Expression::Access(Access::Property(_))
                ) =>
            {
                (append.array, true)
            }
            _ => return None,
        };
        let problem = match (chain_shape(chain), operator) {
            (Err(message), _) => Some(message),
            (Ok(()), None) => {
                Some("this assignment operator is not supported in version 1 (P2R-07)")
            }
            (Ok(()), Some(op)) if append && op != "=" => {
                Some("only = can append to a list (P2R-47)")
            }
            _ => None,
        };
        if let Some(message) = problem {
            // The value's reads and the target's reads still count, so no false P2R-34 or
            // P2R-35 line follows (P2R-03).
            if let Some(value) = value {
                self.record_refused(value);
            }
            self.record_refused(chain);
            self.refuse(expression, message);
            return Some(None);
        }
        Some(self.converted_change(expression, chain, append, operator?, value))
    }

    fn converted_change(
        &mut self,
        expression: &Expression<'_>,
        chain: &Expression<'_>,
        append: bool,
        operator: &str,
        value: Option<&Expression<'_>>,
    ) -> Option<String> {
        let property = self.chain_type(chain);
        let expected = match (&property, append) {
            (Some(RustType::Vec(element)), true) => Some((**element).clone()),
            (_, true) => None,
            (property, false) => property.clone(),
        };
        // PHP evaluates the value first.
        let converted = match value {
            Some(value) if operator == ".=" => self.joined_string(value),
            Some(value) if operator == "=" => self.value(value, expected.as_ref()),
            Some(value) => self.expression(value),
            None if expected == Some(RustType::F64) => Some((RExpr::Float(1.0), RustType::F64)),
            None => Some((RExpr::Int(1), RustType::I64)),
        };
        let target = self.expression(chain);
        let ((expr, value_type), (target, target_type)) = (converted?, target?);
        if !self.change_root(expression, &target) {
            return None;
        }
        let statement = expression.span();
        if append {
            return self.appended(
                expression,
                value?,
                (expr, value_type),
                (target, target_type),
            );
        }
        let (operator, expr) = match operator {
            "=" => {
                let value = value?;
                if nulls::null_for_plain(&value_type, &target_type) {
                    self.refuse(
                        value,
                        "a value that may be null cannot be assigned to a property that is not nullable (P2R-106)",
                    );
                    return None;
                }
                let expr = self.declared_value(value, expr, &value_type, &target_type)?;
                let stored = self.stored(value, expr, &value_type, statement)?;
                if let (Some(variable), Some(name)) = (root_variable(chain), chain_root(&target)) {
                    self.variables.field_write(name, variable.span, statement);
                }
                ("=", nulls::wrapped(stored, &value_type, &target_type))
            }
            ".=" => {
                let problem = match &target_type {
                    RustType::String => None,
                    RustType::Option(_) => Some(nulls::NULL_JOINED),
                    _ => Some(".= needs a string property (P2R-45)"),
                };
                if let Some(message) = problem {
                    self.refuse(expression, message);
                    return None;
                }
                ("+=", self.borrowed(expr, &value_type))
            }
            other => {
                let op = operation(other)?;
                if let Some(message) = numbers::arithmetic_refusal(op, &target_type, &value_type) {
                    self.refuse(expression, message);
                    return None;
                }
                (other, expr)
            }
        };
        let target = self.render(expression, &target, false)?;
        let value = self.render(expression, &expr, false)?;
        Some(format!("{target} {operator} {value};"))
    }

    /// `$x->p[] = v` as `x.p.push(v);`, with the converted value and target.
    fn appended(
        &mut self,
        expression: &Expression<'_>,
        value: &Expression<'_>,
        (expr, value_type): (RExpr, RustType),
        (target, target_type): (RExpr, RustType),
    ) -> Option<String> {
        let RustType::Vec(element) = &target_type else {
            self.refuse(expression, "only a list can be appended to (P2R-47)");
            return None;
        };
        if nulls::null_for_plain(&value_type, element) {
            self.refuse(expression, nulls::NULL_ELEMENT);
            return None;
        }
        // P2R-25 would make the int a float, where PHP keeps it.
        if value_type == RustType::I64 && **element == RustType::F64 {
            self.refuse(
                expression,
                "an int appended to a list of float keeps its type in PHP, so it is not supported (P2R-142)",
            );
            return None;
        }
        let stored = self.stored(value, expr, &value_type, expression.span())?;
        let pushed = nulls::wrapped(stored, &value_type, element);
        let push = RExpr::MethodCall(Box::new(target), "push".to_owned(), vec![pushed]);
        Some(format!("{};", self.render(expression, &push, false)?))
    }

    /// The Rust type of a chain, from the types of its root and of the properties and
    /// list elements it reads, without recording a read.
    pub(super) fn chain_type(&self, chain: &Expression<'_>) -> Option<RustType> {
        match unparenthesized(chain) {
            Expression::Variable(Variable::Direct(variable)) => {
                let var = self.lookup(variable)?;
                self.variables.ty(var).cloned()
            }
            Expression::Access(Access::Property(access)) => {
                let RustType::Class(class) = self.chain_type(access.object)? else {
                    return None;
                };
                let ClassLikeMemberSelector::Identifier(name) = &access.property else {
                    return None;
                };
                self.property_type(&class, name.value)
            }
            Expression::ArrayAccess(access) => match self.chain_type(access.array)? {
                RustType::Vec(element) => Some(*element),
                _ => None,
            },
            _ => None,
        }
    }
}

/// The variable a chain starts from.
fn root_variable<'a, 'arena>(chain: &'a Expression<'arena>) -> Option<&'a DirectVariable<'arena>> {
    match unparenthesized(chain) {
        Expression::Variable(Variable::Direct(variable)) => Some(variable),
        Expression::Access(Access::Property(access)) => root_variable(access.object),
        Expression::ArrayAccess(access) => root_variable(access.array),
        _ => None,
    }
}

/// Checks that `chain` is a root variable followed by property reads and list reads
/// (P2R-142).
fn chain_shape(chain: &Expression<'_>) -> Result<(), &'static str> {
    match unparenthesized(chain) {
        Expression::Variable(Variable::Direct(_)) => Ok(()),
        Expression::Access(Access::Property(access)) => chain_shape(access.object),
        Expression::ArrayAccess(access) => chain_shape(access.array),
        Expression::Call(_) | Expression::Instantiation(_) => {
            Err("a change through a call result or a new expression is not supported (P2R-142)")
        }
        _ => Err(
            "a property can change only on a chain of property and list reads from a variable (P2R-142)",
        ),
    }
}
