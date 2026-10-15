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

php2rust needs `rustfmt` on `PATH`: it lays out its output with
`rustfmt --edition 2024` and an empty configuration, so a `rustfmt.toml` near the output
or in your own rustfmt configuration does not change it. rustup's default profile installs rustfmt; with the minimal profile,
run `rustup component add rustfmt`.

## Use

```sh
php2rust input.php            # Rust on stdout
php2rust input.php -o out.rs  # Rust in out.rs
php2rust src -o crate         # every .php file under src as one library crate in crate/
php2rust app -o crate --include vendor  # also read vendor/ for its types, not converted
```

`--include <path>`, which may be given more than once, reads every `.php` file under the
path (such as Composer's `vendor/`) for its classes, functions and constants only, so the
input's types that name them resolve. Included files are never converted or reported, a
file that cannot be read, is not UTF-8, does not parse or has a path that is not UTF-8
is skipped, and a class or function of PHP or of the input keeps its meaning over one
of the same name in an included file (a polyfill changes nothing).
A type or call that names an included class still stops the conversion until php2rust
learns that class.

With a directory, php2rust analyses all its `.php` files together, so a class of one file
is known in the others, and writes one Rust module for each PHP namespace below the
namespace the files share: `Acme\Shop\Model` in a package whose files all sit under
`Acme\Shop` becomes `model.rs`, and the shared namespace becomes `lib.rs`. A module names
the items of other modules with `use crate::...;` lines. The output directory must not
exist yet; `rustc --edition 2024 --crate-type lib crate/lib.rs` builds the crate (with
the `indexmap` crate for map output, see below).

Output that holds a map uses the `indexmap` crate (`use indexmap::IndexMap;`). The
`Cargo.toml` that builds it needs `indexmap = "2"` under `[dependencies]`; php2rust writes
no `Cargo.toml`. With plain `rustc`, pass the built crate:
`--extern indexmap=<path to libindexmap.rlib> -L dependency=<its deps directory>`.
Output that calls `preg_replace()`, `preg_match()` or `preg_replace_callback()` uses the
`regex` crate through `::regex::` paths, and needs `regex = "1.13"` with its default
features in the same way (`--extern regex=<path to libregex.rlib>` with plain `rustc`);
its patterns keep escapes that older versions reject, and the `u` form below needs the
Unicode tables of the default features. `preg_replace_callback()` also needs `indexmap`, because
its callback takes the matches as an `IndexMap`.

| Exit code | Meaning |
|---|---|
| 0 | Converted. |
| 1 | The file holds PHP that php2rust does not convert yet. Each problem is printed as `path:line:column: message`. Nothing is written. |
| 2 | The arguments are wrong, the file cannot be read, is not UTF-8 or is not valid PHP, rustfmt cannot be run, fails on the output or changes its own layout when run again, or the output cannot be written. For a directory: no `-o`, an `-o` directory that exists or sits inside the input, or no `.php` file. An `--include` path that does not exist, is neither a file nor a directory, or holds a directory that cannot be listed. Nothing is written. |

The output compiles with no warnings under `rustc --edition 2024 -D warnings` (given the
`indexmap` crate for map output and the `regex` crate for pattern output), and
`rustfmt --edition 2024 --check` with the default configuration leaves it unchanged. Its
layout is that of the rustfmt on `PATH`, so another rustfmt version may lay out the same
code differently; the tests use the rustfmt of the pinned toolchain. Its first line is a
comment that points to the known differences below.

## Supported PHP

php2rust reads PHP 8.5. The file must start with `declare(strict_types=1);`. Version 1
converts:

- the types `int` (`i64`), `float` (`f64`), `bool`, `string` (`String`) and `list<T>`
  (`Vec<T>`), and `void` as a return type. Narrower types such as literal values,
  integer ranges, `non-empty-string` and `non-empty-list<int>` widen to these. A
  docblock may refine `array` to `list<T>` or to an array shape.
- array shapes with string keys (`array{year: string, month: string, day: string}`):
  a `#[derive(Clone, Debug)]` struct named from its keys in byte order
  (`DayMonthYearShape`, fields `day`, `month`, `year`). A literal
  `['year' => $y, ...]` becomes a struct literal with its fields in the order written,
  and `$s['year']` becomes `s.year`; `$s['note'] ?? ''` works on a nullable field.
  A shape is passed as `&S` and copied on assignment, as an object is.
- nullable types: `?T` and `T|null` become `Option<T>`, also as list elements
  (`list<?int>`); a nullable `string` or list parameter becomes `Option<&str>` or
  `Option<&[T]>`. `null` becomes `None` and a value goes into an `Option` as `Some(..)`.
  `=== null` and `!== null` become `is_none()` and `is_some()`, and `$a ?? $b` becomes
  `unwrap_or` (or `unwrap_or_else` when `$b` could have an effect). Inside
  `if ($v !== null)`, after an `if ($v === null)` whose body always leaves, and in the
  matching arm of `$v !== null ? .. : ..`, `$v` is a plain value (`if let` and
  `let ... else`). After such an `if ($v === null)` at the top level of a function or
  method body, whose body ends with `return` or `throw`, later statements may assign `$v`
  again when it holds a string, list, map, int, float, bool or enum: a copy line
  (`let mut v = v.to_string();`) makes the plain value the body's own.
  A nullable object or shape (`?Point`) becomes `Option<Point>`, and
  `Option<&Point>` as a parameter.
- one `namespace Acme\Shop;` right after the `declare`, and `use` imports of classes,
  functions and constants, with or without `as`, grouped or not. They give no Rust: names
  resolve as PHP resolves them, and calls such as `\strlen()` and `namespace\helper()`
  find their function.
- enums declared at the top level, pure or backed by `int` or `string`: a Copy Rust enum
  with UpperCamelCase names (`IN_REVIEW` becomes `InReview`). Cases (also `self::Case`
  inside the enum), `===` and `==` comparisons, `?Suit`, lists of enums, `->name`,
  `->value`, `Suit::from()`, `Suit::tryFrom()` (an `Option`) and `Suit::cases()`
  convert. The Rust enum holds only the helper methods the file uses. An enum may hold
  methods (`pub fn name(self, ...)`, with `match ($this)` as `match self`) and static
  methods.
- static methods in classes and enums (`pub fn` without `self`), called as `C::m()`,
  `self::m()` or `static::m()`; the types `self` and `static`, and `new self` and
  `new static`.
- class and enum constants whose value is a literal or an enum case
  (`private const string TAG = 'x';` becomes `pub const TAG: &str = "x";`), read as
  `C::NAME`, `self::NAME` or `static::NAME`. Constant names become SCREAMING_SNAKE_CASE.
- readonly classes declared at the top level whose constructor only promotes typed
  properties (`public function __construct(public int $x) {}`): a `#[derive(Clone,
  Debug)]` struct with a `new` function and `&self` methods. `new C(...)`, property reads
  (`$p->x`), method calls (`$p->m()`, also as a statement), `$this`, objects as
  parameters (`&C`) and return values, and lists of objects convert. An object is copied
  on assignment, as a string is, so no PHP code can tell that Rust holds two copies.
- classes that are not readonly, which may also declare properties in their body with a
  type and a literal default (`public int $count = 0;`). Their properties can change:
  `$o->p = v`, `+=`, `-=`, `*=`, `.=`, `++`, `--` and `$o->p[] = v`, also through a
  chain (`$this->items[0]->qty++`). The struct of such a class, and of a class that
  holds one, derives only `Debug`, because its objects are never copied: each moves, or
  the conversion stops. A method that changes its object takes `&mut self`, and an object
  parameter becomes `&C`, `&mut C` or `C` (when the function keeps it), decided for the
  whole file. A `foreach` over a list of such objects borrows the list
  (`for x in &mut xs`).
- functions declared at the top level, as `pub fn` that can be called before their
  declaration. Every parameter and return type must be declared. A `string` parameter
  becomes `&str` and a `list<T>` parameter `&[T]`. A call may use any letter case, as
  in PHP. Statements outside functions become `fn main()`; a file of only functions
  becomes a library. A function or method may change its own `string` or list
  parameter: its body then starts with a copy (`let mut p = p.to_string();`), as PHP
  passes one.
- parameter defaults that are literals, `null` (for `?T` or `T|null`), `[]` or enum
  cases, in functions, methods and constructors. A call that leaves a parameter out
  passes its default in Rust. Named arguments (`f(1, wide: true)`) convert when they
  follow the order of the parameters.
- variables with PHP's function-wide scope. Names become snake_case, and a Rust keyword
  becomes a raw identifier (`r#type`). Strings and lists are copied on assignment, as
  in PHP.
- `if`, `elseif`, `else`, `while`, `for`, `foreach` (value only and `key => value`),
  `break`, `continue` and `return`. `while (true)` and `for (;;)` become `loop`.
- `match` as the whole value of a `return` or of `$v = match (...) {...}`: a Rust
  `match` on an int, bool, string or enum, whose conditions are literals or enum cases.
  Several conditions in one arm become `a | b`, and `default` becomes `_`, or is left
  out when the other arms already cover every enum case or both `true` and `false`.
- `$c ? $a : $b` with a bool condition: `if c { a } else { b }`, of int, float, bool,
  string, enum, object or shape arms, or of `?int`, `?float`, `?bool`, a nullable enum,
  object or shape where the place it stands in has such a type. String arms are lent,
  and the result is copied once where a `String` is needed:
  `(if c { "a" } else { "b" }).to_string()`.
- `+`, `-`, `*`, unary minus, `+=`, `-=`, `*=`, `++` and `--` on two ints or two floats,
  `%` on ints, `===` and `!==` on two ints, floats, bools or strings, `==` and `!=` on
  two ints, floats or bools, `<`, `>`, `<=` and `>=` on two ints or two floats, and `&&`,
  `||` and `!` on bools. `<<`, `>>`, `<<=` and `>>=` on ints, by a literal count from 0
  to 63.
- the casts `(int)` of a string, which reads the number at its start as PHP does, and
  `(string)` of an int or a string.
- strings built with `.`, `.=` and interpolation of simple variables (`"Hi $name"`,
  `"Hi {$name}"`) from string and int values, and `echo` of string and int values.
- lists: the literal `[a, b]`, the empty list `[]`, append (`$a[] = x`) and read
  (`$a[$i]`). A read of a string offset (`$s[$i]`) gives that byte as a string.
- the built-in functions `count()`, `strlen()`, `intdiv()`, `fdiv()`, `ctype_digit()`,
  `str_starts_with()`, `str_ends_with()`, `str_contains()`, `substr()`,
  `in_array($v, [...], true)` on a list literal, `trim()`, `ltrim()` and `rtrim()` with
  one argument or with a character list (`'a..z'` ranges as PHP reads them),
  `ucfirst()`, `lcfirst()`, `ucwords()` with one argument, `strtolower()`,
  `strtoupper()`, `str_repeat()`, `str_replace()` on strings and on lists, maps, list
  literals and string-or-array unions of strings (replaced in PHP's order by
  `php_str_replace`), `implode()` of a
  list of strings, `explode()` without a limit, `checkdate()`, and `sprintf()` with a
  literal format of text, `%s`, `%d` and `%%`. Where PHP's rules differ from Rust's
  standard library, the output holds a small private helper function that copies them
  (`php_int`, `php_offset`, `php_substr`, `php_trim`, `php_ltrim`, `php_rtrim`,
  `php_ucfirst`, `php_lcfirst`, `php_replace`, `php_explode`, `php_charmask`,
  `php_trim_chars`, `php_ltrim_chars`, `php_rtrim_chars`, `php_checkdate`,
  `php_ucwords`, `php_preg_replace_callback`, `php_str_replace`, `ctype_digit`), only
  when it calls one.
- arrow functions as the callback of `array_map` or `preg_replace_callback`, which run
  it at once: `array_map(fn(int $x): int => $x * $by, $xs)` becomes
  `xs.iter().copied().map(|x: i64| x * by).collect::<Vec<_>>()`, and a string list goes
  through `.map(String::as_str)` first. The callback takes one typed parameter and may
  read int, float, bool and enum variables of the enclosing body, and nullable ones
  of these types. `array_map` also
  takes a first-class callable of a function of the input or of `trim`, `ltrim`,
  `rtrim`, `strtolower`, `strtoupper`, `ucfirst`, `lcfirst` or `ucwords`, as
  `array_map(ucwords(...), $words)`. `preg_replace_callback(P, fn(array $m): string =>
  .., S) ?? 'literal'` gives the callback the matches as an `IndexMap` with the keys
  `"0"`, `"1"` and on, as PHP passes them, for a pattern of the kind that
  `preg_replace()` takes (see below) that cannot match the empty string or a byte above
  `7F`.
- integer literals in every PHP form, float literals, and single-quoted and
  double-quoted strings.
- exceptions of PHP's built-in classes (`Exception`, `RuntimeException`,
  `InvalidArgumentException` and the other SPL exceptions): `throw new
  RuntimeException('msg', 3)` becomes `Err(PhpException::new(..))`, a function that
  throws returns `Result<T, PhpException>`, and a call of one ends in `?`.
  `try`/`catch` (also `catch (A | B $e)`, `Exception` and `Throwable`) becomes labelled
  blocks that test the class with PHP's class tree; `$e->getMessage()`,
  `$e->getCode()` and `throw $e` convert, and so do `default => throw ..` in a `match`
  and `$x ?? throw ..`. An uncaught exception in `fn main()` exits with code 255.
