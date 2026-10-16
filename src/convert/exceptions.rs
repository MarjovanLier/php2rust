//! Exception classes of the input: each joins the class tree of the exception struct, and a
//! constructor becomes a function that returns the exception (P2R-390 to P2R-392).

use std::collections::HashSet;

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use super::control::emit;
use super::{Converter, ParameterSignature, Signature, classes, helpers, throws};
use crate::names;
use crate::printer::{self, RExpr};
use crate::types::RustType;

/// An exception class of the input (P2R-390).
pub(super) struct ExceptionClass {
    /// The lowercase fully qualified PHP name.
    pub(super) key: Vec<u8>,
    /// The fully qualified PHP name as declared, which `is_a` and `uncaught()` print.
    pub(super) name: String,
    /// The name of its parent in the class tree.
    pub(super) parent: String,
    /// The index of its parent when the input declares the parent.
    parent_index: Option<usize>,
    /// Its place in the input, which orders the arms of `is_a` (P2R-390).
    order: usize,
    /// True when it declares a constructor.
    pub(super) constructor: bool,
    /// The Rust name its constructor's function takes, which variables and parameters may
    /// not take (P2R-31, P2R-391).
    pub(super) rust_name: Option<String>,
    /// The function of its constructor (P2R-391).
    pub(super) function: Option<Signature>,
}

/// The names of the properties PHP's `Exception` declares, which a promoted parameter may not
/// take (P2R-391).
const EXCEPTION_PROPERTIES: [&[u8]; 6] =
    [b"message", b"code", b"file", b"line", b"previous", b"trace"];

