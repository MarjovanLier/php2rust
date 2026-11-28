---
paths:
  - "tests/**"
  - "src/**/*.rs"
---

# ADR 0004: Test through folders that run the tool, PHP and the Rust output

Rationale: `docs/adr/0004-folder-tests-against-php.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

- Each form that converts or stops has a folder (P2R-70 to P2R-73):
  - case folder: `input.php` and `expected.rs`. The output equals `expected.rs` byte for byte, and with a `main`, PHP and the compiled Rust print the same stdout.
  - stop folder: `input.php` and `expected.txt`. The tool exits 1 with exactly that stderr and writes no output.
- A unit test does not replace a folder. Unit tests are for helpers with a small interface of their own, such as `patterns.rs` and `members.rs`.
- `PHP2RUST_BLESS=1` rewrites every expected file. Use it only for an intended change of output, and review the whole diff before the commit. A refactor changes no expected file.
- The folder suite needs PHP 8.3, 8.4, 8.5 or 8.6 (`PHP2RUST_PHP` names the binary), rustc and rustfmt. To run one kind or one folder, see the README, section "Development".
