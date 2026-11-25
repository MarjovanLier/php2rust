# Read PHP with the Mago crates, pinned to one exact version

php2rust parses PHP and takes the type of every value from the Mago crates, and every Mago crate, direct and transitive, resolves to one exact version (`=x.y.z`, with `Cargo.lock` committed and checked by a test, P2R-81). The Rust toolchain is pinned to the `rust-version` of that Mago release (P2R-83). We chose Mago because it is a Rust library with a parser, a name resolver and a type analyser for PHP 8.5, so the tool needs no PHP runtime and no type inference of its own.

## Considered Options

- A version range for Mago: rejected, because a Mago release changes which findings it reports and how it words them, and the stop folders compare stderr byte for byte.
- A parser of our own, or PHP's own parser through a PHP process: rejected, because neither gives the types that the conversion depends on.

## Consequences

- A Mago upgrade is a deliberate change: pin the new version, then rewrite and review the stop folders whose text changes. The move from 1.57.0 to 1.61.0 changed 11 of them and required Rust 1.100.
- php2rust has its own rules for forms that PHP rejects but Mago accepts (for example a modifier written twice), because the output must not depend on Mago reporting them.
