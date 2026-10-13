# php2rust specification

Status: version 1 written 11 Oct 2026 and revised the same day after review
by five reviewers. Milestone 2 is added one slice at a time. Section 11
(nullable types) was written 11 Oct 2026 and built on 12 Oct 2026; the
final review of the build added the list rules of P2R-102, the `for` step
rule of P2R-104 and the list places of P2R-106. Section 12 (enums) was
written and built on 12 Oct 2026; the final review of the build added the
case, name and call stops of P2R-110, P2R-111 and P2R-114 and the layouts
of P2R-115. Section 13 (`match`) was written and built on 12 Oct 2026;
the final review of the build added the type, constant and subject rules
at the end of P2R-122 and the call rule of P2R-121. Section 14 (readonly
classes) was written and built on 13 Oct 2026. Section 15 (classes that
change their objects) was written and built on 13 Oct 2026; the final
review of the build added the nullable stop of P2R-141, the unread field
write stop of P2R-142, the `&mut [C]` loop, the temporary list stop and
the copy rule wording of P2R-146, and the borrow limit of form 1 and the
list read limit of form 2 of P2R-147.

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
- **P2R-07** Only the constructs that sections 3 to 5 and 11 to 15 list as
  converting may convert. Any other construct MUST stop the conversion,
  which means it is reported as a problem under P2R-03. Examples that stop
  in version 1:
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
  | `?T` and `T\|null` | `Option<T>` (P2R-100) |
  | an enum `E` | the Rust enum `E` (P2R-112) |
  | a class `C` | the Rust struct `C` (P2R-132) |
  | `void` (return only) | no return type |

- **P2R-21** Narrowed types MUST widen to their base Rust type: literal values
  (`int(5)`, `'abc'`, `true`), integer ranges (`int<0, max>`,
  `positive-int`), refined strings (`non-empty-string`, `numeric-string`) and
  refined lists (`non-empty-list<int>`, `list{int, int}`), and enum cases
  (`Suit::Hearts` widens to `Suit`, section 12). A union whose
  members all widen to the same Rust type maps to that type. The empty array
  `[]` (type `array{}`) widens to `Vec<T>`, with `T` taken from the other
  assignments and reads of the same variable, or from the declared parameter
  or return type it flows into (`return [];`, `f([])`); when no `T` is
  known, the conversion stops.
- **P2R-22** Any other type (`mixed`, a union of different base types other
  than `T|null` (P2R-100), `array` without a list element type, the type
  `null` of a variable, parameter or return value (the literal `null`
  converts by P2R-102), objects other than enums and the classes of
  section 14) MUST stop the conversion
  with a problem that names the type.
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
  the conversion. A name whose Rust form is not an ASCII identifier (a
  letter or `_` first, then only ASCII letters, digits and `_`) MUST stop the
  conversion (`$_2` gives `2`; non-ASCII names trip rustc's confusable-name
  lints).
- **P2R-31** These MUST stop the conversion: two PHP functions in the file,
  or two PHP names in one function, that give the same Rust name
  (`getTotal()` and `get_total()`); a variable whose Rust name equals the
  Rust name of a function in the file; a PHP function named `main`; and a
  function with the name of a function PHP has built in (`next`, `touch`),
  which PHP rejects when it compiles the file and Mago does not report.
- **P2R-32** A call MUST use the declared or built-in function name,
  whatever the letter case of the call (PHP function names ignore case).
- **P2R-33** Statements outside functions, other than enum and class
  declarations (sections 12 and 14), MUST become `fn main()`. Functions
  declared at the top level MUST become `pub fn`, callable before their
  declaration, as in PHP. A file with no statements outside functions other
  than the P2R-12 `declare`, enum declarations and class declarations MUST
  convert to a library file with no `main`. After the P2R-52 line the output
  holds the enums, classes and functions in source order, then `fn main()`.
  A function declared inside another statement stops the conversion.
- **P2R-34** Variables MUST keep PHP's function-wide scope. A variable's Rust
  type is the widened type of every assignment to it and every read of it; a
  `foreach` key is `i64` and a `foreach` value has the list's element type. A
  read converts only when every control-flow path to it assigns the
  variable, with all conditions treated as unknown; otherwise the conversion
  stops. A variable whose assignments give different Rust types stops the
  conversion. A read that Mago gives no type, as for a variable read again in
  one expression after a property read (`$xs[$i]->n + $xs[$i]->n`), has the
  variable's type.
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
  forms. These stop the conversion: by-reference `foreach`, a key and a
  value with the same name, a loop body that changes the list it iterates,
  and a key or value variable that a read after the loop can reach with no
  new assignment in between (PHP keeps the last key and value after the
  loop). A key or value that reuses the name of
  an earlier local variable or an earlier `foreach` binding converts, and
  counts as a new variable for P2R-34 (it may have a different Rust type).
  A key or value that reuses a parameter name follows P2R-24 and P2R-35.
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
  append, `++` or `--`, a call to a function declared in the file, or a
  method call (P2R-135); any other expression statement stops the
  conversion.
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
  stop instead. Int arithmetic that the tool can prove overflows, and `%` by
  a divisor it can prove is 0, MUST stop the conversion: rustc rejects many
  such cases at compile time, and whether it does depends on how the value
  is used later. A function that calls itself on every path MUST stop the
  conversion (rustc rejects it as unconditional recursion). The output MUST
  NOT contain `allow` or `expect` attributes. Clippy is not required on the
  output.
