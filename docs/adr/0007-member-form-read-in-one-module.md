# Read the form of a class member in one module; keep the rules with each caller

How a method or property is written (its modifiers, `&`, the number of parameters, its attribute groups and their names) is read in one module, `MemberForm` in `src/convert/members.rs`. The rules that use these facts, and their stop messages, stay with each kind of member (model members, relation methods, `casts()`, `getRouteKeyName`, and next class methods, interfaces, enums and exceptions), because each kind accepts a different form and names itself in its message. Before this, each kind read the syntax tree itself, and 11 of 80 commits in October and November 2026 were fixes for a form check that one copy had missed.

## Considered Options

- One function that returns the stop message for a member: rejected, because the messages differ by kind and the stop folders compare them byte for byte.
- Facts from the module but locations from the syntax tree: rejected, because a caller would then read two sources again. The module keeps each modifier and attribute group with its span, so a stop message can point at the one at fault.

## Consequences

- A form check that applies to every member is added to `members.rs` once.
- Model members, class and enum methods, declared properties, laravel-data hooks, interface methods and the constructor of an exception class read their form from the module.
- The rules for a modifier written twice, a wider set visibility, and `final` with `private` live in `members.rs`; class declarations, constants and constructor parameters reach them through adapters in `classes.rs`.
- Promoted constructor properties are parameters, not members, and stay outside the module. Checks that only compare a member's name (such as `__construct`) also stay where they are.
