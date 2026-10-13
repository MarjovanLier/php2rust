//! The forms of `self` and of the parameters of a changing type, decided for every method
//! and function of the file together (P2R-143), and the foreach loops that iterate by
//! `&mut` or over a copy (P2R-146).
//!
//! Each pass walks every body with the forms the passes before it gave and only ever
//! turns a form up: `&self` to `&mut self`, and `&` to `&mut` for a change or to owned for
//! a move. The passes stop when nothing turns up. That is the least fixed point: two
//! methods that call each other and change nothing stay `&self`.
//!
//! A receiver's class comes from the types of the chain's root and of the properties it
//! reads, because Mago gives no type to some of those reads. When the class is not known,
//! every class with a method of that name counts: a form turned up too far only adds a
//! `&mut` or a stop, where one left down would give Rust that rustc rejects.

use std::collections::{HashMap, HashSet};

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use super::enums::class_key;
use super::lists::unparenthesized;
use super::{Converter, Mode, ParameterSignature};
use crate::types::{self, RustType};

/// The foreach loops whose form the forms of the file decide (P2R-146), by the span of the
/// foreach statement.
#[derive(Default)]
pub(super) struct Loops {
    /// Over a list of a changing type, whose body changes the value: `&mut`.
    pub(super) by_mut: HashSet<Span>,
    /// Over a list property of Copy values, whose body changes the object that holds the
    /// list: a copy of the list.
    pub(super) copied: HashSet<Span>,
    /// The statements that name an object they change more than once, and the uses of an
    /// iterated object in a loop body, that rustc rejects (P2R-146, P2R-147).
    pub(super) problems: Vec<(Span, &'static str)>,
}

/// The problem of an object a statement changes and names again (P2R-147).
const REPEAT: &str = "this statement changes an object and names it more than once, which Rust does not allow here (P2R-147)";

/// The problem of a use of the object that holds an iterated list (P2R-146).
const ITERATED: &str = "inside a foreach over a list of objects whose class changes them, the object that holds the list can be used only through its other properties (P2R-146)";

/// The PHP names without `$`, the types and the forms of a body's parameters.
type Forms = Vec<(Vec<u8>, RustType, Mode)>;

/// Where a chain starts.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Root {
    This,
    Parameter(usize),
    /// The value of the open foreach at this depth.
    Value(usize),
    Local,
}

/// A foreach whose body is being walked.
struct Open<'ast, 'arena> {
    /// The value's PHP name with `$`.
    value: Vec<u8>,
    list: &'ast Expression<'arena>,
    span: Span,
    /// The roots that its body changes.
    changed: Vec<Root>,
}

/// One body being walked.
struct Body<'ast, 'arena, 'x, 'c> {
    converter: &'x Converter<'c>,
    /// The class of a method body.
    class: Option<usize>,
    parameters: Forms,
    self_mode: Mode,
    open: Vec<Open<'ast, 'arena>>,
    /// The type Mago gives each local variable at one of its reads or writes, by PHP name
    /// with `$`.
    locals: HashMap<Vec<u8>, RustType>,
    loops: Loops,
    /// Each change, `&mut` argument and move: the PHP name of the variable it starts from,
    /// and where it is (P2R-147).
    events: Vec<(Vec<u8>, Span)>,
}

