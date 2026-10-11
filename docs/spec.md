# php2rust specification

Status: draft for version 1, written 11 Oct 2026, revised the same day after
review by five reviewers.

php2rust converts typed PHP source into normal, readable Rust (`Vec`,
`String`, `i64`), without AI. It reads PHP with the Mago crates, uses Mago's
analyser for the type of every expression, and writes Rust only when every
part of the input converts. Code it cannot convert yet stops the conversion,
so each refusal shows what to add to the tool next.

This file is the behavioural contract. On conflict with the code, this file
wins: fix the code, or change this file first. Code and tests cite the
requirement IDs.

## 1. Command line

- **P2R-01** The tool MUST run as `php2rust <input.php> [-o <output.rs>]`.
  `php2rust --help` and `php2rust --version` MUST print to stdout and exit 0;
  `--version` prints the crate version.
- **P2R-02** On success the tool MUST write the Rust source to `<output.rs>`
  when `-o` is given, otherwise to stdout, and exit 0.
- **P2R-03** When the input holds any construct that does not convert
  (P2R-07), the tool MUST print every problem to stderr and exit 1. Each
  problem is one line, `path:line:column: message`, where `path` is the input
  path exactly as given on the command line, `line` and `column` are 1-based,
  and `column` counts bytes. Lines are sorted by line, then column, then
  message, and a repeated line is printed once. When one problem's source
  span contains or equals another's, only one is printed: the outer one;
  for equal spans, a php2rust problem instead of a Mago finding, and
  otherwise the first by message.
- **P2R-04** The tool MUST print the reason to stderr and exit 2 when the
  arguments are wrong or missing (the usage text goes to stderr), the input
  cannot be read, the input is not valid UTF-8, the input is not valid PHP
  (the parser reports any error), `-o` names the input file (the same file
  after resolving `.`, `..` and symbolic links), or the output file cannot
  be written. When any of these holds, the tool MUST exit 2 and MUST NOT
  print P2R-03 problems. Mago's parser recovers from broken input without
  failing, so the parse check is explicit.
- **P2R-05** On exit 1 or 2 the tool MUST write nothing to stdout and MUST
  NOT create, change or truncate the output file. A failed write MUST NOT
  leave a partial output file (review concern, not checked mechanically).
- **P2R-06** The same input MUST always give the same stdout bytes, the same
  output file bytes and the same stderr bytes.
- **P2R-07** Only the constructs that sections 3 to 5 list as converting may
  convert. Any other construct MUST stop the conversion, which means it is
  reported as a problem under P2R-03. Examples that stop in version 1:
  `exit` and `die`, `print`, `printf`, casts, heredoc and nowdoc,
  `namespace`, `use`, `global`, `static` variables, `const` and `define()`,
  constants other than `true` and `false` (`PHP_EOL`, `PHP_INT_MAX`),
  `return` outside a function, and interpolation forms other than those in
  P2R-45.

## 2. Analysis

- **P2R-10** The tool MUST analyse input as PHP 8.5. Input that uses PHP 8.5
  syntax outside the supported subset (the pipe operator `|>`) MUST exit 1,
  not 2.
- **P2R-11** Mago analyser findings at error level, and the
  `possibly-undefined-variable` finding at any level, MUST be reported as
  problems.
- **P2R-12** The input MUST start with `declare(strict_types=1);` as its
  first statement. When it is missing or not first, the conversion stops.
  Without it PHP converts scalar arguments silently (`"5"` to `5`), which
  Rust cannot copy. Strict mode still lets an `int` go where a `float` is
  declared (P2R-25).
- **P2R-13** Text outside PHP tags (inline HTML) MUST stop the conversion.

## 3. Types

- **P2R-20** Every expression, variable and return value MUST map to exactly
  one Rust type (parameters follow P2R-24):

  | PHP type | Rust type |
  |---|---|
  | `int` | `i64` |
  | `float` | `f64` |
  | `bool` | `bool` |
  | `string` | `String` |
  | `list<T>` (one element type) | `Vec<T>` |
  | `void` (return only) | no return type |

- **P2R-21** Narrowed types MUST widen to their base Rust type: literal values
  (`int(5)`, `'abc'`, `true`), integer ranges (`int<0, max>`,
  `positive-int`), refined strings (`non-empty-string`, `numeric-string`) and
  refined lists (`non-empty-list<int>`, `list{int, int}`). A union whose
  members all widen to the same Rust type maps to that type. The empty array
  `[]` (type `array{}`) widens to `Vec<T>`, with `T` taken from the other
  assignments and reads of the same variable, or from the declared parameter
  or return type it flows into (`return [];`, `f([])`); when no `T` is
  known, the conversion stops.
