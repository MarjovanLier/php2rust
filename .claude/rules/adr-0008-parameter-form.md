---
paths:
  - "src/convert/**/*.rs"
---

# ADR 0008: Read the form of a parameter in one module; keep the messages with each kind of callable

Rationale: `docs/adr/0008-parameter-form-read-in-one-module.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

- Read how a parameter is written from `ParameterForm` in `src/convert/parameters.rs`: its name, a `static` type, its modifiers, promotion, hooks, `&`, `...`, its default, its attribute groups and whether it has a type. The module also gives a name that repeats an earlier one, a default before a required parameter, and the list of superglobals.
- A rule that PHP applies to every parameter is a fact of the module. Every kind of callable reads it from there; no kind keeps its own copy.
- Each kind of callable turns the facts into its own stop messages, in its own order and with its own ID. For example, a superglobal's name is P2R-380 in a hook and P2R-31 elsewhere. Do not add one function that returns the first problem of a parameter.
- The type and default value of a parameter still convert in the caller (P2R-23, P2R-24, P2R-168). Read the attribute names that a kind accepts (`#[SensitiveParameter]`, laravel-data's attributes) where that kind accepts them.
- The exception constructor in `exceptions.rs` and `fields` in `classes.rs` still read parameters from the syntax tree. They are the next to move; do not add a third place.