impl Converter<'_> {
    /// Decides the forms of every method and function of `program`, and the foreach loops
    /// they decide (P2R-143, P2R-146).
    pub(super) fn decide_modes(&mut self, program: &Program<'_>) {
        loop {
            let mut grew = false;
            let mut loops = Loops::default();
            for statement in program.statements.iter() {
                match statement {
                    Statement::Function(function) => {
                        let key = function.name.value.to_ascii_lowercase();
                        let Some(signature) = self.functions.get(&key) else {
                            continue;
                        };
                        let parameters = forms(&signature.parameters);
                        let statements = function.body.statements.iter();
                        let (found, _) =
                            self.walk_body(None, parameters, Mode::Shared, statements, &mut loops);
                        if let Some(signature) = self.functions.get_mut(&key) {
                            grew |= turn_up(&mut signature.parameters, &found);
                        }
                    }
                    Statement::Class(declaration) => {
                        let Some(class) = self.class_index(declaration) else {
                            continue;
                        };
                        for member in declaration.members.iter() {
                            let ClassLikeMember::Method(method) = member else {
                                continue;
                            };
                            let (Some(at), MethodBody::Concrete(block)) =
                                (self.method_index(class, method.name.value), &method.body)
                            else {
                                continue;
                            };
                            let info = &self.classes[class].methods[at];
                            let parameters = forms(&info.signature.parameters);
                            let statements = block.statements.iter();
                            let (found, self_mode) = self.walk_body(
                                Some(class),
                                parameters,
                                info.self_mode,
                                statements,
                                &mut loops,
                            );
                            let info = &mut self.classes[class].methods[at];
                            grew |= turn_up(&mut info.signature.parameters, &found);
                            if self_mode != info.self_mode {
                                info.self_mode = self_mode;
                                grew = true;
                            }
                        }
                    }
                    Statement::Enum(_) => {}
                    // A statement of `fn main()`, which decides only its loops.
                    other => {
                        let statements = std::slice::from_ref(other);
                        self.walk_body(None, Vec::new(), Mode::Shared, statements, &mut loops);
                    }
                }
            }
            if !grew {
                self.loops = loops;
                return;
            }
        }
    }

    /// Walks one body with the current forms. Gives the forms of its parameters and of
    /// `self` that its own changes, calls and moves turn up; the loops it decides go into
    /// `loops`.
    fn walk_body<'ast, 'arena: 'ast>(
        &self,
        class: Option<usize>,
        parameters: Forms,
        self_mode: Mode,
        statements: impl IntoIterator<Item = &'ast Statement<'arena>> + std::clone::Clone,
        loops: &mut Loops,
    ) -> (Forms, Mode) {
        let mut locals = Locals {
            converter: self,
            found: HashMap::new(),
        };
        for statement in statements.clone() {
            LocalTypes.walk_statement(statement, &mut locals);
        }
        let mut body = Body {
            converter: self,
            class,
            parameters,
            self_mode,
            open: Vec::new(),
            locals: locals.found,
            loops: Loops::default(),
            events: Vec::new(),
        };
        for statement in statements {
            Scan.walk_statement(statement, &mut body);
        }
        loops.by_mut.extend(body.loops.by_mut);
        loops.copied.extend(body.loops.copied);
        loops.problems.extend(body.loops.problems);
        (body.parameters, body.self_mode)
    }
}

fn forms(parameters: &[ParameterSignature]) -> Forms {
    parameters
        .iter()
        .map(|p| (p.php_name.clone(), p.ty.clone(), p.mode))
        .collect()
}

/// Gives each parameter the form `found` has for it. True when one changed.
fn turn_up(parameters: &mut [ParameterSignature], found: &Forms) -> bool {
    let mut grew = false;
    for (parameter, (_, _, mode)) in parameters.iter_mut().zip(found) {
        if parameter.mode != *mode {
            parameter.mode = *mode;
            grew = true;
        }
    }
    grew
}

/// The forms of each parameter position over several callees.
fn positions<'p>(lists: impl Iterator<Item = &'p [ParameterSignature]>) -> Vec<Vec<Mode>> {
    let mut positions: Vec<Vec<Mode>> = Vec::new();
    for list in lists {
        for (position, parameter) in list.iter().enumerate() {
            if positions.len() <= position {
                positions.push(Vec::new());
            }
            positions[position].push(parameter.mode);
        }
    }
    positions
}

impl<'ast, 'arena> Body<'ast, 'arena, '_, '_> {
    /// Where `chain` starts, seen from inside the first `depth` open loops. `None` for an
    /// expression that is no chain of property and list reads from a variable.
    fn root(&self, chain: &Expression<'_>, depth: usize) -> Option<Root> {
        match unparenthesized(chain) {
            Expression::Variable(Variable::Direct(variable)) => {
                Some(self.root_named(variable.name, depth))
            }
            Expression::Access(Access::Property(access)) => self.root(access.object, depth),
            Expression::ArrayAccess(access) => self.root(access.array, depth),
            _ => None,
        }
    }

    /// A change of the object that `chain` starts from, at the current depth.
    fn change_chain(&mut self, chain: &Expression<'_>) {
        if let Some((root, _)) = chain_parts(chain) {
            self.events.push((root.to_vec(), chain.span()));
        }
        let depth = self.open.len();
        let root = self.root(chain, depth);
        self.change(root, depth);
    }

    /// A change of the object at `root`, inside the first `depth` open loops.
    fn change(&mut self, root: Option<Root>, depth: usize) {
        let Some(root) = root else {
            return;
        };
        for open in &mut self.open[..depth] {
            open.changed.push(root);
        }
        match root {
            Root::This => self.self_mode = Mode::Mut,
            Root::Parameter(at) => {
                if self.parameters[at].2 == Mode::Shared
                    && self.converter.is_changing(&self.parameters[at].1)
                {
                    self.parameters[at].2 = Mode::Mut;
                }
            }
            // A change of the value is a change of the list it is in.
            Root::Value(at) => {
                self.loops.by_mut.insert(self.open[at].span);
                let outer = self.root(self.open[at].list, at);
                self.change(outer, at);
            }
            Root::Local => {}
        }
    }

