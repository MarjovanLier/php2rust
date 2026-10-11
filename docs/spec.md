# php2rust specification

Status: draft for version 1, written 11 Oct 2026.

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
- **P2R-02** On success the tool MUST write the Rust source to `<output.rs>`
  when `-o` is given, otherwise to stdout, and exit 0.
- **P2R-03** When the input holds any construct the tool does not support,
  the tool MUST print every problem in the file to stderr, one per line, as
  `path:line:column: message`, in source order, and exit 1.
- **P2R-04** When the input cannot be read or is not valid PHP (the parser
  reports any error), the tool MUST print the reason to stderr and exit 2.
  Mago's parser recovers from broken input without failing, so this check is
  explicit.
- **P2R-05** On exit 1 or 2 the tool MUST NOT create, change or truncate the
  output file.
- **P2R-06** The same input MUST always give the same output bytes.

## 2. Analysis

- **P2R-10** The tool MUST analyse input as PHP 8.5.
- **P2R-11** Mago analyser findings at error level MUST be reported as
  problems (exit 1). Such code fails in PHP at run time, or its types cannot
  be trusted for conversion.
- **P2R-12** The input MUST start with `declare(strict_types=1);` as its first
  statement; otherwise the tool stops. Without it PHP converts scalar
  arguments silently (`"5"` to `5`), which Rust cannot copy.
- **P2R-13** Text outside PHP tags (inline HTML) MUST stop the conversion.

## 3. Types

- **P2R-20** Every expression, parameter, return value and variable MUST map
  to exactly one Rust type:

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
  members all widen to the same Rust type maps to that type.
- **P2R-22** Any other type (`mixed`, a union of different base types,
  `array` without a list element type, `null`, objects) MUST stop the
  conversion with a problem that names the type.
- **P2R-23** Every function parameter and return type MUST be declared in the
  signature; a docblock may refine `array` to `list<T>`. A missing
  declaration stops the conversion.
- **P2R-24** A `string` parameter MUST become `&str` and a `list<T>`
  parameter MUST become `&[T]`. A function that assigns to or changes its own
  string or list parameter stops the conversion (PHP passes a copy).

## 4. Names and structure

- **P2R-30** Functions and variables MUST become snake_case
  (`getTotal` to `get_total`). A name that is a Rust keyword MUST become a raw
  identifier (`r#type`).
- **P2R-31** Two PHP names that give the same Rust name in one scope MUST stop
  the conversion. A PHP function named `main` MUST stop the conversion.
- **P2R-32** A call MUST use the declared function name, whatever the letter
  case of the call (PHP function names ignore case).
- **P2R-33** Statements outside functions MUST become `fn main()`. Functions
  declared at the top level MUST become Rust functions callable before their
  declaration, as in PHP. A function declared inside another statement stops
  the conversion.
- **P2R-34** Variables MUST keep PHP's function-wide scope: a variable assigned
  inside a block and read after it converts. A read where PHP could see the
  variable unassigned stops the conversion. A variable whose assignments give
  different Rust types stops the conversion.

## 5. Statements and expressions

- **P2R-40** `if`/`elseif`/`else`, `while`, `for`, `foreach`, `break`,
  `continue` and `return` MUST keep PHP behaviour. `continue` inside `for`
  MUST still run the step expression. `break N` or `continue N` with N above
  1 stops the conversion.
- **P2R-41** `foreach` over a list MUST support value-only and `key => value`
  forms (the key is `i64`). By-reference `foreach` stops the conversion. A
  loop body that changes the list it iterates stops the conversion.
- **P2R-42** Conditions of `if`, `while`, `for`, and the operands of `&&`,
  `||` and `!`, MUST be `bool`. A non-bool condition (PHP truthiness) stops
  the conversion.
- **P2R-43** These operators MUST convert:
  - `+`, `-`, `*` and unary minus, with both operands `int` or both `float`
  - `%` on `int`
  - `+=`, `-=`, `*=`
  - `++` and `--` as a statement
  - `===` and `!==` on `int`, `float`, `bool` or `string`
  - `==` and `!=` with both operands the same type, `int`, `float` or `bool`
  - `<`, `>`, `<=`, `>=` with both operands `int` or both `float`
