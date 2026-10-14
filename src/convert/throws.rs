//! Exceptions as `Result`: the exception struct, which functions throw, `throw`, calls
//! that throw, and `try` with `catch` (P2R-210 to P2R-215).

use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use super::control::emit;
use super::lists::unparenthesized;
use super::{Context, Converter, Mode, ParameterSignature};
use crate::names;
use crate::printer::{self, RExpr, Raise};
use crate::types::{self, RustType};

/// PHP's built-in exception classes that convert, each with its parent (P2R-210).
const CLASSES: &[(&str, &str)] = &[
    ("Exception", ""),
    ("ErrorException", "Exception"),
    ("JsonException", "Exception"),
    ("LogicException", "Exception"),
    ("BadFunctionCallException", "LogicException"),
    ("BadMethodCallException", "BadFunctionCallException"),
    ("DomainException", "LogicException"),
    ("InvalidArgumentException", "LogicException"),
    ("LengthException", "LogicException"),
    ("OutOfRangeException", "LogicException"),
    ("RuntimeException", "Exception"),
    ("OutOfBoundsException", "RuntimeException"),
    ("OverflowException", "RuntimeException"),
    ("RangeException", "RuntimeException"),
    ("UnderflowException", "RuntimeException"),
    ("UnexpectedValueException", "RuntimeException"),
];

/// The class of P2R-210 that the resolved, lowercase name `key` names.
fn built_in(key: &[u8]) -> Option<&'static str> {
    CLASSES
        .iter()
        .map(|&(class, _)| class)
        .find(|class| class.as_bytes().eq_ignore_ascii_case(key))
}

/// True for `Exception` and `Throwable`, which catch every exception of P2R-210.
fn catches_all(key: &[u8]) -> bool {
    key == b"exception" || key == b"throwable"
}

/// The lines of the exception struct (P2R-210).
pub(super) fn exception_lines() -> Vec<String> {
    let mut parents: Vec<(&str, Vec<&str>)> = Vec::new();
    for &(class, parent) in CLASSES.iter().filter(|(_, parent)| !parent.is_empty()) {
        match parents.iter_mut().find(|(known, _)| *known == parent) {
            Some((_, children)) => children.push(class),
            None => parents.push((parent, vec![class])),
        }
    }
    let mut lines: Vec<String> = [
        "#[derive(Clone, Debug)]",
        "pub struct PhpException {",
        "    pub class: &'static str,",
        "    pub message: String,",
        "    pub code: i64,",
        "}",
        "",
        "impl PhpException {",
        "    pub fn new(class: &'static str, message: &str, code: i64) -> PhpException {",
        "        PhpException {",
        "            class,",
        "            message: message.to_string(),",
        "            code,",
        "        }",
        "    }",
        "",
        "    pub fn is_a(&self, class: &str) -> bool {",
        "        let mut current = self.class;",
        "        loop {",
        "            if current == class {",
        "                return true;",
        "            }",
        "            current = match current {",
    ]
    .iter()
    .map(|line| (*line).to_owned())
    .collect();
    for (parent, children) in parents {
        let pattern: Vec<String> = children.iter().map(|c| format!("\"{c}\"")).collect();
        lines.push(format!(
            "                {} => \"{parent}\",",
            pattern.join(" | ")
        ));
    }
    lines.extend(
        [
            "                _ => return false,",
            "            };",
            "        }",
            "    }",
            "",
            "    pub fn uncaught(self) -> ! {",
            "        let _ = std::io::Write::flush(&mut std::io::stdout());",
            "        eprintln!(\"PHP Fatal error:  Uncaught {}: {}\", self.class, self.message);",
            "        std::process::exit(255)",
            "    }",
            "}",
        ]
        .iter()
        .map(|line| (*line).to_owned()),
    );
    lines
}