- **P2R-51** The output MUST be unchanged by `rustfmt --edition 2024 --check`
  from the pinned toolchain. The tool MUST NOT run rustfmt or any other
  external program. Version 1 writes every simple statement (an expression
  statement, `return`, a `print!` from `echo`, a `let`) and every block
  opening line (`if`, `while`, `for`, `fn`) on one line: when rustfmt would
  split such a line, the tool MUST stop instead. Block statements span lines
  as rustfmt lays them out. An `if` or `else if` whose condition holds a
  `let` (section 11) MUST take one of these layouts, which are rustfmt's;
  any other condition with a `let` MUST stop the conversion:
  - the `let` alone: one line;
  - two operands, the first a variable, `!` and a variable, `true` or
    `false`, and the second the `let`: one line;
  - the `let` first, then one or more operands: the `if` or `} else if`
    line ends with the `let`, each further operand starts with `&&` on its
    own line one level deeper, and `{` is alone on the next line at the
    level of the `if`.

  A `let ... else` ends its `let` line with `else {`, puts its block on the
  lines after, and ends with `};`. Each line of these layouts follows the
  one-line rule above.
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
  - `E::from()` with a value that no case of the enum `E` has: PHP throws a
    `ValueError` (exit code 255 when uncaught), Rust panics (exit code 101).
  - An `int` property that overflows in `+=`, `-=`, `*=`, `++` or `--`: PHP
    throws a `TypeError` (exit code 255 when uncaught), because a typed
    property cannot hold the float; Rust panics in a debug build, as for a
    variable.
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
  sections 3 to 5 and 11 to 15 appears in at least one case with
  `fn main()`.
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
- **P2R-75** Every requirement ID from P2R-07 to P2R-49 and from P2R-100 to
  P2R-199 MUST appear in the name of at least one case or stop folder. A
  test MUST check this from the folder names.
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

- **Milestone 2:** classes, enums, `match`, interfaces, closures,
  `array<string, T>` maps that keep insertion order, exceptions, more
  built-in functions and constants (`PHP_EOL`), heredoc, and copying
  comments into the output. It is built one slice at a time, each slice in
  its own section from section 11 on, in this order: nullable types
  (section 11), enums (section 12), `match` (section 13), readonly classes
  (section 14), classes that change their own properties (section 15),
  interfaces, maps,
  closures, exceptions, then built-in functions, constants, heredoc and
  comments. Decided on 11 Oct 2026:
  objects become plain structs, and code that changes one object through two
  variables stops; maps use the `indexmap` crate, so output with a map needs
  it; a `throw` becomes an `Err` of a `Result`.
- **Milestone 3:** test conversion, PHPUnit and Pest together, into Rust
  `#[test]` functions with the standard assert macros. Needs milestone 2
  (PHPUnit tests are classes, Pest tests are closures).
- **Later:** whole projects (a Composer package to a Cargo crate).

## 11. Nullable types (milestone 2, slice 1)

Sections 3 to 9 apply to nullable values unless this section says otherwise.

- **P2R-100** A nullable type MUST map to `Option<T>`, where `T`, the inner
  type, is the Rust type that P2R-20 and P2R-21 give its non-null part: `?T`
  and `T|null` (`?int` to `Option<i64>`, `?list<string>` to
  `Option<Vec<String>>`, `list<?int>` to `Vec<Option<i64>>`). A docblock may
  refine `?array` to `?list<T>` (P2R-23). A union whose non-null members give
  different Rust types (`int|string|null`) MUST stop the conversion (P2R-22).
- **P2R-101** A nullable `string` parameter MUST become `Option<&str>` and a
  nullable `list<T>` parameter MUST become `Option<&[T]>`. P2R-24 and P2R-26
  apply to the value inside the `Option`: a function that assigns to its own
  nullable string or list parameter stops the conversion, and an assigned,
  appended or returned nullable string or list behaves as a copy.
- **P2R-102** The literal `null` MUST convert to `None`. Like `[]` (P2R-21),
  `null` takes its type `Option<T>` from the other assignments and reads of
  the same variable, or from the declared parameter, return or element type
  it flows into; when no `T` is known, the conversion stops. A variable whose
  assignments mix `null` or `?T` values with `T` values has the type
  `Option<T>` (an exception to P2R-34); a `?T` value is any value of Rust
  type `Option<T>`, also where Mago types it as `T`. A value of Rust type `T`
  MUST become `Some(...)` where it goes into an `Option<T>` variable,
  parameter, return value or list element: `return $y;` in a function
  declared `?int` gives `return Some(y);`, and `f(5)` for a `?int` parameter
  gives `f(Some(5))`. P2R-25 applies inside `Some` for `?float` (`f(4)` gives
  `f(Some(4.0))`). A list literal where a list of `Option<T>` is expected (a
  declared parameter or return type, `?list<...>` included, or the type of
  the variable it is assigned to) MUST take that type: `[1, 2]` gives
  `[Some(1), Some(2)]`, `[null, null]` gives `[None, None]`, and `[]` takes
  the expected list type. Any other list of `T` where a list of `Option<T>`
  is needed stops the conversion. A list variable holds `Option` elements
  only when its first write does; appending `null` to a list of `T` stops the
  conversion.