    /// A value that moves where it goes: a parameter of a changing type read alone there
    /// becomes owned (P2R-145).
    fn moved(&mut self, value: &Expression<'_>) {
        match unparenthesized(value) {
            Expression::Variable(Variable::Direct(variable)) => {
                if self
                    .type_of(value)
                    .is_some_and(|ty| self.converter.is_changing(&ty))
                {
                    self.events.push((variable.name.to_vec(), value.span()));
                }
                if let Some(Root::Parameter(at)) = self.root(value, self.open.len())
                    && self.converter.is_changing(&self.parameters[at].1)
                {
                    self.parameters[at].2 = Mode::Owned;
                }
            }
            Expression::Match(node) => {
                for arm in node.arms.iter() {
                    self.moved(arm.expression());
                }
            }
            _ => {}
        }
    }

    /// The arguments of a call whose parameter forms at each position are `modes`.
    fn arguments(&mut self, list: &ArgumentList<'_>, modes: &[Vec<Mode>]) {
        for (position, argument) in list.arguments.iter().enumerate() {
            let Argument::Positional(positional) = argument else {
                continue;
            };
            let modes = modes.get(position).map(Vec::as_slice).unwrap_or_default();
            if modes.contains(&Mode::Owned) {
                self.moved(positional.value);
            }
            if modes.contains(&Mode::Mut) {
                self.change_chain(positional.value);
            }
        }
    }

    /// The type of the value `expression` gives, from the chain's root and the
    /// declarations it reads.
    fn type_of(&self, expression: &Expression<'_>) -> Option<RustType> {
        self.type_at(expression, self.open.len())
    }

    /// `type_of`, seen from inside the first `depth` open loops. A loop's list is outside
    /// the loop, so it is typed from the loops around it: `foreach ($x->items as $x)`
    /// reads the outer `$x`.
    fn type_at(&self, expression: &Expression<'_>, depth: usize) -> Option<RustType> {
        let converter = self.converter;
        match unparenthesized(expression) {
            Expression::Variable(Variable::Direct(variable)) => {
                match self.root_named(variable.name, depth) {
                    Root::This => {
                        let class = &converter.classes[self.class?];
                        Some(RustType::Class(class.rust_name.clone()))
                    }
                    Root::Value(at) => match self.type_at(self.open[at].list, at)? {
                        RustType::Vec(element) => Some(*element),
                        _ => None,
                    },
                    Root::Parameter(at) => Some(self.parameters[at].1.clone()),
                    Root::Local => self.locals.get(variable.name).cloned(),
                }
            }
            Expression::Access(Access::Property(access)) => {
                let RustType::Class(class) = self.type_at(access.object, depth)? else {
                    return None;
                };
                let ClassLikeMemberSelector::Identifier(name) = &access.property else {
                    return None;
                };
                converter.property_type(&class, name.value)
            }
            Expression::ArrayAccess(access) => match self.type_at(access.array, depth)? {
                RustType::Vec(element) => Some(*element),
                _ => None,
            },
            Expression::Call(Call::Method(call)) => {
                let ClassLikeMemberSelector::Identifier(name) = &call.method else {
                    return None;
                };
                let (class, at) = *self.methods_at(call.object, name.value, depth).first()?;
                converter.classes[class].methods[at]
                    .signature
                    .returns
                    .clone()
            }
            Expression::Call(Call::Function(call)) => {
                let Expression::Identifier(Identifier::Local(name)) = call.function else {
                    return None;
                };
                let key = name.value.to_ascii_lowercase();
                converter.functions.get(&key)?.returns.clone()
            }
            Expression::Instantiation(instantiation) => {
                let key = class_key(instantiation.class)?;
                let class = converter.classes.iter().find(|c| c.key == key)?;
                Some(RustType::Class(class.rust_name.clone()))
            }
            _ => None,
        }
    }

    /// The methods a call of `name` on `object` may run: the method of its class, or every
    /// method of that name when the class is not known.
    fn methods(&self, object: &Expression<'_>, name: &[u8]) -> Vec<(usize, usize)> {
        self.methods_at(object, name, self.open.len())
    }

    /// `methods`, seen from inside the first `depth` open loops.
    fn methods_at(
        &self,
        object: &Expression<'_>,
        name: &[u8],
        depth: usize,
    ) -> Vec<(usize, usize)> {
        let converter = self.converter;
        if let Some(RustType::Class(class)) = self.type_at(object, depth)
            && let Some(class) = converter.class_named(&class)
        {
            return converter
                .method_index(class, name)
                .map(|at| (class, at))
                .into_iter()
                .collect();
        }
        (0..converter.classes.len())
            .filter_map(|class| Some((class, converter.method_index(class, name)?)))
            .collect()
    }
}