- **P2R-22** Any other type (`mixed`, a union of different base types,
  `array` without a list element type, `null`, objects) MUST stop the
  conversion with a problem that names the type.
- **P2R-23** Every function parameter and return type MUST be declared in the
  signature; a docblock may refine `array` to `list<T>`. A missing
  declaration stops the conversion.
- **P2R-24** A `string` parameter MUST become `&str` and a `list<T>`
  parameter MUST become `&[T]`. A function that assigns to or changes its own
  string or list parameter stops the conversion (PHP passes a copy).
- **P2R-25** An `int` value passed to a `float` parameter (declared or
  built-in) or returned from a function declared `float` MUST convert to
  `f64`: a literal as a float literal (`4` to `4.0`), any other value with
  `as f64`.
- **P2R-26** Assigning a `string` or list value to a variable, appending it to
  a list, or returning it MUST behave as a copy, as in PHP: a later change to
  one variable MUST NOT show in the other. The output copies (`.clone()`, or
  `.to_string()` and `.to_vec()` from a parameter) unless the source is not
  read again.

## 4. Names and structure

- **P2R-30** Functions and variables MUST become snake_case as heck's
  `to_snake_case` gives it (`getTotal` to `get_total`, `getHTTPResponse` to
  `get_http_response`). A name that is a Rust 2024 strict or reserved
  keyword (`gen` included) MUST become a raw identifier (`r#type`). A name
  whose Rust form is `self`, `Self`, `super`, `crate`, `_` or empty MUST stop
  the conversion.
- **P2R-31** These MUST stop the conversion: two PHP functions in the file,
  or two PHP names in one function, that give the same Rust name
  (`getTotal()` and `get_total()`); a variable whose Rust name equals the
  Rust name of a function in the file; a PHP function named `main`.
- **P2R-32** A call MUST use the declared or built-in function name,
  whatever the letter case of the call (PHP function names ignore case).
- **P2R-33** Statements outside functions MUST become `fn main()`. Functions
  declared at the top level MUST become `pub fn`, callable before their
  declaration, as in PHP. A file with no statements outside functions other
  than the P2R-12 `declare` MUST convert to a library file with no `main`.
  A function declared inside another statement stops the conversion.
- **P2R-34** Variables MUST keep PHP's function-wide scope. A variable's Rust
  type is the widened type of every assignment to it and every read of it; a
  `foreach` key is `i64` and a `foreach` value has the list's element type. A
  read converts only when every control-flow path to it assigns the
  variable, with all conditions treated as unknown; otherwise the conversion
  stops. A variable whose assignments give different Rust types stops the
  conversion.
- **P2R-35** An unused parameter and an unused `foreach` key or value MUST get
  a leading `_` (`_key`). A value that is assigned and never read before the
  next assignment or the end of the function stops the conversion. A
  parameter assigned before it is read stops the conversion.

## 5. Statements and expressions

- **P2R-40** `if`/`elseif`/`else`, `while`, `for`, `foreach`, `break`,
  `continue` and `return` MUST keep PHP behaviour. `continue` inside `for`
  MUST still run the step expression. `while (true)` and `for (;;)` MUST
  become `loop`. These stop the conversion: `break N` or `continue N` with N
  above 1, and a statement that no control-flow path reaches (after
  `return`, `break` or `continue`, after an `if`/`else` whose every branch
  leaves, or after a `loop` with no `break`).
- **P2R-41** `foreach` over a list MUST support value-only and `key => value`
  forms. These stop the conversion: by-reference `foreach`, a loop body that
  changes the list it iterates, and a key or value variable that a read
  after the loop can reach with no new assignment in between (PHP keeps the
  last key and value after the loop). A later `foreach` that reuses the name
  as its key or value converts, and its binding counts as a new variable
  for P2R-34 (it may have a different Rust type).
- **P2R-42** Conditions of `if`, `while`, `for`, and the operands of `&&`,
  `||` and `!`, MUST be `bool`. A non-bool condition (PHP truthiness) stops
  the conversion.
- **P2R-43** These operators MUST convert:
  - `+`, `-`, `*` and unary minus, with both operands `int` or both `float`
  - `%` on `int`
  - `+=`, `-=`, `*=`, with the same operand rule as `+`, `-`, `*`
  - `++` and `--` as a statement or a `for` step, on an `int` or `float`
    variable
  - `===` and `!==` with both operands the same type, `int`, `float`,
    `bool` or `string`
  - `==` and `!=` with both operands the same type, `int`, `float` or `bool`
  - `<`, `>`, `<=`, `>=` with both operands `int` or both `float`

  Parentheses in the output MUST follow Rust precedence, not the source text.
  An expression statement converts only when it is an assignment, an
  append, `++` or `--`, or a call to a function declared in the file; any
  other expression statement stops the conversion.
