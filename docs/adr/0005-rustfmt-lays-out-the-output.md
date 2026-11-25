# The first rustfmt on PATH lays out the output, with an empty configuration

php2rust does not lay out Rust itself: every output goes through the first `rustfmt` on `PATH` with `--edition 2024` and an empty configuration file, so a `rustfmt.toml` near the output or in the user's configuration does not change it (P2R-190). We chose this over a printer of our own that matches rustfmt, because the output is meant to read as normal Rust, and matching rustfmt's layout rules by hand would be a second formatter to keep in step.

## Consequences

- php2rust needs `rustfmt` at run time; rustup's minimal profile leaves it out.
- The expected files depend on one rustfmt version, so the tests fail when `rustfmt --version` does not name the commit of the pinned rustc (P2R-83).