impl<'ast, 'arena> Body<'ast, 'arena, '_, '_> {
    /// Checks each part of `statement` that runs as one Rust statement: an object that it
    /// changes, through a change, a `&mut self` call, a `&mut` argument or a move, may be
    /// named in it only once, but for three forms rustc accepts (P2R-147).
    fn check_statement(&mut self, statement: &Statement<'_>) {
        for unit in units(statement) {
            let spans: Vec<Span> = unit.iter().map(|e| e.span()).collect();
            let mut roots: Vec<&[u8]> = Vec::new();
            for (root, at) in &self.events {
                if spans.iter().any(|span| contains(*span, *at))
                    && !roots.contains(&root.as_slice())
                {
                    roots.push(root);
                }
            }
            if roots.is_empty() {
                continue;
            }
            let mut found = Found::default();
            for &expression in &unit {
                Chains.walk_expression(expression, &mut found);
            }
            found.calls = found
                .pending
                .iter()
                .map(|&call| (call.arguments(), self.call_modes(call)))
                .collect();
            let repeated = roots.iter().any(|root| {
                let named = found.chains.iter().filter(|c| c.root == *root).count();
                named > 1
                    && !(unit.len() == 1
                        && (self.assigned_once(unit[0], root, &found)
                            || self.lent_once(root, &found)
                            || self.passed_once(root, &found)))
            });
            if repeated {
                let span = match unit.as_slice() {
                    [one] => one.span(),
                    _ => statement.span(),
                };
                self.loops.problems.push((span, REPEAT));
            }
        }
    }

    /// Form 1 of P2R-147: a top-level `=` on a property, or `+=`, `-=` or `*=` on an int
    /// or float property, of the object `root`, which its value may read and change.
    fn assigned_once(&self, top: &Expression<'_>, root: &[u8], found: &Found<'_, '_>) -> bool {
        let Expression::Assignment(assignment) = unparenthesized(top) else {
            return false;
        };
        let number = matches!(
            self.type_of(assignment.lhs),
            Some(RustType::I64 | RustType::F64)
        );
        let operator = match assignment.operator {
            AssignmentOperator::Assign(_) => true,
            AssignmentOperator::Addition(_)
            | AssignmentOperator::Subtraction(_)
            | AssignmentOperator::Multiplication(_) => number,
            _ => false,
        };
        let target = matches!(
            unparenthesized(assignment.lhs),
            Expression::Access(Access::Property(_))
        ) && chain_parts(assignment.lhs).is_some_and(|(r, _)| r == root);
        let in_target = found
            .chains
            .iter()
            .filter(|c| c.root == root && contains(assignment.lhs.span(), c.expression.span()))
            .count();
        operator && target && in_target == 1 && self.value_reads(assignment.rhs, root, found)
    }

    /// True when the value of a form 1 assignment names `root` only in ways that hold no
    /// borrow across a change of `root` in the same value: when the value changes `root`,
    /// each other mention is the receiver of a call (with no list read when the call takes
    /// `&mut self`) or a read of an int or float property outside string building, which
    /// `format!` would borrow (P2R-147).
    fn value_reads(&self, value: &Expression<'_>, root: &[u8], found: &Found<'_, '_>) -> bool {
        let span = value.span();
        let changes = self
            .events
            .iter()
            .any(|(r, at)| r == root && contains(span, *at));
        !changes
            || found
                .chains
                .iter()
                .filter(|c| c.root == root && contains(span, c.expression.span()))
                .all(|chain| match chain.receiver {
                    Some(call) => !(chain.steps.contains(&None) && self.changes_receiver(call)),
                    None => {
                        !chain.steps.is_empty()
                            && matches!(
                                self.type_of(chain.expression),
                                Some(RustType::I64 | RustType::F64)
                            )
                            && !found
                                .strings
                                .iter()
                                .any(|s| contains(*s, chain.expression.span()))
                    }
                })
    }

    /// Form 2 of P2R-147: a `&mut self` call on a chain from `root` without a list read,
    /// whose arguments read `root` only through Copy property reads and `&self` calls. A
    /// list read in the receiver goes through `IndexMut`, which rustc borrows at once.
    fn lent_once(&self, root: &[u8], found: &Found<'_, '_>) -> bool {
        found.chains.iter().any(|receiver| {
            let Some(call) = receiver.receiver else {
                return false;
            };
            if receiver.root != root
                || receiver.steps.contains(&None)
                || !self.changes_receiver(call)
            {
                return false;
            }
            let arguments = call.argument_list.span();
            found.chains.iter().all(|chain| {
                chain.root != root
                    || std::ptr::eq(chain, receiver)
                    || (contains(arguments, chain.expression.span()) && self.only_reads(chain))
            })
        })
    }

