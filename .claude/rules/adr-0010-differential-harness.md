---
paths:
  - "tests/**"
---

# ADR 0010: One differential harness for PHP and Rust, from fixtures written once

Rationale: `docs/adr/0010-one-differential-harness-for-php-and-rust.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

The harness is `tests/differential.rs`, specified in section 53 of the spec (P2R-600 to P2R-609).

- A fixture is one JSON file, `tests/differential/<unit|feature|golden|e2e>/<id>-<slug>.json`. Write unit, feature and e2e expectations by hand; never record them from a tool run.
- A golden fixture lists inputs only. Record its files with `PHP2RUST_RECORD=1` on your machine, read every recorded file in the diff before you commit it, and never record in CI. `PHP2RUST_BLESS` must not write a golden file.
- Fix a value that changes per run by passing it to both sides as an argument; mask it in the fixture only when it cannot be passed.
- PHP is the oracle. Check a fixture against PHP before you trust a Rust failure.
- Test both platforms through the one PHP driver that the harness writes from the fixture. Do not add a second runner in Rust or in PHP.
- Keep an integer and a float distinct in a fixture's JSON (`1` and `1.0`). The driver prints every result through a typed print helper, never through a bare `echo`.
- A fixture adds cover; it never replaces the case or stop folder of a form.
- A subject is a hand-written file or directory under `tests/differential/subjects/`, or a corpus path (ADR 0011). Never copy corpus code into a subject.
- Run a Laravel subject's PHP side on the real packages pinned in `tests/differential/laravel/composer.lock`, through the harness bootstrap, never on an `include/` stand-in. Never commit `tests/differential/laravel/vendor/`.