- **P2R-103** `$v === null` and `null === $v` MUST convert to `v.is_none()`,
  and `$v !== null` and `null !== $v` to `v.is_some()`, for any expression
  `$v` of Rust type `Option<T>`. `===` and `!==` between two `Option<T>`
  values, or between an `Option<T>` value and a `T` value, MUST convert to
  `==` and `!=` on `Option` values when P2R-43 or P2R-112 allows `===` on
  `T`; a string
  side compares as `Option<&str>` (`v.as_deref()`, `Some("ann")`). These
  stop the conversion: `==`, `!=`, `<`, `>`, `<=` and `>=` with an `Option`
  or `null` operand (`null == 0` is true in PHP), and a comparison with
  `null` of a value whose Rust type is not `Option`.
- **P2R-104** A read of a variable, parameter or list element whose Rust type
  is `Option<T>` MUST have that type, also where Mago types the read as `T`
  (after a check against `null` or an assignment). Only these two forms give
  a binding of type `T`. The binding has the Rust name of `$v` and is a new
  variable, as a `foreach` value is (P2R-41).
  - `$v !== null` (or `null !== $v`), where `$v` is a variable or parameter,
    as the condition of an `if` or `elseif` or as an operand of the
    top-level `&&` chain of that condition, becomes `let Some(v) = ...` in an
    `if let` or `else if let` (a let chain when the condition has other
    operands). The operands after it and the body read the binding.
  - `if ($v === null)` (or `null === $v`), where `$v` is a variable or
    parameter, with no `elseif` or `else` and a body that always leaves by
    `return`, `break` or `continue`, becomes
    `let Some(v) = ... else { ... };`. The statements after it in the same
    block read the binding.

  For an inner type `string` or `list<T>`, the binding has the parameter type
  of P2R-24 (`&str`, `&[T]`) and follows P2R-26 as a parameter does. When
  nothing reads the binding, the condition converts by P2R-103 instead
  (`if v.is_some() {`, `if v.is_none() {`). A write to `$v` in the body of
  such an `if let`, or anywhere after such a `let ... else` in its block
  (nested blocks included), stops the conversion. A read or write of `$v` in
  the step of a `for` whose body holds such a `let ... else` at any depth
  stops the conversion: the step also runs at each `continue`, where the
  binding can shadow the variable or not exist yet.
- **P2R-105** `$a ?? $b` MUST convert when `$a` is a variable, parameter,
  function call or `E::tryFrom()` call (P2R-114) of Rust type `Option<T>` and
  `$b` has the type `T`. `$b` MUST
  be evaluated only when `$a` is null, as in PHP, and P2R-26 applies to
  `$a`. These stop the conversion: `??` whose left side has a Rust type that
  is not `Option` or is a list element (PHP checks it with `isset`), any
  other type of `$b` (`$r ?? []` included), and `??=`.
- **P2R-106** A value of Rust type `Option<T>` MUST stop the conversion in
  these places, where PHP gives `null` a meaning Rust does not copy or Rust
  needs a `T`: an operand of `.`, `.=`, interpolation or `echo` (PHP prints
  `null` as an empty string); an operand of `+`, `-`, `*`, `%`, unary minus,
  `+=`, `-=`, `*=`, `++` or `--`; a condition or an operand of `&&`, `||` or
  `!` (P2R-42); the list of a `foreach`; the list or the index of a list
  read or an append; an element of a list literal, or an appended value,
  whose list holds values that cannot be null; an argument of a built-in
  function (P2R-48), of `E::from()` or of `E::tryFrom()`; the enum of
  `->name` or `->value`; the subject of a `match` (P2R-120); an argument for
  a parameter that is not nullable; and a `return` value in a function
  whose return type is not nullable.
  These also stop: a parameter
  default value, and `return;` or a path that reaches the end of a function
  with a nullable return type.

## 12. Enums (milestone 2, slice 2)

- **P2R-110** An `enum` declared at the top level, pure or backed by `int` or
  `string`, whose body holds only `case` lines, MUST become a Rust `pub enum`
  with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and one variant per
  case, in source order (P2R-33 places it in the output). A backing value
  MUST be an int literal, with or without unary minus, or a string literal,
  converted by P2R-49. These stop the conversion:
  - an enum declaration after a statement outside functions other than the
    P2R-12 `declare`, a function, enum or class declaration: PHP
    declares an enum only when execution reaches it, so an earlier use dies
    with `Class "E" not found`;
  - an enum with a method, a constant, `implements` or `use`, or a backing
    value of any other form (`1 + 1`, `PHP_INT_MAX`);
  - a case without a value in a backed enum, a case with a value in a pure
    enum, and a backing type other than `int` or `string`: PHP rejects all
    three when it compiles the file;
  - an attribute on an enum or a case (P2R-07);
  - an enum declared inside a function or another statement.
