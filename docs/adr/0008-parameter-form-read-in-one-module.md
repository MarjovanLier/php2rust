# Read the form of a parameter in one module; keep the messages with each kind of callable

How a parameter is written (its name, a `static` type, its modifiers, promotion, hooks, `&`, `...`, its default, its attribute groups and whether it has a type) is read in one module, `ParameterForm` in `src/convert/parameters.rs`, and so are two facts of a parameter list: a name that repeats an earlier one, and a default before a required parameter. The module also holds the list of superglobals. Every kind of callable (function, method, constructor, property hook, closure inside a hook, the callback of `when`, the arrow function callback) reads these facts from the module and turns them into its own stop message, in its own order. Before this, each kind read the syntax tree itself, five fix commits each added one of PHP's parameter rules to one more copy, and the arrow function callback still had no name check, so it converted a file PHP refuses.

## Considered Options

- Parameters in the member form module of ADR 0007: rejected, because a parameter is not a member, and ADR 0007 keeps promoted constructor properties out of that module for that reason.
- One function that returns the first problem of a parameter: rejected, because the kinds check in different orders (a hook checks `static` before promotion and promotion before the name) and tag one rule with different IDs (a superglobal's name is P2R-380 in a hook and P2R-31 elsewhere). The stop folders compare the first line of each problem byte for byte, so the module gives facts and each kind keeps its order and messages.

## Consequences

- A rule that PHP applies to every parameter is a fact of the module, and each kind of callable reads it from there.
- The type of a parameter and its default value still convert with the caller (P2R-23, P2R-24, P2R-168); attribute names that a kind accepts (`#[SensitiveParameter]`, laravel-data's attributes) are still read where they are accepted.
- The modifier rules of a constructor parameter still reach `members.rs` through the adapter in `classes.rs` that ADR 0007 names.
- Every form check reads the module, also the constructor of an exception class, `fields` in `classes.rs`, interface method parameters and the Blueprint closure of a migration. The module gives a modifier that PHP rejects on a parameter as a fault with its span, and the class and exception constructors map it to their message. A parameter's name is still read from the syntax tree where it names a Rust variable or looks up the codebase, because that is naming, not a form check.
