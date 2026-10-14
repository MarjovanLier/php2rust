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
list read limit of form 2 of P2R-147. Section 16 (namespaces, static
methods and constants) was written on 14 Oct 2026; it changes P2R-07,
P2R-31, P2R-32, P2R-48, P2R-72, P2R-105, P2R-110, P2R-111, P2R-112,
P2R-114, P2R-115, P2R-120, P2R-123, P2R-130, P2R-133, P2R-134, P2R-135,
P2R-137 and P2R-143. The two spec reviewers added the `use` stops and the
neutral `namespace` and `use` of P2R-150, the call resolution of P2R-151,
`self::Case` and the call rules of P2R-153, the `static` and `self`
stops of P2R-154, the helper names and the function rules of P2R-155,
the case, `class`, `final private`, width and P2R-50 rules of P2R-156,
and the non-ASCII stop of P2R-157. Section 17 (strings and scalars) was
written on 14 Oct 2026; it changes P2R-07, P2R-31, P2R-33, P2R-43,
P2R-47, P2R-48, P2R-60, P2R-72, P2R-106 and P2R-130. P2R-114, P2R-133,
P2R-135 and P2R-153 keep their text: their calls convert as calls of a
function do, so P2R-168 and P2R-169 apply to them. The two spec reviewers added the lending, call and naming
rules of P2R-160, the needle forms of P2R-163, the literal, sign and
deprecated cast rules of P2R-166, the literal, cast and P2R-50 rules of
P2R-167, the null, `static::` and declaration rules of P2R-168, and the
scope and argument position rules of P2R-169; the final review of the
build added the wide list stop of P2R-163 and the `(int)` quirk of P2R-60.
Section 18 (several files as one crate) was written on 14 Oct 2026; for a
directory input it changes P2R-01, P2R-03, P2R-04, P2R-33, P2R-70,
P2R-71, P2R-73, P2R-74 and P2R-76, and P2R-171 says how "the file" reads
in sections 1 to 17. The two spec reviewers added the duplicate,
letter case, module name and child module stops of P2R-172 and P2R-173,
the global function fallback and path rules of P2R-171, the Rust-line and
sort rules of P2R-174, and the folder rules of P2R-176; the final review of
the build added the primitive and between-module stops of P2R-172.
Section 19 (everyday expressions) was written on 14 Oct 2026; it changes
P2R-07, P2R-24, P2R-42, P2R-48, P2R-60, P2R-72, P2R-101 and P2R-171. The
spec reviewer added the context type, `_i64`, `&str` arm, nesting, byte
width and `match` arm rules of P2R-180, the owned-local rule of P2R-181,
PHP's argument order in P2R-185 and P2R-186, Mago's count check in
P2R-184, and the `%` grammar of P2R-187. Section 20 (layout by rustfmt)
was written on 14 Oct 2026; it changes P2R-04, P2R-06, P2R-51, P2R-82,
P2R-83 and P2R-176, and P2R-191 lists the requirements whose layout rules
it removes. Section 21 (array shapes and nullable objects) was written on
14 Oct 2026; it changes P2R-07, P2R-20, P2R-22, P2R-23, P2R-33, P2R-72,
P2R-75, P2R-104, P2R-105, P2R-106, P2R-130, P2R-132, P2R-133, P2R-134,
P2R-136, P2R-154, P2R-171, P2R-172, P2R-173, P2R-174 and P2R-180. The two
spec reviewers added the self-holding and empty-enum stops of P2R-130 and
P2R-200, the int-for-float stop and the lent literal argument of P2R-202,
the `null` comparison exception of P2R-136 and P2R-201, the owned `??` of
P2R-204, the struct literal `use` rule of P2R-174 and the literal read
stop of P2R-203; the build added `??` on a shape element (P2R-203). The
final review of the build added the `?:` arm stop of P2R-202, the list
read stop of P2R-203, the narrowed shape rule of P2R-200 and the lent
string arm of P2R-205.

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
  after resolving `.`, `..` and symbolic links), the output file cannot
  be written, or rustfmt cannot lay out the output (P2R-190). When any of
  these holds, the tool MUST exit 2 and MUST NOT
  print P2R-03 problems. Mago's parser recovers from broken input without
  failing, so the parse check is explicit.
- **P2R-05** On exit 1 or 2 the tool MUST write nothing to stdout and MUST
  NOT create, change or truncate the output file. A failed write MUST NOT
  leave a partial output file (review concern, not checked mechanically).
- **P2R-06** With the same rustfmt (P2R-190), the same input MUST always give
  the same stdout bytes, the same output file bytes and the same stderr
  bytes.
- **P2R-07** Only the constructs that sections 3 to 5, 11 to 19 and 21
  list as converting may convert. Any other construct MUST stop the conversion,
  which means it is reported as a problem under P2R-03. Examples that stop
  in version 1 (section 16 converts `namespace`, `use` and class
  constants, and section 17 two casts):
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
  | an array shape `array{k: T}` | a Rust struct (P2R-200) |
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
  than `T|null` (P2R-100), `array` without a list element type or a shape
  (P2R-200), the type
  `null` of a variable, parameter or return value (the literal `null`
  converts by P2R-102), objects other than enums and the classes of
  section 14) MUST stop the conversion
  with a problem that names the type.
- **P2R-23** Every function parameter and return type MUST be declared in the
  signature; a docblock may refine `array` to `list<T>` or to an array shape
  (P2R-200). A missing declaration stops the conversion.
- **P2R-24** A `string` parameter MUST become `&str` and a `list<T>`
  parameter MUST become `&[T]`. A function that assigns to or changes its own
  string or list parameter works on a copy, as PHP passes one (P2R-181). A
  foreach key or value that takes the name of a string or list parameter
  that no assignment, `.=` or append changes stops the conversion.
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
  Rust name of a function in the file; a PHP function named `main`; a
  function, variable, parameter or foreach key or value whose Rust name is
  the name of a helper of P2R-160; and a
  function with the name of a function PHP has built in (`next`, `touch`),
  which PHP rejects when it compiles the file and Mago does not report (in
  a namespace PHP accepts it, P2R-151).
- **P2R-32** A call MUST use the declared or built-in function name,
  whatever the letter case of the call (PHP function names ignore case).
- **P2R-33** Statements outside functions, other than enum and class
  declarations (sections 12 and 14), MUST become `fn main()`. Functions
  declared at the top level MUST become `pub fn`, callable before their
  declaration, as in PHP. A file with no statements outside functions other
  than the P2R-12 `declare`, enum declarations and class declarations MUST
  convert to a library file with no `main`. After the P2R-52 line the output
  holds the structs of array shapes (P2R-200), then the enums, classes and
  functions in source order, then the helpers of P2R-160, then `fn main()`.
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
- **P2R-42** Conditions of `if`, `while`, `for` and `?:` (P2R-180), and the
  operands of `&&`, `||` and `!`, MUST be `bool`. A non-bool condition (PHP truthiness) stops
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
  - `<<`, `>>`, `<<=` and `>>=` as P2R-167 allows them

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
  to an index (`$a[$i] = x`) stops the conversion. `$s[$i]` on a string is
  a string offset (P2R-165).