- **P2R-111** The Rust names of an enum and its cases MUST be heck's
  `to_upper_camel_case` of the PHP names (`Suit` stays `Suit`, `IN_REVIEW`
  becomes `InReview`), and MUST follow the identifier rules of P2R-30 (an
  ASCII identifier that is not a keyword, `Self` or empty). Every use of an
  enum or a case MUST take the Rust name of its declaration, whatever the
  letter case of the use (`suit::Hearts` gives `Suit::Hearts`), as P2R-32
  does for functions. These stop the conversion: a name that breaks those
  rules; two enums, or two cases of one enum, with the same Rust name; an
  enum named `Option`, `String`, `Vec` or `ToOwned`, which the output uses;
  and an enum with a name PHP reserves, a keyword or a type name (`list`,
  `readonly`, `match`, `bool`, `self` and the others; `enum` is allowed), or
  the name of a class, interface, enum or trait PHP has built in
  (`Exception`, `Countable`), which PHP rejects. Mago's parser accepts a
  keyword there, so the tool checks the name itself.
- **P2R-112** An enum type `E` MUST map to the Rust enum `E` (P2R-20), passed
  and returned by value; `?E` maps to `Option<E>` (P2R-100) and `list<E>` to
  `Vec<E>`. A case `E::Name` MUST convert to `E::Name`. An enum value is
  Copy, so an assignment needs no copy (P2R-26). `===`, `!==`, `==` and `!=`
  between two values of one enum type MUST convert to `==` and `!=`: PHP
  compares enum cases by identity with all four. `<`, `>`, `<=`, `>=`, and
  every operator with an enum operand and an operand of another type, stop
  the conversion.
- **P2R-113** `$e->name` MUST give the case's PHP name and `$e->value` the
  backing value of a backed enum: `e.name()` and `e.value()`. `name()` and a
  string `value()` return `&'static str`, which follows P2R-24 and P2R-26 as
  a string parameter does; an int `value()` returns `i64`. `->name` and
  `->value` inside an interpolated string stop the conversion (P2R-45).
- **P2R-114** `E::from($v)` MUST convert to `E::from(v)` of type `E`,
  `E::tryFrom($v)` to `E::try_from(v)` of type `Option<E>` (P2R-100), and
  `E::cases()` to `E::cases()` of type `Vec<E>`, with the cases in source
  order. The argument of `from` and `try_from` converts as an argument for a
  parameter of the backing type (P2R-24): a string as `&str`, an int as
  `i64`. `E::from()` with a value that no case has panics (P2R-60). These
  stop the conversion: `E::from()` on an enum without cases, which never
  returns, so rustc rejects the code after it (P2R-50); a named or spread
  argument (P2R-07); and any other static call on an enum.
- **P2R-115** The output MUST hold, for each enum, only the methods the file
  uses, in one `impl E` block right after the enum, in the order `name`,
  `value`, `from`, `try_from`, `cases`, with one blank line between methods.
  `name`, `value`, `from` and `try_from` are a `match` with one arm per case
  in source order; `from` ends with
  `_ => panic!("{:?} is not a valid backing value", value),` and `try_from`
  with `_ => None,`. All of these take rustfmt's layout (P2R-51): an arm
  wider than 100 columns becomes `=> {`, its value on the next line one level
  deeper, and `}` with no comma; a `match` without arms, for an enum without
  cases, is `match self {}` on one line; the `vec!` of `cases` holds the
  cases on one line when there is one case or they fit in 60 columns,
  otherwise one case per line with a comma after each. An arm with non-ASCII
  text that is longer than 100 bytes stops the conversion, because rustfmt
  counts columns and the tool counts bytes.

## 13. match (milestone 2, slice 3)

- **P2R-120** A `match` expression MUST convert to a Rust `match` when it is
  the whole value of a `return` or of an assignment to a variable
  (`$v = match (...) {...}`), its subject has Rust type `i64`, `bool`,
  `String` or `&str`, or an enum of the file with at least one case, and
  every arm condition is a literal of the subject's type (an int literal
  with or without unary minus, `true`, `false`, or a string literal,
  converted by P2R-49) or a case of the subject's enum. These stop the
  conversion:
  - a `match` anywhere else: an argument, an operand, `echo`, an append, a
    compound assignment, a `for` init or step, an arm result, a statement on
    its own;
  - a subject of any other type, among them a subject of Rust type
    `Option<T>` (P2R-106), also where Mago types it as `T` after a check
    against null, and a subject of an enum without cases, because rustc
    rejects the code around such a `match` as unreachable (P2R-50);
  - a condition of any other form: a variable, a call, `null`, an
    expression, or an integer literal that PHP reads as a float (P2R-49);
    so `match (true)` with conditions stops;
  - more than one `default` arm: PHP rejects the file when it compiles it,
    and Mago reports nothing;
  - an arm whose result Mago gives no type, because Mago finds the arm
    unreachable (after an arm that is always true, or because an earlier
    check or assignment narrowed the subject), other than a `default` that
    P2R-121 leaves out.