- **P2R-44** These MUST stop the conversion in version 1, because the Rust
  result would differ or needs more design: `/` (the result is `int` or
  `float` depending on the values), `**`, `<=>`, `==` or `!=` on strings
  (`"1e3" == "1000"` is true in PHP), `<` and the other ordering operators on
  strings, arithmetic that mixes `int` and `float`, and `++` or `--` used as
  a value.
- **P2R-45** Strings MUST build with `.`, `.=` and double-quoted
  interpolation of simple variables (`"Hi $name"`, `"Hi {$name}"`). Every
  joined value MUST be `string` or `int`. A `float` or `bool` value stops the
  conversion (PHP prints `0.1 + 0.2` as `0.3`, Rust as
  `0.30000000000000004`).
- **P2R-46** `echo` with one or more `string` or `int` arguments MUST convert
  to `print!`. A `float` or `bool` argument stops the conversion.
- **P2R-47** Lists MUST support the literal `[a, b]`, the empty literal `[]`
  when the element type is known, append `$a[] = x`, and read `$a[$i]` with an
  `int` index. Writing to an index (`$a[$i] = x`) stops the conversion.
- **P2R-48** These built-in functions MUST convert: `count()` on a list
  (length as `i64`), `strlen()` (byte length as `i64`), `intdiv()` (integer
  division towards zero) and `fdiv()` (IEEE float division). A call to any
  other function not declared in the file stops the conversion.

## 6. Output

- **P2R-50** The output MUST compile with `rustc --edition 2024 -D warnings`
  and MUST be formatted. When the tool cannot produce warning-free Rust for
  an input (for example an unused variable), it MUST stop instead.

## 7. Known differences

- **P2R-60** The README MUST list these differences between PHP and the
  generated Rust:
  - An `int` that overflows: PHP turns it into a `float`. Rust panics in a
    debug build, and in a release build the value wraps silently unless the
    crate sets `overflow-checks = true` in `[profile.release]`.
  - Reading a list past its end: PHP warns and gives `null`. Rust panics.
  - An uncaught error (division by zero in `intdiv()` or `%`): PHP ends with a
    fatal error and exit code 255, Rust panics with exit code 101 and a
    different message.

## 8. Tests and CI

- **P2R-70** Each supported construct MUST have at least one case folder
  `tests/cases/<name>/` holding `input.php` and `expected.rs`. Converting
  `input.php` MUST give `expected.rs` byte for byte.
- **P2R-71** Each `expected.rs` MUST compile with
  `rustc --edition 2024 -D warnings`.
- **P2R-72** For each case, `php -n input.php` and the compiled Rust program
  MUST print the same stdout. Both MUST exit 0, the stdout MUST NOT be empty,
  and PHP's stderr MUST be empty. `-n` (no `php.ini`) makes local runs and CI
  runs give the same output.
- **P2R-73** Each refusal rule MUST have at least one stop folder
  `tests/stops/<name>/` holding `input.php` and `expected.txt`. The tool MUST
  exit 1, its stderr MUST equal `expected.txt`, and no output file may be
  created.
- **P2R-74** CI MUST run on every push and pull request: `cargo fmt --check`,
  `cargo clippy -- -D warnings` and `cargo test`, with PHP 8.5 and stable
  Rust.

## 9. Repository

- **P2R-80** The licence MUST be MIT OR Apache-2.0, with both licence files.
- **P2R-81** All Mago crates MUST be pinned to one exact version.
- **P2R-82** The README MUST state what the tool does, how to run it, the
  supported PHP, the refusal rules in short, and the known differences
  (P2R-60).

## 10. Roadmap (not yet specified)

Each milestone gets its own requirements in this file when its work starts.

- **Milestone 2:** classes, enums, interfaces, nullable `?T` to `Option<T>`,
  closures, `array<string, T>` maps that keep insertion order, exceptions,
  more built-in functions.
- **Milestone 3:** test conversion, PHPUnit and Pest together, into Rust
  `#[test]` functions with the standard assert macros. Needs milestone 2
  (PHPUnit tests are classes, Pest tests are closures).
- **Later:** whole projects (a Composer package to a Cargo crate).

## Spec checklist

- [x] No `[NEEDS CLARIFICATION]` markers left.
- [x] Every requirement is testable by a case folder, a stop folder, CI, or a
  file in the repository.
- [x] No speculative features: later milestones carry no requirements yet.