/// The class an exception class of the input or of P2R-210 names in a `throw` or `catch`.
pub(super) enum Thrown<'s> {
    BuiltIn(&'static str),
    Input(&'s ExceptionClass),
}

impl Converter<'_> {
    /// Finds the exception classes of the input before the other classes register, so they
    /// take no struct (P2R-390). A parent may come later in the input, so the search
    /// repeats until it finds no more.
    pub(super) fn register_exceptions(&mut self, program: &Program<'_>) {
        let candidates: Vec<(Vec<u8>, Vec<u8>, &Class<'_>)> = program
            .statements
            .iter()
            .filter_map(|statement| match statement {
                Statement::Class(declaration) => Some(declaration),
                _ => None,
            })
            .filter_map(|declaration| {
                let extends = declaration.extends.as_ref()?;
                let [parent] = extends.types.as_slice() else {
                    return None;
                };
                let key = self.name_key(&declaration.name, declaration.name.value);
                Some((key, self.identifier_key(parent), declaration))
            })
            .collect();
        loop {
            let mut grew = false;
            for (order, (key, parent_key, declaration)) in candidates.iter().enumerate() {
                if self.exceptions.iter().any(|known| &known.key == key) {
                    continue;
                }
                let parent = match throws::built_in(parent_key) {
                    Some(name) => Some((name.to_owned(), None)),
                    None => self
                        .exceptions
                        .iter()
                        .position(|known| &known.key == parent_key)
                        .map(|at| (self.exceptions[at].name.clone(), Some(at))),
                };
                let Some((parent, parent_index)) = parent else {
                    continue;
                };
                let constructor = declaration.members.iter().any(|member| {
                    matches!(member, ClassLikeMember::Method(method)
                        if method.name.value.eq_ignore_ascii_case(b"__construct"))
                });
                let name = self.declared_name(&declaration.name);
                let rust_name = constructor
                    .then(|| names::rust_name(&String::from_utf8_lossy(declaration.name.value)))
                    .flatten();
                self.exceptions.push(ExceptionClass {
                    key: key.clone(),
                    name,
                    parent,
                    parent_index,
                    order,
                    constructor,
                    rust_name,
                    function: None,
                });
                grew = true;
            }
            if !grew {
                break;
            }
        }
    }

    /// The name and parent of each exception class, in source order (P2R-390).
    pub(super) fn exception_pairs(&self) -> Vec<(String, String)> {
        let mut classes: Vec<&ExceptionClass> = self.exceptions.iter().collect();
        classes.sort_by_key(|class| class.order);
        classes
            .iter()
            .map(|class| (class.name.clone(), class.parent.clone()))
            .collect()
    }

    /// The fully qualified PHP name of a declaration, as written, without the leading
    /// backslash.
    fn declared_name(&self, name: &LocalIdentifier<'_>) -> String {
        let full = self
            .analysed(name)
            .resolved
            .resolve(&name.span().start)
            .unwrap_or(name.value);
        String::from_utf8_lossy(full.strip_prefix(b"\\").unwrap_or(full)).into_owned()
    }

    /// True when `name` is the Rust name of a function of the file: a PHP function or the
    /// function of an exception constructor, which a variable or parameter of that name
    /// would hide (P2R-31, P2R-391).
    pub(super) fn is_function_name(&self, name: &str) -> bool {
        self.functions.values().any(|f| f.rust_name == name)
            || self
                .exceptions
                .iter()
                .any(|class| class.rust_name.as_deref() == Some(name))
    }

    /// The index of the exception class `declaration` declares (P2R-390).
    pub(super) fn exception_index(&self, declaration: &Class<'_>) -> Option<usize> {
        let key = self.name_key(&declaration.name, declaration.name.value);
        self.exceptions.iter().position(|known| known.key == key)
    }

    /// The class a resolved, lowercase name names in a `throw` or `catch`: one of P2R-210
    /// or an exception class of the input (P2R-392).
    pub(super) fn thrown_class(&self, key: &[u8]) -> Option<Thrown<'_>> {
        if let Some(name) = throws::built_in(key) {
            return Some(Thrown::BuiltIn(name));
        }
        self.exceptions
            .iter()
            .find(|known| known.key == key)
            .map(Thrown::Input)
    }

    /// Registers the function of each constructor of an exception class, once the
    /// functions are known, whose Rust names it may not take (P2R-391).
    pub(super) fn register_exception_functions(&mut self, program: &Program<'_>) {
        let mut taken: HashSet<String> = self
            .functions
            .values()
            .map(|f| f.rust_name.clone())
            .collect();
        // A parameter may hide neither kind of function (P2R-31).
        let hidden: HashSet<String> = taken
            .iter()
            .cloned()
            .chain(self.exceptions.iter().filter_map(|c| c.rust_name.clone()))
            .collect();
        for statement in program.statements.iter() {
            let Statement::Class(declaration) = statement else {
                continue;
            };
            let Some(index) = self.exception_index(declaration) else {
                continue;
            };
            let Some(constructor) = constructor(declaration) else {
                continue;
            };
            let Some(rust_name) = self.rust_name(declaration.name.value, &declaration.name) else {
                continue;
            };
            if rust_name == "main" || helpers::is_helper(&rust_name) || taken.contains(&rust_name) {
                self.refuse(
                    &declaration.name,
                    format!("{rust_name} is already the Rust name of a function, a helper or another exception class (P2R-391)"),
                );
                continue;
            }
            let Some(signature) =
                self.constructor_signature(declaration, constructor, rust_name.clone(), &hidden)
            else {
                continue;
            };
            taken.insert(rust_name);
            self.exceptions[index].function = Some(signature);
        }
    }

    /// The signature of the function of `constructor`, which returns the exception
    /// (P2R-391).
    fn constructor_signature(
        &mut self,
        declaration: &Class<'_>,
        constructor: &Method<'_>,
        rust_name: String,
        taken: &HashSet<String>,
    ) -> Option<Signature> {
        let key = self.name_key(&declaration.name, declaration.name.value);
        let codebase = self.codebase;
        let Some(metadata) = codebase.get_method(&key, b"__construct") else {
            self.refuse(
                &constructor.name,
                "Mago has no signature for this constructor (P2R-23)",
            );
            return None;
        };
        let mut converted = self.defaults_last(&constructor.parameter_list);
        let mut parameters: Vec<ParameterSignature> = Vec::new();
        for (parameter, meta) in constructor
            .parameter_list
            .parameters
            .iter()
            .zip(metadata.parameters.iter())
        {
            if parameter.is_promoted_property()
                && EXCEPTION_PROPERTIES.contains(&&parameter.variable.name[1..])
            {
                self.refuse(
                    parameter,
                    "PHP's Exception already declares a property of this name (P2R-391)",
                );
                converted = false;
                continue;
            }
            let declared = meta.get_type_metadata().map(|t| &t.type_union);
            match self.parameter(parameter, declared, taken, &parameters) {
                Some(signature) => parameters.push(signature),
                None => converted = false,
            }
        }
        converted.then(|| Signature {
            rust_name,
            parameters,
            returns: Some(RustType::Class("PhpException".to_owned())),
            throws: false,
        })
    }

    /// The function of the exception class at `index`, or nothing for a class without a
    /// constructor, after the checks of P2R-390 and P2R-391.
    pub(super) fn exception_item(
        &mut self,
        declaration: &Class<'_>,
        index: usize,
    ) -> Option<Vec<String>> {
        self.exception_shape(declaration, index);
        let constructor = constructor(declaration)?;
        let signature = self.exceptions[index].function.clone()?;
        let MethodBody::Concrete(block) = &constructor.body else {
            return None;
        };
        let statements: Vec<&Statement<'_>> = block.statements.iter().collect();
        let Some((last, rest)) = statements.split_last() else {
            self.refuse(
                &constructor.name,
                "the constructor of an exception class must end with parent::__construct(..) (P2R-391)",
            );
            return None;
        };
        let Some(call) = parent_construct(last) else {
            self.refuse(
                last,
                "the constructor of an exception class must end with parent::__construct(..) (P2R-391)",
            );
            return None;
        };
        let mut found: Vec<Span> = Vec::new();
        for statement in rest {
            Bound.walk_statement(statement, &mut found);
        }
        Bound.walk_argument_list(&call.argument_list, &mut found);
        if let Some(span) = found.first() {
            self.refuse(
                span,
                "the constructor of an exception class may not use $this, return, or another parent:: access (P2R-391)",
            );
            return None;
        }
        if self.body_throws(statements.iter().copied(), None) {
            self.refuse(
                &constructor.name,
                "the constructor of an exception class may not throw (P2R-391)",
            );
            return None;
        }
        let name = self.exceptions[index].name.clone();
        self.uses_exceptions = true;
        let node = last_expression(last);
        // No key: a call of a PHP function whose key equals the class's is no self call.
        Some(
            self.function_lines(None, &declaration.name, rest, &signature, |this, body| {
                // An endless loop before it leaves the call unreached (P2R-40).
                if this.unreachable(last) {
                    return;
                }
                if let Some(exception) =
                    this.new_exception(Some(&call.argument_list), node, &name, true)
                {
                    body.push(emit(
                        1,
                        format!("return {};", printer::expr(&exception, false)),
                    ));
                }
                this.leave("return");
            }),
        )
    }

    /// Refuses what an exception class may not hold (P2R-390, P2R-391).
    fn exception_shape(&mut self, declaration: &Class<'_>, index: usize) {
        if let Some(attributes) = declaration.attribute_lists.first() {
            self.refuse(attributes, "attributes are not supported (P2R-07)");
        }
        // P2R-111 applies to its name, although it takes no struct.
        self.declared_type_name(&declaration.name, "class");
        if let Some(modifier) = declaration
            .modifiers
            .iter()
            .find(|m| !matches!(m, Modifier::Final(_)))
        {
            self.refuse(
                modifier,
                "an exception class may be final, and no other modifier is supported (P2R-390)",
            );
        } else if let Some(modifier) = classes::doubled_modifier(&declaration.modifiers) {
            self.refuse(modifier, classes::DOUBLED_MODIFIER);
        }
        if let Some(implements) = &declaration.implements {
            self.refuse(
                implements,
                "an exception class that implements an interface is not supported (P2R-390)",
            );
        }
        let mut constructors = 0;
        for member in declaration.members.iter() {
            let constructor = matches!(member, ClassLikeMember::Method(method)
                if method.name.value.eq_ignore_ascii_case(b"__construct"));
            if !constructor {
                self.refuse(
                    member,
                    "an exception class may hold only a constructor (P2R-390)",
                );
            } else {
                constructors += 1;
                // PHP rejects a second one, in any letter case; Mago does not report it.
                if constructors > 1 {
                    self.refuse(member, "a class can have one constructor only (P2R-130)");
                }
            }
        }
        if let Some(parent) = self.exceptions[index].parent_index {
            let parent_constructor = self.exceptions[parent].constructor;
            let parent_key = self.exceptions[parent].key.clone();
            if let Some(extends) = &declaration.extends {
                if parent_constructor {
                    self.refuse(
                        extends,
                        "a class whose parent exception class of the input has a constructor is not supported, because PHP would run the parent's constructor (P2R-391)",
                    );
                }
                let codebase = self.codebase;
                if codebase
                    .get_class_like(&parent_key)
                    .is_some_and(|parent| parent.flags.is_final())
                {
                    self.refuse(
                        extends,
                        "PHP rejects a class that extends a final class (P2R-390)",
                    );
                }
                // PHP declares the class only when execution reaches it if its parent comes
                // later in the file.
                let later = codebase.get_class_like(&parent_key).is_some_and(|parent| {
                    parent.span.file_id == declaration.span().file_id
                        && parent.span.start.offset > declaration.span().start.offset
                });
                if later {
                    self.refuse(
                        extends,
                        "an exception class must come after its parent in the same file, because PHP declares it only when execution reaches it otherwise (P2R-390)",
                    );
                }
            }
        }
        if let Some(constructor) = constructor(declaration) {
            if let Some(modifier) = constructor
                .modifiers
                .iter()
                .find(|m| !matches!(m, Modifier::Public(_) | Modifier::Final(_)))
            {
                self.refuse(
                    modifier,
                    "the constructor of an exception class may be public and final, and no other modifier is supported (P2R-391)",
                );
            } else if let Some(modifier) = classes::doubled_modifier(&constructor.modifiers) {
                self.refuse(modifier, classes::DOUBLED_MODIFIER);
            }
            for parameter in constructor.parameter_list.parameters.iter() {
                if let Some((_, problem)) = classes::parameter_modifier(&parameter.modifiers) {
                    self.refuse(parameter, problem);
                }
            }
            if let Some(hint) = &constructor.return_type_hint {
                self.refuse(hint, "PHP rejects a return type on a constructor (P2R-391)");
            }
            if let Some(parameter) = constructor
                .parameter_list
                .parameters
                .iter()
                .find(|parameter| parameter.hooks.is_some())
            {
                self.refuse(
                    parameter,
                    "a property hook runs when the constructor sets the property, which the function does not do (P2R-391)",
                );
            }
            if constructor.ampersand.is_some() {
                self.refuse(
                    &constructor.name,
                    "the constructor of an exception class may not return by reference (P2R-391)",
                );
            }
            if let Some(attributes) = constructor.attribute_lists.first() {
                self.refuse(attributes, "attributes are not supported (P2R-07)");
            }
        }
    }

    /// `PhpException::new("<class>", message, code)` from the arguments `list` of a `new`
    /// or, when `parent` holds, of `parent::__construct`, which may pass only the message
    /// and the code (P2R-212, P2R-391).
    pub(super) fn new_exception(
        &mut self,
        list: Option<&ArgumentList<'_>>,
        node: &Expression<'_>,
        class: &str,
        parent: bool,
    ) -> Option<RExpr> {
        let parameters = throws::exception_parameters();
        let extra = list.is_some_and(|list| {
            list.arguments.len() > 2
                || list.arguments.iter().any(|argument| match argument {
                    Argument::Named(named) => !matches!(named.name.value, b"message" | b"code"),
                    Argument::Positional(_) => false,
                })
        });
        if extra {
            self.refuse(
                node,
                if parent {
                    "parent::__construct(..) converts only with the arguments message and code (P2R-391)"
                } else {
                    "a throw converts only with the arguments message and code (P2R-212)"
                },
            );
            return None;
        }
        let mut arguments = match list {
            Some(list) => self.arguments(list, &parameters, node)?,
            None => vec![RExpr::Str(String::new()), RExpr::Int(0)],
        };
        arguments.insert(0, RExpr::Str(class.to_owned()));
        self.uses_exceptions = true;
        Some(RExpr::Call("PhpException::new".into(), arguments))
    }
}