- **P2R-48** These built-in functions MUST convert: `count()` on a list
  (length as `i64`), `strlen()` (byte length as `i64`), `intdiv()` (integer
  division towards zero) and `fdiv()` (IEEE float division), and the
  functions of sections 17 and 19. A call to any other function not
  declared in the file stops the conversion.
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
  with rustfmt's default configuration, run with the rustfmt that laid it
  out (the pinned toolchain's for the tests). The tool lays out its output by
  running rustfmt (P2R-190), so no rule of this file copies rustfmt's
  layout, and no statement stops because rustfmt would split it over
  several lines (P2R-191).
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
  - Reading a string offset past either end of the string: PHP warns and
    gives `""`. Rust panics.
  - `substr()` or a string offset that cuts a character of more than one
    byte: PHP gives the raw bytes, Rust gives U+FFFD (three bytes) in their
    place, so later lengths, comparisons and joins of that value differ
    too. The README lists these two; no case folder can show them, because
    PHP warns or prints bytes that are not UTF-8.
  - `(int)` of a string of 19 digits above `PHP_INT_MAX` followed by `e+`
    or `e-` without exponent digits (`"9300000000000000000e+"`): PHP's
    number parser reads one byte late and gives a wrapped value
    (-9146744073709551616); `php_int` gives the largest or smallest int.
  - `str_repeat()` with a count that is negative at run time: PHP throws a
    `ValueError` (exit code 255 when uncaught); Rust panics, or gives an
    empty string when the string is empty.
  - `explode()` with an empty separator that is not a literal: PHP throws a
    `ValueError`, Rust panics (exit code 101).
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
  `php -n -d display_errors=stderr -d error_reporting=E_ALL input.php`
  (with `-d extension=ctype` added for a PHP build that loads `ctype` as a
  shared extension, which `-n` skips) and
  the Rust program built with `rustc --edition 2024` (debug) MUST print the
  same stdout. Both MUST exit 0, the stdout MUST NOT be empty, and PHP's
  stderr MUST be empty. A case without `fn main()` gets the checks of P2R-70
  and P2R-71 only. Review concern, not checked mechanically: each case's
  stdout depends on every construct the case covers, and every construct of
  sections 3 to 5, 11 to 17, 19 and 21 appears in at least one case with
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
  P2R-299 MUST appear in the name of at least one case or stop folder. A
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
  (`cargo install --locked`), that it needs `rustfmt` on `PATH` (P2R-190),
  which rustup's minimal profile leaves out, how to run it, the supported
  PHP, the refusal rules in short, the known differences (P2R-60) and the
  command of P2R-76.
- **P2R-83** The Rust toolchain MUST be pinned in `rust-toolchain.toml`. The
  `rust-version` in `Cargo.toml` MUST equal the `rust-version` of the pinned
  Mago crates (1.99.0 for Mago 1.55.0), and the pinned toolchain MUST be at
  least that version. The tests MUST fail when `rustfmt --version` does not
  name the commit of the pinned rustc, because the layout of the expected
  files is that rustfmt's (P2R-190).

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
  it; a `throw` becomes an `Err` of a `Result`. On 14 Oct 2026 a trial on
  two real libraries converted none of their files, so the order after
  section 15 follows what real code needs: namespaces, static methods and
  constants (section 16); several files converted together into one crate
  with one module per namespace; built-in string, regex and array
  functions (section 17 converts the first string functions, default
  parameter values and named arguments); maps and array shapes; interfaces
  and inheritance; exceptions; closures. The goal is
  code of the kind a Laravel application holds.
- **Order after section 19**, set on 14 Oct 2026 under Marjo's direction of
  that day to take the recommended option without asking: layout by
  rustfmt (section 20); array shapes as structs and nullable object types
  (section 21); exceptions as `Result`; interfaces and inheritance, with `ArrayAccess` as
  Rust's `Index`; maps and `preg_*`; closures; then the
  `StringManipulation` library, which `SouthAfricanIDValidator` calls.
  Three points are open:
  - PHP functions with no counterpart in Rust's standard library
    (`preg_replace`, `DateTime::createFromFormat`): a short one becomes a
    helper written into the output, as `php_int` is (section 17); one that
    needs an engine, such as `preg_replace`, takes a crate, as maps take
    `indexmap`. Which formats of `DateTime::createFromFormat` a helper
    covers is not decided.
  - Output that needs a crate cannot be built with plain `rustc`, which
    P2R-50, P2R-72 and P2R-176 use; how such output and its tests are built
    (a `Cargo.toml` beside it) is not decided.
  - Dynamic PHP (`mixed`, magic methods, facades, reflection, arrays with
    values of mixed types) has no plain Rust form. Converting it needs a
    dynamic value type, which no section specifies yet.
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
  nullable string or list parameter works on a copy (P2R-181), and an
  assigned, appended or returned nullable string or list behaves as a copy.
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
  (after a check against `null` or an assignment). Only these two forms and
  the `?:` of P2R-205 give a binding of type `T`. The binding has the Rust name of `$v` and is a new
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
  of P2R-24 (`&str`, `&[T]`), and for a class or shape `&S` (P2R-204); it
  follows P2R-26 and P2R-132 as a parameter does. When
  nothing reads the binding, the condition converts by P2R-103 instead
  (`if v.is_some() {`, `if v.is_none() {`). A write to `$v` in the body of
  such an `if let`, or anywhere after such a `let ... else` in its block
  (nested blocks included), stops the conversion. A read or write of `$v` in
  the step of a `for` whose body holds such a `let ... else` at any depth
  stops the conversion: the step also runs at each `continue`, where the
  binding can shadow the variable or not exist yet.
- **P2R-105** `$a ?? $b` MUST convert when `$a` is a variable, parameter,
  function call, static method call (P2R-153), `E::tryFrom()` call
  (P2R-114) or array shape element (P2R-203) of Rust type `Option<T>` and
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
  `->name` or `->value`; the subject of a `match` (P2R-120); an operand of
  a cast (P2R-166) or of `<<`, `>>`, `<<=` or `>>=` (P2R-167); the string
  or the index of a string offset read (P2R-165); the object of a property
  read or a method call and the shape of an element read (P2R-204); an
  argument for a
  parameter that is not nullable; and a `return` value in a function whose
  return type is not nullable.
  These also stop: a parameter default value other than those of P2R-168,
  and `return;` or a path that reaches the end of a function with a
  nullable return type.

## 12. Enums (milestone 2, slice 2)

- **P2R-110** An `enum` declared at the top level, pure or backed by `int` or
  `string`, whose body holds `case` lines and the methods and constants of
  P2R-155 and P2R-156, MUST become a Rust `pub enum`
  with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and one variant per
  case, in source order (P2R-33 places it in the output). A backing value
  MUST be an int literal, with or without unary minus, or a string literal,
  converted by P2R-49. These stop the conversion:
  - an enum declaration after a statement outside functions other than the
    P2R-12 `declare`, a function, enum or class declaration: PHP
    declares an enum only when execution reaches it, so an earlier use dies
    with `Class "E" not found`;
  - an enum with `implements` or `use`, or a backing value of any other
    form (`1 + 1`, `PHP_INT_MAX`); section 16 converts methods and
    constants (P2R-155, P2R-156);
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
  enum named `Option`, `String`, `Vec`, `ToOwned`, `None` or `Some`, which
  the output uses;
  and an enum with a name PHP reserves, a keyword or a type name (`list`,
  `readonly`, `match`, `bool`, `self` and the others; `enum` is allowed), or
  the name of a class, interface, enum or trait PHP has built in
  (`Exception`, `Countable`), which PHP rejects, compared by the fully
  qualified name (P2R-151); the other name checks here compare the short
  name, in any namespace. Mago's parser accepts a
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
  argument (P2R-07); and any other static call on an enum, other than a
  call to a static method the enum declares (P2R-153).
- **P2R-115** The output MUST hold, for each enum, only the helpers the file
  uses, in one `impl E` block right after the enum, in the order `name`,
  `value`, `from`, `try_from`, `cases`, with one blank line between methods;
  the same block holds the enum's constants and methods (P2R-157).
  `name`, `value`, `from` and `try_from` are a `match` with one arm per case
  in source order; `from` ends with
  `_ => panic!("{:?} is not a valid backing value", value),` and `try_from`
  with `_ => None,`. rustfmt lays all of these out (P2R-190).

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
    expression, a class or enum constant (P2R-156), or an integer literal
    that PHP reads as a float (P2R-49);
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
- **P2R-123** The `match` MUST be the whole value of its `return`, `let` or
  assignment, with one arm per PHP arm other than a `default` that P2R-121
  leaves out, and rustfmt lays it out (P2R-190).
- **P2R-124** For P2R-34 and P2R-35, each arm MUST be its own path from the
  subject, and the paths join after the `match`.

## 14. Readonly classes (milestone 2, slice 4)

- **P2R-130** A `readonly class` declared at the top level, with or without
  `final`, MUST become a Rust `pub struct` with `#[derive(Clone, Debug)]` and
  one `pub` field per property, in source order, followed by an `impl` block
  that holds `pub fn new` and the class's methods (P2R-33 places both). Its
  body MUST hold only methods (P2R-135, P2R-152), constants (P2R-156) and
  at most one constructor
  `__construct` whose parameters all promote to properties and whose body is
  empty. A parameter promotes with any promotion modifier: a visibility,
  `readonly`, `final` or an asymmetric visibility such as `private(set)`. A
  constructor docblock `@param` or an inline `@var` on a promoted parameter
  may refine `array` to `list<T>` or to an array shape (P2R-23, P2R-201). A
  class without a constructor
  gets a `new` without parameters. PHP declares such a class before any
  statement runs, so it may come after the statements outside functions.
  These stop the conversion:
  - `extends`, `implements`, `abstract`, a trait, a property that is not
    promoted, a static property, a constructor parameter that is not
    promoted, a constructor with a body, and a parameter default value
    other than those of P2R-168 (section 16 converts constants and static
    methods);
  - a promoted parameter without a type, with a property hook, by reference
    or variadic, which PHP rejects or makes fail and Mago does not report;
  - a method whose PHP name starts with `__`, other than the constructor,
    and an attribute on the class or a member (P2R-07);
  - a class that is not `readonly` (section 15 converts it), and a class
    declared inside a function or another statement;
  - a property whose type does not convert (P2R-20, P2R-22; a nullable
    class type converts by P2R-204), a property that holds its own class
    directly, as `?C` (P2R-204) or through other classes other than through
    a list, because the Rust struct would have no size (`?Node $next` in
    `Node` stops), and a property of an enum without
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
  argument, `$this` is `self` and a `C` parameter is passed as it is. A
  nullable class type `?C` converts by P2R-204. A function or method that
  assigns to its own `C` parameter stops the conversion, as P2R-24 stops one
  for a string or list.
- **P2R-133** `new C(args)` MUST convert to `C::new(args)`, with the arguments
  converted as for a function whose parameters are the constructor's (P2R-24,
  P2R-25); `new self` and `new static` convert by P2R-154. `C::new` takes
  `&str` for a string, `&[T]` for a list and `&D` for
  a class, and stores an owned copy of each: `name.to_string()`,
  `xs.to_vec()`, `d.clone()`, and `s.map(ToOwned::to_owned)` for an
  `Option<&str>`, `Option<&[T]>` or `Option<&S>` (P2R-204); a Copy value is
  stored as it is. The
  store counts as a read of each parameter, so no constructor parameter gets
  the `_` of P2R-35. `new C` without parentheses converts as `new C()`. `new`
  with any other class form (a variable, an anonymous class) stops the
  conversion, and so does a call to `__construct` or to any
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
  argument, and an `Option` of a class or shape `.as_ref()` (P2R-204). A `foreach` over a list field borrows it when its elements are
  Copy (`for &s in &self.scores {`) and iterates a copy otherwise
  (`for n in self.names.clone() {`), with the rules of P2R-41. These stop
  the conversion: a write to a property (Mago reports
  one outside the constructor, P2R-11), `?->`, a property name that is not a
  plain identifier, a property read inside an interpolated string (P2R-45),
  and a property read or a method call on the left of `??` (a later slice).
