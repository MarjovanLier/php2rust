# Test PHP and its Rust output with one differential harness, from fixtures written once

Beside the folder tests of ADR 0004, php2rust gets a differential harness: each fixture is one JSON file that states a call or a run and what it must give, and the same fixture runs on PHP and on the Rust that php2rust writes, under every supported PHP version (ADR 0009). PHP is the oracle: the harness first checks the fixture's expectation against PHP, then the Rust output against the same expectation, so a wrong fixture and a wrong conversion fail with different messages.

The harness does not keep a test runner in each language. From the fixture it writes one driver in PHP, a function `php2rust_fixture()` that calls the entry point with the fixture's arguments and prints each result. `echo` alone prints `1`, `1.0`, `"1"` and `true` alike, so the driver passes each result through a typed print helper of its own (`print_int(int $v)`, `print_string(string $v)` and so on) that writes the type before the value. A result of the wrong type then fails in PHP with a `TypeError`, because the driver declares `strict_types`, and does not compile in Rust; the one exception is the `int` that PHP's strict mode lets into a `float` parameter. PHP runs the subject with the driver. php2rust converts the subject and the driver together as a directory (P2R-170), which gives a library crate with no top-level statements (P2R-175), and the Rust runner builds that library crate with a one-line `main` that calls the converted `php2rust_fixture()`, then runs it. So the test itself is written once, in PHP, and converts like any other input; a fixture whose driver does not convert stops, as ADR 0001 requires, and that stop is the result. A probe on 28 Nov 2026 put a one-function subject and such a driver through `php2rust <dir> -o <out>` and built the crate with a one-line `main`; the Rust printed what PHP printed. The probe used plain `echo`, not the typed helpers.

An `e2e` fixture has no driver: its subject is a program with top-level statements, which converts as a single input with `fn main()` (P2R-72). The harness runs PHP and the built Rust program as processes in a temporary directory that holds the fixture's starting files, with the fixture's stdin and arguments.

There are three tiers, one directory each under `tests/differential/`:

- `unit/`: one function of a subject, called with the arguments of each case; its return value and its output are compared.
- `feature/`: one file or area of a subject (for example formatting or escaping), several entry points over one set of cases.
- `e2e/`: a whole program run as a process, with its stdin, arguments and starting files; its stdout, exit code and the files it writes are compared.

A fixture is `<tier>/<id>-<slug>.json`, where `<id>` is the requirement it exercises, as for folders (`unit/p2r-46-echo-scalars.json`). It names its subject: a hand-written PHP file under `tests/differential/subjects/`, or a path in the WordPress corpus (ADR 0011), which runs only when the corpus is present. A case gives its name, its arguments as JSON values (an integer and a float stay distinct), and for `e2e` its stdin, arguments and files, and the expected stdout, return value, exit code and files written. A fixture may list `php` versions when it applies to fewer than all.

## Considered Options

- A runner in each language that reads the JSON (a PHP script, and Rust code with `serde_json` that builds each converted function's arguments): rejected, because the Rust side would have to map JSON onto every converted signature, and two runners that must agree drift as two copies do (ADRs 0007 and 0008).
- Run WordPress's own PHPUnit tests through php2rust: rejected, because PHPUnit, its mocks and the test case classes are far outside what php2rust converts, and that code is GPL (ADR 0011). Its inputs can still be read when a fixture is written.
- More case folders only: rejected, because a case folder pins one program and its byte-exact Rust, and a function needs many inputs; writing one folder per input does not scale to WordPress.
- Expected values recorded from PHP by a command, as `PHP2RUST_BLESS=1` does for folders: rejected for now, because a recorded value is not reviewed; a fixture's expectation is written by hand and checked against PHP on every run.

This ADR is not implemented yet; no harness, fixture or directory `tests/differential/` exists until the implementing change lands.

## Consequences

- A fixture runs on Rust only when its whole subject converts, because php2rust writes nothing for a file with one stop (ADR 0001). A `unit` fixture on a WordPress file gives a Rust result only once that whole file converts; until then it runs on PHP alone, which still checks the fixture.
- The folder tests stay the contract for what php2rust writes; a fixture never replaces a case or stop folder for a form (ADR 0004).
- A fixture can test only what the driver's typed print helpers can show. Objects, resources and binary strings (JSON holds UTF-8 only) are out of reach until the form grows.
- Global state and starting globals need the converted driver to set them, which php2rust does not convert today, so a fixture that needs them stops until it does.
- The harness runs PHP once and rustc once per fixture and version; it needs PHP, rustc and rustfmt as the folders do, and `PHP2RUST_FIXTURE=<text>` keeps only the fixtures whose name holds the text, as `PHP2RUST_FOLDER` does.
- How the driver prints a map with keys that Rust orders differently, and whether a case may expect a stop instead of a result, are open questions for the change that implements this.

## Spec changes for the change that implements this

New requirements for the fixture format, the driver, the tiers and the runner, next to P2R-70 to P2R-76; P2R-75 (whether a fixture counts as cover for a requirement ID); P2R-76 (the rewrite command does not touch fixtures); P2R-77 (CI runs the harness); P2R-82 (the README names the harness and its filter).
