---
paths:
  - "docs/spec.md"
  - "src/**/*.rs"
  - "tests/**"
---

# ADR 0003: docs/spec.md is the contract; requirement IDs tie code and tests to it

Rationale: `docs/adr/0003-spec-with-requirement-ids.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

- Each stop message ends with the ID of the rule that stopped it, for example `(P2R-31)`.
- When the code and the spec disagree, the spec wins. Fix the code, or change the spec in its own commit before the code.
- Never reuse or renumber an ID. A change of ID changes stop messages and folder names.
