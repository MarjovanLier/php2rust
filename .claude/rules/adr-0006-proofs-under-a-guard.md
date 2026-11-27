---
paths:
  - "scripts/prove.sh"
  - "verus/**"
  - "src/printer.rs"
  - "src/convert/numbers.rs"
---

# ADR 0006: Run Kani and Verus proofs locally, through a memory guard, not in CI

Rationale: `docs/adr/0006-proofs-run-locally-under-a-memory-guard.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

- Never run Kani or Verus without the script: Kani has no memory limit, and macOS refuses `ulimit -v`.
- CI does not check a proven function: `octal_overflow` in `src/printer.rs`, and the `#[kani::proof]` in `src/convert/numbers.rs`. Before you commit a change to one, run both script commands.
- `verus/octal.rs` is a copy of `octal_overflow`. When the function changes, change the copy.
- The script turns on `trim_prefix_suffix` for Kani builds only, so normal builds stay on the pinned Mago. Remove the flag when a Kani release ships a nightly where that feature is stable.
