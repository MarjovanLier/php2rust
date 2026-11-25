# Run Kani and Verus proofs locally, through a memory guard, not in CI

The Kani proofs (`#[cfg(kani)]` modules in `src/`) and the Verus proof (`verus/octal.rs`) run only through `scripts/prove.sh`, never in CI. Kani 0.68 and its solvers have no memory option and macOS refuses `ulimit -v`, and on 24 Nov 2026 parallel Kani runs in another project used all the memory of the machine. So the script runs one tool in its own process group and stops the whole group above a memory or time cap.

## Considered Options

- Step Mago back to 1.59.0, the newest release that Kani's nightly compiles as it is: rejected. The script instead turns on `trim_prefix_suffix` for Kani builds only, so normal builds stay on Mago 1.61.0. Remove the flag when a Kani release ships a nightly on which the feature is stable.
- Prove the real function in Verus: not possible today, because Verus checks code written in its own syntax. The Verus proof checks a copy of `octal_overflow`, which must change when the function changes.

## Consequences

- A change to a proven function is not checked by CI; run `scripts/prove.sh kani` and `scripts/prove.sh verus` before committing one.