- classes that implement `ArrayAccess` with a docblock `@implements ArrayAccess<K, V>`:
  `offsetGet()` and the other three methods become plain methods whose `mixed`
  parameters and return take K and V, and `#[Override]` on them is left out.
  `gettype($v)` gives its answer from `$v`'s type, and so does `is_scalar($v)` for a
  scalar `$v`; for any other `$v`, Mago reports the check as impossible and the
  conversion stops.
- declared union types (`int|string $id`, `bool|array{...}|string|null`): each becomes
  a generated enum with one variant per member (`IntOrString { Int(i64), String(String)
  }`), values go into it as `IntOrString::Int(5)`, and `is_scalar()`, `gettype()` and
  `(string)` on a union value become a `match` over its variants. `is_int()`,
  `is_float()`, `is_bool()` and `is_string()` on a union value become `matches!`, and
  `if (!is_string($v)) { continue; }` or `if (is_string($v)) { .. }` binds the string
  (`let IntOrString::String(v) = &v else { continue; };` then `let v = v.as_str();`), as
  a check against null does.
  Other uses of a union value (echo, maths, comparisons) stop.
- `$o?->p` and `$o?->m($x)` on a nullable object become `o.as_ref().map(|o| ..)`,
  and `??` works on the result.
- maps (`array<string, V>`, `array<int, V>`, `array<array-key, V>`): each becomes an
  `IndexMap`, which keeps PHP's key order. A string or `array-key` key becomes text, so an
  int key `5` and the key `'5'` are one key, as in PHP. A literal with a key on every
  element or none (keys 0, 1, 2), `$m = []`, `$m[k]`, `$m[k] = v`, `unset($m[k])`,
  `isset($m[k])`, `array_key_exists()`, `$m[k] ?? $d`, `count()` and `foreach` convert.
  A declared map may hold a union (`array<int|string, scalar>`), and a declared union
  may hold one map (`array<string, int>|string` gives `MapOrString { Map(..), String(..)
  }`). An array shape value goes where a map with text keys is needed as
  `IndexMap::from(shape)`: the output then holds a `From` impl that puts the PHP keys
  in the order of the array literals that build the shape, which must all list them in
  one order.