- **P2R-121** Several conditions in one arm MUST become one pattern joined by
  ` | `, in source order, and `default` MUST become `_`, written as the
  last arm wherever it stands in the source: PHP tries `default` only when
  no other arm matches. When the other arms cover every case of the enum, or
  both `true` and `false`, the `default` arm is left out, because Rust
  rejects a `_` that no value reaches; its contents are not converted, a
  read inside it does not count as a read (P2R-35), and a call inside it
  does not count as a call (P2R-50). Two conditions with the
  same value in one `match` (`16` and `0x10`, `'a'` and `"a"`) stop the
  conversion, because Rust rejects the second as unreachable. The tool
  decides whether the arms of a `match` without `default` cover every value
  from the subject's Rust type and the enum's declared cases, never from the
  type Mago gives the subject, because Mago narrows that type after a check,
  an assignment, a `%` or a docblock and then reports nothing, while rustc
  rejects the `match`. Without `default`, a `match` stops when its subject is
  an int or a string, when its subject is a bool and `true` or `false` is
  missing, and when its subject is an enum and a case is missing. Where Mago
  does report `match-not-exhaustive` (P2R-11), PHP throws an
  `UnhandledMatchError`.
- **P2R-122** Each arm result MUST convert as the value of its `return` or
  assignment does, with P2R-25, P2R-26, P2R-102 and P2R-106 applied to each
  arm result, never to the whole `match`, and the arm results MUST have one
  Rust type after that. A `let` whose `match` has no arm result that fixes
  the Rust type (int literals, `None`, and `Some(..)` or lists built only
  from them do not) names the type, as other first writes do (P2R-49):
  `let v: i64 = match x {`, `let v: Option<i64> = match x {`. A new variable
  takes its type from the arm results and the type Mago gives the whole
  `match`; when they give no one Rust type, the conversion stops: arms that
  are all `[]` or all `null`, or list arms whose elements differ in whether
  they may be null (`[null, 1]` and `[5]`). A variable that already has a
  type gives that type to every arm. A `match` with one Rust arm is a
  constant to rustc, so P2R-50 applies to its value as to a plain
  assignment. An int subject that nothing types gets `_i64` (P2R-49). Only
  the arm PHP chooses runs, as in PHP. A subject of Rust type `String`
  compares as `s.as_str()`; a subject that is already `&str` (a string
  parameter, a P2R-104 binding, `->name`, a string `->value`) stays as it is.
- **P2R-123** The `return`, `let` or assignment line MUST end with
  `match subject {`; each arm MUST be one line one level deeper, ending with
  `,`; and `};` closes the `match` at the level of the opening line, as
  rustfmt writes it. The opening line and each arm line follow the one-line
  rule of P2R-51: when rustfmt would split one of them, or an arm's pattern
  or value, the conversion stops.
- **P2R-124** For P2R-34 and P2R-35, each arm MUST be its own path from the
  subject, and the paths join after the `match`.

## 14. Readonly classes (milestone 2, slice 4)

- **P2R-130** A `readonly class` declared at the top level, with or without
  `final`, MUST become a Rust `pub struct` with `#[derive(Clone, Debug)]` and
  one `pub` field per property, in source order, followed by an `impl` block
  that holds `pub fn new` and the class's methods (P2R-33 places both). Its
  body MUST hold only methods (P2R-135) and at most one constructor
  `__construct` whose parameters all promote to properties and whose body is
  empty. A parameter promotes with any promotion modifier: a visibility,
  `readonly`, `final` or an asymmetric visibility such as `private(set)`. A
  constructor docblock `@param` or an inline `@var` on a promoted parameter
  may refine `array` to `list<T>` (P2R-23). A class without a constructor
  gets a `new` without parameters. PHP declares such a class before any
  statement runs, so it may come after the statements outside functions.
  These stop the conversion:
  - `extends`, `implements`, `abstract`, a trait, a constant, a property that
    is not promoted, a static member, a constructor parameter that is not
    promoted, a constructor with a body, and a parameter default value;
  - a promoted parameter without a type, with a property hook, by reference
    or variadic, which PHP rejects or makes fail and Mago does not report;
  - a method whose PHP name starts with `__`, other than the constructor,
    and an attribute on the class or a member (P2R-07);
  - a class that is not `readonly` (section 15 converts it), and a class
    declared inside a function or another statement;
  - a property whose type does not convert (P2R-20, P2R-22), a nullable
    class type (`?C`, a later slice), a property that holds its own class
    directly or through other classes other than through a list, because
    the Rust struct would have no size, and a property of an enum without
    cases, directly or through other classes other than through a list,
    because no value of the struct could exist and rustc rejects the code
    after its `new` (P2R-50).
- **P2R-131** The Rust name of a class MUST follow P2R-111: heck's
  `to_upper_camel_case` of the PHP name, the identifier rules of P2R-30, and
  every use takes the declaration's Rust name, whatever its letter case.
  Classes and enums share one namespace, as in PHP: two classes, or a class
  and an enum, with the same Rust name stop the conversion, and so does every
  name P2R-111 refuses for an enum. A class name inside a Mago type is read
  whatever its letter case, as Mago keeps the case a declaration wrote
  (`point` for `Point`). Property names follow P2R-30 (`$firstName` becomes
  `first_name`); method names follow P2R-30, and a call may use any letter
  case (P2R-32). These stop the conversion: two properties or two methods of
  one class with the same Rust name; a method whose Rust name is `new`, which
  `new` takes; and a method whose Rust name is `clone`, which would replace
  the copy of P2R-132 at every `.clone()`.