/// The parameters of a built-in exception's constructor, as `new` of P2R-133 takes them
/// (P2R-212).
fn exception_parameters() -> [ParameterSignature; 2] {
    let parameter = |name: &str, ty: RustType, default: RExpr| ParameterSignature {
        php_name: name.as_bytes().to_vec(),
        rust_name: name.to_owned(),
        snake: name.to_owned(),
        default: Some((default, ty.clone())),
        ty,
        mode: Mode::Shared,
    };
    [
        parameter("message", RustType::String, RExpr::Str(String::new())),
        parameter("code", RustType::I64, RExpr::Int(0)),
    ]
}

/// The hints of a `catch`: one, or each member of a union.
fn hints<'h, 'a>(hint: &'h Hint<'a>, found: &mut Vec<&'h Hint<'a>>) {
    match hint {
        Hint::Union(union) => {
            hints(union.left, found);
            hints(union.right, found);
        }
        other => found.push(other),
    }
}

/// Finds a `throw`, or a call of a function or method that throws, outside the body of a
/// `try` with a catch-all clause (P2R-211).
struct Throwing;

struct Found<'x, 'c> {
    converter: &'x Converter<'c>,
    /// The class whose method holds the body, which `self::` and `static::` name.
    class: Option<usize>,
    throws: bool,
    /// The number of `try` bodies with a catch-all clause around the walk.
    protected: usize,
}

impl Found<'_, '_> {
    /// True when the other arms of `node` cover every value of its subject, so P2R-121
    /// leaves its `default` out: a bool with both `true` and `false`, or an enum with
    /// every case.
    fn default_dropped(&self, node: &Match<'_>) -> bool {
        let converter = self.converter;
        let subject = converter
            .expression_type(node.expression)
            .and_then(|union| types::rust_type(union, &converter.type_names).ok());
        let mut patterns = Vec::new();
        for arm in node.arms.iter() {
            let MatchArm::Expression(arm) = arm else {
                continue;
            };
            for &condition in arm.conditions.iter() {
                let pattern = match (unparenthesized(condition), &subject) {
                    (Expression::Literal(Literal::True(_)), Some(RustType::Bool)) => {
                        Some("true".to_owned())
                    }
                    (Expression::Literal(Literal::False(_)), Some(RustType::Bool)) => {
                        Some("false".to_owned())
                    }
                    (
                        Expression::Access(Access::ClassConstant(access)),
                        Some(RustType::Enum(name)),
                    ) => converter
                        .enum_case_of(access)
                        .filter(|(found, _)| found == name)
                        .map(|(found, case)| format!("{found}::{case}")),
                    _ => None,
                };
                patterns.extend(pattern);
            }
        }
        let has = |pattern: &str| patterns.iter().any(|p| p == pattern);
        match &subject {
            Some(RustType::Bool) => has("true") && has("false"),
            Some(RustType::Enum(name)) => converter.case_paths(name).iter().all(|p| has(p)),
            _ => false,
        }
    }

    fn method_throws(&self, class: Option<usize>, name: &[u8]) -> bool {
        class
            .and_then(|index| {
                let at = self.converter.method_index(index, name)?;
                Some(self.converter.classes[index].methods[at].signature.throws)
            })
            .unwrap_or(false)
    }
}

impl<'ast, 'arena> Walker<'ast, 'arena, Found<'_, '_>> for Throwing {
    fn walk_in_throw(&self, _: &'ast Throw<'arena>, found: &mut Found<'_, '_>) {
        found.throws |= found.protected == 0;
    }

    fn walk_in_function_call(&self, call: &'ast FunctionCall<'arena>, found: &mut Found<'_, '_>) {
        if found.protected == 0
            && let Expression::Identifier(name) = call.function
        {
            let key = found.converter.call_key(name);
            found.throws |= found
                .converter
                .functions
                .get(&key)
                .is_some_and(|s| s.throws);
        }
    }