- **P2R-135** A method MUST become `pub fn name(&self, params) -> R` in the
  `impl` block, after `new`, in source order, with one blank line between
  methods (a static method has no `self`, P2R-152, and an enum method takes
  `self`, P2R-155). Its parameters, return type and body follow the rules
  for functions (sections 3 to 5 and 11 to 13), with `$this` read as
  `self`.
  `$o->m(args)` MUST convert to `o.m(args)`, and `$this->m(args)` to
  `self.m(args)`, also as a statement (P2R-43). A method that calls the same
  method on every path, through any object of its class (`$this`, a
  parameter, a list element, a new object), stops the conversion, as a
  function that calls itself does (P2R-50): rustc matches the method, not
  the object. These stop the conversion: `parent::` and the native type
  `parent` (section 16 converts static methods, `self::`, `static::` and
  the types `self` and `static`; a docblock type that names `self`, such as
  `list<self>`, is the class to Mago and converts); `$this` outside a
  method, a write to `$this` and a parameter named `$this`, which PHP
  rejects and Mago does not report; a by-reference parameter; a `final
  private` method other than the constructor, for which PHP prints a
  warning and Mago reports nothing; two methods whose PHP names are equal
  ignoring letter case, which PHP rejects and Mago does not report; and a
  call to a method the class does not declare (Mago reports it, P2R-11).
- **P2R-136** These MUST stop the conversion: `===`, `!==`, `==`, `!=`, `<`,
  `>`, `<=`, `>=` and `<=>` with an object operand, also of a nullable class
  type (P2R-204), because Rust has no object identity and PHP compares
  properties loosely, other than `===` and `!==` with the literal `null`
  (P2R-103); `echo` and string
  joining of an object; `instanceof`; `clone`, also with new property
  values; a cast of an object; and an object as a condition.
- **P2R-137** The struct MUST list one field per property
  (`pub x: i64`), and one blank line separates it from `impl C {`. `new`
  comes first in the `impl`
  block, after the constants of P2R-157, as `pub fn new(params) -> C` with
  the constructor's parameters, and its body is the struct literal as the
  tail expression, `C { x, y }`; rustfmt lays these out (P2R-190). A field
  that stores
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
  single- or double-quoted string literal (`String::from("hi")`; heredoc
  and nowdoc stop, as everywhere in version 1); `null`, or a literal of the inner
  type (`Some(5)`), for a nullable type; `[]` for a list type; or a case of
  an enum, never a class or enum constant (P2R-156). `new` stores the
  default (`count: 0`, `items: Vec::new()`,
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
  P2R-102). These stop the conversion: a change
  through a call result or a `new` expression; `??=`; a write to a list
  element of a property (`$x->p[0] = v`, P2R-47); a change used as a value
  (P2R-44); an int appended to a `list<float>` property, or a list with int
  elements assigned to one, because PHP keeps the ints while P2R-25 would
  make them floats; an assignment to a foreach value of a changing type,
  because PHP then leaves the list unchanged; and a `=` write through
  property reads only (`$g->a->n = 1`, not `$g->kids[0]->n = 1`, whose list
  read rustc counts as a use) to a local variable or an owned parameter that
  no path reads afterwards, which rustc rejects as an unused assignment
  (P2R-35).
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
  at `&`, where a static method and an enum method have no "changes its
  object" flag (P2R-152, P2R-155); (2) each pass sets the flags "changes
  its object", "changes this parameter" and "moves this parameter" from the
  bodies and the forms of the previous pass, and a flag only ever turns on; (3) the passes stop when no
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
  statement, and a declared property's default MUST be stored in the struct
  literal of P2R-137 as `name: default`.

## 16. Namespaces, static methods and constants (milestone 2, slice 6)

Real libraries put every class in a namespace, call static methods and
read class constants. This section converts those forms for one input
file. Converting several files together is a later slice.

- **P2R-150** A file MAY hold one `namespace` statement without braces
  (`namespace Acme\Shop;`) as its first statement after the P2R-12
  `declare`. The statements after it MUST convert as if they stood at the
  top level (P2R-33 places them, P2R-110 orders them). A `use` statement at
  the top level (a class, function or constant import, with or without
  `as`, grouped or not) MUST NOT change the output, and the names it
  imports resolve as PHP resolves them. The `namespace` statement and every
  top-level `use` statement give no Rust lines and count as no statement
  for P2R-33 and P2R-110, so a file of a namespace, `use` lines and
  declarations is a library. These stop the conversion:
  - a namespace with braces; a second `namespace` statement; a `namespace`
    statement after another top-level statement, which PHP rejects and
    Mago's parser accepts (one inside a function is a parse error,
    P2R-04); and `__NAMESPACE__` (P2R-07);
  - imports that PHP rejects when it compiles the file and Mago does not
    report: two imports of one kind (class, function or constant) with the
    same name, ignoring letter case except for constants; a class or
    function import whose name is the short name of a class, enum or
    function the file declares under another fully qualified name; a class
    import whose name is one PHP keeps for its own types (`self`, `parent`,
    `static`, `array`, `bool`, `callable`, `false`, `float`, `int`,
    `iterable`, `mixed`, `never`, `null`, `object`, `string`, `true`,
    `void`), with or without `as`; and an `as` name that is a PHP keyword
    (`as match`), which PHP cannot parse and Mago's parser accepts (a
    keyword as the last part of the imported name, `use Foo\Match;`, is
    allowed);
  - an import of a one-part name without `as`, of any kind, in a file
    without a namespace (`use Foo;`, `use function strlen;`), for which PHP
    prints a warning;
  - a constant import named `true`, `false` or `null` in any letter case,
    which PHP reads in place of the literal and the tool does not.
- **P2R-151** The tool MUST find every enum, class and function of the file
  by its fully qualified name in lowercase (`acme\shop\size`), as Mago's
  name resolution gives it; Mago's types hold the fully qualified name too
  (`enum(Acme\Shop\Size)`). The Rust name stays the one that P2R-30,
  P2R-111 and P2R-131 give the short name (`Size`, `helper`). Only the
  check of P2R-111 and P2R-131 against the class-likes PHP has built in
  compares the fully qualified name, so a class `Acme\Exception` converts;
  the checks against reserved and Rust-used names compare the short name
  in any namespace. A call MUST go to the function Mago's name resolution
  gives it: a call by a qualified or fully qualified name
  (`\Acme\Shop\helper()`, `namespace\helper()`) finds the file's function
  as an unqualified call does, and a call that resolves to a function PHP
  has built in converts the same way whether it is written `strlen()`,
  `\strlen()`, or `strlen()` inside a namespace, where PHP falls back to
  the global function (this changes P2R-32 and P2R-48). A call whose name
  a `use function` import maps to a function of the file calls that
  function. A function declared in a namespace with the name of a built-in
  function stops the conversion (P2R-31), although PHP accepts it there
  and calls it in place of the built-in.
- **P2R-152** A method declared `static`, in a class of section 14 or 15 or
  in an enum (P2R-155), MUST become `pub fn name(params) -> R` with no
  `self` parameter. Its parameters, return type and body follow the rules
  for functions (sections 3 to 5, 11 to 13 and 15): a parameter of a
  changing type takes its form from P2R-143 as a function's parameter does.
  A static method that calls itself on every path, through `self::`,
  `static::` or its class name, stops the conversion (P2R-50). These stop
  the conversion: `$this` in a static method (PHP throws an `Error`), and a
  static property. A `final private` method (PHP prints a warning for it)
  stops by P2R-135.