- **P2R-132** A class type `C` MUST map to the Rust struct `C` (P2R-20), and
  `list<C>` to `Vec<C>`; Mago's `$this(C)` widens to `C` (P2R-21). A `C`
  parameter becomes `&C`, as a string parameter becomes `&str` (P2R-24), and
  a returned `C` is owned. A value of class type is not Copy: every place
  where P2R-26 copies a string (an assignment, an append, a list element, a
  `return`, a `match` arm result) copies it, with `.clone()` unless the
  source is an owned variable that is not read again. `$this` and a `C`
  parameter are references, so they always copy with `.clone()` there. As an
  argument, `$this` is `self` and a `C` parameter is passed as it is. These
  stop the conversion: a nullable class type `?C`, and a function or method
  that assigns to its own `C` parameter, as P2R-24 stops one for a string or
  list.
- **P2R-133** `new C(args)` MUST convert to `C::new(args)`, with the arguments
  converted as for a function whose parameters are the constructor's (P2R-24,
  P2R-25). `C::new` takes `&str` for a string, `&[T]` for a list and `&D` for
  a class, and stores an owned copy of each: `name.to_string()`,
  `xs.to_vec()`, `d.clone()`, and `s.map(ToOwned::to_owned)` for an
  `Option<&str>` or `Option<&[T]>`; a Copy value is stored as it is. The
  store counts as a read of each parameter, so no constructor parameter gets
  the `_` of P2R-35. `new C` without parentheses converts as `new C()`. `new`
  with any other class form (a variable, `static`, `self`, an anonymous
  class) stops the conversion, and so does a call to `__construct` or to any
  other method whose name starts with `__`.
- **P2R-134** A property read `$o->p` MUST convert to `o.p`, and `$this->p`
  inside a method to `self.p`. A property read has the Rust type of the
  property's declaration, and a method call the Rust type of its declared
  return type, also where Mago narrows the type (after a check against null)
  or gives none (the object of a property read, a second read of one
  variable): a nullable property read after a check is still an `Option`, so
  its plain uses stop by P2R-106. A field of a Copy type reads as its value.
  A field of type `String`, `Vec<T>` or a class lends a reference, as a
  string parameter does: it is read in place where Rust takes a reference
  (`echo`, joining, a comparison, an argument), and copied with `.clone()`
  at every place where P2R-26 copies (`self.name.clone()` in a `return`). An
  `Option<String>` or `Option<Vec<T>>` field lends `.as_deref()` as an
  argument. A `foreach` over a list field borrows it when its elements are
  Copy (`for &s in &self.scores {`) and iterates a copy otherwise
  (`for n in self.names.clone() {`), with the rules of P2R-41. rustfmt counts
  a field read as an element of a chain: a chain of two or more `.` elements
  after its root that is wider than 60 columns stops the conversion
  (P2R-51). These stop the conversion: a write to a property (Mago reports
  one outside the constructor, P2R-11), `?->`, a property name that is not a
  plain identifier, a property read inside an interpolated string (P2R-45),
  and a property read or a method call on the left of `??` (a later slice).
- **P2R-135** A method MUST become `pub fn name(&self, params) -> R` in the
  `impl` block, after `new`, in source order, with one blank line between
  methods. Its parameters, return type and body follow the rules for
  functions (sections 3 to 5 and 11 to 13), with `$this` read as `self`.
  `$o->m(args)` MUST convert to `o.m(args)`, and `$this->m(args)` to
  `self.m(args)`, also as a statement (P2R-43). A method that calls the same
  method on every path, through any object of its class (`$this`, a
  parameter, a list element, a new object), stops the conversion, as a
  function that calls itself does (P2R-50): rustc matches the method, not
  the object. These stop the conversion: a static method; `self::`,
  `static::` and `parent::`; the native types `self`, `static` and `parent`
  (a later slice; a docblock type that names `self`, such as `list<self>`,
  is the class to Mago and converts); `$this` outside a method, a write to
  `$this` and a parameter named `$this`, which PHP rejects and Mago does not
  report; a by-reference parameter; and a call to a method the class does
  not declare (Mago reports it, P2R-11).
- **P2R-136** These MUST stop the conversion: `===`, `!==`, `==`, `!=`, `<`,
  `>`, `<=`, `>=` and `<=>` with an object operand, because Rust has no
  object identity and PHP compares properties loosely; `echo` and string
  joining of an object; `instanceof`; `clone`, also with new property
  values; a cast of an object; and an object as a condition.
- **P2R-137** The struct MUST list one field per line (`    pub x: i64,`),
  and one blank line separates it from `impl C {`. `new` comes first in the
  `impl` block, as `pub fn new(params) -> C {` with the constructor's
  parameters on that line (the one-line rule of P2R-51), and its body is the
  struct literal as the tail expression: on one line, `C { x, y }`, when the
  text between the braces is at most 18 columns, otherwise one field per
  line with a comma after each, as rustfmt writes it. A field that stores
  its parameter unchanged uses the shorthand `x`; any other field stores the
  copy of P2R-133 (`name: name.to_string()`). A class without properties
  gives `pub struct C {}` and `C {}`. The methods follow `new`, each after
  one blank line.

## 15. Classes that change their objects (milestone 2, slice 5)

