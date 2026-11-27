---
paths:
  - "src/**/*.rs"
  - "docs/spec.md"
---

# ADR 0001: Convert only what behaves the same; stop on everything else

Rationale: `docs/adr/0001-convert-or-stop.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

- Convert a construct only when the Rust prints what the PHP prints (P2R-07, P2R-72). When that is not certain, stop.
- A stop is one `path:line:column: message` line on stderr and exit 1, and no output file is written (P2R-03, P2R-05).
- Never write best-effort output: no placeholder, no `todo!()`, no guessed type, no partial file.
- The same input gives the same bytes on every run (P2R-06). Nothing in the output may change between runs.
- A difference between PHP and Rust that cannot be closed is a known difference in the spec (P2R-60), never a silent change.