- **P2R-153** `C::m(args)`, `self::m(args)` and `static::m(args)`, where `m`
  is a static method, MUST convert to `C::m(args)` with the Rust name of
  the class or enum `C` that declares `m`, also as a statement (P2R-43).
  The arguments convert as for a function with the method's parameters
  (P2R-24, P2R-25, P2R-102, P2R-144), and the call has the Rust type of the
  declared return type (P2R-134). `self` and `static` name the class or
  enum whose body holds the call; `static` names the same class as `self`,
  because no class of the file can extend another (P2R-130), and both are
  read whatever their letter case (`SELF::`). In the body of an enum,
  `self::Case` and `static::Case` convert as `E::Case` (P2R-112), also as
  a `match` condition (P2R-120), and `self::from()`, `self::tryFrom()` and
  `self::cases()`, with `static::` too, as P2R-114 converts `E::from()`,
  `E::tryFrom()` and `E::cases()`. A static call counts as a function call
  for P2R-50, P2R-143 to P2R-145 and P2R-147, and for the left side of
  `??` (P2R-105). These stop the conversion: `self::m()`, `static::m()` or
  `C::m()` where `m` is not
  static (PHP throws an `Error`, or calls it on `$this` inside a method);
  a static method called through an object (`$o->m()`, `$this->m()`);
  `parent::`; a class given by a variable or an expression (`$c::m()`); a
  first-class callable (`C::m(...)`); and `self::` or `static::` outside the
  body of a class or enum.
- **P2R-154** The type `self`, and the return type `static`, in a method's
  signature or a promoted parameter, MUST mean the class or enum whose body
  holds it: `self` in `Size` gives `Size`, and `?self` in an enum gives
  `Option<Size>` (a nullable class type converts by P2R-204). Mago's type
  `C&static`, which `new self` and `new static` have, widens to `C`
  (P2R-21). `new self(args)` and `new static(args)` in the body of a class
  MUST convert as `new C(args)` does (P2R-133). These stop the conversion:
  the type `parent`; `static` as a parameter type, which PHP rejects and
  Mago's parser accepts; and `self` or `static` in the signature of a
  function outside a class or enum, which PHP rejects and Mago does not
  report.
- **P2R-155** An enum MAY hold methods, static or not, and constants
  (P2R-156) among its cases. A method that is not static MUST become
  `pub fn name(self, params) -> R`: an enum value is Copy (P2R-112), so the
  method takes `self` by value, and `$this` reads as `self`. Its other
  parameters, return type and body follow the rules for functions
  (sections 3 to 5, 11 to 13 and 15), and P2R-143 decides the form of each
  parameter of a changing type; `self` keeps its form. `$e->m(args)` MUST
  convert to `e.m(args)`, and `$this->m(args)` to `self.m(args)`, as
  P2R-135 converts a method call, also as a statement. These stop the
  conversion: a method whose Rust name is `name`, `value`, `from`,
  `try_from` or `cases`, which the helpers of P2R-115 take, and a method
  whose PHP name is `cases`, `from` or `tryFrom` in any letter case (PHP
  rejects `cases` in every enum and `from` and `tryFrom` in a backed enum,
  and Mago does not report it); the method stops of P2R-130, P2R-131 and
  P2R-135 (a name that starts with `__`, `__construct` included, which PHP
  rejects in an enum and Mago does not report, the Rust name `new` or
  `clone`, two methods with one Rust name or with PHP names equal ignoring
  letter case, `final private`); a method that calls itself on every path
  (P2R-50, as in P2R-135); and a method call on a value of Rust type
  `Option<E>` (P2R-106).
- **P2R-156** A class of section 14 or 15 and an enum MAY declare
  constants, one per `const` statement, with or without a visibility,
  `final` and a type. The value MUST be an int or float literal, with or
  without unary minus, a bool literal, a single- or double-quoted string
  literal (P2R-49), or a case of an enum of the file (`self::Small`,
  `Size::Small`). Each constant MUST become `pub const NAME: T = value;`,
  where `T` is the Rust type of the declared type or, without one, of the
  value: `i64`, `f64`, `bool`, `&str`, or the enum; the declared type
  `self` or `static` in an enum is that enum. An int literal for a `float`
  constant gives the float PHP stores (`-0` gives `0.0`, P2R-25). The Rust
  name is heck's `to_shouty_snake_case` of the PHP name, because rustc
  warns on an associated constant that is not upper case, and it follows
  the identifier rules of P2R-30. `C::NAME`, `self::NAME` and
  `static::NAME` MUST convert to `C::NAME` with the Rust name of the
  declaring class or enum, with the declared type, never Mago's literal
  type (P2R-21). A string constant reads as a `&'static str`, which
  follows P2R-24 and P2R-26 as a string parameter does (P2R-113). For
  P2R-50 a read of an int constant has the constant's value, as a literal
  does: arithmetic on it that overflows, and `%` or `intdiv()` by a
  constant 0, stop the conversion. These stop the conversion: a value of
  any other form (an expression, another constant, a list, heredoc, and a
  case written `static::Small`, which PHP rejects in a constant's value and
  Mago does not report); a
  declared type other than `int`, `float`, `bool`, `string` or an enum of
  the file; two constants in one `const` statement; two constants of one
  class or enum with the same Rust name (`X` and `x`); a constant whose
  Rust name equals the Rust name of a case of its enum (`case X` and
  `const x`), or whose PHP name is the name of a case (PHP rejects it, and
  Mago does not report it); a constant named `class` in any letter case,
  and a constant that is `final` and `private`, which PHP rejects and Mago
  does not report; `C::class`; `$o::NAME`; and a
  constant declared at the
  top level or with `define()` (P2R-07). A constant is never a `match`
  condition (P2R-120), also when its value is a case or a literal.
- **P2R-157** The `impl` block of a class or enum MUST hold, in this order,
  its constants in source order, one per line with no blank line between
  them; then `new` for a class, or the helpers of P2R-115 for an enum; then
  its methods, static or not, in source order. One blank line separates
  the constants from what follows, and each function from the next. An
  enum with no constants, no helpers in use and no methods has no `impl`
  block, as before. In a `match` (section 13), an arm with two or more
  conditions joins them with ` | `, and rustfmt lays the pattern out
  (P2R-190).

## 17. Strings and scalars for real code (milestone 2, slice 7)

The main class of the real library reads digits out of strings with
built-in functions, string offsets and casts, shifts bits, and calls
functions with default and named arguments. Where Rust's standard library
behaves as PHP does, the output uses it. Where PHP's rules differ (a cast
reads a leading number, `substr` clamps its offsets), the output holds a
small helper function that copies PHP's rules.

- **P2R-160** The output MUST hold one private helper function for each
  helper the output calls, and no other (a call left out with a `default`
  arm, P2R-121, does not count): `ctype_digit`, `php_int`, `php_offset`
  and `php_substr`, in that order, after the enums, classes and functions
  and before `fn main()`, one blank line before each.
  Each helper takes its string as `&str`, and its string argument is lent
  as a string argument is (P2R-24, P2R-134): `php_substr(&s, ..)` for a
  `String` variable, `php_int(&self.text)` for a field, `ctype_digit(s)`
  for a `&str`. Each helper MUST give PHP's result for every argument,
  except for the differences P2R-60 lists; a case folder that runs each
  helper on many values against PHP checks it (P2R-72). A function,
  variable, parameter or foreach key or value whose Rust name is the name
  of a helper stops the conversion (P2R-31), because a Rust local of that
  name hides the helper.
- **P2R-161** `ctype_digit($s)` with a `string` argument MUST convert to
  `ctype_digit(s)`, true when the string is not empty and every byte is an
  ASCII digit. An argument of another type stops the conversion (PHP treats
  an int argument as a character code, and deprecates it).
- **P2R-162** `str_starts_with($a, $b)`, `str_ends_with($a, $b)` and
  `str_contains($a, $b)` with `string` arguments MUST convert to
  `a.starts_with(b)`, `a.ends_with(b)` and `a.contains(b)`, with `b` lent
  as an argument (P2R-24): all three compare bytes, and an empty `$b`
  gives true in both languages.
- **P2R-163** `in_array($v, [...], true)` MUST convert to
  `[a, b].contains(&v)` when the second argument is a list literal whose
  elements are all int literals, with or without unary minus, all string
  literals without interpolation, or all cases of one enum, of the Rust
  type of `$v`. The needle is `&v.as_str()` for a `String`, `&v` for a
  `&str`, an int or an enum, and `&(..)` around an operator expression
  (`&(v + 1)`); when the needle and every element are literals, the first
  element gets `_i64` (P2R-49). These stop the
  conversion: `in_array()` without the third
  argument or with any other third argument, because PHP then compares
  loosely (`in_array('1e3', ['1000'])` is true); a second argument of any
  other form; a list literal without elements; and an integer literal that
  PHP reads as a float (P2R-49).