- `preg_replace(P, R, S) ?? 'literal'` and `preg_match(P, S)` read as a bool (a
  condition, `!`, `&&`, `||`, or `=== 1`, `!== 0`, `=== 0` and `!== 1`), where `P` is a
  string literal, or a class or enum constant of the input with a literal value,
  without the `u` flag, such as `'#\D#'` or `'# {2,}#'`. The pattern becomes a
  `regex::bytes` pattern in `(?-u)` mode, which matches bytes with ASCII classes as
  PCRE does without `u`, and the output holds the helper `php_preg_replace` or
  `php_preg_match`. The pattern may use literals and escaped punctuation, `.`,
  `\d \w \s \b` and their capitals, `\t \n \r \f \xhh`, `\x{..}` up to `7F`, classes
  with ranges and POSIX names, `( )`, `(?: )`, `|`, the quantifiers and their lazy
  forms (a count `{n}` not after a group), `^`, `\A`, `\z`, and `$` with the `m` flag;
  the flags `i`, `m` and `s`. One form with `u` converts in `preg_match()` read as a
  bool: `'#(?<!\p{L})mc(?! )#u'`, with another delimiter that does not end the
  pattern early (such as `/`, `~` or `{ }`), `u` once or more, and any word of ASCII
  letters and digits in place of `mc`, becomes
  `r"(?:^|\P{L})mc(?:[^ ]|$)"`, which matches when the look-around does.
