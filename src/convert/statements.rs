//! Statements inside `fn main()` and function bodies (P2R-40, P2R-46). Block statements
//! are in control.rs.

use mago_syntax::cst::*;

use super::lists::unparenthesized;
use super::variables::changes_variable;
use super::{Context, Converter, nulls, statement_kind};
use crate::printer::{self, RExpr};
use crate::types::RustType;

impl Converter<'_> {
    pub(super) fn statement(
        &mut self,
        statement: &Statement<'_>,
        depth: usize,
        context: Context<'_>,
        body: &mut Vec<String>,
    ) {
        if self.unreachable(statement) {
            return;
        }
        match statement {
            Statement::Echo(echo) => self.echo(echo, depth, body),
            Statement::Return(ret) => {
                self.return_statement(ret, depth, context, body);
                // A return refused in `fn main()` does not end the path, so the lines after
                // it are not refused as well (P2R-03).
                if matches!(context, Context::Function(_)) {
                    self.leave("return");
                }
            }
            Statement::Expression(expression) if changes_variable(expression.expression) => {
                if !self.match_assignment(expression.expression, depth, body) {
                    self.assignment_statement(expression.expression, depth, body)
                }
            }
            Statement::Expression(expression) => {
                self.expression_statement(expression.expression, depth, body)
            }
            Statement::Noop(_) => {}
            Statement::If(r#if) => self.if_statement(r#if, depth, context, body),
            Statement::While(r#while) => self.while_statement(r#while, depth, context, body),
            Statement::For(r#for) => self.for_statement(r#for, depth, context, body),
            Statement::Break(r#break) => self.break_statement(r#break, depth, body),
            Statement::Continue(r#continue) => self.continue_statement(r#continue, depth, body),
            Statement::Foreach(foreach) => self.foreach_statement(foreach, depth, context, body),
            Statement::Declare(_) => {
                self.refuse(
                    statement,
                    "declare(strict_types=1); must be the first statement (P2R-12)",
                );
            }
            Statement::Enum(declaration) => self.refuse(
                &declaration.name,
                "an enum must be declared outside functions and blocks (P2R-110)",
            ),
            Statement::Class(declaration) => self.refuse(
                &declaration.name,
                "a class must be declared outside functions and blocks (P2R-130)",
            ),
            other => self.refuse(
                other,
                format!(
                    "{} is not supported in version 1 (P2R-07)",
                    statement_kind(other)
                ),
            ),
        }
    }

    pub(super) fn return_statement(
        &mut self,
        ret: &Return<'_>,
        depth: usize,
        context: Context<'_>,
        body: &mut Vec<String>,
    ) {
        if let (Context::Function(Some(returns)), Some(value)) = (context, ret.value)
            && let Expression::Match(node) = unparenthesized(value)
        {
            self.match_return(node, returns, depth, body);
            return;
        }
        let text = match (context, ret.value) {
            (Context::Main, _) => {
                self.refuse(ret, "return outside a function is not supported (P2R-07)");
                return;
            }
            (Context::Function(None), None) => "return;".to_owned(),
            (Context::Function(Some(returns)), Some(value)) => {
                // Mago reports a value that does not match the declared return type (P2R-11).
                let Some((expr, ty)) = self.value(value, Some(returns)) else {
                    return;
                };
                if nulls::null_for_plain(&ty, returns) {
                    self.refuse(
                        value,
                        "a value that may be null cannot be returned where the return type is not nullable (P2R-106)",
                    );
                    return;
                }
                let Some(expr) = self.declared_value(value, expr, &ty, returns) else {
                    return;
                };
                let expr = nulls::wrapped(self.owned(expr, &ty), &ty, returns);
                let Some(rendered) = self.render(value, &expr, false) else {
                    return;
                };
                format!("return {rendered};")
            }
            _ => {
                self.refuse(
                    ret,
                    "this return does not match the declared return type (P2R-23)",
                );
                return;
            }
        };
        let line = self.emit(ret, depth, text);
        body.push(line);
    }

    /// A function or method call as a statement, or as a `for` initialisation or step
    /// (P2R-43, P2R-135).
    pub(super) fn expression_statement(
        &mut self,
        expression: &Expression<'_>,
        depth: usize,
        body: &mut Vec<String>,
    ) {
        let converted = match expression {
            Expression::Call(Call::Function(call)) => self.call(call, expression),
            Expression::Call(Call::Method(call)) => self.method_call(call, expression),
            _ => {
                self.record_refused(expression);
                self.refuse(
                    expression,
                    "this expression statement is not supported (P2R-43)",
                );
                return;
            }
        };
        let Some((expr, _)) = converted else {
            return;
        };
        let Some(rendered) = self.render(expression, &expr, false) else {
            return;
        };
        let line = self.emit(expression, depth, format!("{rendered};"));
        body.push(line);
    }

    pub(super) fn echo(&mut self, echo: &Echo<'_>, depth: usize, body: &mut Vec<String>) {
        for &value in echo.values.iter() {
            if let Some(text) = self.print_statement(value) {
                let line = self.emit(value, depth, text);
                body.push(line);
            }
        }
    }

    /// One `print!` per echo argument, in source order (P2R-46).
    pub(super) fn print_statement(&mut self, value: &Expression<'_>) -> Option<String> {
        let (expr, ty) = self.expression(value)?;
        match expr {
            RExpr::Str(text) => Some(format!("print!({});", printer::format_text(&text))),
            // PHP and Rust print an int the same way, so a literal goes into the format string.
            RExpr::Int(value) => Some(format!("print!(\"{value}\");")),
            // A joined string prints with its own format text, not through `format!`.
            RExpr::Format(text, values) => match printer::format_arguments(&text, &values) {
                Ok(arguments) => Some(format!("print!({arguments});")),
                Err(_) => {
                    self.refuse(value, "this statement is too long for one line (P2R-51)");
                    None
                }
            },
            expr if matches!(ty, RustType::I64 | RustType::String) => {
                let arguments = format!("\"{{}}\", {}", self.render(value, &expr, true)?);
                if arguments.len() > printer::ARGS_WIDTH {
                    self.refuse(value, "this statement is too long for one line (P2R-51)");
                    return None;
                }
                Some(format!("print!({arguments});"))
            }
            _ if matches!(ty, RustType::Vec(_)) => {
                self.refuse(value, "echo of a list is not supported (P2R-46)");
                None
            }
            _ if matches!(ty, RustType::Class(_)) => {
                self.refuse(value, "echo of an object is not supported (P2R-136)");
                None
            }
            _ if matches!(ty, RustType::Option(_)) => {
                self.refuse(
                    value,
                    "echo of a value that may be null is not supported, because PHP prints null as an empty string (P2R-106)",
                );
                None
            }
            _ => {
                self.refuse(value, "echo of a float or bool is not supported (P2R-46)");
                None
            }
        }
    }
}
