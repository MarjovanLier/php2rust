# docs/spec.md is the contract, and requirement IDs tie code and tests to it

The behaviour of php2rust is defined in `docs/spec.md` as requirements with stable IDs (`P2R-46`). Each stop message ends with the ID of the rule that stopped it, test folders are named after the ID they exercise (`tests/cases/p2r-41-foreach-key-value`), and when the code and the spec disagree, the spec wins: the code is fixed, or the spec is changed first in its own commit. We chose this over documentation written after the code, because the supported subset of PHP grows one slice at a time and each slice must state exactly which forms convert and which stop.

## Consequences

- A user who reads a stop message can find the rule in the spec by its ID.
- Renaming or renumbering a requirement changes stop messages and test folder names, so IDs are never reused or renumbered.