- traits that hold only constants: `use T;` in a class or enum copies the trait's
  constants into it, and the trait itself writes nothing. A constant whose value is an
  array of string keys and string values becomes a slice of pairs
  (`pub const MAP: &[(&str, &str)] = &[("a", "b")];`) that `strtr($s, self::MAP)` reads
  as `php_strtr(s, Name::MAP)`, a helper with PHP's rule: the longest key that matches
  at each position wins, and replaced text is not read again. `strtr($s, [$k => $v])`
  with an array literal of string keys and values becomes
  `php_strtr(s, &[(k, v)])`.
- `DateTime` as the `Copy` struct `PhpDateTime` (year, month, day, hour, minute,
  second), and `DateTime|false` as `Option<PhpDateTime>`.
  `DateTime::createFromFormat($format, $text)` becomes
  `php_date_create_from_format(format, text)`, a helper that reads the format at run
  time with PHP's rules for `d j m n Y y H G i s`, `!`, `|`, a space, `#`, `?` and `\`:
  the widths of the numbers, a field out of range that carries into the next one
  (`19800230` is 1 March 1980), and a field the format does not set that takes the
  current UTC date or time. `$d->format($f)` becomes `php_date_format(d, f)` with the
  same letters. `$v instanceof DateTime`, `$v === false` and their negations become
  `is_some()` and `is_none()`, or bind the date as a null check does, also as the
  first operand of `if (!$v instanceof DateTime || ..) { return ..; }`, which becomes a
  let-else and an `if` with the rest of the condition. `date_parse($d->format('Y-m-d
  H:i:s'))` becomes the struct of the shape `array{year: int, month: int, day: int,
  hour: int, minute: int, second: int}`, read from the date's fields.

