# Convert for every target PHP version from 8.3 to 8.6, and stop where they differ

php2rust converts a file for a set of target PHP versions, each one of 8.3, 8.4, 8.5 and 8.6. The user names them with `--php <version>`, which may be given more than once (`--php 8.4 --php 8.5`). Without `--php`, the targets are all four. The tool analyses the input once per target, with Mago's analyser set to that version (`Settings::new(PHPVersion::PHP83)` and so on), and the input converts only when every analysis converts and gives the same Rust. A finding or a stop for any one target stops the conversion. A stop that every target gives prints as it does today; a stop that only some targets give names them. So a form that behaves differently between the targets converts only when it behaves the same in each of them, and stops otherwise (ADR 0001).

Three things catch a difference between versions, and none of them alone is enough:

- Mago's analysis per target catches a built-in symbol outside its availability window. On 28 Nov 2026 no input that php2rust converts shows this: each such symbol (`array_find`, new in 8.4) is also outside what php2rust converts, and its own stop covers the same span (P2R-03), so the check guards the built-ins that php2rust learns later.
- php2rust's own rules, each naming the versions it applies to, catch what that analysis does not: syntax that an older version rejects (a property hook, `new X()->m()` without parentheses, `final` on a promoted parameter), and a built-in whose result, warning or deprecation changed (PHP 8.5 deprecates the `(binary)` cast, and Mago does not report it).
- The folder tests run the PHP side of every case under each supported PHP binary, and CI runs them once per version, so a difference that no rule knows yet fails a test instead of reaching a user.

Implemented on 28 Nov 2026: P2R-01, P2R-03, P2R-10, P2R-72, P2R-73 and P2R-77 hold the rules.

We chose all four as the default because a converted program must print what the PHP prints on every version the user may still run, and a smaller default would let a form through that one of them runs differently. A user who runs one version narrows the set with `--php`.

Facts this rests on, checked on 28 Nov 2026:

- PHP 8.6.0 was released on 19 Nov 2026 (php.net). This machine has Homebrew's PHP 8.3, 8.4 and 8.5; `php@8.6` is in Homebrew but not installed. `shivammathur/setup-php`, which CI uses, lists 8.6 as stable.
- Mago 1.61.0, the newest release, defines `PHP83` to `PHP86`. `LATEST` is 8.5 and `NEXT` is 8.6, and the prelude of built-in symbols is scanned with `LATEST`. The prelude's stubs carry 330 `Mago\AvailableSince` attributes, and the analyser reports a symbol outside the configured version's window as an error (`utils/availability.rs`), which P2R-11 already turns into a stop.
- The parser takes no version. `PHPVersion::is_supported(Feature)` gates property hooks and the pipe operator at 8.4 and 8.5 and partial function application at 8.6, but of the crates php2rust uses only the analyser reads a gate, and only for asymmetric visibility (`visibility/mod.rs`), where the version decides who may write a readonly property; it reports no `private(set)` before 8.4. So an analysis set to 8.3 does not stop 8.4 syntax, and php2rust stops a set visibility before 8.4 itself (P2R-10).
- Of the 218 case folders with `fn main()`, the PHP run of P2R-72 gives the same output under PHP 8.3, 8.4 and 8.5 for 216. `p2r-130-readonly-classes` fails under 8.3 and 8.4 (`final` on a promoted parameter, new in 8.5), and `p2r-133-new-objects` fails under 8.3 (`new X()->m()` without parentheses, new in 8.4). With all four as the default, these two need a narrower set.
- CI installs PHP 8.5 only, and `tests/folders.rs` fails on any other version.

Not checked: the full list of built-in functions whose result, warning or deprecation differs between 8.3 and 8.6, and which functions PHP 8.6 adds that the 8.5 prelude lacks. A call to a function the prelude does not know stops already, so that gap stops rather than converts.

Found while implementing, on 28 Nov 2026 with Homebrew's PHP 8.6.0: of the 218 case folders with `fn main()`, 217 print the same under 8.6 as under 8.5. In `p2r-188-helpers-against-php`, PHP 8.6's `trim`, `ltrim` and `rtrim` also remove a form feed (`\x0C`) by default, which 8.3 to 8.5 keep, so a call with one argument now stops for the target 8.6 (P2R-182), and the five case folders that make such a call name 8.3 to 8.5 in `php.txt`. A lint of every PHP file under `tests/` with PHP 8.3, 8.4 and 8.5 found no other syntax in a converting case that an older version refuses.

## Considered Options

- One version, chosen by the user, with no default: rejected, because a missing option would then be an error for every run, and the safe choice is known.
- The newest version only, as today with 8.5: rejected, because output converted for 8.6 can print something else under 8.3 (a deprecation that 8.5 adds writes to stderr, and a changed built-in returns another value).
- Read the range from `composer.json` (`require.php`): rejected for now, because such a range is often open (`>=7.4`) or names versions outside 8.3 to 8.6, and the tool would have to guess. It can come later as a default that `--php` overrides.
- One analysis at the lowest target: rejected, because an analysis at 8.3 does not see a deprecation or a removal that a later target adds.

## Consequences

- Parsing and Mago's prelude stay once per run. The scan, the analysis and the conversion run once per target, because the conversion reads the analysis and the Rust of each target is compared. Building the prelude dominates a small run, so with it built once, a debug build converted `p2r-130-readonly-classes` in 0.37 s for four targets against 0.35 s for one. The files of `--include` are parsed and scanned once per target, so with Laravel 13.44.1's `vendor/` (6,105 PHP files) included, `p2r-350-data-struct` took 10.8 s for four targets against 3.6 s for one. Parsing the included files once is a later change if that cost matters.
- A form that only a newer version accepts (an 8.4 property hook, the 8.5 pipe operator) converts only with a `--php` set that leaves the older versions out. A folder that needs such a set names it in a file `php.txt` in the folder, one version per line, and the runner passes those versions to the tool and runs the PHP side only under them.
- php2rust's own version rules (a form PHP rejects and Mago accepts, ADR 0002) say which versions they apply to.
- PHP 8.6 support depends on Mago: until a Mago release makes 8.6 its `LATEST`, its prelude is that of 8.5. A Mago upgrade stays its own change (ADR 0002).
- The folder runner takes the PHP binary from `PHP2RUST_PHP` (default `php`) and fails when its version is not 8.3 to 8.6; a full local run is one run per installed binary.

## Spec changes for the change that implements this

P2R-01 (the `--php` option), P2R-03 (the versions in a stop that only some targets give), P2R-10 (analysis per target, not as 8.5), P2R-72 and P2R-73 (the PHP binary per run, and `php.txt`), P2R-77 (CI once per version), P2R-81 (the note that the prelude is built with `LATEST`), P2R-82 (the supported versions and the option in the README).