- **P2R-164** `substr($s, $start)` and `substr($s, $start, $length)` with a
  `string` and int arguments MUST convert to `php_substr(s, start, None)`
  and `php_substr(s, start, Some(length))`, of Rust type `String`. The
  helper copies PHP 8's rules: a start past the end gives `""`, a negative
  start counts from the end and stops at 0, a negative length leaves out
  that many bytes at the end, and a length past the end stops at the end.
  A `null` length stops the conversion.
- **P2R-165** A read of a string offset, `$s[$i]` with a `string` `$s` and
  an int `$i`, MUST convert to `php_offset(s, i)`, the one byte at that
  offset as a `String`; a negative offset counts from the end. Where Mago
  knows the value of the string, it reports an offset outside it (a
  negative one included) as `invalid-array-index`, and the conversion
  stops (P2R-11). A write to a string offset stops the conversion.
- **P2R-166** `(int) $s` with a `string` operand MUST convert to
  `php_int(s)`, which reads the number at the start of the string as PHP
  does: leading whitespace, a sign, digits, a fraction and an exponent;
  `0` when there is no number; the integer part of a float string (`"1.9"`
  gives 1, `"1e3"` gives 1000); the largest or smallest int when the
  number is past the int range; and `0` for a number too large for a
  float. `(string) $n` with an int operand MUST convert to
  `n.to_string()`; an operand made only of literals gets `_i64` on its
  first literal (P2R-49), and a negative operand or an operator expression
  gets parentheses (`(-5_i64).to_string()`, `(-x).to_string()`). `(int)`
  and `(string)` convert in any letter case and with spaces inside the
  parentheses. Every other cast still stops the conversion (P2R-07): of a
  float (PHP and Rust differ past the int range), of a bool, null, array
  or object; `(int)` of an int and `(string)` of a string; `(bool)`,
  `(float)`, `(array)`, `(object)` and `(unset)`; and `(integer)` and
  `(binary)`, which PHP 8.5 deprecates with a message and Mago does not
  report.
- **P2R-167** `<<`, `>>`, `<<=` and `>>=` with int operands, whose right
  operand is an int literal from 0 to 63, MUST convert to the Rust
  operators, which shift the bits of an `i64` as PHP does (`1 << 63` is the
  smallest int, `>>` keeps the sign). The shift count stays a plain literal
  (Rust shifts an `i64` by an `i32`), an exception to P2R-49; a left
  operand made only of literals gets `_i64` (`1_i64 << 40`), and a left
  operand that ends in a cast gets parentheses (`(xs.len() as i64) << 2`),
  because Rust reads `as i64 <<` as generic arguments. For P2R-50 a shift
  of known values has its value, as a literal does: `(1 << 63) - 1` and
  `$b = 1 << 62; $b * 2` stop. Any other shift count stops the conversion:
  PHP gives 0 or -1 for a count of 64 or more and throws for a negative
  one, while Rust panics. `<<=` and `>>=` on a variable convert as `+=`
  does (P2R-43), and on a chain as `+=` does in P2R-142; form 1 of P2R-147
  does not apply to them.
- **P2R-168** A parameter of a function, a method or a constructor MAY have
  a default value: an int or float literal, with or without unary minus
  (`-0` for a `float` gives `0.0`, P2R-156), a bool literal, a
  single-quoted string literal or a double-quoted one without
  interpolation, `null` for a parameter whose declared type is nullable
  (`?T` or `T|null`), `[]` for a list or nullable list type, or a case of
  an enum of the file written `E::Case` or `self::Case`. A call that leaves
  the parameter out MUST pass the default, converted at its declaration
  (`self` names the class or enum that declares the parameter) and passed
  as an argument for that parameter (P2R-24, P2R-25, P2R-102): a filled-in
  `[]` is `&[]`, `&mut []`, `Vec::new()` or `Some(&[])` by the parameter's
  form (P2R-143). These stop the conversion: a default of any other form (a
  constant, an expression, `new`, an interpolated string, `static::Case`,
  which PHP rejects and Mago does not report); a `null` default for a type
  that is not nullable (`int $x = null`), which PHP deprecates, or rejects
  on a promoted parameter, and Mago does not report as an error; a default
  on a parameter before one without a default, which PHP deprecates with a
  message when it compiles the file; and a default on a by-reference or
  variadic parameter (P2R-07).
- **P2R-169** A call of a function, method, static method or constructor
  of the file MAY name its arguments (`f(a: 1, b: 2)`), after its
  positional arguments, in the order of the parameters, and leave out a
  parameter that has a default. It MUST convert as the call with every
  argument in parameter order and each left-out default filled in. For
  P2R-21, P2R-102 and P2R-143 to P2R-147, a named argument counts at the
  position of its parameter, and a filled-in default names no variable.
  These stop the conversion: named arguments in another order, because PHP
  evaluates them in the written order and Rust in parameter order; a
  positional argument after a named one, which PHP rejects and Mago does
  not report; a named argument of a built-in function or of `E::from()`
  and `E::tryFrom()` (P2R-114); a spread argument; and a name the function
  does not have, which PHP rejects when it runs and Mago does not report.

## 18. Several files as one crate (milestone 2, slice 8)

Real libraries and applications put one class in each file and name the
classes of other files. This section converts a directory of PHP files
together into one Rust library crate: Mago analyses all the files
together, so a class of another file is known, and each namespace becomes a
Rust module. One input file still converts as sections 1 to 17 say.

- **P2R-170** The tool MUST also run as
  `php2rust <input-dir> -o <output-dir>`. The input is every regular file
  whose name ends in `.php` under `<input-dir>`, at any depth, without
  following symbolic links, in byte order of its path relative to
  `<input-dir>`; other files are ignored. With a directory input `-o` is
  required, and it MUST name a path that does not exist yet whose parent
  directory exists: the tool creates it as a directory that holds the crate
  (P2R-172), so it never changes or deletes a file it did not write. P2R-04
  applies, and these also exit 2, each with its reason: no `-o`; an `-o`
  path that exists; an `-o` path inside `<input-dir>`; no `.php` file under
  `<input-dir>`; and an input file that cannot be read, is not valid UTF-8
  or is not valid PHP, named by its path (P2R-171). P2R-05 applies to the
  directory: on exit 1 or 2 the `-o` path MUST NOT exist afterwards, and a
  failed write MUST NOT leave a partial directory (the tool writes a
  temporary directory beside it and renames it). P2R-06 covers every file
  of the crate.
- **P2R-171** A problem (P2R-03) MUST name its file as `<input-dir>`
  exactly as given on the command line, joined with the file's relative
  path by one `/` (none is added when `<input-dir>` ends in `/`). Problem
  lines are sorted by that path, then by line, column and message, and the
  span rule of P2R-03 compares problems of one file only. Mago analyses
  every input file together, so a class, enum or function of one file is
  known in every other file. For a directory input, "the file" in sections
  1 to 17, 19 and 21 means every input file together (P2R-31, P2R-43, P2R-48,
  P2R-111, P2R-115, P2R-120, P2R-131, P2R-151, P2R-153, P2R-156, P2R-160,
  P2R-163, P2R-168, P2R-169 and others), except in P2R-12, P2R-13, P2R-110
  and P2R-150, which apply to each file alone (a `use` in one file may name
  a class that another file declares). An unqualified call in a
  namespace, whose name no `use function` import maps and whose namespaced
  name no function of the crate has, MUST call the crate's function of that
  name in the global namespace, as PHP falls back to it (the helpers file
  of a framework).