- **P2R-44** These MUST stop the conversion in version 1, because the Rust
  result would differ or needs more design: `/` (the result is `int` or
  `float` depending on the values), `**`, `<=>`, `==` or `!=` on strings
  (`"1e3" == "1000"` is true in PHP), `<` and the other ordering operators on
  strings, arithmetic or comparison that mixes `int` and `float`, `++` or
  `--` on a string, `++` or `--` used as a value, and an assignment (`=`,
  `+=`, `-=`, `*=`, `.=`) used as a value (`$a = $b = 5`).
- **P2R-45** Strings MUST build with `.`, `.=` and double-quoted
  interpolation of simple variables (`"Hi $name"`, `"Hi {$name}"`). Every
  joined value MUST be `string` or `int`. A `float` or `bool` value stops the
  conversion (PHP prints `0.1 + 0.2` as `0.3`, Rust as
  `0.30000000000000004`). Literal text inside a Rust format string MUST
  escape `{` as `{{` and `}` as `}}`, and values MUST be positional
  arguments (`{}`), never inline names.
- **P2R-46** `echo` with N `string` or `int` arguments MUST convert to N
  `print!` calls, one per argument, in source order (PHP prints each argument
  before it evaluates the next). A `float` or `bool` argument stops the
  conversion.
- **P2R-47** Lists MUST support the literal `[a, b]`, the empty literal `[]`
  (P2R-21), append `$a[] = x`, and read `$a[$i]` with an `int` index. Writing
  to an index (`$a[$i] = x`) stops the conversion.
- **P2R-48** These built-in functions MUST convert: `count()` on a list
  (length as `i64`), `strlen()` (byte length as `i64`), `intdiv()` (integer
  division towards zero) and `fdiv()` (IEEE float division). A call to any
  other function not declared in the file stops the conversion.
- **P2R-49** Literals MUST convert by their PHP value and type, never by
  copying their text:
  - Integer literals in decimal, hexadecimal, octal (`017` and `0o17`) and
    binary, with or without `_` separators, convert by value (`017` to `15`).
    Every integer literal MUST have type `i64` in the output, never Rust's
    default `i32`.
  - Float literals convert by value; one that overflows becomes
    `f64::INFINITY`. An integer literal that PHP reads as a float (above
    `PHP_INT_MAX`) converts as a float.
  - Single-quoted and double-quoted strings convert from their decoded bytes,
    escaped again for Rust. A string whose decoded bytes are not valid UTF-8
    (`"\xFF"`), or that holds an octal escape above `\377`, stops the
    conversion.

## 6. Output

- **P2R-50** The output MUST compile with no warnings under
  `rustc --edition 2024 -D warnings` from the pinned toolchain (P2R-83): as a
  program when it has `fn main()`, as a library (`--crate-type lib`)
  otherwise. When the tool cannot produce such code for an input, it MUST
  stop instead. The output MUST NOT contain `allow` or `expect` attributes.
  Clippy is not required on the output.
- **P2R-51** The output MUST be unchanged by `rustfmt --edition 2024 --check`
  from the pinned toolchain. The tool MUST NOT run rustfmt or any other
  external program. Version 1 writes every statement on one line: when
  rustfmt would lay a statement out over several lines, the tool MUST stop
  instead.
- **P2R-52** The first line of every output MUST be the comment
  `// Generated by php2rust. In release builds set overflow-checks = true, see the README.`
  Comments and docblocks from the input are not copied in version 1.

## 7. Known differences

- **P2R-60** The README MUST list these differences between PHP and the
  generated Rust:
  - An `int` that overflows in `+`, `-`, `*`, `+=`, `-=`, `*=`, `++`, `--` or
    unary minus: PHP turns it into a `float`. Rust panics in a debug build,
    and in a release build the value wraps silently unless the crate sets
    `overflow-checks = true` in `[profile.release]`.
  - `%` of `PHP_INT_MIN` by `-1`: PHP gives `0`, Rust panics in every build.
  - `intdiv(PHP_INT_MIN, -1)`, and division by zero in `intdiv()` or `%`:
    PHP throws an error (exit code 255 when uncaught), Rust panics (exit
    code 101) with a different message.
  - Reading a list at a negative index or past its end: PHP warns and gives
    `null`. Rust panics.
  - Comments and docblocks are not copied (version 1).

## 8. Tests and CI

- **P2R-70** Each case folder `tests/cases/<id>-<slug>/` holds `input.php`
  and `expected.rs`, where `<id>` is the requirement the case exercises
  (`p2r-41-foreach-key-value`). Converting `input.php` MUST give
  `expected.rs` byte for byte, and two separate runs MUST give the same
  bytes (P2R-06).