    fn walk_in_method_call(&self, call: &'ast MethodCall<'arena>, found: &mut Found<'_, '_>) {
        if found.protected > 0 {
            return;
        }
        let ClassLikeMemberSelector::Identifier(name) = &call.method else {
            return;
        };
        let class = found
            .converter
            .expression_type(call.object)
            .and_then(|union| types::rust_type(union, &found.converter.type_names).ok());
        if let Some(RustType::Class(class) | RustType::Enum(class)) = class {
            let index = found.converter.class_named(&class);
            found.throws |= found.method_throws(index, name.value);
        }
    }

    fn walk_in_static_method_call(
        &self,
        call: &'ast StaticMethodCall<'arena>,
        found: &mut Found<'_, '_>,
    ) {
        if found.protected > 0 {
            return;
        }
        let ClassLikeMemberSelector::Identifier(name) = &call.method else {
            return;
        };
        let index = match call.class {
            Expression::Self_(_) | Expression::Static(_) => found.class,
            Expression::Identifier(identifier) => {
                let key = found.converter.identifier_key(identifier);
                found.converter.classes.iter().position(|c| c.key == key)
            }
            _ => None,
        };
        found.throws |= found.method_throws(index, name.value);
    }

    fn walk_try(&self, node: &'ast Try<'arena>, found: &mut Found<'_, '_>) {
        let converter = found.converter;
        let protects = node.catch_clauses.iter().any(|clause| {
            let mut hinted = Vec::new();
            hints(&clause.hint, &mut hinted);
            hinted.iter().any(|hint| match hint {
                Hint::Identifier(identifier) => catches_all(&converter.identifier_key(identifier)),
                _ => false,
            })
        });
        found.protected += usize::from(protects);
        self.walk_block(&node.block, found);
        found.protected -= usize::from(protects);
        for clause in node.catch_clauses.iter() {
            self.walk_block(&clause.block, found);
        }
        if let Some(finally) = &node.finally_clause {
            self.walk_block(&finally.block, found);
        }
    }

    /// A `default` arm that P2R-121 leaves out never runs, so it does not throw.
    fn walk_match(&self, node: &'ast Match<'arena>, found: &mut Found<'_, '_>) {
        self.walk_expression(node.expression, found);
        let dropped = found.default_dropped(node);
        for arm in node.arms.iter() {
            match arm {
                MatchArm::Default(_) if dropped => {}
                MatchArm::Default(arm) => self.walk_expression(arm.expression, found),
                MatchArm::Expression(arm) => {
                    for &condition in arm.conditions.iter() {
                        self.walk_expression(condition, found);
                    }
                    self.walk_expression(arm.expression, found);
                }
            }
        }
    }

    fn walk_function(&self, _: &'ast Function<'arena>, _: &mut Found<'_, '_>) {}

    fn walk_closure(&self, _: &'ast Closure<'arena>, _: &mut Found<'_, '_>) {}

    fn walk_arrow_function(&self, _: &'ast ArrowFunction<'arena>, _: &mut Found<'_, '_>) {}

    fn walk_class(&self, _: &'ast Class<'arena>, _: &mut Found<'_, '_>) {}
}

/// Finds a `break` or `continue` that leaves the statements it walks, outside the loops
/// inside them (P2R-214).
struct Leaving;

impl<'ast, 'arena> Walker<'ast, 'arena, bool> for Leaving {
    fn walk_in_break(&self, _: &'ast Break<'arena>, found: &mut bool) {
        *found = true;
    }

    fn walk_in_continue(&self, _: &'ast Continue<'arena>, found: &mut bool) {
        *found = true;
    }

    fn walk_while(&self, _: &'ast While<'arena>, _: &mut bool) {}

    fn walk_do_while(&self, _: &'ast DoWhile<'arena>, _: &mut bool) {}

    fn walk_for(&self, _: &'ast For<'arena>, _: &mut bool) {}

    fn walk_foreach(&self, _: &'ast Foreach<'arena>, _: &mut bool) {}

    fn walk_function(&self, _: &'ast Function<'arena>, _: &mut bool) {}

    fn walk_closure(&self, _: &'ast Closure<'arena>, _: &mut bool) {}
}

