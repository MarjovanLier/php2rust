---
paths:
  - "Cargo.toml"
  - "Cargo.lock"
  - "rust-toolchain.toml"
  - "tests/lockfile.rs"
---

# ADR 0002: Read PHP with the Mago crates, pinned to one exact version

Rationale: `docs/adr/0002-mago-pinned-to-one-version.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

- Every Mago crate, direct and transitive, resolves to one exact version: `=x.y.z` in `Cargo.toml`, and `Cargo.lock` committed (P2R-81, checked by `tests/lockfile.rs`).
- Never give Mago a version range. A Mago release changes the findings it reports and their words, and the stop folders compare stderr byte for byte.
- `rust-toolchain.toml` pins the `rust-version` of that Mago release (P2R-83).
- A Mago upgrade is its own change: pin the new version, then rewrite the stop folders whose text changes and review each diff.
- A form that PHP rejects and Mago accepts gets its own php2rust rule. The output must not depend on Mago reporting it.