- **P2R-172** The crate MUST hold one Rust module for each PHP namespace
  that declares an enum, class or function, and for each namespace between
  the root and such a namespace. In this section namespace names compare
  ignoring ASCII letter case, as PHP compares them. The crate root is the
  file `lib.rs`, for the root namespace: the longest namespace that the
  namespace of every input file that declares an enum, class or function
  starts with, compared segment by segment (the global namespace when they
  share none). A namespace below the root becomes the module path of its
  segments below the root, each segment its snake_case name (P2R-30, a Rust
  keyword as a raw identifier: `r#enum`). The module `a::b` is in the file
  `a/b.rs`, and `a` in `a.rs` beside `lib.rs`, the layout `rustc` follows
  from `pub mod a;` in `lib.rs` and `pub mod b;` in `a.rs`; a module that
  holds no item still has its file, for its `pub mod` lines. The file name
  leaves out `r#` (`enum.rs`). Each module file holds the P2R-52 line,
  then the `pub mod` lines of its child modules, then its `use` lines
  (P2R-174), then its items: in `lib.rs` first the structs of the array
  shapes the crate uses (P2R-200), then the enums, classes and functions of
  every
  input file of that namespace, files in path order and items in source
  order, then the helpers (P2R-160) that its items call. One blank line
  comes before each group and each item, and none inside a group, because
  rustfmt sorts the `pub mod` lines and the `use` lines only within a group
  of lines with no blank line between them (P2R-190). These stop the
  conversion: a namespace segment whose snake_case name has no Rust form
  (P2R-30), or is `self`, `super`, `crate` or `Self`, which a raw
  identifier cannot carry; a module named `mod`, and a module directly
  below the crate root named `lib` or `main`, which rustc rejects or warns
  about beside `lib.rs`; a module named like a Rust primitive type (`bool`,
  `char`, `str`, `f32`, `f64`, the `i` and `u` integer types, `isize` and
  `usize`), which would take over paths the output writes, such as
  `usize::from`; two namespaces of different names whose module
  paths are equal (`Acme\FooBar` and `Acme\Foo_Bar`, also as a module
  between the root and another: `Acme\FooBar\Deep`); and two input files
  whose namespaces hold, under equal leading segments, a segment that is
  equal ignoring letter case but not byte for byte (`Acme\Shop` and
  `ACME\Shop\Sub`, or `App\FooBar` and `App\Foobar`, which PHP reads as one
  namespace), at the name in the namespace statement of the later file in
  path order.
- **P2R-173** Each enum, class and function of the crate MUST have a Rust
  name that no other enum, class or function of the crate has, in any
  module, because a module names the items of other modules by their Rust
  name alone (P2R-174). Two declarations whose Rust names are equal, and
  two enums or classes, or two functions, whose fully qualified names are
  equal ignoring letter case, stop the conversion at each declaration, with
  the span of the whole declaration; an enum and a function of one name
  (`App\Item` and `App\item`) are apart in PHP and in Rust. The struct of an
  array shape is an item of the crate root, and P2R-200 stops one whose name
  an enum or class has.
  Mago's `duplicate-definition` finding is not printed, because the
  declaration it names depends on how the path is written. A function whose
  Rust name is the name of a child module of its own module stops the
  conversion, because a `use` line of that function would import the
  module too.
- **P2R-174** A module whose Rust lines name an enum, class or function of
  another module, as a type, as the first segment of a path (`Gender::Male`,
  `Item::new`) or as a called function, MUST hold one
  `use crate::<module path>::<Name>;` line for it (`use crate::<Name>;` for
  an item of the crate root), and no other `use` lines. The Rust lines
  count, not the PHP source: a default filled in at a call (P2R-168) names
  its enum, and a `default` arm left out (P2R-121) names nothing. An enum
  case, method, field or constant whose Rust name equals an item's is not
  a use of it, and a name of the module's own items needs no line. A module
  other than the crate root whose Rust lines name the struct of an array
  shape (P2R-200), as a type or as a struct literal (`S { .. }`), MUST hold
  `use crate::<Name>;` for it. rustfmt sorts the `use` lines (P2R-190).
- **P2R-175** A directory input converts to a library: a statement outside
  functions other than the P2R-12 `declare`, the P2R-150 `namespace` and
  `use` statements, and enum and class declarations stops the conversion,
  because a library has no `fn main()` (this replaces the `fn main()` rule
  and the output order of P2R-33 for a directory input). The decisions that
  P2R-115, P2R-141, P2R-143 and P2R-146 make for the whole file, which
  helpers an enum holds, which classes change their objects and the form
  of each object parameter, MUST be made for the whole crate.
- **P2R-176** A case folder MAY hold a directory `input/` in place of
  `input.php` and a directory `expected/` in place of `expected.rs`; the
  crate written from `input/` MUST hold the same set of files as
  `expected/`, each equal byte for byte, and every file MUST start with the
  P2R-52 line. `rustfmt --edition 2024 --config-path <empty file> --check
  expected/lib.rs` and
  `rustc --edition 2024 --crate-type lib -D warnings expected/lib.rs` MUST
  pass; both follow the modules. A stop folder MAY hold a directory `input/`
  in place of `input.php`; run as `php2rust input -o out`, the tool MUST exit
  1 with `expected.txt` on stderr and nothing on stdout, and `out` MUST NOT
  exist afterwards. An error folder (P2R-74) MAY hold a directory `input/`
  in place of `input.php`. The P2R-76 command rewrites `expected/` too and
  removes the files the tool no longer writes. Review concern, not checked
  mechanically: a call between two files of a crate is checked by rustc
  only, because P2R-72 runs only a case with `fn main()` and a crate has
  none.

## 19. Everyday expressions (milestone 2, slice 9)

Application code chooses values with `?:`, works on a copy of a string
parameter, and builds text with the common string functions. This section
converts those forms.

- **P2R-180** `$c ? $a : $b` with a `bool` condition (P2R-42) MUST convert
  to `if c { a } else { b }`. The `if` takes its type from the place it
  stands in, as a `match` does (P2R-122): a `return` of a declared type, an
  argument for a parameter, or a variable that has a type; P2R-25, P2R-26
  and P2R-102 apply to each arm. Anywhere else both arms must have one Rust
  type, and arms of two Rust types stop the conversion; Mago's type for the
  whole `?:` is never used, because Mago types it as one arm where it
  decides the condition. When only int literals type the `if`, its first
  arm gets `_i64`, and a first write names the type (`let x: i64 = ..`,
  P2R-49). String arms are lent as `&str` (a literal, a constant, a
  parameter, an enum's `name` or `value`, or `??` on a variable or parameter
  as it is; a `String` variable, or a property read, list element or shape
  element (P2R-203) on a variable, parameter or `$this`, as `v.as_str()`),
  so the `if` lends a
  `&str` as `??` does
  (P2R-105); where a `String` is needed (P2R-26) it is copied once:
  `(if c { "a" } else { "b" }).to_string()`. Each arm is its own path for
  the reads and writes of variables (P2R-34), as an arm of a `match` is
  (P2R-124). The `if` gets parentheses where it is an operand of an
  operator, a cast, a method call, a field read or `&`. These stop the
  conversion: the short form `$a ?: $b`,
  which tests PHP truthiness; a condition that is not a bool (P2R-42); an
  arm or a condition that is itself a `?:`, with or without parentheses,
  because Mago reads an unparenthesized nested `?:` from the left where PHP
  rejects it; an arm that assigns, `++` or `--`; a string arm of any other
  form, such as a call, a joined string, `??` on a call, or a property read,
  list element or shape element on a call or `new`, because Rust drops that
  string at the
  end of the arm; an `if` of lists, or of strings or lists that may be null
  (an `if` of objects and shapes converts by P2R-205); and a value that may
  be null where the type is not nullable (P2R-106). A check against null
  narrows inside an arm only in the forms of P2R-205, so
  `$x !== null ? $x + 1 : 0` gives `if let Some(x) = x { x + 1 } else { 0 }`.
- **P2R-181** A function, method or static method MAY change a parameter of
  type `string` or `list<T>` by an assignment, `.=` or an append: the body
  MUST then start with one copy line per such parameter, in parameter
  order, before the other lines: `let mut p = p.to_string();`,
  `let mut p = p.to_vec();`, or for a nullable parameter
  `let mut p = p.map(ToOwned::to_owned);`. The body then works on its own
  copy, as PHP passes one. The parameter keeps its borrowed type in the
  signature (P2R-24), and from the copy line on it is an owned local
  variable for every rule (P2R-26, P2R-34, P2R-35, P2R-104, P2R-105,
  P2R-122, P2R-160), so a later foreach key or value may take its name
  (P2R-41). These still stop: a foreach key or value with the name of a
  parameter that no assignment, `.=` or append changes (P2R-24), a parameter
  of a type that changes its objects (P2R-143), and a promoted constructor
  parameter.
- **P2R-182** `trim($s)`, `ltrim($s)` and `rtrim($s)` with one `string`
  argument MUST convert to `php_trim(s)`, `php_ltrim(s)` and `php_rtrim(s)`,
  of type `String`: helpers (P2R-160) that remove the bytes PHP removes by
  default, space, `\t`, `\n`, `\r`, `\0` and `\x0B`, where Rust's `trim`
  also removes `\x0C` and Unicode spaces and keeps `\0`. A second argument
  stops the conversion.
- **P2R-183** `strtolower($s)` and `strtoupper($s)` with a `string` argument
  MUST convert to `s.to_ascii_lowercase()` and `s.to_ascii_uppercase()`,
  which change only ASCII letters, as PHP 8.2 and later do. `ucfirst($s)`
  and `lcfirst($s)` MUST convert to the helpers `php_ucfirst(s)` and
  `php_lcfirst(s)`, which change the first byte when it is an ASCII letter.