## What stops the conversion

Anything not listed above stops the conversion. php2rust then prints every problem with
its line and column, and writes nothing. Its own messages end with the requirement ID;
the others are findings of the Mago analyser. In short:

- PHP that version 1 does not convert, for example closures other than the callbacks
  above, interfaces, casts other
  than the two above, `print`, `printf`, `exit`, heredoc, global constants other than
  `true` and `false`, and built-in functions other than those above. `in_array()`
  without `true` as its third argument, because PHP then compares loosely.
- A parameter default that is not a literal, `null`, `[]` or an enum case, a `null`
  default for a type that is not nullable, and a default before a parameter without
  one. Named arguments in another order than the parameters, a positional argument
  after a named one, a spread argument, and a named argument of a built-in function.
- A function, variable or parameter named like a helper (`php_int`, `php_substr` and the
  others above), which would hide it. A shift by a count that is not a literal from 0 to
  63, and a write to a string offset.
- For the string functions: a `trim()`, `ltrim()` or `rtrim()` character list, written
  as a literal or a class constant, with a `..` that starts no range or a byte above
  `7F`, a `str_repeat()` count that may be negative, a `str_replace()` with a string
  search and an array replacement (PHP throws a `TypeError`), a subject that is not a
  string, a list of other elements or with a spread, an array literal with keys, or a
  union with other members or of another form than a literal, constant, variable,
  parameter or property read on one of them or on `$this`,
  `implode()` with one argument or
  with a glue other than a literal, constant, variable, parameter or property read on
  one of them, `explode()` with a limit or an empty separator, and any
  `sprintf()` format other than text, `%s`, `%d` and `%%`, or with an argument count
  that does not match.
- For patterns: a pattern that is not a literal or a constant of the input, the `u`
  flag outside the one `preg_match()` form above (also that form with another flag, or
  in `preg_replace()` and `preg_replace_callback()`), every flag other than `i`, `m`
  and `s`, a byte that is not printable ASCII (a
  real tab), `$` without `m`, `^` with `m`, `\p` and look-around outside the `u` form
  above, backreferences, `\v`, `\Z`, a
  quantifier after a quantifier or an assertion, a count after a group, a repeat of a
  part that can match the empty string, and every form not listed above.
  `preg_replace()` without `??`, with a default that is not a string literal, with a
  limit, with a pattern that can match the empty string, with a replacement that is not
  a literal or a constant or that holds `$` or `\`, or with a subject that is not a
  string. `preg_match()` with matches, or with its result used any other way
  (`=== false`, `== 1`, stored).
- For `DateTime`: `new DateTime()`, every method but `createFromFormat()` and
  `format()`, `DateTimeImmutable` and `DateTimeInterface`, a union of `DateTime` with a
  type other than `false`, a comparison of two dates, `=== null`, `??` and `?->` on a
  `DateTime|false` value (it holds `false`, not `null`), a literal format with a letter
  the helpers do not read, or with `*` or `+` for `createFromFormat()`, `format()` on a
  value that may be `false`, `instanceof` with another class, a read of the date in
  the rest of a `||` condition where the let-else does not apply (the body reads the
  date or does not end with `return`, `throw`, `break` or `continue`, the `if` has an
  `else`, or a `for` with a step holds a `continue` or `break` of the body), and
  `date_parse()` of any other text or into a variable whose docblock gives another
  type.
- For traits and `strtr`: a trait with a method, a property or a `use` of another
  trait, a `use` with a block of conflict rules, a trait the input does not declare, a
  trait constant whose name the class or another trait it uses declares too, a trait
  named twice in one class, a trait named like another class-like or with a name PHP
  reserves, a trait that no class or enum of the input uses, and a trait inside a
  function. A
  string map constant with an empty or repeated key, a key or value that is not a string
  literal, no pairs, or a use other than the second argument of `strtr`. `strtr` with
  three arguments, with an array other than such a constant or a literal of string
  pairs, or with an empty or int key in the literal.
- For callbacks: an arrow function anywhere else, without a parameter type or with
  more than one parameter, `fn&`, a read of a string, list, object or map of the
  enclosing body, a write, a call that throws or `?? throw` in the body, `$this`, a
  `match` body, a nested arrow function, and a long `function () { .. }` closure.
  `array_map` over a map, a list of other elements or several lists, or with a
  callback given as a string or a variable. A `preg_replace_callback` pattern that can
  match the empty string or a byte above `7F`, and a callback without `array $m` and
  the return type `string`. `ucwords()` with delimiters.
- A `?:` without its middle part (`$a ?: $b`), inside another `?:`, of lists, of a
  nullable string or list, or with a string arm that is a call, a joined string, or a
  property read or list element on a call or `new`.
- For array shapes: an optional key (`key?:`), an int key, a key with no Rust name (a
  shape open to other keys, `...<string, int>`, is a map), a shape that holds an object or an enum without
  cases, two shapes that give one struct name, a struct name that an enum or class
  already has, comparing, printing or changing a shape, `isset` or `empty` of an
  element, `??` on an element of a list element, a key that is not a string literal,
  a read on an array
  literal, a literal with a repeated key, a spread or an element without a key, an int
  for a `float` field (PHP keeps the int), a list of nullable objects (`list<?Point>`),
  and a class that holds itself through `?C` (`?Node $next` in `Node`). A shape where a
  map is needed when two literals build it in different key orders, when no literal
  builds it, when a field has another type than the map's values, or when the map has
  int keys. An array literal where a union is needed, and a union's map value used in a
  condition, `count()`, `is_array()` or `(string)`.
- For exceptions: the input's own exception classes (they need `extends`), `Error` and
  `TypeError`, an unimported `RuntimeException` inside a namespace, a constructor
  argument other than the message and the code, an exception object kept or passed
  on, `finally`, a `try` inside a `try` or its catch, `break` or `continue` that leaves
  a `try` body, a `try` body that throws nothing, catches with different variable
  names, a catch after `Exception` or `Throwable`, a catch variable the function
  already uses, a `match` whose every arm throws, a `throw` in a `?:` arm, and a call
  that throws in the default of `??`.
- For `ArrayAccess`: a class without `@implements ArrayAccess<K, V>`, or with a K or V
  that has no Rust type or is a float, another interface beside it, and `$o['k']` on
  such an object (call `offsetGet()`). Any attribute other than one `#[Override]` on
  those four methods.
