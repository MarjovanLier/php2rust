# php2rust

php2rust converts typed PHP into normal, readable Rust. It uses no AI. It reads the PHP
with the [Mago](https://github.com/carthage-software/mago) crates, takes the type of
every value from Mago's analyser, and writes Rust only when every part of the file
converts. Code it cannot convert yet stops the conversion and is listed, so each refusal
shows what to add to the tool next.

The behavioural contract is [docs/spec.md](docs/spec.md). Requirement IDs such as
P2R-46 in messages and tests point into it.

## Install

Clone the repository, then:

```sh
cargo install --locked --path .
```

`--locked` keeps every Mago crate on the pinned version.

## Use

```sh
php2rust input.php            # Rust on stdout
php2rust input.php -o out.rs  # Rust in out.rs
```

| Exit code | Meaning |
|---|---|
| 0 | Converted. |
| 1 | The file holds PHP that php2rust does not convert yet. Each problem is printed as `path:line:column: message`. Nothing is written. |
| 2 | The arguments are wrong, the file cannot be read, is not UTF-8 or is not valid PHP, or the output cannot be written. Nothing is written. |

The output compiles with no warnings under `rustc --edition 2024 -D warnings`, and
`rustfmt --edition 2024 --check` leaves it unchanged. Its first line is a comment that
points to the known differences below.

## Supported PHP

php2rust reads PHP 8.5. The file must start with `declare(strict_types=1);`. Version 1
converts:

- the types `int` (`i64`), `float` (`f64`), `bool`, `string` (`String`) and `list<T>`
  (`Vec<T>`), and `void` as a return type. Narrower types such as literal values,
  integer ranges, `non-empty-string` and `non-empty-list<int>` widen to these. A
  docblock may refine `array` to `list<T>`.
- nullable types: `?T` and `T|null` become `Option<T>`, also as list elements
  (`list<?int>`); a nullable `string` or list parameter becomes `Option<&str>` or
  `Option<&[T]>`. `null` becomes `None` and a value goes into an `Option` as `Some(..)`.
  `=== null` and `!== null` become `is_none()` and `is_some()`, and `$a ?? $b` becomes
  `unwrap_or` (or `unwrap_or_else` when `$b` could have an effect). Inside
  `if ($v !== null)`, and after an `if ($v === null)` whose body always leaves, `$v` is
  a plain value (`if let` and `let ... else`).
- enums declared at the top level, pure or backed by `int` or `string`, whose body holds
  only cases: a Copy Rust enum with UpperCamelCase names (`IN_REVIEW` becomes
  `InReview`). Cases, `===` and `==` comparisons, `?Suit`, lists of enums, `->name`,
  `->value`, `Suit::from()`, `Suit::tryFrom()` (an `Option`) and `Suit::cases()`
  convert. The Rust enum holds only the helper methods the file uses.
- functions declared at the top level, as `pub fn` that can be called before their
  declaration. Every parameter and return type must be declared. A `string` parameter
  becomes `&str` and a `list<T>` parameter `&[T]`. A call may use any letter case, as
  in PHP. Statements outside functions become `fn main()`; a file of only functions
  becomes a library.
- variables with PHP's function-wide scope. Names become snake_case, and a Rust keyword
  becomes a raw identifier (`r#type`). Strings and lists are copied on assignment, as
  in PHP.
- `if`, `elseif`, `else`, `while`, `for`, `foreach` (value only and `key => value`),
  `break`, `continue` and `return`. `while (true)` and `for (;;)` become `loop`.
- `+`, `-`, `*`, unary minus, `+=`, `-=`, `*=`, `++` and `--` on two ints or two floats,
  `%` on ints, `===` and `!==` on two ints, floats, bools or strings, `==` and `!=` on
  two ints, floats or bools, `<`, `>`, `<=` and `>=` on two ints or two floats, and `&&`,
  `||` and `!` on bools.
- strings built with `.`, `.=` and interpolation of simple variables (`"Hi $name"`,
  `"Hi {$name}"`) from string and int values, and `echo` of string and int values.
- lists: the literal `[a, b]`, the empty list `[]`, append (`$a[] = x`) and read
  (`$a[$i]`).
- the built-in functions `count()`, `strlen()`, `intdiv()` and `fdiv()`.
- integer literals in every PHP form, float literals, and single-quoted and
  double-quoted strings.

## What stops the conversion

Anything not listed above stops the conversion. php2rust then prints every problem with
its line and column, and writes nothing. Its own messages end with the requirement ID;
the others are findings of the Mago analyser. In short:

- PHP that version 1 does not convert, for example classes, closures, `match`,
  `namespace`, `use`, casts, `print`, `printf`, `exit`, heredoc, constants other than
  `true` and `false`, and built-in functions other than the four above.
- An enum with a method, a constant, an interface or a backing value that is not a plain
  literal, and an enum declared after other code outside functions: PHP declares an
  enum only when execution reaches it.
- A type with no single Rust type: `mixed`, `null` on its own, `array` without a list
  element type, or a union of different types other than `T|null`. A missing parameter or
  return type.
- A value that may be null where PHP gives null a meaning Rust does not copy, or where a
  plain value is needed: in `echo`, string building, arithmetic, a condition, `foreach`,
  a list read, a built-in function, or for a parameter or return type that is not
  nullable. Also `==` with `null`, `??` on a list element, `??=`, a parameter default
  value, and a write to a variable while a check against null binds it.
- PHP behaviour that Rust would not copy exactly: `/` (use `intdiv()` or `fdiv()`), `**`,
  `<=>`, `==` and ordering on strings, arithmetic or comparison that mixes int and float,
  a float or bool in `echo` or in a string, a condition that is not a bool, an
  assignment or `++` used as a value, a write to a list index, a function that changes
  its own string or list parameter, and a `foreach` by reference or one that changes the
  list it iterates.
- A read that a path reaches with no assignment before it, a variable whose type
  changes, a value that is assigned and never read, and two PHP names that give the same
  Rust name.
- A name whose snake_case form is not a plain ASCII Rust name: `$_2` (it would become
  `2`), and any name with a letter outside ASCII, such as `$größe` or `$naïve`.
- Code that rustc or rustfmt would reject or change: a statement that no path reaches,
  int arithmetic that always overflows, `%` or `intdiv()` by a zero the tool can prove,
  a function that calls itself on every path, and a statement too long for one line.

## Known limitations

php2rust refuses some valid PHP that version 1 converts in other forms:

- After a literal list and an append (`$a = [1, 2]; $a[] = 3;`), Mago reads the list
  type wrongly, so a read at a literal index (`$a[0]`, `$a[2]`) stops with a Mago error
  such as `undefined-int-array-index`. A read at a variable index converts.
- An empty list that is read while it is still empty (`$a = []; echo count($a);`) stops
  when its variable name also holds a list of another element type in the same body,
  for example as a `foreach` value, because php2rust takes the element type from every
  read and write of the name.
- A variable that a `while (true)` loop assigns before each of two `break` statements,
  and that is read after the loop, stops, because Mago gives it the type `mixed`.
- `$s = "$s"` stops as an assignment of a variable to itself.
- Every int overflow and every zero divisor that the tool can prove stops (P2R-50), also
  where rustc would accept the code, for example `%` by a variable that is 0 on every
  path through an `if`.
- A variable that may be null stays an `Option` after an assignment that PHP knows makes
  it not null: after `if ($c === null) { $c = 0; }`, a plain use such as `$c + 1` or
  `echo $c` stops. A new variable from `??` works: `$n = $c ?? 0;`.
- `$a ?? $b ?? 'x'` on local string variables in `echo` or an assignment is wider than
  rustfmt's 60 columns for call arguments and method chains, so it stops.
- A list holds `Option` elements only when its first write holds `null`:
  `$xs = [1, 2]; $xs[] = null;` stops.
- A `for` step that reads a variable that a let-else in the loop body checks stops, also
  where Rust would accept the code.

## Known differences from PHP

- An `int` that overflows in `+`, `-`, `*`, `+=`, `-=`, `*=`, `++`, `--` or unary
  minus: PHP turns it into a `float`. Rust panics in a debug build, and in a release
  build the value wraps silently unless the crate sets `overflow-checks = true` in
  `[profile.release]`.
- `%` of `PHP_INT_MIN` by `-1`: PHP gives `0`, Rust panics in every build.
- `intdiv(PHP_INT_MIN, -1)`, and division by zero in `intdiv()` or `%`: PHP throws an
  error (exit code 255 when uncaught), Rust panics (exit code 101) with a different
  message.
- Reading a list at a negative index or past its end: PHP warns and gives `null`. Rust
  panics.
- `Suit::from()` with a value that no case has: PHP throws a `ValueError` (exit code 255
  when uncaught), Rust panics (exit code 101).
- Comments and docblocks are not copied (version 1).

## Development

The tests need PHP 8.5 on `PATH` and the Rust toolchain pinned in
`rust-toolchain.toml` (`rustup toolchain install` installs it). Each folder under
`tests/cases`, `tests/stops` and `tests/errors` is one test, and `cargo test` also checks
that each requirement from P2R-07 to P2R-49 and from P2R-100 to P2R-199 starts the name of
a case or stop folder.
To rewrite every
`expected.rs` and `expected.txt` from the current output:

```sh
PHP2RUST_BLESS=1 cargo test --test folders
```

Review the diff before you commit it.

## Licence

MIT OR Apache-2.0, at your option.