- **P2R-184** `str_repeat($s, $n)` with a `string` and an int argument MUST
  convert to `s.repeat(n)` for a literal count from 0 up, which stays a
  plain literal (an exception to P2R-49), to `s.repeat(xs.len())` for a
  count that is a call of `count()` or `strlen()`, whose length is a `usize`
  already, and to `s.repeat(n as usize)` for any other count. Mago reports a
  count that may be negative (P2R-11), so a
  plain `int` count stops and a count from `count()` or `strlen()`
  converts; a negative literal count stops too. A count that is negative
  at run time anyway is a P2R-60 difference: PHP throws a `ValueError`, Rust
  panics, or gives an empty string when `$s` is empty.
- **P2R-185** `str_replace($search, $replace, $subject)` with three `string`
  arguments MUST convert to `php_replace(search, replace, subject)`, in
  PHP's order, a helper that gives the subject unchanged for an empty
  search, as PHP does, where Rust's `replace` inserts the replacement
  between every character. An array argument and a fourth argument stop
  the conversion.
- **P2R-186** `implode($glue, $list)` with a `string` and a `list<string>`
  argument MUST convert to `list.join(glue)`, of type `String`. Rust
  evaluates the list first, so the glue must be a value whose reading has no
  effect: a literal, a constant, a variable, a parameter, or a chain of
  property reads on a variable, a parameter or `$this`; any other glue
  stops the conversion. `explode($separator, $s)` with two `string` arguments
  MUST convert to `php_explode(separator, s)`, a helper that gives a
  `Vec<String>` as PHP gives the list, and panics on an empty separator
  where PHP throws a `ValueError` (a P2R-60 difference). These stop the
  conversion: `implode` with one argument or with a list of another element
  type, `explode` with a third argument (a limit), and an empty literal
  separator, which Mago does not report.
- **P2R-187** `sprintf($format, ...)` with a format that is a string literal
  without interpolation MUST convert to `format!(..)` when every `%` in the
  format starts `%s`, `%d` or `%%`, and the call passes exactly one argument
  for each `%s` and `%d`: a `%s` takes a string or int argument, a `%d` an
  int argument, and `%%` gives `%`. The text's braces are doubled (P2R-45).
  These stop the conversion: any other `%`, among them a width, a flag, a
  position, a length modifier, another conversion and a `%` at the end,
  which PHP reads as other specifiers or rejects with a `ValueError` and
  Mago does not report; an argument count that does not match, which PHP
  rejects or ignores; and an argument of another type.
- **P2R-188** The helpers of P2R-160 MUST include, after `php_substr` and in
  this order, `php_trim`, `php_ltrim`, `php_rtrim`, `php_ucfirst`,
  `php_lcfirst`, `php_replace` and `php_explode`, each written only when the
  output calls it, and P2R-31 covers their names. A case folder runs each of
  them on many values against PHP (P2R-72).

## 20. Layout by rustfmt (milestone 2, slice 10)

Copying rustfmt's layout rules into the tool cost a stop for every
statement rustfmt would split, about 150 lines in one Laravel application
and 3 in the real library, and three of the final reviews found a width
rule copied wrong. The tool now lets rustfmt lay out the output, as a person
writing Rust does. P2R-51 records the change.

- **P2R-190** Every output the tool writes, to stdout, to the `-o` file or
  as a file of a crate, MUST equal what the first `rustfmt` on `PATH` gives
  for it with `--edition 2024` and `--config-path` naming an empty
  configuration file, so a `rustfmt.toml` near the output or in the user's
  configuration does not change the layout. The files and the empty
  configuration file are written to a private directory under the system's
  temporary directory for this, and a crate is laid out from its `lib.rs`,
  which names the module files. rustfmt runs in the tool's working
  directory and environment, with `RUSTUP_AUTO_INSTALL=0` so a rustup proxy
  never downloads a toolchain. rustfmt runs only when the input converts,
  so an input that stops exits 1 whether or not rustfmt can run. When
  rustfmt cannot be started, exits with an error, or would change its own
  result when run again on it, the tool MUST print one fixed message for
  each of the three to stderr and exit 2 (P2R-04); a private directory that
  cannot be made or written gives exit 2 with the operating system's
  reason. In each case P2R-05 and P2R-170 apply: nothing at `-o`, and no
  temporary file or directory left. The
  output then depends on the rustfmt version as well as the input (P2R-06);
  the tests run the pinned toolchain's rustfmt (P2R-83). rustfmt keeps a
  statement it cannot fit, such as one with a long string literal, as the
  tool gave it, so the tool writes each statement and each block line on a
  line of its own, indented four spaces for each block level, with no
  trailing whitespace. CLI tests check that a `.rustfmt.toml` in the home
  directory and a `rustfmt.toml` in the working directory leave the output
  unchanged, and run the tool with an empty `PATH`, a `rustfmt` that fails
  and one that changes its result on a second run, checking the three exits
  2, their stderr bytes, and an empty temporary directory afterwards. The
  case folder of this requirement holds a `rustfmt.toml` beside its input,
  which the check of P2R-71 must ignore too.
- **P2R-191** A construct MUST NOT stop the conversion because of the width
  of its Rust text or the layout rustfmt gives it. The rules that sections
  6 and 11 to 19 stated only to copy rustfmt's layout are removed from
  P2R-51, P2R-115, P2R-123, P2R-134, P2R-137, P2R-142, P2R-148, P2R-153,
  P2R-156, P2R-157, P2R-160, P2R-163, P2R-168, P2R-172, P2R-174 and
  P2R-180. A case folder holds the forms that stopped before, run against
  PHP (P2R-72): a long statement, a long condition with a check against
  null, with the `let` first and after other operands, a long `match`
  pattern, also with non-ASCII text, a non-ASCII enum arm, a long `?:`, a
  `?:` as a `match` arm's value, a long chain, a long list, a long
  `pub const`, a long `new` call and a long string literal inside a call.

## 21. Array shapes and nullable objects (milestone 2, slice 11)

Real code passes a small record as an array with fixed string keys, which a
docblock types as an array shape (`array{year: string, month: string, day:
string}`), and keeps an object that may be missing in a nullable property
(`?DateComponents`). This section converts both.

- **P2R-200** An array shape type MUST map to a Rust struct when Mago gives
  it known keys only: every key a string, no key optional (`key?:`), and no
  general key and value type beside the keys (`...`). The fields are the
  keys in Mago's order, the byte order of the keys, because Mago keeps the
  keys of a shape sorted and loses the order a docblock or a literal writes.
  Each field is `pub`, has the snake_case name of its key (P2R-30) and the
  Rust type of its value (P2R-20, P2R-21). The Rust name of the struct is
  heck's `to_upper_camel_case` of each key, joined in that order, followed
  by `Shape`, so `array{year: string, month: string, day: string}` gives
  `pub struct DayMonthYearShape` with the fields `day`, `month` and `year`,
  each a `String`, and `#[derive(Clone, Debug)]`. The output holds one
  struct for each shape a type of the input maps to, the shape of a field
  included, in byte order of their names: after the P2R-52 line and before
  the enums, classes and functions (P2R-33), or for a directory input in
  `lib.rs` after its `use` lines (P2R-172). The empty array `array{}` keeps
  P2R-21. These stop the conversion, with a problem at a place whose type
  maps to the shape: a shape with an integer key (PHP casts the key `'1'`
  to an int), a key that names a class constant, an empty key, an optional
  key or a general type; a key whose snake_case name has no Rust form
  (P2R-30); two keys of one shape that give one field name; a value type
  that does not convert (P2R-22); a value type that is or holds a class (a
  later slice); a value of an enum without cases, because no value of the
  shape could exist and rustc rejects the code after it (P2R-130); two
  shapes of one Rust name with different keys or different field types
  (`array{a_b: int}` and `array{a: int, b: int}` both give `ABShape`), at
  the places of the shape the tool meets second, other than a shape whose
  fields are those of the first with a `?T` field typed `T` or `null`,
  which Mago gives a shape after a check of that field against null and
  which maps to the struct of the first (P2R-104); and a shape whose Rust
  name is the Rust name of an enum or class of the input (P2R-131,
  P2R-173).
