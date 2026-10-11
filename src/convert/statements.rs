//! Statements inside `fn main()` and function bodies (P2R-40, P2R-46).

use mago_syntax::cst::*;

use super::{Context, Converter, statement_kind};
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
        match statement {
            Statement::Echo(echo) => self.echo(echo, depth, body),
            Statement::Return(ret) => self.return_statement(ret, depth, context, body),
            Statement::Expression(expression) => self.expression_statement(expression, depth, body),
            Statement::Noop(_) => {}
            Statement::Declare(_) => {
                self.refuse(
                    statement,
                    "declare(strict_types=1); must be the first statement (P2R-12)",
                );
            }
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
        let text = match (context, ret.value) {
            (Context::Main, _) => {
                self.refuse(ret, "return outside a function is not supported (P2R-07)");
                return;
            }
            (Context::Function(None), None) => "return;".to_owned(),
            (Context::Function(Some(_)), Some(value)) => {
                // Mago reports a value that does not match the declared return type (P2R-11).
                let Some((expr, _)) = self.expression(value) else {
                    return;
                };
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

    pub(super) fn expression_statement(
        &mut self,
        statement: &ExpressionStatement<'_>,
        depth: usize,
        body: &mut Vec<String>,
    ) {
        let Expression::Call(Call::Function(call)) = statement.expression else {
            self.refuse(
                statement,
                "this expression statement is not supported (P2R-43)",
            );
            return;
        };
        let Some((expr, _)) = self.call(call, statement.expression) else {
            return;
        };
        let Some(rendered) = self.render(statement.expression, &expr, false) else {
            return;
        };
        let line = self.emit(statement, depth, format!("{rendered};"));
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
            expr if matches!(ty, RustType::I64 | RustType::String) => {
                let arguments = format!("\"{{}}\", {}", self.render(value, &expr, true)?);
                if arguments.len() > printer::ARGS_WIDTH {
                    self.refuse(value, "this statement is too long for one line (P2R-51)");
                    return None;
                }
                Some(format!("print!({arguments});"))
            }
            _ => {
                self.refuse(value, "echo of a float or bool is not supported (P2R-46)");
                None
            }
        }
    }
}
