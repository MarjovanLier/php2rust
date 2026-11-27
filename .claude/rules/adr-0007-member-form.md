---
paths:
  - "src/convert/**/*.rs"
---

# ADR 0007: Read the form of a class member in one module; keep the rules with each caller

Rationale: `docs/adr/0007-member-form-read-in-one-module.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

- Read how a member is written (its modifiers, `&`, the number of parameters, its attribute groups and their names) from `MemberForm` in `src/convert/members.rs`. Do not read it from the syntax tree in a caller.
- A form check that applies to every member goes into `members.rs` once.
- Keep each rule and its stop message with its kind of member. The messages differ by kind and the stop folders compare them byte for byte, so do not add one function that returns the stop message of a member.
- Point a stop message at the span that the module keeps for the modifier or attribute group at fault.
- Outside the module: promoted constructor properties (they are parameters, see ADR 0008) and checks that compare only a member's name, such as `__construct`.