- A namespace with braces, a second namespace, or one after other code. Imports that PHP
  rejects (two imports with one name, an import named like a class or function of the
  file, `as self`) or warns about (`use Foo;` outside a namespace).
- A class that is abstract, extends a class, implements an interface other than
  `ArrayAccess`, uses a trait other than one of constants, implements `ArrayAccess` or
  uses a trait and comes after code outside functions (PHP declares it only when
  execution reaches it), or
  holds a static property, a constructor with a body or a method whose name starts with
  `__`. A declared property without a type or a literal default, or that is static,
  readonly or hooked. A nullable type that holds objects whose class changes them
  (`?Cell`, `?list<Cell>`), the type `parent` and `parent::`,
  a class that holds itself, a method named `clone` or `new`, a `final private` method,
  two methods whose names differ only in letter case, and a function that assigns to
  its own object parameter.
- A static method called through an object, a method that is not static called with
  `::`, `$this` in a static method, and `self::` outside a class or enum.
- A constant whose value is not a literal or an enum case, two constants in one
  `const`, a constant named like a case or another constant of its type, `C::class`, and
  a constant as a `match` condition. An enum method named `name`, `value`, `from`,
  `tryFrom` or `cases`.
- Comparing, printing, cloning or testing an object (`==`, `===`, `echo`, `clone`,
  `instanceof`, `if ($object)`), and `??` on a property or a method call. `?->` on a
  value that cannot be null, on a call result, in a chain (`$a?->b->c`), on a class that
  changes its objects, on a void method or one that throws, and with an argument that is
  more than a literal, a constant, a variable or a property read.
- A union inside a list or a shape field, a union whose PHP type and docblock type
  disagree on int and float, `(string)` of a union that may hold a float, a list, a map,
  a shape or an object (outside `is_scalar($v) ? (string) $v : ..`), and a union value passed
  where another union type is needed. `is_string()` and its kin on a value that is not a
  union, a use of the member value after a test in any other form
  (`if (!is_string($v) || $b) { return; } strlen($v)`), which narrows nothing, and a write
  to a narrowed variable while its binding holds.
- For maps: a value type that is `mixed` or a class that changes its objects, a union
  value type outside a declared type, a second map inside a union, a key type other than
  int, string or `array-key`, a list, or a shape outside the rules above, where a map is
  needed, a map where a list or shape is needed, a map where a map of other key or value
  types is needed (`array<string, int>` for `array<string, ?int>`), a float, bool, null or union key, a
  string key of an int-keyed map, an int for a `float` value, `$m[] = v`, a key with
  `foreach` over a map with text keys, `+=`, `++` or `??=` on an element, a write or
  `unset` of an element of a map property, comparing or printing a map, `isset` or
  `array_key_exists()` on anything but a map, and a `foreach` that changes its map.
