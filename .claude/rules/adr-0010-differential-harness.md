---
paths:
  - "tests/**"
---

# ADR 0010: One differential harness for PHP and Rust, from fixtures written once

Rationale: `docs/adr/0010-one-differential-harness-for-php-and-rust.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

Not implemented yet: these checks apply to the change that builds the harness and to work after it.

- A fixture is one JSON file, `tests/differential/<unit|feature|e2e>/<id>-<slug>.json`. Write it by hand; never record its expectation from a tool run.
- PHP is the oracle. Check a fixture against PHP before you trust a Rust failure.
- Test both platforms through the one PHP driver that the harness writes from the fixture. Do not add a second runner in Rust or in PHP.
- Keep an integer and a float distinct in a fixture's JSON (`1` and `1.0`). The driver prints every result through a typed print helper, never through a bare `echo`.
- A fixture adds cover; it never replaces the case or stop folder of a form.
- A subject is a hand-written file under `tests/differential/subjects/`, or a corpus path (ADR 0011). Never copy corpus code into a subject.