    /// Form 3 of P2R-147: `$this`, a `&mut` parameter or the value of a foreach by `&mut`
    /// passed as it is to a `&mut` parameter, with later arguments that read `root` as in
    /// form 2.
    fn passed_once(&self, root: &[u8], found: &Found<'_, '_>) -> bool {
        let reference = match self.root_named(root, self.open.len()) {
            Root::This => true,
            Root::Parameter(at) => self.parameters[at].2 == Mode::Mut,
            Root::Value(at) => self.loops.by_mut.contains(&self.open[at].span),
            Root::Local => false,
        };
        if !reference {
            return false;
        }
        found.calls.iter().any(|(list, modes)| {
            list.arguments
                .iter()
                .enumerate()
                .any(|(position, argument)| {
                    let Argument::Positional(positional) = argument else {
                        return false;
                    };
                    let alone = matches!(unparenthesized(positional.value),
                    Expression::Variable(Variable::Direct(v)) if v.name == root);
                    let lent = modes.get(position).is_some_and(|m| m.contains(&Mode::Mut));
                    if !alone || !lent {
                        return false;
                    }
                    let later = list
                        .arguments
                        .iter()
                        .skip(position + 1)
                        .map(|a| a.span())
                        .collect::<Vec<_>>();
                    found.chains.iter().all(|chain| {
                        chain.root != root
                            || chain.expression.span() == positional.value.span()
                            || (later
                                .iter()
                                .any(|span| contains(*span, chain.expression.span()))
                                && self.only_reads(chain))
                    })
                })
        })
    }

    /// True for a chain that reads its object as form 2 of P2R-147 allows: a property read
    /// that gives a Copy value, or the receiver of a `&self` call.
    fn only_reads(&self, chain: &Chain<'_, '_>) -> bool {
        match chain.receiver {
            Some(call) => !self.changes_receiver(call),
            None => {
                !chain.steps.is_empty()
                    && self
                        .type_of(chain.expression)
                        .is_some_and(|ty| ty.is_copy())
            }
        }
    }

    /// True when the method `call` may run takes `&mut self`.
    fn changes_receiver(&self, call: &MethodCall<'_>) -> bool {
        let ClassLikeMemberSelector::Identifier(name) = &call.method else {
            return false;
        };
        let classes = &self.converter.classes;
        self.methods(call.object, name.value)
            .iter()
            .any(|&(class, at)| classes[class].methods[at].self_mode == Mode::Mut)
    }

    /// The root that the variable with the PHP name `name` is, seen from inside the first
    /// `depth` open loops.
    fn root_named(&self, name: &[u8], depth: usize) -> Root {
        if name == b"$this" {
            Root::This
        } else if let Some(at) = self.open[..depth].iter().rposition(|o| o.value == name) {
            Root::Value(at)
        } else if let Some(at) = self.parameters.iter().position(|p| p.0 == name[1..]) {
            Root::Parameter(at)
        } else {
            Root::Local
        }
    }

    /// The forms of the parameters a call may pass its arguments to, by position.
    fn call_modes(&self, call: CallNode<'_, '_>) -> Vec<Vec<Mode>> {
        let converter = self.converter;
        match call {
            CallNode::Function(call) => {
                let Expression::Identifier(Identifier::Local(name)) = call.function else {
                    return Vec::new();
                };
                match converter.functions.get(&name.value.to_ascii_lowercase()) {
                    Some(signature) => positions(std::iter::once(signature.parameters.as_slice())),
                    None => Vec::new(),
                }
            }
            CallNode::Method(call) => {
                let ClassLikeMemberSelector::Identifier(name) = &call.method else {
                    return Vec::new();
                };
                let methods = self.methods(call.object, name.value);
                positions(methods.iter().map(|&(class, at)| {
                    converter.classes[class].methods[at]
                        .signature
                        .parameters
                        .as_slice()
                }))
            }
        }
    }

    /// Checks the body of a foreach over a list of a changing type: the object that holds
    /// the list may appear only in chains that leave the iterated chain at a property
    /// before any list read, and as the receiver of a `&self` call in a foreach by `&`
    /// (P2R-146).
    fn check_iterated(&mut self, foreach: &Foreach<'_>, by_mut: bool) {
        let Some((root, steps)) = chain_parts(foreach.expression) else {
            return;
        };
        let mut found = Found::default();
        match &foreach.body {
            ForeachBody::Statement(statement) => Chains.walk_statement(statement, &mut found),
            ForeachBody::ColonDelimited(body) => {
                for statement in body.statements.iter() {
                    Chains.walk_statement(statement, &mut found);
                }
            }
        }
        for chain in found.chains.iter().filter(|c| c.root == root) {
            let shared_call = !by_mut
                && chain.steps.is_empty()
                && chain
                    .receiver
                    .is_some_and(|call| !self.changes_receiver(call));
            if !leaves(&chain.steps, &steps) && !shared_call {
                self.loops
                    .problems
                    .push((chain.expression.span(), ITERATED));
            }
        }
    }
}