PHP passes an object as a handle, so two variables can name one object and
a change through one shows through the other. A Rust struct is a value. The
rules below keep exactly one Rust value per PHP object, so the difference
never shows; code that needs two names for one changing object stops, as
decided on 11 Oct 2026.

- **P2R-140** A class declared at the top level without `readonly`, with or
  without `final`, MUST convert as section 14 converts a readonly class,
  with these differences. Its properties are its promoted constructor
  parameters, then the properties its body declares, in source order. A
  declared property needs a type and a default value that is a literal of
  that type: an int or float literal, with or without unary minus (an int
  literal for a float property stores the float, `1.0`); a bool literal; a
  string literal (`String::from("hi")`); `null`, or a literal of the inner
  type (`Some(5)`), for a nullable type; `[]` for a list type; or a case of
  an enum. `new` stores the default (`count: 0`, `items: Vec::new()`,
  `note: None`, `side: Side::Left`). A promoted parameter with `readonly` is
  set only by `new` (Mago reports a write anywhere else, P2R-11). These stop
  the conversion: a declared property without a type or a default, with a
  default of any other form (a list with elements included), with a hook,
  `static`, or `readonly` (PHP rejects a default on a readonly property and
  Mago does not report it); `var`; and the stops of P2R-130 other than "a
  class that is not readonly".
- **P2R-141** A changing type is a class with a property that can change (a
  class that is not readonly with a property that is not `readonly`), a
  class with a property of a changing type, and a list of a changing type.
  A struct of a changing type MUST derive only `Debug`, because its value is
  never copied (P2R-145); every other struct derives `Clone` and `Debug` as
  before. A nullable type that holds a changing type (`?list<Cell>`, also as
  a list element) stops the conversion wherever it is declared or written,
  because the `Option` forms of P2R-101 to P2R-105 copy or borrow it in ways
  this section does not define.
- **P2R-142** A chain is a root (`$this`, a variable, a parameter or a
  foreach value) followed by property reads and list reads with an int
  index (`$this->carts[0]->count`). These changes MUST convert, where `$x` is
  a chain: `$x->p = v` to `x.p = v;`; `$x->p += v`, `-=`, `*=` and `.=` to
  the Rust operator as for a variable (P2R-43, P2R-45); `$x->p++` and
  `$x->p--` as P2R-43 does for a variable (`x.p += 1;`, `x.p += 1.0;` for a
  float); and `$x->p[] = v` to `x.p.push(v);`. The value converts as for an
  assignment to a variable of the property's type (P2R-25, P2R-26,
  P2R-102), and the chain follows the 60-column chain rule of P2R-134, the
  target and `.push(v)` included. These stop the conversion: a change
  through a call result or a `new` expression; `??=`; a write to a list
  element of a property (`$x->p[0] = v`, P2R-47); a change used as a value
  (P2R-44); an int appended to a `list<float>` property, or a list with int
  elements assigned to one, because PHP keeps the ints while P2R-25 would
  make them floats; an assignment to a foreach value of a changing type,
  because PHP then leaves the list unchanged; and a `=` write to a property
  of a local variable or an owned parameter that no path reads afterwards,
  which rustc rejects as an unused assignment (P2R-35).
- **P2R-143** A method MUST take `&mut self` when its body changes its
  object: a P2R-142 change on a chain whose root is `$this`; a `&mut self`
  method call on such a chain; such a chain as the argument for a `&mut`
  parameter (P2R-144); or a foreach by `&mut` over a list on such a chain
  (P2R-146). A parameter of a changing type MUST be owned when its body
  moves it (P2R-145), with `mut` when the body also changes it; `&mut` when
  the body changes it without moving it; and `&` otherwise. A changing list
  parameter is `&[C]`, `&mut [C]` or `Vec<C>`; a change to the objects in a
  list parameter is not a change of the list (P2R-24 does not stop it). The
  tool decides these forms for every method and function of the file
  together: (1) every method starts at `&self` and every changing parameter
  at `&`; (2) each pass sets the flags "changes its object", "changes this
  parameter" and "moves this parameter" from the bodies and the forms of the
  previous pass, and a flag only ever turns on; (3) the passes stop when no
  flag changes. This is the least fixed point: two methods that call each
  other and change nothing stay `&self`. P2R-144 to P2R-147 then apply with
  the final forms.
- **P2R-144** `new` takes a property of a changing type by value
  (`item: Item`, `items: Vec<Item>`) and stores it as it is; a list literal
  argument for it is `vec![..]` or `Vec::new()`. An argument for an owned
  parameter MUST move (P2R-145). An argument for a `&mut` parameter is
  `&mut v` for a local variable or an owned parameter `v`; `self` for
  `$this` in a `&mut self` method; the parameter itself for a `&mut`
  parameter; the value itself for the value of a foreach by `&mut`;
  `&mut x.p` and `&mut x.p[i]` for a chain on one of these; and
  `&mut C::new(..)` for a `new` expression. Any other argument for a `&mut`
  parameter stops the conversion. An owned parameter passes to a `&`
  parameter as a local variable does (`&c`). A `&mut self` method call
  converts on a chain whose root is `$this` in a `&mut self` method, a local
  variable, an owned parameter, a `&mut` parameter or the value of a foreach
  by `&mut`, and on a `new` expression or a call result.