/// A `catch` clause, checked.
struct Clause<'c, 'arena> {
    /// The classes it tests; empty for a catch-all.
    classes: Vec<&'static str>,
    variable: Option<&'c DirectVariable<'arena>>,
    block: &'c Block<'arena>,
}

impl Converter<'_> {
    /// Decides which functions and methods throw, for the whole input, as a least fixed
    /// point: each pass marks those whose bodies throw with the marks of the pass before
    /// (P2R-211).
    pub(super) fn decide_throws(&mut self, program: &Program<'_>) {
        loop {
            let mut grew = false;
            for statement in program.statements.iter() {
                match statement {
                    Statement::Function(function) => {
                        let key = self.function_key(function);
                        let open = self.functions.get(&key).is_some_and(|s| !s.throws);
                        if open && self.body_throws(function.body.statements.iter(), None) {
                            if let Some(signature) = self.functions.get_mut(&key) {
                                signature.throws = true;
                            }
                            grew = true;
                        }
                    }
                    Statement::Class(declaration) => {
                        if let Some(index) = self.class_index(declaration) {
                            grew |= self.mark_methods(declaration.members.iter(), index);
                        }
                    }
                    Statement::Enum(declaration) => {
                        let key = self.name_key(&declaration.name, declaration.name.value);
                        if let Some(index) = self.classes.iter().position(|c| c.key == key) {
                            grew |= self.mark_methods(declaration.members.iter(), index);
                        }
                    }
                    _ => {}
                }
            }
            if !grew {
                break;
            }
        }
    }

    /// Marks each method of the class at `index` whose body throws; true when one was
    /// marked.
    fn mark_methods<'m, 'a: 'm>(
        &mut self,
        members: impl Iterator<Item = &'m ClassLikeMember<'a>>,
        index: usize,
    ) -> bool {
        let mut grew = false;
        for member in members {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            let MethodBody::Concrete(block) = &method.body else {
                continue;
            };
            let Some(at) = self.method_index(index, method.name.value) else {
                continue;
            };
            if self.classes[index].methods[at].signature.throws {
                continue;
            }
            // `self::Case` in a condition names the class of the method (P2R-153).
            let outer = self.current_class.replace(index);
            let throws = self.body_throws(block.statements.iter(), Some(index));
            self.current_class = outer;
            if throws {
                self.classes[index].methods[at].signature.throws = true;
                grew = true;
            }
        }
        grew
    }

    /// True when `statements` throw outside a `try` body with a catch-all clause (P2R-211).
    fn body_throws<'ast, 'arena: 'ast>(
        &self,
        statements: impl IntoIterator<Item = &'ast Statement<'arena>>,
        class: Option<usize>,
    ) -> bool {
        let mut found = Found {
            converter: self,
            class,
            throws: false,
            protected: 0,
        };
        for statement in statements {
            Throwing.walk_statement(statement, &mut found);
        }
        found.throws
    }

    /// Starts the body of a function or method, which `throws` or not; gives the state to
    /// hand back to [`Converter::end_body`] (P2R-211).
    pub(super) fn begin_body(&mut self, throws: bool) -> (bool, Option<Raise>) {
        let outer = (self.throwing, self.raise);
        self.throwing = throws;
        self.raise = throws.then_some(Raise::Propagate);
        // Its `Result` names the exception struct, even where no `throw` of it converts.
        self.uses_exceptions |= throws;
        outer
    }

    /// Ends that body: a `void` function that throws ends with `Ok(())` where a path
    /// reaches its end (P2R-211).
    pub(super) fn end_body(
        &mut self,
        void: bool,
        depth: usize,
        body: &mut Vec<String>,
        outer: (bool, Option<Raise>),
    ) {
        if self.throwing && void && self.variables.left.is_none() {
            body.push(emit(depth, "Ok(())".to_owned()));
        }
        (self.throwing, self.raise) = outer;
    }

    /// The Rust form of a call that throws, `call` when `throws` is false (P2R-213).
    pub(super) fn raised(
        &mut self,
        node: &impl HasSpan,
        call: RExpr,
        throws: bool,
    ) -> Option<RExpr> {
        if !throws {
            return Some(call);
        }
        match self.raise {
            Some(raise) => {
                if raise == Raise::Attempt {
                    self.path_to_handlers(false);
                }
                Some(RExpr::Raised(Box::new(call), raise))
            }
            None => {
                self.refuse(
                    node,
                    "a call that throws cannot stand in the default of ?? or the message of ?? throw, which Rust runs in a closure (P2R-213)",
                );
                None
            }
        }
    }

    /// Records a path from here to the handlers of the `try` whose body is converting; the
    /// body goes on in a new block unless it `leaves` here (P2R-214).
    fn path_to_handlers(&mut self, leaves: bool) {
        let current = self.variables.flow.current();
        let Some(points) = &mut self.attempt else {
            return;
        };
        points.push(current);
        if !leaves {
            let next = self.variables.flow.block();
            self.variables.flow.edge(current, next);
            self.variables.flow.enter(next);
        }
    }

    /// The exception a `throw` throws: a new built-in exception, or the variable of the
    /// `catch` it stands in (P2R-212).
    fn thrown(&mut self, throw: &Throw<'_>) -> Option<RExpr> {
        match unparenthesized(throw.exception) {
            Expression::Instantiation(new) => {
                let class = match new.class {
                    Expression::Identifier(identifier) => {
                        built_in(&self.identifier_key(identifier))
                    }
                    _ => None,
                };
                let Some(class) = class else {
                    self.record_refused(throw.exception);
                    self.refuse(
                        throw.exception,
                        "a throw converts only for new of one of PHP's built-in exception classes (P2R-212)",
                    );
                    return None;
                };
                let parameters = exception_parameters();
                let extra = new.argument_list.as_ref().is_some_and(|list| {
                    list.arguments.len() > 2
                        || list.arguments.iter().any(|argument| match argument {
                            Argument::Named(named) => {
                                !matches!(named.name.value, b"message" | b"code")
                            }
                            Argument::Positional(_) => false,
                        })
                });
                if extra {
                    self.record_refused(throw.exception);
                    self.refuse(
                        throw.exception,
                        "a throw converts only with the arguments message and code (P2R-212)",
                    );
                    return None;
                }
                let mut arguments = match &new.argument_list {
                    Some(list) => self.arguments(list, &parameters, throw.exception)?,
                    None => vec![RExpr::Str(String::new()), RExpr::Int(0)],
                };
                arguments.insert(0, RExpr::Str(class.to_owned()));
                self.uses_exceptions = true;
                Some(RExpr::Call("PhpException::new".into(), arguments))
            }
            Expression::Variable(Variable::Direct(variable)) => {
                match self.caught_name(variable.name) {
                    Some(name) => Some(RExpr::Var(name)),
                    None => {
                        self.record_refused(throw.exception);
                        self.refuse(
                            throw.exception,
                            "a throw of a variable converts only for the variable of its catch (P2R-212)",
                        );
                        None
                    }
                }
            }
            _ => {
                self.record_refused(throw.exception);
                self.refuse(
                    throw.exception,
                    "a throw converts only for new of one of PHP's built-in exception classes and for the variable of its catch (P2R-212)",
                );
                None
            }
        }
    }

    /// The Rust text of a `throw`, without a `;`, which leaves: `return Err(..)`,
    /// `break 'throw ..` or `..uncaught()` (P2R-212).
    pub(super) fn throw_text(&mut self, throw: &Throw<'_>) -> Option<String> {
        let exception = self.thrown(throw)?;
        let text = printer::expr(&exception, false);
        match self.raise {
            Some(Raise::Propagate) => Some(format!("return Err({text})")),
            Some(Raise::Attempt) => {
                self.path_to_handlers(true);
                Some(format!("break 'throw {text}"))
            }
            Some(Raise::Abort) => Some(format!("{text}.uncaught()")),
            None => {
                self.refuse(
                    throw,
                    "a throw cannot stand in the default of ?? or the message of ?? throw, which Rust runs in a closure (P2R-213)",
                );
                None
            }
        }
    }

    /// The new exception of `$a ?? throw new C(..)`, for `ok_or_else` (P2R-215); `None`
    /// when a problem is recorded.
    pub(super) fn thrown_lazily(&mut self, throw: &Throw<'_>) -> Option<RExpr> {
        if self.raise != Some(Raise::Propagate) {
            self.record_refused(throw.exception);
            self.refuse(
                throw,
                "?? throw converts only in a function that throws, outside a try body (P2R-215)",
            );
            return None;
        }
        // The message runs in the closure of `ok_or_else`.
        let outer = self.raise.take();
        let exception = self.thrown(throw);
        self.raise = outer;
        exception
    }

    /// The Rust name of the variable of an open `catch` called `php` (with its `$`), and
    /// marks it read.
    pub(super) fn caught_name(&mut self, php: &[u8]) -> Option<String> {
        let found = self
            .caught
            .iter_mut()
            .rev()
            .find(|(name, ..)| *name == php)?;
        found.2 = true;
        Some(found.1.clone())
    }

    /// `$e->getMessage()` and `$e->getCode()` on the variable of an open `catch`
    /// (P2R-214); `None` when the object is no such variable.
    pub(super) fn caught_call(
        &mut self,
        call: &MethodCall<'_>,
        node: &Expression<'_>,
    ) -> Option<Option<(RExpr, Option<RustType>)>> {
        let Expression::Variable(Variable::Direct(variable)) = unparenthesized(call.object) else {
            return None;
        };
        let name = self.caught_name(variable.name)?;
        let field = match &call.method {
            ClassLikeMemberSelector::Identifier(method)
                if call.argument_list.arguments.is_empty() =>
            {
                match method.value.to_ascii_lowercase().as_slice() {
                    b"getmessage" => Some(("message", RustType::String)),
                    b"getcode" => Some(("code", RustType::I64)),
                    _ => None,
                }
            }
            _ => None,
        };
        let Some((field, ty)) = field else {
            self.refuse(
                node,
                "a caught exception converts only in getMessage(), getCode() and throw (P2R-214)",
            );
            return Some(None);
        };
        Some(Some((
            RExpr::Field(Box::new(RExpr::Var(name)), field.to_owned()),
            Some(ty),
        )))
    }

    /// `try { .. } catch (C $e) { .. }` as
    /// `'attempt: { let e = 'throw: { ..; break 'attempt; }; .. }`, or as
    /// `let e = 'throw: { .. };` and the handlers when no path reaches the end of the body.
    /// A `throw` in the body leaves `'throw` with its exception, so the handlers run only
    /// for a throw and the end of the body skips them, for rustc as in PHP (P2R-214).
    pub(super) fn try_statement(
        &mut self,
        node: &Try<'_>,
        depth: usize,
        context: Context<'_>,
        body: &mut Vec<String>,
    ) {
        let Some(clauses) = self.clauses(node) else {
            self.forget_try(node);
            return;
        };
        self.trying += 1;
        let slot = self.slot(depth, body);
        let before = self.variables.values();
        let start = self.variables.flow.current();
        let handler = self.variables.flow.block();
        let after = self.variables.flow.block();
        let block = self.variables.flow.block();
        self.variables.flow.edge(start, block);
        self.variables.flow.enter(block);
        // The opening lines wait until the body shows whether a path reaches its end.
        let opening = body.len();
        body.push(String::new());
        body.push(String::new());
        let scope = self.variables.open(slot, false);
        let outer_raise = self.raise.replace(Raise::Attempt);
        let outer_attempt = self.attempt.replace(Vec::new());
        for statement in node.block.statements.iter() {
            self.statement(statement, depth + 2, context, body);
        }
        let reaches_end = self.variables.left.is_none();
        let exits = std::mem::replace(&mut self.attempt, outer_attempt).unwrap_or_default();
        self.variables.close(scope);
        self.raise = outer_raise;
        // A `default` arm left out holds no throw that runs (P2R-121).
        if exits.is_empty() {
            self.refuse(
                node,
                "this try body throws nothing, so no catch of it ever runs (P2R-214)",
            );
        }
        let mut arriving = Vec::new();
        if reaches_end {
            body.push(emit(depth + 2, "break 'attempt;".to_owned()));
            let end = self.variables.flow.current();
            self.variables.flow.edge(end, after);
            arriving.push(self.variables.values());
        }
        for exit in exits {
            self.variables.flow.edge(exit, handler);
        }
        // Without `'attempt` the handlers sit one level higher; rustfmt lays out the body.
        let level = usize::from(reaches_end);
        body.push(emit(depth + level, "};".to_owned()));
        // A handler starts with the values before the body, less those the body changes.
        self.variables.set_values(&before);
        for statement in node.block.statements.iter() {
            self.forget_writes(statement);
        }
        let entry = self.variables.values();
        let variable = clauses.iter().find_map(|clause| clause.variable);
        let php = variable.map(|variable| variable.name.to_vec());
        let name = variable
            .and_then(|variable| names::rust_name(&String::from_utf8_lossy(&variable.name[1..])));
        let tests = !clauses[0].classes.is_empty();
        let binding = name.clone().unwrap_or_else(|| "_".to_owned());
        let mut read = false;
        let inner = depth + level + usize::from(tests);
        for (index, clause) in clauses.iter().enumerate() {
            if tests {
                let keyword = if index == 0 { "if" } else { "} else if" };
                let line = if clause.classes.is_empty() {
                    "} else {".to_owned()
                } else {
                    let test: Vec<String> = clause
                        .classes
                        .iter()
                        .map(|class| format!("{binding}.is_a(\"{class}\")"))
                        .collect();
                    format!("{keyword} {} {{", test.join(" || "))
                };
                body.push(emit(depth + level, line));
            }
            self.variables.set_values(&entry);
            self.variables.left = None;
            let clause_block = self.variables.flow.block();
            self.variables.flow.edge(handler, clause_block);
            self.variables.flow.enter(clause_block);
            let clause_scope = self.variables.open(slot, false);
            if let (Some(php), Some(name)) = (&php, &name) {
                self.caught.push((php.clone(), name.clone(), false));
            }
            for statement in clause.block.statements.iter() {
                self.statement(statement, inner, context, body);
            }
            if php.is_some() && name.is_some() {
                read |= self.caught.pop().is_some_and(|(_, _, read)| read);
            }
            self.variables.close(clause_scope);
            if self.variables.left.is_none() {
                let end = self.variables.flow.current();
                self.variables.flow.edge(end, after);
                arriving.push(self.variables.values());
            }
        }
        if tests {
            if clauses
                .last()
                .is_some_and(|clause| !clause.classes.is_empty())
            {
                // No clause catches it, so it goes on as PHP throws it on (P2R-211).
                body.push(emit(depth + level, "} else {".to_owned()));
                let rethrow = match self.raise {
                    Some(Raise::Abort) => format!("{binding}.uncaught();"),
                    _ => format!("return Err({binding});"),
                };
                body.push(emit(inner, rethrow));
            }
            body.push(emit(depth + level, "}".to_owned()));
        }
        let binding = if tests || read {
            binding
        } else {
            "_".to_owned()
        };
        let throw = format!("let {binding} = 'throw: {{");
        if reaches_end {
            body[opening] = emit(depth, "'attempt: {".to_owned());
            body[opening + 1] = emit(depth + 1, throw);
            body.push(emit(depth, "}".to_owned()));
        } else {
            body[opening] = emit(depth, throw);
            body.remove(opening + 1);
        }
        self.trying -= 1;
        self.variables.flow.enter(after);
        self.join(arriving, "a try whose body and catches all leave");
    }

    /// The `catch` clauses of `node`, checked, with a problem recorded for each fault;
    /// `None` when there is none (P2R-214).
    fn clauses<'c, 'a>(&mut self, node: &'c Try<'a>) -> Option<Vec<Clause<'c, 'a>>> {
        let mut fine = true;
        if let Some(finally) = &node.finally_clause {
            self.refuse(finally, "finally is not supported (P2R-214)");
            fine = false;
        }
        // Its labels would shadow those of the outer `try`, which rustc warns about.
        if self.trying > 0 {
            self.refuse(
                node,
                "a try inside another try, its catch bodies included, is not supported (P2R-214)",
            );
            fine = false;
        }
        let mut leaves = false;
        for statement in node.block.statements.iter() {
            Leaving.walk_statement(statement, &mut leaves);
        }
        if leaves {
            self.refuse(
                node,
                "a break or continue cannot leave the body of a try, which Rust holds in a labelled block (P2R-214)",
            );
            fine = false;
        }
        let mut clauses: Vec<Clause<'c, 'a>> = Vec::new();
        for clause in node.catch_clauses.iter() {
            if clauses
                .last()
                .is_some_and(|c: &Clause<'_, '_>| c.classes.is_empty())
            {
                self.refuse(
                    clause,
                    "a catch after a catch of Exception or Throwable never runs (P2R-214)",
                );
                fine = false;
                continue;
            }
            let mut found = Vec::new();
            hints(&clause.hint, &mut found);
            let mut classes = Vec::new();
            let mut all = false;
            for hint in found {
                let key = match hint {
                    Hint::Identifier(identifier) => Some(self.identifier_key(identifier)),
                    _ => None,
                };
                match key {
                    Some(key) if catches_all(&key) => all = true,
                    Some(key) => match built_in(&key) {
                        Some(class) => classes.push(class),
                        None => {
                            self.refuse(
                                hint,
                                "a catch converts only for PHP's built-in exception classes, Exception and Throwable (P2R-214)",
                            );
                            fine = false;
                        }
                    },
                    None => {
                        self.refuse(hint, "a catch needs a class name (P2R-214)");
                        fine = false;
                    }
                }
            }
            // A clause whose classes were all refused still tests, so it is no catch-all.
            if classes.is_empty() && !all {
                classes.push("Exception");
            }
            if all {
                classes.clear();
            }
            if let Some(variable) = &clause.variable
                && self.lookup(variable).is_some()
            {
                self.refuse(
                    variable,
                    "the variable of a catch must be one the function does not use already, because PHP overwrites it (P2R-214)",
                );
                fine = false;
            }
            clauses.push(Clause {
                classes,
                variable: clause.variable.as_ref(),
                block: &clause.block,
            });
        }
        let mut variables = clauses.iter().filter_map(|clause| clause.variable);
        if let Some(first) = variables.next()
            && variables.any(|other| other.name != first.name)
        {
            self.refuse(
                node,
                "the catches of one try must use one variable name (P2R-214)",
            );
            fine = false;
        }
        let tests = clauses
            .first()
            .is_some_and(|clause| !clause.classes.is_empty());
        if fine && tests && clauses.iter().all(|clause| clause.variable.is_none()) {
            self.refuse(
                node,
                "a catch of a class other than Exception or Throwable needs a variable (P2R-214)",
            );
            fine = false;
        }
        if fine
            && let Some(variable) = clauses.iter().find_map(|clause| clause.variable)
            && names::rust_name(&String::from_utf8_lossy(&variable.name[1..])).is_none()
        {
            self.refuse(variable, "this variable name has no Rust form (P2R-30)");
        }
        // A `try` with a problem still converts, so the paths through it stay right and no
        // problem follows from it elsewhere; its output is never written.
        (!clauses.is_empty()).then_some(clauses)
    }

    /// Records the variables of a refused `try`, so its reads and writes are not reported
    /// as well.
    fn forget_try(&mut self, node: &Try<'_>) {
        for statement in node.block.statements.iter() {
            self.forget_writes(statement);
        }
    }
}