/// A chain of property and list reads from a variable, as the source writes it.
struct Chain<'ast, 'arena> {
    expression: &'ast Expression<'arena>,
    /// The root variable's PHP name with `$`.
    root: &'arena [u8],
    steps: Steps<'arena>,
    /// The method call whose receiver it is.
    receiver: Option<&'ast MethodCall<'arena>>,
}

/// A call found in a statement.
#[derive(Clone, Copy)]
enum CallNode<'ast, 'arena> {
    Function(&'ast FunctionCall<'arena>),
    Method(&'ast MethodCall<'arena>),
}

impl<'ast, 'arena> CallNode<'ast, 'arena> {
    fn arguments(self) -> &'ast ArgumentList<'arena> {
        match self {
            CallNode::Function(call) => &call.argument_list,
            CallNode::Method(call) => &call.argument_list,
        }
    }
}

/// The chains of a statement, each one whole, and its calls with the forms of their
/// parameters.
#[derive(Default)]
struct Found<'ast, 'arena> {
    chains: Vec<Chain<'ast, 'arena>>,
    calls: Vec<(&'ast ArgumentList<'arena>, Vec<Vec<Mode>>)>,
    pending: Vec<CallNode<'ast, 'arena>>,
    /// The concatenations and interpolated strings, which become `format!`.
    strings: Vec<Span>,
}

/// The property names a chain reads in order, `None` for a list read.
type Steps<'arena> = Vec<Option<&'arena [u8]>>;

/// The root and the steps of `expression` when it is a chain.
fn chain_parts<'arena>(expression: &Expression<'arena>) -> Option<(&'arena [u8], Steps<'arena>)> {
    match expression {
        Expression::Parenthesized(inner) => chain_parts(inner.expression),
        Expression::Variable(Variable::Direct(variable)) => Some((variable.name, Vec::new())),
        Expression::Access(Access::Property(access)) => {
            let ClassLikeMemberSelector::Identifier(name) = &access.property else {
                return None;
            };
            let (root, mut steps) = chain_parts(access.object)?;
            steps.push(Some(name.value));
            Some((root, steps))
        }
        Expression::ArrayAccess(access) => {
            let (root, mut steps) = chain_parts(access.array)?;
            steps.push(None);
            Some((root, steps))
        }
        _ => None,
    }
}

/// True when a chain with `steps` leaves the iterated chain with `iterated` at a property
/// before either reads a list: rustc then sees two different fields.
fn leaves(steps: &[Option<&[u8]>], iterated: &[Option<&[u8]>]) -> bool {
    for (step, other) in steps.iter().zip(iterated) {
        match (step, other) {
            (Some(a), Some(b)) if a != b => return true,
            (Some(_), Some(_)) => {}
            _ => return false,
        }
    }
    false
}

/// True when `inner` lies inside `outer`.
fn contains(outer: Span, inner: Span) -> bool {
    outer.start.offset <= inner.start.offset && inner.end.offset <= outer.end.offset
}