- A copy of an object whose class changes its objects: `$b = $a;` followed by a read of
  `$a`, `return $this;`, or a property or list element of such an object used as a value.
  A change through a call result, `??=` on a property, a write to a list element of a
  property, an assignment to a `foreach` value of such a class, and a `foreach` over a
  list of such objects that a call or a list literal gives. A property write
  (`$g->n = 1;`) that nothing reads afterwards, which rustc rejects as unused. An int
  appended to a `list<float>` property, or a list of ints assigned to one, because PHP
  keeps the ints. For a parameter that changes its object, an argument that is not a
  variable, a parameter, `$this`, a `foreach` value, a chain of property and list reads
  on one of them, or `new`.
- A statement that changes an object and names it again, such as `$a->add($a->next())`
  or `$this->text .= $this->text`, other than the three forms rustc accepts (see P2R-147
  in `docs/spec.md`). Inside a `foreach` over a list of such objects, a use of the object
  that holds the list other than through its other properties.
- A `match` anywhere other than a `return` or an assignment, on a value that may be
  null, with a condition that is not a literal or an enum case, with two `default`
  arms or two conditions of the same value, or without `default` where its arms leave a
  value out (every int or string `match` needs `default`).
- An enum with an interface, a trait other than one of constants, an attribute or a
  backing value that is not a
  plain literal, an enum declared inside a function or a block, and an enum declared
  after other code outside functions: PHP declares an enum only when execution reaches
  it.
- A type with no single Rust type: `mixed`, `null` on its own, `array` without a list
  element type, a key and value type or a shape, or a union of different types other than
  `T|null`. A missing
  parameter or return type.
- A value that may be null where PHP gives null a meaning Rust does not copy, or where a
  plain value is needed: in `echo`, string building, arithmetic, a shift, a cast, a
  condition, `foreach`, a list read, a string offset, a built-in function, or for a
  parameter or return type that is not nullable. Also `==` with `null`, `??` on a list
  element, `??=`, and a write to a variable while a check against null binds it.
- PHP behaviour that Rust would not copy exactly: `/` (use `intdiv()` or `fdiv()`), `**`,
  `<=>`, `==` and ordering on strings, arithmetic or comparison that mixes int and float,
  a float or bool in `echo` or in a string, a condition that is not a bool, an
  assignment or `++` used as a value, a write to a list index, and a `foreach` by
  reference or one that changes the list it iterates.
- A read that a path reaches with no assignment before it, a variable whose type
  changes, a value that is assigned and never read, and two PHP names that give the same
  Rust name.
- A name whose snake_case form is not a plain ASCII Rust name: `$_2` (it would become
  `2`), and any name with a letter outside ASCII, such as `$größe` or `$naïve`.
- Code that rustc would reject: a statement that no path reaches, int arithmetic that
  always overflows, `%` or `intdiv()` by a zero the tool can prove, and a function that
  calls itself on every path.

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
- A list holds `Option` elements only when its first write holds `null`:
  `$xs = [1, 2]; $xs[] = null;` stops.
- A `for` step that reads a variable that a let-else in the loop body checks stops, also
  where Rust would accept the code.
- A write to a variable after an `if ($v === null)` that returns stops when the check is
  not at the top level of the function body, when the body leaves in another way than
  by ending with `return` or `throw` (an `if` and `else` that both return), and when a
  `foreach` after it takes the name `$v`.
- `Nothing::from($v)` on an enum without cases stops: the call never returns, and rustc
  rejects the code after it.
- Visibility is not copied: private methods and constants become `pub` in Rust.
- A single input file still converts alone, so a class from another file of a library is
  unknown there (Mago reports it); convert the library's directory instead. A directory
  converts to a library only: code outside functions stops, and an item of a package
  outside the directory is unknown.
- In a directory, every enum, class and function needs a Rust name of its own across the
  whole crate, and two spellings of one namespace in other letter case stop.
- A `match` on a value that Mago narrows stops where PHP never fails. After
  `if ($s === Suit::Hearts) { return 0; }`, a `match` with arms for every other case
  stops: without `default` because Rust needs every case, and with `default` because
  Mago finds that arm unreachable and gives it no type. `match ($n % 2)` with arms `0`,
  `1` and `-1` stops for the same two reasons. Handle the narrowed case inside the
  `match` instead of before it.
- A nullable property read after a check against null is still an `Option`, so
  `if ($this->name !== null) { return $this->name . '!'; }` stops. Read the property into
  a variable first.
- A `match` that starts a new variable stops when its arms are all `[]` or all `null`,
  or when its list arms differ in whether their elements may be null (`[null, 1]` and
  `[5]`), because no arm gives the variable its type. Assign the variable first.
- When php2rust cannot work out the class of an object a method is called on, it treats
  the call as a change when any class of the file has a method of that name that changes
  its object. The caller's object parameter then becomes `&mut`, which can stop code
  that would otherwise convert.
