---
paths:
  - "src/main.rs"
  - "src/printer.rs"
  - "rust-toolchain.toml"
---

# ADR 0005: The first rustfmt on PATH lays out the output, with an empty configuration

Rationale: `docs/adr/0005-rustfmt-lays-out-the-output.md`. The ADR wins on conflict; change this file in the same commit as the ADR.

- Leave the layout to rustfmt. Do not add code to the printer that copies rustfmt's layout rules.
- Every output goes through the first `rustfmt` on `PATH`, with `--edition 2024` and an empty configuration file (P2R-190). A `rustfmt.toml` near the output or in the user's configuration must not change the output.
- php2rust needs `rustfmt` at run time; rustup's minimal profile leaves it out.
- The expected files depend on one rustfmt version. The tests fail when `rustfmt --version` does not name the commit of the pinned rustc (P2R-83).