/// The parts of a statement that each run as one Rust statement: an expression statement,
/// an echo, a return value, and each condition, `for` expression and foreach list.
fn units<'a, 'arena>(statement: &'a Statement<'arena>) -> Vec<Vec<&'a Expression<'arena>>> {
    match statement {
        Statement::Expression(inner) => vec![vec![inner.expression]],
        Statement::Echo(echo) => vec![echo.values.iter().copied().collect()],
        Statement::Return(r#return) => r#return
            .value
            .map(|value| vec![value])
            .into_iter()
            .collect(),
        Statement::If(r#if) => super::control::branches(r#if)
            .into_iter()
            .filter_map(|(condition, _)| condition.map(|c| vec![c]))
            .collect(),
        Statement::While(r#while) => vec![vec![r#while.condition]],
        Statement::DoWhile(r#do) => vec![vec![r#do.condition]],
        Statement::For(r#for) => r#for
            .initializations
            .iter()
            .chain(r#for.conditions.iter())
            .chain(r#for.increments.iter())
            .map(|&e| vec![e])
            .collect(),
        Statement::Foreach(foreach) => vec![vec![foreach.expression]],
        _ => Vec::new(),
    }
}

/// Collects the whole chains of an expression, and the receivers of method calls, with
/// the chains inside their list indexes and arguments.
struct Chains;

impl<'ast, 'arena> Walker<'ast, 'arena, Found<'ast, 'arena>> for Chains {
    fn walk_expression(
        &self,
        expression: &'ast Expression<'arena>,
        found: &mut Found<'ast, 'arena>,
    ) {
        if let Some((root, steps)) = chain_parts(expression) {
            found.chains.push(Chain {
                expression,
                root,
                steps,
                receiver: None,
            });
            self.walk_indexes(expression, found);
            return;
        }
        if let Expression::Call(Call::Method(call)) = expression
            && let Some((root, steps)) = chain_parts(call.object)
        {
            found.chains.push(Chain {
                expression: call.object,
                root,
                steps,
                receiver: Some(call),
            });
            found.pending.push(CallNode::Method(call));
            self.walk_indexes(call.object, found);
            self.walk_argument_list(&call.argument_list, found);
            return;
        }
        match expression {
            Expression::Call(Call::Method(call)) => found.pending.push(CallNode::Method(call)),
            Expression::Call(Call::Function(call)) => found.pending.push(CallNode::Function(call)),
            Expression::Binary(binary)
                if matches!(binary.operator, BinaryOperator::StringConcat(_)) =>
            {
                found.strings.push(expression.span());
            }
            Expression::CompositeString(_) => found.strings.push(expression.span()),
            _ => {}
        }
        mago_syntax::walker::walk_expression(self, expression, found);
    }

    fn walk_closure(&self, _: &'ast Closure<'arena>, _: &mut Found<'ast, 'arena>) {}

    fn walk_arrow_function(&self, _: &'ast ArrowFunction<'arena>, _: &mut Found<'ast, 'arena>) {}
}

impl Chains {
    /// Walks the list indexes inside a chain.
    fn walk_indexes<'ast, 'arena>(
        &self,
        expression: &'ast Expression<'arena>,
        found: &mut Found<'ast, 'arena>,
    ) {
        match expression {
            Expression::Parenthesized(inner) => self.walk_indexes(inner.expression, found),
            Expression::Access(Access::Property(access)) => self.walk_indexes(access.object, found),
            Expression::ArrayAccess(access) => {
                self.walk_indexes(access.array, found);
                self.walk_expression(access.index, found);
            }
            _ => {}
        }
    }
}

/// Collects the type Mago gives each variable of a body.
struct Locals<'x, 'c> {
    converter: &'x Converter<'c>,
    found: HashMap<Vec<u8>, RustType>,
}

struct LocalTypes;

impl<'ast, 'arena, 'x, 'c> Walker<'ast, 'arena, Locals<'x, 'c>> for LocalTypes {
    fn walk_in_direct_variable(
        &self,
        variable: &'ast DirectVariable<'arena>,
        locals: &mut Locals<'x, 'c>,
    ) {
        if locals.found.contains_key(variable.name) {
            return;
        }
        let converter = locals.converter;
        let ty = converter
            .artifacts
            .get_expression_type(variable)
            .and_then(|union| types::rust_type(union, &converter.type_names).ok());
        if let Some(ty) = ty {
            locals.found.insert(variable.name.to_vec(), ty);
        }
    }

    fn walk_function(&self, _: &'ast Function<'arena>, _: &mut Locals<'x, 'c>) {}

    fn walk_closure(&self, _: &'ast Closure<'arena>, _: &mut Locals<'x, 'c>) {}

    fn walk_arrow_function(&self, _: &'ast ArrowFunction<'arena>, _: &mut Locals<'x, 'c>) {}

    fn walk_class(&self, _: &'ast Class<'arena>, _: &mut Locals<'x, 'c>) {}
}

/// Turns up the forms of one body from its changes, calls and moves.
struct Scan;

impl<'ast, 'arena, 'x, 'c> Walker<'ast, 'arena, Body<'ast, 'arena, 'x, 'c>> for Scan {
    fn walk_in_assignment(
        &self,
        assignment: &'ast Assignment<'arena>,
        body: &mut Body<'ast, 'arena, 'x, 'c>,
    ) {
        match unparenthesized(assignment.lhs) {
            target @ Expression::Access(Access::Property(_)) => body.change_chain(target),
            Expression::ArrayAppend(append)
                if matches!(
                    unparenthesized(append.array),
                    Expression::Access(Access::Property(_))
                ) =>
            {
                body.change_chain(append.array)
            }
            _ => {}
        }
        if matches!(assignment.operator, AssignmentOperator::Assign(_)) {
            body.moved(assignment.rhs);
        }
    }

    fn walk_in_unary_prefix(
        &self,
        unary: &'ast UnaryPrefix<'arena>,
        body: &mut Body<'ast, 'arena, 'x, 'c>,
    ) {
        if matches!(
            unary.operator,
            UnaryPrefixOperator::PreIncrement(_) | UnaryPrefixOperator::PreDecrement(_)
        ) && matches!(
            unparenthesized(unary.operand),
            Expression::Access(Access::Property(_))
        ) {
            body.change_chain(unary.operand);
        }
    }

    fn walk_in_unary_postfix(
        &self,
        unary: &'ast UnaryPostfix<'arena>,
        body: &mut Body<'ast, 'arena, 'x, 'c>,
    ) {
        if matches!(
            unparenthesized(unary.operand),
            Expression::Access(Access::Property(_))
        ) {
            body.change_chain(unary.operand);
        }
    }

    fn walk_in_method_call(
        &self,
        call: &'ast MethodCall<'arena>,
        body: &mut Body<'ast, 'arena, 'x, 'c>,
    ) {
        if body.changes_receiver(call) {
            body.change_chain(call.object);
        }
        let modes = body.call_modes(CallNode::Method(call));
        body.arguments(&call.argument_list, &modes);
    }

    fn walk_in_function_call(
        &self,
        call: &'ast FunctionCall<'arena>,
        body: &mut Body<'ast, 'arena, 'x, 'c>,
    ) {
        let modes = body.call_modes(CallNode::Function(call));
        body.arguments(&call.argument_list, &modes);
    }

    fn walk_in_instantiation(
        &self,
        instantiation: &'ast Instantiation<'arena>,
        body: &mut Body<'ast, 'arena, 'x, 'c>,
    ) {
        let (Some(key), Some(list)) =
            (class_key(instantiation.class), &instantiation.argument_list)
        else {
            return;
        };
        let classes = &body.converter.classes;
        let Some(class) = classes.iter().find(|c| c.key == key) else {
            return;
        };
        let modes = positions(std::iter::once(class.fields.as_slice()));
        body.arguments(list, &modes);
    }

    fn walk_in_return(
        &self,
        r#return: &'ast Return<'arena>,
        body: &mut Body<'ast, 'arena, 'x, 'c>,
    ) {
        if let Some(value) = r#return.value {
            body.moved(value);
        }
    }

    fn walk_in_array(&self, array: &'ast Array<'arena>, body: &mut Body<'ast, 'arena, 'x, 'c>) {
        for element in array.elements.iter() {
            if let ArrayElement::Value(element) = element {
                body.moved(element.value);
            }
        }
    }

    // The list is outside the loop: only the body sees the value.
    fn walk_foreach(&self, foreach: &'ast Foreach<'arena>, body: &mut Body<'ast, 'arena, 'x, 'c>) {
        self.walk_expression(foreach.expression, body);
        let value = match unparenthesized(foreach.target.value()) {
            Expression::Variable(Variable::Direct(variable)) => variable.name.to_vec(),
            _ => Vec::new(),
        };
        body.open.push(Open {
            value,
            list: foreach.expression,
            span: foreach.span(),
            changed: Vec::new(),
        });
        self.walk_foreach_target(&foreach.target, body);
        self.walk_foreach_body(&foreach.body, body);
        let Some(open) = body.open.pop() else {
            return;
        };
        let copy_list = matches!(
            body.type_of(foreach.expression),
            Some(RustType::Vec(element)) if element.is_copy()
        );
        let property = matches!(
            unparenthesized(foreach.expression),
            Expression::Access(Access::Property(_))
        );
        let holder = body.root(foreach.expression, body.open.len());
        if copy_list && property && holder.is_some_and(|root| open.changed.contains(&root)) {
            body.loops.copied.insert(open.span);
        }
        let changing = matches!(
            body.type_of(foreach.expression),
            Some(RustType::Vec(element)) if body.converter.is_changing(&element)
        );
        if changing {
            let by_mut = body.loops.by_mut.contains(&open.span);
            body.check_iterated(foreach, by_mut);
        }
    }

    fn walk_out_statement(
        &self,
        statement: &'ast Statement<'arena>,
        body: &mut Body<'ast, 'arena, 'x, 'c>,
    ) {
        body.check_statement(statement);
    }

    // A nested function, closure or class has variables of its own.
    fn walk_function(&self, _: &'ast Function<'arena>, _: &mut Body<'ast, 'arena, 'x, 'c>) {}

    fn walk_closure(&self, _: &'ast Closure<'arena>, _: &mut Body<'ast, 'arena, 'x, 'c>) {}

    fn walk_arrow_function(
        &self,
        _: &'ast ArrowFunction<'arena>,
        _: &mut Body<'ast, 'arena, 'x, 'c>,
    ) {
    }

    fn walk_class(&self, _: &'ast Class<'arena>, _: &mut Body<'ast, 'arena, 'x, 'c>) {}
}
