# Test through folders that run the tool, PHP and the Rust output

The main tests are folders, not unit tests (P2R-70 to P2R-73). A case folder holds `input.php` and `expected.rs`: the tool must give `expected.rs` byte for byte, and when the case has a `main`, PHP and the compiled Rust must print the same stdout. A stop folder holds `input.php` and `expected.txt`: the tool must exit 1 with exactly that stderr and write no output. `PHP2RUST_BLESS=1` rewrites the expected files, and the diff is reviewed before a commit. We chose this because the claim the tool makes is about whole programs (the Rust prints what PHP prints), which only a run of both can check.

## Consequences

- The folder test needs PHP 8.3, 8.4, 8.5 or 8.6 (`PHP2RUST_PHP`, ADR 0009), rustc and rustfmt on the machine, and takes about six minutes per PHP version.
- Unit tests cover helpers with a small interface of their own, such as the pattern converter (`patterns.rs`) and the member form (`members.rs`, ADR 0007); they do not replace a folder for a converted or refused form.