- A `foreach` over a list of objects whose class changes them stops when a call or a
  list literal gives the list, also where only reading would compile. Put the list in a
  variable first.
- The rule of one changed object per statement is stricter than rustc: for example
  `$a->log .= "{$a->hits}"` stops, and so does a call that changes a list element
  when its arguments read the same object, even another property of it
  (`$this->items[0]->add($this->qty)`). Read the value into a variable first.

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
- Reading a map at a key it does not have (`$m['k']` without `??` or `isset`): PHP warns
  and gives `null`. Rust panics.
- Reading a string offset past either end of the string: PHP warns and gives `""`. Rust
  panics.
- `substr()` or a string offset that cuts a character of more than one byte: PHP gives
  the raw bytes, Rust gives U+FFFD (three bytes) in their place, so later lengths,
  comparisons and joins of that value differ too. The same holds for a
  `preg_replace()` that removes part of such a character, and for `trim()` with a
  character list that is not a literal or a constant and holds a byte above `7F`.
- `trim()` with such a character list that holds a `..` that starts no range: PHP
  warns and Rust does not; both remove the same bytes.
- A pattern that runs past PCRE's backtrack, recursion or JIT stack limit: PHP's
  `preg_replace()` and `preg_replace_callback()` give `null`, so `??` gives the
  default, and `preg_match()` gives `false`. Rust's `regex` has no such limit and gives
  the full result.
- The `u` form of `preg_match()` where a letter that only one Unicode version has
  stands right before the word: Rust's `\p{L}` follows Unicode 16.0 (`regex-syntax`
  0.8.11, which the tests build), PHP's follows the PCRE2 it links (17.0 for PCRE2
  10.48, 14.0 for 10.42), so that occurrence can count in one and not in the other.
- The `u` form of `preg_match()` on a subject that is not valid UTF-8 in PHP (after a
  cut character, above): PHP gives `false`, so a condition, `=== 1` and `=== 0` are
  false and `!== 0`, `!== 1` and `!` are true; the Rust subject holds U+FFFD there and
  matches as any other.
- An array shape that becomes a map, where a PHP caller outside the converted code
  built the array in another key order: the Rust map has the order of the converted
  code's literals, because a struct has no order.
- `Suit::from()` with a value that no case has: PHP throws a `ValueError` (exit code 255
  when uncaught), Rust panics (exit code 101).
- An `int` property that overflows in `+=`, `-=`, `*=`, `++` or `--`: PHP throws a
  `TypeError` (exit code 255 when uncaught), because a typed property cannot hold the
  float; Rust panics in a debug build, as for a variable.
- `str_repeat()` with a count that is negative at run time, and `explode()` with a
  separator that is empty at run time: PHP throws a `ValueError` (exit code 255 when
  uncaught), Rust panics (exit code 101), or `str_repeat()` of an empty string gives an
  empty string.
- `str_replace()` where the search is a string at run time and the replacement an
  array (a union hides it from the tool): PHP throws a `TypeError`, Rust panics.
- `strtr()` with an array literal whose key is empty at run time, on a text that is
  not empty: PHP warns and skips the key, Rust skips it without the warning.
- `DateTime::createFromFormat()` where the format leaves a field unset: PHP fills it
  from the current date and time of its default time zone (`date.timezone`, UTC when
  not set), Rust from the current UTC date and time. Under a zone other than UTC, PHP
  also moves a local time that falls in a gap of the zone forward, and Rust does not.
- A date format read at run time with a letter the helpers do not read (or `*` or `+`
  for `createFromFormat()`), and a text with a NUL byte: PHP reads the letter or throws
  a `ValueError`, Rust panics (exit code 101).
- `date_parse()` of a date whose year is above 9999, which the carries can give: PHP
  misreads the text (the year less 8000, hour 10), Rust gives the date's fields.
- The errors above that Rust turns into panics are not exceptions, so a
  `catch (Throwable $e)` does not catch them.
- An uncaught exception writes a shorter message than PHP's (no file, line or stack
  trace); the exit code is 255, as in PHP.
- Comments and docblocks are not copied (version 1).

## Development

The tests need PHP 8.5 on `PATH` and the Rust toolchain pinned in
`rust-toolchain.toml` (`rustup toolchain install` installs it). Each folder under
`tests/cases`, `tests/stops` and `tests/errors` is one test, and `cargo test` also checks
that each requirement from P2R-07 to P2R-49 and from P2R-100 to P2R-399 starts the name of
a case or stop folder.
To rewrite every
`expected.rs` and `expected.txt` from the current output:

```sh
PHP2RUST_BLESS=1 cargo test --test folders
```

Review the diff before you commit it.

## Licence

MIT OR Apache-2.0, at your option.