- **P2R-201** A value of a shape type MUST behave as a value of a readonly
  class (P2R-132): a shape parameter becomes `&S`, a returned shape is
  owned, `list<S>` gives `Vec<S>`, and every place where P2R-26 copies a
  string copies a shape, with `.clone()` unless the source is an owned
  variable that is not read again. A docblock may give a parameter, return
  value, promoted property or declared property (P2R-140) declared `array`
  or `?array` a shape type (P2R-23). A new value for a whole shape variable converts as for an
  object (`$s = [...]`). These stop the conversion: `===`, `!==`, `==`,
  `!=`, `<`, `>`, `<=`, `>=` and `<=>` with a shape operand, also of a
  nullable shape type, other than `===` and `!==` with the literal `null`
  (P2R-103), because PHP's `===` on arrays also compares the order of the
  keys, which a struct does not hold, and its `==` compares the values
  loosely (`['a' => '1'] == ['a' => 1]` is true); `echo`, interpolation and
  string joining of a shape; a shape as a condition, as the operand of a
  cast, as the list of a `foreach` and as the argument of a built-in
  function (`count`, `array_keys`); a write to an element (`$s['k'] = v`, a
  compound assignment, `++`, `--`, `??=`), an append and `unset`; `isset`
  and `empty` of an element; and an assignment to a function's own shape
  parameter, as P2R-132 stops one for a class. Where a requirement of
  sections 3 to 5 already stops the same form for other types, the problem
  cites that requirement (P2R-41 for a `foreach`, P2R-42 for a condition,
  P2R-48 for a built-in function, P2R-07 for a cast, `isset` and `empty`).
- **P2R-202** An array literal whose elements all have a key that is a
  string literal (single-quoted, or double-quoted without interpolation)
  MUST convert to a struct literal when its type is a shape (P2R-200):
  `['year' => $y, 'month' => $m, 'day' => $d]` gives
  `DayMonthYearShape { year: y, month: m, day: d }`. The fields are written
  in the literal's source order, because Rust evaluates the fields of a
  struct literal in the order written, as PHP evaluates the elements. The
  literal has the shape of the place it flows into when that place declares
  one: a return value, an argument for a parameter (of `new` too), an
  assignment to a property (P2R-142), or an element of a list literal or an
  arm of a `match` or `?:` (P2R-122, P2R-180) that stands in such a place.
  Where the place is nullable the literal goes inside `Some` (P2R-102), and
  as an argument it is lent as any owned value is: `&S { .. }` for an `&S`
  parameter and `Some(&S { .. })` for an `Option<&S>` parameter (P2R-204).
  Elsewhere, an assignment to a local variable included, it has the shape
  Mago gives it, widened (P2R-21), so `['n' => null]` converts only where a
  declared shape gives `n` a nullable type, and a variable that holds such
  a literal stops where it goes to a place of a declared shape with other
  field types (P2R-200). Each value converts as a value assigned to a
  variable of its field's type: P2R-26 and P2R-132 (`.to_string()`,
  `.clone()`, or a move when no later field of the literal and no later
  statement reads the variable) and P2R-102 (`Some(..)` and `None`). Every
  field is written `field: value`, also where the value is a variable of
  the field's name, because whether that variable moves or is copied is
  known only at the end of the body (P2R-26). These stop the conversion: an
  int value for a `float` or `?float` field, also where only one arm of a
  `?:` gives an int (`$b ? 1 : 2.5`), and a list with int elements for a
  `list<float>` field, because PHP does not check a docblock shape and
  keeps the int (P2R-25); a key that appears twice, which PHP keeps once and
  rustc rejects; a literal whose keys are not the keys of the shape it flows
  into; and an element without a key, with a key of another form (a
  variable, a constant, an int), a spread element (`...$a`) or a
  by-reference element (`&$v`).
- **P2R-203** `$s['k']`, where `$s` has a shape type and `'k'` is a string
  literal key of that shape, MUST convert to the field read `s.k`, with the
  field's Rust type, lent as a property read is (P2R-134): a Copy field
  reads as its value, a `String`, `Vec<T>` or shape field is read in
  place where Rust takes a reference and copied with `.clone()` where
  P2R-26 copies, and a nullable field lends as P2R-134 and P2R-204 say.
  Reads nest (`$s['a']['b']` gives `s.a.b`). `$s['k'] ?? $d` on a field of
  type `Option<T>` MUST convert by P2R-105 as for a variable, because a
  shape always holds its keys, so PHP's isset of the element is a check
  against null: `data.name.as_deref().unwrap_or("unknown")`, and a field
  that is not Copy is copied first
  (`data.when.clone().unwrap_or_else(|| make())` for a default with an
  effect, as P2R-105 says).
  These stop the conversion: a key that is not a string literal, a key the
  shape does not have (Mago reports it, P2R-11), a read on an array literal
  (Rust rejects a struct literal there in an `if` condition), a read on a
  shape that may be null (P2R-106), also on the left of `??`, and `??` on a
  shape element whose chain reads a list element (`$ps[0]['k'] ?? 'x'`),
  where PHP's isset gives the default for an index out of range and Rust
  panics.
- **P2R-204** A nullable class type (`?C`, `C|null`) and a nullable shape
  type MUST map to `Option<S>` as P2R-100 maps other types, where `S` is the
  struct; such a type is a nullable struct type below. A parameter of a
  nullable struct type becomes `Option<&S>`, as a nullable string parameter
  becomes `Option<&str>` (P2R-101). An argument for an `Option<&S>`
  parameter MUST be `None` for `null`; `x.as_ref()` for an owned `Option<S>`
  value (a local variable, a property read, a call result, a `?:`); the
  parameter itself for an `Option<&S>` parameter; `Some(x)` for an `S`
  parameter, a binding of P2R-104 and `$this` (`Some(self)`); and
  `Some(&x)` for any other `S` value (a local variable, a property read, a
  list element, a shape literal of P2R-202, a call result, `new`, a `?:` or
  a `??`). `new` stores an `Option<&S>` parameter as
  `s.map(ToOwned::to_owned)` (P2R-133), and a property read of type
  `Option<S>` lends `.as_ref()` as an argument where a string property
  lends `.as_deref()` (P2R-134). Where P2R-26 copies, an owned `Option<S>`
  copies with `.clone()` unless it moves, and an `Option<&S>` parameter
  with `.map(ToOwned::to_owned)`. P2R-102 to P2R-105 apply: `$o === null`
  gives `o.is_none()`, `null` gives `None`, an `S` value where an
  `Option<S>` is needed gives `Some(..)`, and `$o ?? $b` gives an owned `S`
  (`o.unwrap_or_else(|| S::new(..))`, after `.map(ToOwned::to_owned)` for
  an `Option<&S>` parameter), lent with `&(..)` where a reference is needed
  (P2R-105). The
  binding of a check against null (P2R-104) has the type `&S`, as an `S`
  parameter has: `let Some(o) = o.as_ref()` for an owned variable and
  `let Some(o) = o` for a parameter. These stop the conversion: a nullable
  type that holds a changing type (P2R-141); a list of a nullable struct
  type (`list<?C>`); `?->` (P2R-134); a property read, a method call or a
  shape element read on a value that may be null (P2R-106); a comparison of
  a value of a nullable struct type other than with the literal `null`
  (P2R-136, P2R-201); and an assignment to a function's own parameter of a
  nullable struct type (P2R-132).
- **P2R-205** `$v !== null ? $a : $b` and `null !== $v ? $a : $b`, where
  `$v` is a variable or parameter of Rust type `Option<T>`, MUST convert to
  `if let Some(v) = .. { a } else { b }` with the pattern of P2R-104 when
  `$a` reads `$v`: `$a` reads the binding of P2R-104 with its type, and `$b`
  reads `$v`. `$v === null ? $b : $a` and `null === $v ? $b : $a` convert
  to the same `if let`, with the arms swapped, which gives PHP's result
  because only one arm runs. When `$a` does not read `$v`, the condition
  converts by P2R-103. P2R-180 applies to the `if let` as to any `if`, with
  one change for every `if`: an `if` of a class, a shape or a nullable
  struct type converts. Each of its arms gives an owned value (P2R-26,
  P2R-132: `.clone()` unless it moves), and where a reference is needed the
  `if` is lent with `&(..)` for an `&S` parameter and `(..).as_ref()` for an
  `Option<&S>` parameter. So `$d !== null ? C::fromArray($d) : null` as
  the argument for a `?C` parameter, with `$d` a local variable, gives
  `(if let Some(d) = d.as_ref() { Some(C::from_array(d)) } else { None })`
  followed by `.as_ref()`. A string arm that reads the binding is lent as
  the binding's `&str` (P2R-180).

## Spec checklist

- [x] No `[NEEDS CLARIFICATION]` markers left. Decided in review: problems
  inside or equal to a reported span are not reported (P2R-03); exit 2 wins
  over exit 1 (P2R-04); comments are not copied in version 1 (P2R-52); `[]`
  takes its element type from the same variable or the declared type it
  flows into (P2R-21); a file of only functions becomes a library (P2R-33).
- [x] Every requirement is testable: P2R-07 to P2R-49 and P2R-100 to
  P2R-299 by case and stop folders (P2R-75 enforces this), P2R-01 to P2R-06
  by error folders and tests, P2R-50 to P2R-52 by P2R-71, the rest by CI or
  a file in the
  repository. Stated review concerns (P2R-05 partial write, P2R-72 case
  coverage, P2R-176 calls between files checked by rustc only) are the only
  parts not checked mechanically.
- [x] No speculative features: later milestones carry no requirements yet.