/// The constructor of a class.
pub(super) fn constructor<'d, 'a>(declaration: &'d Class<'a>) -> Option<&'d Method<'a>> {
    declaration.members.iter().find_map(|member| match member {
        ClassLikeMember::Method(method)
            if method.name.value.eq_ignore_ascii_case(b"__construct") =>
        {
            Some(method)
        }
        _ => None,
    })
}

/// The call of `statement` when it is `parent::__construct(..);`.
fn parent_construct<'s, 'a>(statement: &'s Statement<'a>) -> Option<&'s StaticMethodCall<'a>> {
    let Statement::Expression(expression) = statement else {
        return None;
    };
    let Expression::Call(Call::StaticMethod(call)) = expression.expression else {
        return None;
    };
    let parent = matches!(call.class, Expression::Parent(_));
    let named = matches!(&call.method, ClassLikeMemberSelector::Identifier(name)
        if name.value.eq_ignore_ascii_case(b"__construct"));
    (parent && named).then_some(call)
}

/// The expression of an expression statement.
fn last_expression<'s, 'a>(statement: &'s Statement<'a>) -> &'s Expression<'a> {
    match statement {
        Statement::Expression(expression) => expression.expression,
        _ => unreachable!("parent_construct accepted only an expression statement"),
    }
}

/// Finds what the body of an exception constructor may not hold: `$this`, `return`, and a
/// `parent::` access other than the last statement's (P2R-391).
struct Bound;

impl<'ast, 'arena> Walker<'ast, 'arena, Vec<Span>> for Bound {
    fn walk_in_direct_variable(
        &self,
        variable: &'ast DirectVariable<'arena>,
        found: &mut Vec<Span>,
    ) {
        if variable.name == b"$this" {
            found.push(variable.span());
        }
    }

    fn walk_in_return(&self, node: &'ast Return<'arena>, found: &mut Vec<Span>) {
        found.push(node.span());
    }

    fn walk_in_parent_keyword(&self, keyword: &'ast Keyword<'arena>, found: &mut Vec<Span>) {
        found.push(keyword.span());
    }
}