- **P2R-71** Each `expected.rs` MUST pass the checks of P2R-50 and P2R-51,
  and MUST start with the P2R-52 line.
- **P2R-72** For each case with `fn main()`, the PHP run
  `php -n -d display_errors=stderr -d error_reporting=E_ALL input.php` and
  the Rust program built with `rustc --edition 2024` (debug) MUST print the
  same stdout. Both MUST exit 0, the stdout MUST NOT be empty, and PHP's
  stderr MUST be empty. A case without `fn main()` gets the checks of P2R-70
  and P2R-71 only. Review concern, not checked mechanically: each case's
  stdout depends on every construct the case covers, and every construct of
  sections 3 to 5 appears in at least one case with `fn main()`.
- **P2R-73** Each stop folder `tests/stops/<id>-<slug>/` holds `input.php`
  and `expected.txt`. Run from inside the folder, once as
  `php2rust input.php` and once as `php2rust input.php -o out.rs`, the tool
  MUST exit 1 both times, its stderr MUST equal `expected.txt`, its stdout
  MUST be empty, and `out.rs` MUST NOT exist afterwards.
- **P2R-74** Each error folder `tests/errors/<slug>/` holds `args.txt` (the
  command-line arguments, one per line), an optional `input.php` and
  `expected.txt`. Run from inside the folder, the tool MUST exit 2, its
  stderr MUST equal `expected.txt`, its stdout MUST be empty, no output file
  may be created, and every file in the folder MUST be unchanged.
- **P2R-75** Every requirement ID from P2R-07 to P2R-49 MUST appear in the
  name of at least one case or stop folder. A test MUST check this from the
  folder names.
- **P2R-76** One command named in the README MUST rewrite every `expected.rs`
  and `expected.txt` from the current tool output; CI MUST NOT run it. A
  failing folder MUST be reported by name with its expected and actual
  output, and every folder MUST run when one fails.
- **P2R-77** CI MUST run on every push and pull request: `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test` and the
  `Cargo.lock` check of P2R-81, with the pinned toolchain (P2R-83) and
  PHP 8.5.

## 9. Repository

- **P2R-80** The licence MUST be MIT OR Apache-2.0, with both licence files.
- **P2R-81** Every Mago crate, direct and transitive, MUST resolve to one
  exact version: direct dependencies as `=x.y.z`, `Cargo.lock` committed,
  and CI checks every `mago-*` entry in `Cargo.lock`. The pinned version's
  `PHPVersion::LATEST` MUST be PHP 8.5, because Mago always builds its
  prelude with `LATEST`. Version 1 pins Mago 1.55.0.
- **P2R-82** The README MUST state what the tool does, how to install it
  (`cargo install --locked`), how to run it, the supported PHP, the refusal
  rules in short, the known differences (P2R-60) and the command of P2R-76.
- **P2R-83** The Rust toolchain MUST be pinned in `rust-toolchain.toml`. The
  `rust-version` in `Cargo.toml` MUST equal the `rust-version` of the pinned
  Mago crates (1.99.0 for Mago 1.55.0), and the pinned toolchain MUST be at
  least that version.

## 10. Roadmap (not yet specified)

Each milestone gets its own requirements in this file when its work starts.

- **Milestone 2:** classes, enums, interfaces, nullable `?T` to `Option<T>`,
  closures, `array<string, T>` maps that keep insertion order, exceptions,
  more built-in functions and constants (`PHP_EOL`), heredoc, and copying
  comments into the output.
- **Milestone 3:** test conversion, PHPUnit and Pest together, into Rust
  `#[test]` functions with the standard assert macros. Needs milestone 2
  (PHPUnit tests are classes, Pest tests are closures).
- **Later:** whole projects (a Composer package to a Cargo crate).

## Spec checklist

- [x] No `[NEEDS CLARIFICATION]` markers left. Decided in review: problems
  inside or equal to a reported span are not reported (P2R-03); exit 2 wins
  over exit 1 (P2R-04); comments are not copied in version 1 (P2R-52); `[]`
  takes its element type from the same variable or the declared type it
  flows into (P2R-21); a file of only functions becomes a library (P2R-33).
- [x] Every requirement is testable: P2R-07 to P2R-49 by case and stop
  folders (P2R-75 enforces this), P2R-01 to P2R-06 by error folders and
  tests, P2R-50 to P2R-52 by P2R-71, the rest by CI or a file in the
  repository. Stated review concerns (P2R-05 partial write, P2R-72 case
  coverage) are the only parts not checked mechanically.
- [x] No speculative features: later milestones carry no requirements yet.