- **P2R-145** A value of a changing type MUST NOT be copied. Where P2R-26
  and P2R-132 copy a value with `.clone()`, a value of a changing type moves
  instead, and the conversion stops where it cannot move. A local variable
  or an owned parameter moves when no path reads it again (P2R-26) and the
  statement that moves it names it nowhere else (`$a->parts[] = $a;` and
  `keep($a, $a)` stop); `$this`, a `&` or `&mut` parameter, a property read,
  a list element and a foreach value never move. So `$b = $a;` converts only
  when `$a` is not read again, and `return $this;`, `return $this->item;`,
  `$x = $list[0];` and `$cart->put($a);` followed by a read of `$a` stop.
- **P2R-146** A foreach over a list of a changing type MUST iterate by
  reference: for a list in a variable, an owned parameter or on a chain,
  `for x in &mut xs {` when the body changes the value (P2R-142 to
  P2R-144), otherwise `for x in &xs {`; for a `&[C]` parameter,
  `for x in xs {`; for a `&mut [C]` parameter, `for x in xs.iter_mut() {`
  when the body changes the value and otherwise `for x in xs.iter() {`,
  because iterating the `&mut [C]` itself would move it; with a key,
  `for (k, x) in (0_i64..).zip(&mut xs) {` and the same with the other
  forms. A list of a changing type in no variable, parameter or chain (a
  call result or a list literal) stops the conversion. The value is a reference: it never moves (P2R-145) and is never
  assigned (P2R-142). In the body, the root of the iterated chain may appear
  only as a read or a change of a chain that is neither the iterated chain,
  nor a part of it, nor an extension of it. A method call on that root or on
  a part of the iterated chain, that root or such a part as an argument, and
  a read or change of the iterated chain stop the conversion, because rustc
  lets one field be borrowed while another is used but not the whole object;
  in a foreach by `&`, a `&self` method call on the root converts. A
  foreach over a list property of a Copy type (section 14 borrows it,
  `for &h in &self.history {`) MUST iterate a copy,
  `for h in self.history.clone() {`, when the body changes the root of the
  list's chain (`$this` for `$this->a->log`): PHP iterates a copy of the
  array too.
- **P2R-147** In one statement, the root of a chain that the statement
  changes (by a P2R-142 change, a `&mut self` call, a `&mut` argument or a
  move) MUST appear only once, the receiver, the arguments and the target
  included, except in these forms, which rustc accepts and which evaluate
  in PHP's order:
  1. a top-level `=` on a property, or a top-level `+=`, `-=` or `*=` on an
     int or float property, whose value may read and change the same root;
     when the value changes it, each other mention of the root in the value
     is the receiver of a method call whose arguments do not change the
     root (without a list read when the call takes `&mut self`), or a read
     of an int or float property outside a string, and no list index in
     such a mention changes the root, because `format!`, a `&` argument and `==` on other values keep
     a borrow across the change (`$this->qty += $this->next();`,
     `$this->qty = $this->qty * $this->next();`);
  2. a `&mut self` method call on a chain without list reads, whose
     arguments read the same root only through property reads that give a
     Copy value and through `&self` method calls (`$a->add($a->total())`,
     `$a->setName($a->label())`); a list read in the receiver goes through
     `IndexMut`, which rustc borrows before the arguments run;
  3. `$this`, a `&mut` parameter or the value of a foreach by `&mut` passed
     as it is to a `&mut` parameter, with later arguments that read the same
     root as in form 2 (`bump($this, $this->qty)`).

  Every other repeat stops the conversion, among them `bump($a, $a->qty)`,
  `$a->add($a->next())` where `next` changes `$a`,
  `$this->items[] = $this->make()`, `$this->text .= $this->text`,
  `$a->name . $a->bumpName()`, `$a->copyTo($a)`, `pair($a->part, $a)`,
  `$a->items[0]->add($a->total())` and
  `$this->name = $this->name . $this->bumpName()`.
- **P2R-148** A local variable or an owned parameter that is the root of a
  chain that a P2R-142 change, a `&mut self` call, a `&mut` argument or a
  `for ... in &mut` changes MUST be declared `let mut` (P2R-34), or
  `mut c: C`; a foreach value never is. A property write MUST be one
  statement on one line (P2R-51), and a declared property's default MUST
  follow the store layout of P2R-137 as `name: default`.

## Spec checklist

- [x] No `[NEEDS CLARIFICATION]` markers left. Decided in review: problems
  inside or equal to a reported span are not reported (P2R-03); exit 2 wins
  over exit 1 (P2R-04); comments are not copied in version 1 (P2R-52); `[]`
  takes its element type from the same variable or the declared type it
  flows into (P2R-21); a file of only functions becomes a library (P2R-33).
- [x] Every requirement is testable: P2R-07 to P2R-49 and P2R-100 to
  P2R-199 by case and stop folders (P2R-75 enforces this), P2R-01 to P2R-06
  by error folders and tests, P2R-50 to P2R-52 by P2R-71, the rest by CI or
  a file in the
  repository. Stated review concerns (P2R-05 partial write, P2R-72 case
  coverage) are the only parts not checked mechanically.
- [x] No speculative features: later milestones carry no requirements yet.
