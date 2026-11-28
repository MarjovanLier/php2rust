---
paths:
  - "src/main.rs"
  - "src/convert/mod.rs"
  - "tests/folders.rs"
  - ".github/workflows/ci.yml"
---

# ADR 0009: Convert for every target PHP version from 8.3 to 8.6

Rationale: `docs/adr/0009-convert-for-every-target-php-version.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

- The targets come from `--php <version>` (8.3, 8.4, 8.5 or 8.6, repeatable). Without it, the targets are all four.
- Analyse once per target, with Mago's analyser set to that version. Never hard-code one `PHPVersion` in a new analysis or scan.
- The input converts only when every target converts and gives the same Rust. A stop for one target stops the run; only a stop that some targets give names them.
- Mago's analysis does not stop syntax that an older target rejects (a property hook, `new X()->m()`). A php2rust rule does, and every such rule names the versions it applies to. Syntax rules go in `screen` in `src/convert/mod.rs`; a built-in whose result changed reads the converter's `php` field where the built-in converts.
- A folder that needs a narrower set lists its versions in `php.txt`; the PHP side of that folder runs only under those versions.
- The folder runner reads the PHP binary from `PHP2RUST_PHP` and accepts only 8.3 to 8.6. CI runs the folder suite once per version.
- Moving to a Mago release whose `LATEST` is 8.6 is its own change (ADR 0002).
