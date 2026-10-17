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
string arm of P2R-205. Section 22 (exceptions as `Result`) was written on
14 Oct 2026; it changes P2R-07, P2R-33, P2R-43, P2R-60, P2R-72, P2R-104,
P2R-105, P2R-120, P2R-160, P2R-171, P2R-172, P2R-174 and P2R-175. The two
spec reviewers replaced the first `if let` form of P2R-214 with the
labelled blocks that rustc follows as PHP does, and added the flow rule of
P2R-212, the catch-all, naming and nesting rules of P2R-214, the
all-throw stop of P2R-215 and the exit rules of P2R-216. The final review
of the build added the handler `break` stop, the plain block around the
binding and the `getCode()` limits of P2R-214. Section 23 (`ArrayAccess`
classes) was written on 14 Oct 2026; it changes P2R-07, P2R-22, P2R-48,
P2R-72, P2R-130, P2R-135, P2R-166, P2R-171, P2R-180, P2R-201 and the
roadmap of section 10. The spec reviewer added the float stop and the
declared-type rule of P2R-220, the single `#[Override]` of P2R-221 and
the `&str`, `$this` and `while` rules of P2R-222. Section 24 (union types
and `?->`) was written on 15 Oct 2026; it changes P2R-07, P2R-22, P2R-25,
P2R-105, P2R-134, P2R-174, P2R-204, P2R-210, P2R-220, P2R-222 and the
roadmap of section 10. The two spec reviewers added the written-type
stops of P2R-25 and P2R-230, the owned and lent forms and the
other-union stop of P2R-231, the guarded `unreachable!()` of P2R-233
(Mago's narrowed type alone proved unsound), the argument stop of
P2R-234, the `use` line of P2R-174, and the `Result`, `Ok` and `Err`
names of P2R-210. Section 25 (maps) was written on 15 Oct 2026; it
changes P2R-07, P2R-22, P2R-50, P2R-60, P2R-82, P2R-111, P2R-172,
P2R-174, P2R-200 and the roadmap of section 10, whose open point on
output that needs a crate it decides. The two spec reviewers added the
`m[&k]` rule (`IndexMap` also indexes by position), the borrowed and
owned key forms, the `??` and `values()` forms, the decimal text of an
int key, the stops for changing classes, union keys, lists or shapes
where a map is needed and the `IndexMap` name, and the `use` line
placement. The build added `$m = []` on a variable, Mago's shape types
of a map variable after writes by literal keys, and the stop for map
property writes; the two final reviewers added the stop for a map of
other values and the stop for a module named `indexmap` at any depth.
Section 26 (unions as map values, type tests and their narrowing) was
written on 15 Oct 2026; it changes P2R-07, P2R-106, P2R-230, P2R-232,
P2R-240, P2R-241 and the roadmap of section 10. The two spec reviewers
added the `as_str()` line after a string member's binding, enums among
the Copy members, the forms for the `&U` binding of a null check, and
the exceptions in P2R-106 and P2R-232; the build added plain-value
literals where a map is needed and the declared map type of a `[]`.
Section 27 (regular expressions without the `u` flag) was written on 15
Oct 2026; it changes P2R-07, P2R-60, P2R-171, P2R-188 and the roadmap of
section 10. Its final review stopped `[.a.]`, `[=a=]` and a count after
a group, which PCRE rejects, and let `[:]` convert as the plain class it
is in both engines. Section 28 (maps as union members, and shapes as
maps) was written on 15 Oct 2026; it changes P2R-07, P2R-60, P2R-233,
P2R-240, P2R-250 and the roadmap of section 10. Section 29 (traits with
constants, string map constants and `strtr`) was written on 15 Oct 2026;
it changes P2R-07, P2R-110, P2R-111, P2R-130, P2R-156, P2R-173, P2R-175,
P2R-188 and the roadmap of section 10. Section 30 (writes after a null
check, `trim` with a character list and `checkdate`) was written on 15
Oct 2026; it changes P2R-48, P2R-60, P2R-104, P2R-181, P2R-182, P2R-188
and the roadmap of section 10. Section 31 (closures as callbacks of
`array_map` and `preg_replace_callback`, and `ucwords`) was written on 15
Oct 2026; it changes P2R-07, P2R-48, P2R-60, P2R-75, P2R-188 and the
roadmap of section 10. Section 32 (arrays in `str_replace` and `strtr`)
was written on 15 Oct 2026; it changes P2R-07, P2R-48, P2R-60, P2R-185,
P2R-188, P2R-232, P2R-282 and the roadmap of section 10. Section 33
(look-around in two `u` patterns read as a bool) was written on 15 Oct
2026; it changes P2R-07, P2R-60, P2R-260, P2R-262 and the roadmap of
section 10. Section 34 (`DateTime` from a format, and `date_parse` of
its text) was written on 15 Oct 2026; it changes P2R-07, P2R-48,
P2R-60, P2R-104, P2R-136, P2R-174, P2R-188 and the roadmap of section
10. Its
PHP spec reviewer found that PHP ends a format at a NUL byte, throws
at a NUL in the text, and moves a local time in any gap of its zone;
its Rust spec reviewer found the missing `use` line of a module whose
helper alone names `PhpDateTime`; the build found that Mago types
`date_parse()` as `mixed` and that `DateTime|false` must not take the
forms of `null`. Section 35 (code that the input uses but does not
convert) was written on 15 Oct 2026; it changes P2R-04, P2R-07, P2R-60,
P2R-73, P2R-82 and the roadmap of section 10. Its tool spec reviewer
found that a namespaced included function must block the built-in of
its short name, that an included file needs a name no input can have,
and that Mago's findings in an included file must not be printed; its
Mago spec reviewer found that the same holds for a global function of
the input, and that an attribute on a function or a parameter
converted, against P2R-07. The side-by-side measure then found that
`StringManipulation` stopped on its `#[\SensitiveParameter]`, which
P2R-341 now leaves out. The break reviewer found that a skipped
included file hides such a function, now a P2R-60 difference, and that
a cycle of docblock type aliases crashes Mago, now an exception in
P2R-04. Section 36 (Spatie `Data` classes as plain structs) was
written on 15 Oct 2026; it changes P2R-07, P2R-73, P2R-130, P2R-135,
P2R-153, P2R-221, P2R-340, P2R-341 and the roadmap of section 10. Its
spec reviewers found that PHP rejects attribute arguments that are not
constant expressions, a list of parents and an incompatible override
when it compiles the file, where Mago does not always report them, and
that with Composer's autoloader PHP declares such a class only when
execution reaches it. Section 37 (Spatie's `Optional`) was written on
16 Oct 2026; it changes P2R-07, P2R-168, P2R-333 and the roadmap of
section 10. Section 38 (interfaces of the input as traits) was written
on 16 Oct 2026; it changes P2R-07, P2R-33, P2R-110, P2R-130, P2R-172,
P2R-173, P2R-174, P2R-175, P2R-220, the intro of section 23 and the
roadmap of section 10. Its spec reviewers found that an enum method takes
its value where a class method borrows, that PHP matches method names
ignoring letter case where Rust names differ, that PHP rejects several
interface forms that Mago's analyser does not report, and that a trait
method meets the methods Rust's prelude gives every type. Section 39
(laravel-data's request hooks left out) was written on 16 Oct 2026; it
changes P2R-07, P2R-340, P2R-341, P2R-350, P2R-351, section 14 and the
roadmap of section 10. Its spec reviewers found that PHP rejects several
forms of such a method when it loads or compiles the class, that Mago
checks a hook against an `@method` tag that PHP never reads, and that
PHP runs a `new` in an attribute argument only when reflection reads it.
Section 40 (exception classes of the input) was written on 16 Oct 2026;
it changes P2R-07, P2R-31, P2R-153, the intro of section 22, P2R-210,
P2R-212, P2R-214, section 14 and the roadmap of section 10. Its spec
reviewers found constructor forms that PHP rejects and Mago does not
report, on ordinary classes too, that PHP rejects a modifier written
twice, a modifier on the parameter of a function, `readonly` on a
method, a set visibility wider than its property's visibility and
`final private` on a declared property, and that PHP rejects a
parameter named like a superglobal anywhere, not only in a hook. It
also changes P2R-135, P2R-140 and P2R-380. Section 41 (Eloquent models
from migrations) was written on 16 Oct 2026; it changes P2R-07, P2R-73,
P2R-75, P2R-82, P2R-130, P2R-350, the checklist and the roadmap of
section 10. Its spec
reviewers found that the app's migrations seed rows with `DB::table`,
call `uuid()` without a name and write `: void` on their closures, that
an accessor, a `casts()` method or an observer changes what an attribute
read gives, that PHP reads a boolean or decimal column differently per
database driver, and that `foreignIdFor` takes its column from the
parent's key. Section 42 (creating and finding models) was written on 16
Oct 2026; it changes P2R-07, P2R-75, P2R-401, P2R-404, the checklist
and the roadmap of section 10.

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
  (P2R-07), the tool MUST print every problem it finds to stderr and exit
  1; inside one form that does not convert, it may stop at the first
  problem it meets, so a refused form gives at least one line. Each
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
  failing, so the parse check is explicit. One known exception: Mago 1.57
  recurses without end when a docblock type uses a cycle of
  `@psalm-type` or `@phpstan-type` aliases (`X = Y` and `Y = X`), in the
  input or in an included class whose method, its constructor included,
  the input calls (P2R-340);
  the tool then overflows its stack and aborts with no reason (exit 134).
  A larger stack would not help, because the recursion has no end.
- **P2R-05** On exit 1 or 2 the tool MUST write nothing to stdout and MUST
  NOT create, change or truncate the output file. A failed write MUST NOT
  leave a partial output file (review concern, not checked mechanically).
- **P2R-06** With the same rustfmt (P2R-190), the same input MUST always give
  the same stdout bytes, the same output file bytes and the same stderr
  bytes.
- **P2R-07** Only the constructs that sections 3 to 5, 11 to 19 and 21 to
  48 list as converting may convert. Any other construct MUST stop the conversion,
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
- **P2R-22** Any other type (`mixed`, other than at the places of the
  `ArrayAccess` methods that take K or V (P2R-220), a union of different
  base types other
  than `T|null` (P2R-100) and the declared unions of P2R-230, `array` without a list element type, a shape or the key and value types of a map (P2R-240)
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
  `as f64`. PHP converts the int only where the written PHP type says
  `float`, so these stop the conversion: a declared type whose Mago type
  holds `float` while its written type does not (`@param float` on a
  `mixed` parameter), or holds `float` and not `int` while its written type
  holds `int` (`@param float` on `int|float`), because PHP keeps the int
  there.
- **P2R-26** Assigning a `string` or list value to a variable, appending it to
  a list, or returning it MUST behave as a copy, as in PHP: a later change to
  one variable MUST NOT show in the other. The output copies (`.clone()`, or
  `.to_string()` and `.to_vec()` from a parameter) unless the source is not
  read again and is not read earlier in the same statement, where Rust may
  still hold that read's borrow (`pair(&s.b, &show(s))`).

## 4. Names and structure

- **P2R-30** Functions and variables MUST become snake_case as heck's
  `to_snake_case` gives it (`getTotal` to `get_total`, `getHTTPResponse` to
  `get_http_response`). A name that is a Rust 2024 strict or reserved
  keyword (`gen` included) MUST become a raw identifier (`r#type`). A name
  whose Rust form is `self`, `Self`, `super`, `crate`, `_` or empty MUST stop
  the conversion. A name whose Rust form is not an ASCII identifier (a
  letter or `_` first, then only ASCII letters, digits and `_`) MUST stop the
  conversion (`$_2` gives `2`; non-ASCII names trip rustc's confusable-name
  lints). The parameters of a `when` callback are the exception: they are
  no variables of the function, and P2R-452 names the binding.
- **P2R-31** These MUST stop the conversion: two PHP functions in the file,
  or two PHP names in one function, that give the same Rust name
  (`getTotal()` and `get_total()`); a variable whose Rust name equals the
  Rust name of a function in the file; a PHP function named `main`; a
  function, variable, parameter or foreach key or value whose Rust name is
  the name of a helper of P2R-160; and a
  function with the name of a function PHP has built in (`next`, `touch`),
  which PHP rejects when it compiles the file and Mago does not report (in
  a namespace PHP accepts it, P2R-151); and a parameter named like a
  superglobal (`$_GET`, `$GLOBALS` and the others) and a parameter of a
  function with a modifier (`public`, `readonly`, `static`), which PHP
  rejects and Mago does not report either. Of these clauses only the
  superglobal one applies to the parameters of a `when` callback
  (P2R-451); P2R-452 names the binding.
- **P2R-32** A call MUST use the declared or built-in function name,
  whatever the letter case of the call (PHP function names ignore case).
- **P2R-33** Statements outside functions, other than enum, class and
  interface declarations (sections 12, 14 and 38), MUST become `fn main()`. Functions
  declared at the top level MUST become `pub fn`, callable before their
  declaration, as in PHP. A file with no statements outside functions other
  than the P2R-12 `declare`, enum, class and interface declarations MUST
  convert to a library file with no `main`. After the P2R-52 line the output
  holds the structs of array shapes (P2R-200), then the enums, classes,
  traits and functions in source order, then the exception struct of P2R-210, then the
  helpers of P2R-160, then `fn main()`.
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
  append, `++` or `--`, a call to a function declared in the file, a
  method call (P2R-135) or a `throw` (P2R-212); any other expression
  statement stops the conversion.
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
  functions of sections 17, 19, 23, 25, 26, 27, 29, 30, 31, 32 and 34. A call to any other function not
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
  otherwise, with `--extern indexmap=..` for output that names `IndexMap`
  (P2R-243). When the tool cannot produce such code for an input, it MUST
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
  - Reading a list at a negative index or past its end, or a map at a key it
    does not have (P2R-242): PHP warns and gives `null`. Rust panics.
  - `E::from()` with a value that no case of the enum `E` has: PHP throws a
    `ValueError` (exit code 255 when uncaught), Rust panics (exit code 101).
  - An `int` property that overflows in `+=`, `-=`, `*=`, `++` or `--`: PHP
    throws a `TypeError` (exit code 255 when uncaught), because a typed
    property cannot hold the float; Rust panics in a debug build, as for a
    variable.
  - Reading a string offset past either end of the string: PHP warns and
    gives `""`. Rust panics.
  - `substr()`, a string offset or a `preg_replace` of P2R-261 that cuts a
    character of more than one byte: PHP gives the raw bytes, Rust gives
    U+FFFD (three bytes) in their place, so later lengths, comparisons and
    joins of that value differ too. The README lists these; no case folder
    can show them, because PHP warns or prints bytes that are not UTF-8.
  - A `preg_replace` or `preg_match` of section 27 or 33, or a
    `preg_replace_callback` of P2R-302, past PCRE's backtrack,
    recursion or JIT stack limit (`/(?:a|b)*\z/` on 400000 bytes): PHP
    gives null or `false`, so the default of `??` applies, a condition,
    `=== 1` and `=== 0` are false, and `!== 0`, `!== 1` and `!` are true;
    the `regex` crate has no such limit and gives the match.
  - A `preg_match` of P2R-320 where a letter of only one Unicode version
    stands right before the word: Rust's `\p{L}` follows Unicode 16.0
    with `regex-syntax` 0.8.11, which the tests build, and PHP's follows
    the PCRE2 it links (17.0 for PCRE2 10.48, 14.0 for 10.42), so that
    occurrence can count in one and not in the other (`"\u{10940}mc"`
    gives no match in PHP with PCRE2 10.48 and a match in Rust).
  - A `preg_match` of P2R-320 on a subject that is not valid UTF-8 in PHP
    (after a cut of a bullet above): PHP's `u` gives `false`, so a
    condition, `=== 1` and `=== 0` are false, and `!== 0`, `!== 1` and
    `!` are true; the Rust subject holds U+FFFD there, a character that
    is not a letter, and matches as any other.
  - `DateTime::createFromFormat()` of P2R-331 where no letter, `!` or `|`
    sets a field: PHP fills it from the current date and time in its
    default time zone (`date.timezone`, UTC when not set), Rust from the
    current UTC date and time.
  - `DateTime::createFromFormat()` of P2R-331 under a `date.timezone`
    other than UTC, whatever the format reads: PHP moves a local time
    that falls in a gap of that zone forward (`2026-03-29 02:30:00` in
    `Europe/Amsterdam` gives 03:30), Rust keeps it.
  - `DateTime::createFromFormat()` of P2R-331 on a text that holds a NUL
    byte: PHP throws a `ValueError`, Rust panics (exit code 101).
  - A format of P2R-331 read at run time with an ASCII letter that the
    helper does not read, or with `*` or `+`, and a format of P2R-332
    read at run time with such a letter: PHP reads it, Rust panics (exit
    code 101).
  - `date_parse()` of P2R-335 on a date whose year is above 9999, which
    the carries of P2R-331 can give (`9999-12-31 25:00:00`): PHP misreads
    the text as the year less 8000 with the hour 10, the minute and the
    second 0 (year 2000 there), Rust gives the date's own fields.
  - A shape that P2R-271 turns into a map, where a PHP caller outside the
    input built the array in another key order than the input's literals:
    the Rust map has the input's order, because a struct has none.
  - `(int)` of a string of 19 digits above `PHP_INT_MAX` followed by `e+`
    or `e-` without exponent digits (`"9300000000000000000e+"`): PHP's
    number parser reads one byte late and gives a wrapped value
    (-9146744073709551616); `php_int` gives the largest or smallest int.
  - `str_repeat()` with a count that is negative at run time: PHP throws a
    `ValueError` (exit code 255 when uncaught); Rust panics, or gives an
    empty string when the string is empty.
  - `explode()` with an empty separator that is not a literal: PHP throws a
    `ValueError`, Rust panics (exit code 101).
  - `str_replace()` of P2R-310 where the search is a string at run time
    (a string, or a union that holds one) and the replacement is a list or
    map at run time (a list, a map, a list literal, or a union that holds
    one): PHP throws a `TypeError`, Rust panics (exit code 101).
  - `strtr()` of P2R-311 with a key that is empty at run time and a text
    that is not empty: PHP warns and ignores the key, Rust ignores it
    without the warning.
  - `trim()`, `ltrim()` or `rtrim()` with a second argument whose value
    the tool does not know (P2R-291): where it holds a `..` that starts no range, PHP
    warns and Rust does not, and both remove the same bytes; where its mask
    holds a byte above `7F`, a cut character gives U+FFFD in Rust, as for
    `substr()`.
  - An unqualified function call inside a namespace, where that namespace
    has a function of the same short name in code the tool does not read
    (code given neither as input nor by `--include`, a symbolic link below
    an include path, or an included file that P2R-340 skips): PHP calls
    that function, Rust calls the built-in or the input's global function
    of that name. A link's target can be given as its own `--include`.
  - The errors above that Rust turns into panics are not exceptions of
    P2R-210, so a `catch (Throwable $e)` (P2R-214) does not catch them.
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
  stderr MUST be empty; a case that ends in an uncaught exception follows
  P2R-216 instead. A case without `fn main()` gets the checks of P2R-70
  and P2R-71 only. Review concern, not checked mechanically: each case's
  stdout depends on every construct the case covers, and every construct of
  sections 3 to 5, 11 to 17, 19 and 21 to 23 appears in at least one case with
  `fn main()`.
- **P2R-73** Each stop folder `tests/stops/<id>-<slug>/` holds `input.php`
  and `expected.txt`. Run from inside the folder, once as
  `php2rust input.php` and once as `php2rust input.php -o out.rs`, the tool
  MUST exit 1 both times, its stderr MUST equal `expected.txt`, its stdout
  MUST be empty, and `out.rs` MUST NOT exist afterwards. A case, stop or
  error folder that holds a directory `include/` runs the tool with the
  added option `--include` naming that directory (P2R-340); the PHP run
  of P2R-72 does not load it, other than through `prepend.php` below, so
  an included file that changes what the input does belongs in a stop
  folder. A case folder that holds
  `prepend.php` runs PHP from inside the folder with that file loaded
  before `input.php` (`-d auto_prepend_file=prepend.php`); it loads the
  files of `include/` that declare a package's classes the input names
  (P2R-350), so PHP and the tool see the same declarations, and the tool
  never reads it. A case or stop folder that holds a directory
  `migrations/` runs the tool with the added option `--migrations`
  naming that directory (P2R-400).
- **P2R-74** Each error folder `tests/errors/<slug>/` holds `args.txt` (the
  command-line arguments, one per line), an optional `input.php` and
  `expected.txt`. Run from inside the folder, the tool MUST exit 2, its
  stderr MUST equal `expected.txt`, its stdout MUST be empty, no output file
  may be created, and every file in the folder MUST be unchanged.
- **P2R-75** Every requirement ID from P2R-07 to P2R-49 and from P2R-100 to
  P2R-472 MUST appear in the name of at least one case or stop folder. A
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
  prelude with `LATEST`. Version 1 pins Mago 1.57.0.
- **P2R-82** The README MUST state what the tool does, how to install it
  (`cargo install --locked`), that it needs `rustfmt` on `PATH` (P2R-190),
  which rustup's minimal profile leaves out, how to run it, the supported
  PHP, the refusal rules in short, the known differences (P2R-60), the
  `indexmap` dependency of output with a map (P2R-243), the options
  `--include` (P2R-340) and `--migrations` (P2R-400) and the command of
  P2R-76.
- **P2R-83** The Rust toolchain MUST be pinned in `rust-toolchain.toml`. The
  `rust-version` in `Cargo.toml` MUST equal the `rust-version` of the pinned
  Mago crates (1.99.0 for Mago 1.57.0), and the pinned toolchain MUST be at
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
  (section 21); exceptions as `Result` (section 22); `ArrayAccess` classes
  as plain methods (section 23, not Rust's `Index`, which cannot throw);
  declared unions as generated enums and `?->` (section 24); maps on
  `indexmap` (section 25); unions as map values, narrowing a union value
  with `is_string()` and its kin (section 26); `preg_replace` and
  `preg_match` without the `u` flag (section 27); maps as union members
  and shapes as maps (section 28); traits with constants, string map
  constants and `strtr` (section 29); writes after a null check, `trim`
  with a character list and `checkdate` (section 30); closures as
  callbacks of `array_map` and `preg_replace_callback`, and `ucwords`
  (section 31); arrays in `str_replace` and `strtr` (section 32); the
  two look-around patterns with `u` that `nameFix` reads as a bool,
  rewritten for `regex` (section 33); `DateTime::createFromFormat()`
  with a format read at run time, `format()`, `instanceof`, and
  `date_parse` of its text, the last lines of `StringManipulation` that
  `SouthAfricanIDValidator` calls (section 34); then, for a Laravel
  application, the classes of Composer's `vendor/` folder read for their
  types only (section 35), Spatie's `Data` classes as plain structs
  (section 36), their `Optional` properties (section 37), their request
  hooks left out (section 39) and later their `from()` and `toArray()`,
  and the framework's other forms; the input's
  own interfaces as traits (section 38); the input's own exception
  classes (section 40); Eloquent models as structs of the columns the
  migrations declare (section 41), then, on 16 Oct 2026 decided under the
  same direction, their `create`, `find()` and `where(..)->first()` on a
  store that the output holds in memory, checked against a PHP stand-in
  of the same behaviour (section 42), then `sole()`, `firstOrFail()` and
  `findOrFail()` with Laravel's exceptions (section 43), with a database
  behind it later;
  unions as shape fields (section 44); Laravel's collections as lists,
  read in order, with `get()` and `all()` on the store (section 45);
  conditional filters with `when()` on store queries (section 46);
  models with observers, read but not created (section 47);
  relation methods on models, left out of the output (section 48);
  unions as list values; values
  of interface types, interface inheritance and class inheritance;
  closures.
  `SouthAfricanIDValidator` calls `StringManipulation::isValidDate`, which
  lives in another package, and P2R-153 resolves calls inside the input
  only, so the validator converts completely only in one run of a
  directory that holds both packages' sources, and only once every line
  of `StringManipulation` converts too. Real `mixed` (facades, arrays of mixed values)
  still needs a dynamic value type, which no section specifies yet.
  Three points are open:
  - PHP functions with no counterpart in Rust's standard library
    (`preg_replace`, `DateTime::createFromFormat`): a short one becomes a
    helper written into the output, as `php_int` is (section 17); one that
    needs an engine, such as `preg_replace`, takes a crate, as maps take
    `indexmap`. Section 34 decides the formats that the
    `DateTime::createFromFormat` helper covers.
  - Output that needs a crate is built with plain `rustc` and
    `--extern`, which P2R-243 decides for `indexmap`; the tool writes no
    `Cargo.toml`, and the README names the dependency.
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
  (after a check against `null` or an assignment). Only these two forms,
  the `?:` of P2R-205, and the forms of P2R-333 and P2R-334 give a
  binding of type `T`. The binding has the Rust name of `$v` and is a new
  variable, as a `foreach` value is (P2R-41).
  - `$v !== null` (or `null !== $v`), where `$v` is a variable or parameter,
    as the condition of an `if` or `elseif` or as an operand of the
    top-level `&&` chain of that condition, becomes `let Some(v) = ...` in an
    `if let` or `else if let` (a let chain when the condition has other
    operands). The operands after it and the body read the binding.
  - `if ($v === null)` (or `null === $v`), where `$v` is a variable or
    parameter, with no `elseif` or `else` and a body that always leaves by
    `return`, `break`, `continue` or `throw` (P2R-212), becomes
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
  binding can shadow the variable or not exist yet. P2R-290 lets a write
  after a top-level `let ... else` convert.
- **P2R-105** `$a ?? $b` MUST convert when `$a` is a variable, parameter,
  function call, static method call (P2R-153), `E::tryFrom()` call
  (P2R-114), `?->` (P2R-234) or array shape element (P2R-203) of Rust type `Option<T>` and
  `$b` has the type `T`. `$b` MUST
  be evaluated only when `$a` is null, as in PHP, and P2R-26 applies to
  `$a`. These stop the conversion: `??` whose left side has a Rust type that
  is not `Option` or is a list element (PHP checks it with `isset`), any
  other type of `$b` (`$r ?? []` included) other than a `throw`
  (P2R-215), and `??=`.
- **P2R-106** A value of Rust type `Option<T>` MUST stop the conversion in
  these places, where PHP gives `null` a meaning Rust does not copy or Rust
  needs a `T`: an operand of `.`, `.=`, interpolation or `echo` (PHP prints
  `null` as an empty string); an operand of `+`, `-`, `*`, `%`, unary minus,
  `+=`, `-=`, `*=`, `++` or `--`; a condition or an operand of `&&`, `||` or
  `!` (P2R-42); the list of a `foreach`; the list or the index of a list
  read or an append; an element of a list literal, or an appended value,
  whose list holds values that cannot be null; an argument of a built-in
  function (P2R-48) other than the tests on an `Option<U>` of P2R-233 and
  P2R-251, of `E::from()` or of `E::tryFrom()`; the enum of
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
    P2R-12 `declare`, a function, enum, class, interface (P2R-370) or
    trait (P2R-280) declaration: PHP
    declares an enum only when execution reaches it, so an earlier use dies
    with `Class "E" not found`;
  - an enum with `implements` other than of the interfaces of P2R-371, or
    a `use` other than one of a trait of
    constants (P2R-280), or a backing value of any other
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
  enum named `Option`, `String`, `Vec`, `ToOwned`, `None`, `Some`,
  `IndexMap` (P2R-240) or `From`, whose `impl From<..> for IndexMap<..>`
  a shape writes (P2R-271), which the output uses;
  and an enum with a name PHP reserves, a keyword or a type name (`list`,
  `readonly`, `match`, `bool`, `self` and the others; `enum` is allowed), or
  the name of a class, interface, enum or trait PHP has built in
  (`Exception`, `Countable`), which PHP rejects, compared by the fully
  qualified name (P2R-151); the other name checks here compare the short
  name, in any namespace. Mago's parser accepts a
  keyword there, so the tool checks the name itself. The built-in names
  are those of Mago 1.57's stubs, which lack some class-likes PHP 8.5 has
  (`ClosedGeneratorException`, `Dom\DOMException`, `Pdo\Odbc` and others,
  some only with an extension loaded); a name of those converts, where PHP
  stops with "Cannot redeclare". That is a known limit.
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
  converted by P2R-49) or a case of the subject's enum. An arm result that
  is a `throw` converts by P2R-215. These stop the conversion:
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
  statement runs, so it may come after the statements outside functions;
  a class that implements an interface (P2R-220, P2R-371) or uses a trait
  (P2R-280) PHP declares only when execution reaches it, as it does an
  enum (P2R-110), so such a class after a statement outside functions
  stops.
  These stop the conversion:
  - `extends` (other than the `Spatie\LaravelData\Data` of P2R-350; an
    exception class of P2R-390 follows section 40 and a model of P2R-401
    follows section 41 instead),
    `implements` (other than the `ArrayAccess` of P2R-220 and the
    interfaces of P2R-371),
    `abstract`, a trait other than one of constants (P2R-280), a property that is not
    promoted, a static property, a constructor parameter that is not
    promoted, a constructor with a body, a constructor with a return type
    or `static`, a modifier written twice or a second visibility on
    a class, method, promoted parameter or declared property (`final
    final`, `public public`), and a set visibility wider than the
    visibility of its promoted parameter or declared property (`private
    public(set)`; no visibility reads as `public`), which PHP rejects
    and Mago does not report, and a parameter default value other than those of P2R-168
    (section 16 converts constants and static methods);
  - a promoted parameter without a type, with a property hook, by reference
    or variadic, with `static` or `abstract`, or named `$this` or like a superglobal
    (P2R-31), which PHP rejects or makes fail and Mago does not report;
  - a method whose PHP name starts with `__`, other than the constructor,
    and an attribute on the class or a member (P2R-07), other than the
    `#[Override]` of P2R-221 and the attributes of P2R-351 and P2R-381;
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
  one outside the constructor, P2R-11), `?->` other than P2R-234, a property name that is not a
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
  the object. These stop the conversion: `parent::`, other than the call
  that ends the constructor of an exception class (P2R-391), and the native type
  `parent` (section 16 converts static methods, `self::`, `static::` and
  the types `self` and `static`; a docblock type that names `self`, such as
  `list<self>`, is the class to Mago and converts); `$this` outside a
  method, a write to `$this` and a parameter named `$this`, which PHP
  rejects and Mago does not report; a by-reference parameter or return
  (`function &offsetGet`); a `final
  private` method other than the constructor, for which PHP prints a
  warning and Mago reports nothing; `readonly` or a set visibility on a
  method, and `abstract` or `final` on the parameter of a method other
  than the constructor, which PHP rejects and Mago does not report; two methods whose PHP names are equal
  ignoring letter case, which PHP rejects and Mago does not report; and a
  call to a method the class does not declare itself (Mago reports it,
  P2R-11, other than one the parent of P2R-350 declares).
- **P2R-136** These MUST stop the conversion: `===`, `!==`, `==`, `!=`, `<`,
  `>`, `<=`, `>=` and `<=>` with an object operand, also of a nullable class
  type (P2R-204), because Rust has no object identity and PHP compares
  properties loosely, other than `===` and `!==` with the literal `null`
  (P2R-103); `echo` and string
  joining of an object; `instanceof`, other than the form of P2R-333,
  which names it; `clone`, also with new property
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
  Mago does not report it), or with `final` and `private` together, which
  PHP rejects on a declared property (it accepts them on a promoted one);
  `var`; and the stops of P2R-130 other than "a
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
  because no class of the file can extend another class of the file
  (P2R-130, P2R-350; an exception class of P2R-390 has no members to
  call), and both are
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
  `parent::`, other than the call that ends the constructor of an
  exception class (P2R-391); a class given by a variable or an expression (`$c::m()`); a
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
  any other form (an expression, another constant, a list or an array
  other than the string map of P2R-281, heredoc, and a
  case written `static::Small`, which PHP rejects in a constant's value and
  Mago does not report); a
  declared type other than `int`, `float`, `bool`, `string`, an enum of
  the file, or the `array` of P2R-281; two constants in one `const` statement; two constants of one
  class or enum with the same Rust name (`X` and `x`); a constant whose
  Rust name equals the Rust name of a case of its enum (`case X` and
  `const x`), or whose PHP name is the name of a case (PHP rejects it, and
  Mago does not report it); a constant named `class` in any letter case, a
  constant that is `final` and `private`, and a constant with modifiers
  other than one visibility and one `final` (`static`, `readonly`,
  `abstract`, two visibilities, `final final`), which PHP rejects and Mago
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
  and the exception struct of P2R-210 and before `fn main()`, one blank
  line before each.
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
  gets parentheses (`(-5_i64).to_string()`, `(-x).to_string()`).
  `(string) $s` with a string operand MUST convert to the operand itself,
  copied where P2R-26 copies (section 23). `(int)`
  and `(string)` convert in any letter case and with spaces inside the
  parentheses. Every other cast still stops the conversion (P2R-07): of a
  float (PHP and Rust differ past the int range), of a bool, null, array
  or object; `(int)` of an int; `(bool)`,
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
  (`?T` or `T|null`, also `T|Optional|null` of P2R-360, where it is
  `PhpOptional::Present(None)`), `[]` for a list or nullable list type,
  or a case of an enum of the file written `E::Case` or `self::Case`. A
  call that leaves
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
  1 to 17, 19, 21 to 23 and 27 means every input file together (P2R-31, P2R-43, P2R-48,
  P2R-111, P2R-115, P2R-120, P2R-131, P2R-151, P2R-153, P2R-156, P2R-160,
  P2R-163, P2R-168, P2R-169 and others), except in P2R-12, P2R-13, P2R-110
  and P2R-150, which apply to each file alone (a `use` in one file may name
  a class that another file declares). An unqualified call in a
  namespace, whose name no `use function` import maps and whose namespaced
  name no function of the crate has, MUST call the crate's function of that
  name in the global namespace, as PHP falls back to it (the helpers file
  of a framework).
- **P2R-172** The crate MUST hold one Rust module for each PHP namespace
  that declares an enum, class, interface (P2R-370) or function, and for
  each namespace between the root and such a namespace. In this section
  namespace names compare ignoring ASCII letter case, as PHP compares
  them. The crate root is the file `lib.rs`, for the root namespace: the
  longest namespace that the namespace of every input file that declares
  an enum, class, interface or function starts with, compared segment by segment (the global namespace when they
  share none). A namespace below the root becomes the module path of its
  segments below the root, each segment its snake_case name (P2R-30, a Rust
  keyword as a raw identifier: `r#enum`). The module `a::b` is in the file
  `a/b.rs`, and `a` in `a.rs` beside `lib.rs`, the layout `rustc` follows
  from `pub mod a;` in `lib.rs` and `pub mod b;` in `a.rs`; a module that
  holds no item still has its file, for its `pub mod` lines. The file name
  leaves out `r#` (`enum.rs`). Each module file holds the P2R-52 line,
  then the `pub mod` lines of its child modules, then its `use` lines
  (P2R-174), then its items: in `lib.rs` first the structs of the array
  shapes the crate uses (P2R-200) and then the exception struct
  (P2R-210), then the enums, classes, traits (P2R-370) and functions of
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
  `usize::from`; a module named `indexmap` at any depth, which would take
  over the crate of P2R-240 in its parent module; two namespaces of different names whose module
  paths are equal (`Acme\FooBar` and `Acme\Foo_Bar`, also as a module
  between the root and another: `Acme\FooBar\Deep`); and two input files
  whose namespaces hold, under equal leading segments, a segment that is
  equal ignoring letter case but not byte for byte (`Acme\Shop` and
  `ACME\Shop\Sub`, or `App\FooBar` and `App\Foobar`, which PHP reads as one
  namespace), at the name in the namespace statement of the later file in
  path order.
- **P2R-173** Each enum, class, interface (P2R-370) and function of the
  crate MUST have a Rust name that no other of them has, in any module,
  because a module names the items of other modules by their Rust name
  alone (P2R-174). Two declarations whose Rust names are equal, and two
  enums, classes, interfaces or PHP traits (P2R-280; a PHP trait has no
  Rust name), or two functions, whose fully qualified names are
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
  an item of the crate root), and no other `use` lines than these and
  `use indexmap::IndexMap;` (P2R-240). The Rust lines
  count, not the PHP source: a default filled in at a call (P2R-168) names
  its enum, and a `default` arm left out (P2R-121) names nothing. An enum
  case, method, field or constant whose Rust name equals an item's is not
  a use of it, and a name of the module's own items needs no line. A module
  other than the crate root whose Rust lines name the struct of an array
  shape (P2R-200), as a type or as a struct literal (`S { .. }`), a union
  enum (P2R-230), the exception struct of P2R-210, or the `PhpDateTime`
  struct of P2R-330, also where only a helper of the module names it,
  MUST hold `use crate::<Name>;` for it. A module that does not declare
  the trait of an interface (P2R-370) and whose Rust lines name it in an
  `impl Name for C` line, or call a method of such an `impl` block, MUST
  hold one `use` line for that trait (P2R-372). rustfmt sorts the `use`
  lines (P2R-190).
- **P2R-175** A directory input converts to a library: a statement outside
  functions other than the P2R-12 `declare`, the P2R-150 `namespace` and
  `use` statements, and enum, class, trait (P2R-280) and interface
  (P2R-370) declarations stops the conversion,
  because a library has no `fn main()` (this replaces the `fn main()` rule
  and the output order of P2R-33 for a directory input). The decisions that
  P2R-115, P2R-141, P2R-143, P2R-146 and P2R-211 make for the whole file,
  which helpers an enum holds, which classes change their objects, the
  form of each object parameter and which functions throw, MUST be made
  for the whole crate.
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
  element (P2R-203) on a variable, parameter or `$this`, as `v.as_str()`;
  a `(string)` cast of a string operand of one of these forms, lent as its
  operand (P2R-166); and `gettype()`, a literal (P2R-222)), so the `if`
  lends a
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
  (P2R-41). A write to the owned binding of a null check (P2R-290) does not
  change the parameter. These still stop: a foreach key or value with the name of a
  parameter that no assignment, `.=` or append changes (P2R-24), a parameter
  of a type that changes its objects (P2R-143), and a promoted constructor
  parameter.
- **P2R-182** `trim($s)`, `ltrim($s)` and `rtrim($s)` with one `string`
  argument MUST convert to `php_trim(s)`, `php_ltrim(s)` and `php_rtrim(s)`,
  of type `String`: helpers (P2R-160) that remove the bytes PHP removes by
  default, space, `\t`, `\n`, `\r`, `\0` and `\x0B`, where Rust's `trim`
  also removes `\x0C` and Unicode spaces and keeps `\0`. A second argument
  converts by P2R-291.
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
  between every character. An array argument converts by P2R-310, and a
  fourth argument stops the conversion.
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
  `php_lcfirst`, `php_replace`, `php_explode`, `php_preg_replace`,
  `php_preg_match` (P2R-261, P2R-262), `php_strtr` (P2R-282), `php_charmask`,
  `php_trim_chars`, `php_ltrim_chars`, `php_rtrim_chars` (P2R-291),
  `php_checkdate` (P2R-292), `php_ucwords` (P2R-303),
  `php_preg_replace_callback` (P2R-302), `php_str_replace` (P2R-310),
  `php_date_create_from_format` (P2R-331) and `php_date_format`
  (P2R-332), each written only when the
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
  struct for each shape a type of the input maps to that the output names,
  the shape of a field included (Mago's shape type of a map variable of
  P2R-240 names a shape no item uses), in byte order of their names: after the P2R-52 line and before
  the enums, classes and functions (P2R-33), or for a directory input in
  `lib.rs` after its `use` lines (P2R-172). The empty array `array{}` keeps
  P2R-21. These stop the conversion, with a problem at a place whose type
  maps to the shape: a shape with an integer key (PHP casts the key `'1'`
  to an int), a key that names a class constant, an empty key or an
  optional key; a general key and value type beside the keys makes the
  type a map (P2R-240); a key whose snake_case name has no Rust form
  (P2R-30); two keys of one shape that give one field name; a value type
  that does not convert (P2R-22); a value type that is or holds a class (a
  later slice); a value of an enum without cases, because no value of the
  shape could exist and rustc rejects the code after it (P2R-130); two
  shapes of one Rust name with different keys or different field types
  (`array{a_b: int}` and `array{a: int, b: int}` both give `ABShape`), at
  the places of the shape the tool meets second, other than a shape whose
  fields are those of the first with a `?T` field typed `T` or `null`,
  which Mago gives a shape after a check of that field against null and
  which maps to the struct of the first (P2R-104), a type Mago gives a
  read and never a second declared shape, and a shape that maps
  to the struct of a shape with a union field (P2R-430); and a shape whose Rust
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
  function (`count`, `array_keys`) other than `is_scalar()` and `gettype()`
  (P2R-222); a write to an element (`$s['k'] = v`, a
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
  type (`list<?C>`); `?->` other than P2R-234 (P2R-134); a property read, a method call or a
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

## 22. Exceptions as Result (milestone 2, slice 12)

Application code throws PHP's built-in exceptions and catches them around
calls; the 11 Oct decision makes a `throw` an `Err` of a Rust `Result`.
Section 40 adds exception classes that the input declares.

- **P2R-210** The output MUST hold one struct for the exceptions when a
  `throw` or `catch` converts, a form of P2R-421 converts or P2R-391
  writes a function, and no struct otherwise:
  `#[derive(Clone, Debug)] pub struct PhpException { pub class: &'static
  str, pub message: String, pub code: i64 }`, with `pub fn new(class:
  &'static str, message: &str, code: i64) -> PhpException`, which stores a
  copy of the message (P2R-133); `pub fn is_a(&self, class: &str) -> bool`,
  true when the exception's class is that class or extends it in PHP's
  class tree; and `pub fn uncaught(self) -> !`, which flushes stdout,
  writes `PHP Fatal error:  Uncaught <class>: <message>` to stderr and
  exits with code 255, the exit code PHP gives an uncaught exception (PHP's
  own message also names the file and a stack trace); its paths start
  with `::std`, so a module `std` that a namespace `Std` gives the crate
  does not hide them. The classes are
  PHP's `Exception`, `ErrorException`, `JsonException`, `LogicException`
  with `BadFunctionCallException` (and `BadMethodCallException` under it),
  `DomainException`, `InvalidArgumentException`, `LengthException` and
  `OutOfRangeException`, and `RuntimeException` with
  `OutOfBoundsException`, `OverflowException`, `RangeException`,
  `UnderflowException` and `UnexpectedValueException`. A class name is resolved as PHP resolves it
  (P2R-151), so an unimported `RuntimeException` inside a namespace, which
  names a class of that namespace, stops. The struct comes after the
  enums, classes and functions and before the helpers (P2R-33, P2R-160),
  or for a directory input in `lib.rs` (P2R-172). An enum or class whose
  Rust name is `PhpException`, `Result`, `Ok` or `Err` stops the
  conversion, as other names the output uses do (P2R-111, P2R-131),
  because a struct of that name hides the type the output writes.
- **P2R-211** A function, method, static method or enum method MUST throw
  when its body holds a `throw`, or a call of a function or method that
  throws, outside the body of a `try` with a catch-all clause (P2R-214); a
  `throw` or call in a `catch` body is outside its `try`, and one in a
  `default` arm that P2R-121 leaves out does not count, because that arm
  never runs. The tool decides
  this for the whole input as a least fixed point, as it decides the forms
  of P2R-143. A function that throws returns `Result<T, PhpException>`, or
  `Result<(), PhpException>` for `void`: `return $v;` gives `return
  Ok(v);`, `return;` gives `return Ok(());`, and a `void` body whose end a
  path reaches ends with `Ok(())`. P2R-25, P2R-26 and P2R-102 apply inside
  `Ok(..)`.
- **P2R-212** `throw new C(...)`, where `C` is a class of P2R-210, MUST
  convert to `return Err(PhpException::new("C", message, code));` in a
  function that throws, where `message` is the first argument, a `string`
  lent as for a `&str` parameter (`""` when left out), and `code` the
  second, an `int` (`0` when left out); named arguments `message:` and
  `code:` convert by P2R-169. In `fn main()` it converts to
  `PhpException::new(..).uncaught();`, and inside a `try` body to
  `break 'throw PhpException::new(..);` (P2R-214). `throw $e;`, where `$e`
  is the variable of the `catch` it stands in, converts the same way with
  `e`. A `throw` statement converts where P2R-43 lists the expression
  statements. For every rule that follows control flow (P2R-26, P2R-34,
  P2R-35, P2R-40, P2R-50, P2R-104, P2R-106, P2R-145 and the end rule of
  P2R-211), a `throw` leaves its block as `return` does, and each `throw`
  and each call that throws in a `try` body is a path to its handlers
  (P2R-214). These stop the conversion: for a class of P2R-210, or an
  exception class of the input without a constructor, any argument other
  than `message` and `code`, positional or named; `new` of any other
  class than these
  and the exception classes of the input (P2R-392); a `throw` of
  any other expression; and an exception object used in any other way
  (`new C(..)` outside a `throw` stops by P2R-133).
- **P2R-213** A call of a function or method that throws MUST convert with
  `?` after it (`f(x)?`) in a function that throws, to
  `f(x).unwrap_or_else(|e| e.uncaught())` in `fn main()` outside a `try`
  body, and inside a `try` body to `match f(x) { Ok(v) => v, Err(e) =>
  break 'throw e }`, or `if let Err(e) = f(x) { break 'throw e; }` for a
  call statement of a `void` function. A call that throws
  inside a closure that the output writes stops the conversion: the lazy
  default of `??` (P2R-105) and the message of `?? throw` (P2R-215).
- **P2R-214** `try { body } catch (C $e) { handler } ...` MUST convert to
  `'attempt: { let e = 'throw: { body; break 'attempt; }; handlers }` when a
  path reaches the end of the body, and to `{ let e = 'throw: { body };
  handlers }` when none does, because rustc rejects a label that nothing
  names; the block ends the binding with the handlers. Each `throw` and each call that throws in the body
  leaves `'throw` with its exception (P2R-212, P2R-213), so the handlers
  run only after a throw and the end of the body skips them; rustc follows
  these paths as PHP does, so a variable that the body and a handler both
  assign is assigned once on each path. The handlers are one
  `if e.is_a("C") { .. } else if ..` chain in source order, a union
  `catch (A | B $e)` testing `e.is_a("A") || e.is_a("B")`. A clause whose
  class PHP resolves to `\Exception` or `\Throwable`, alone or in a
  union, is a catch-all: it
  ends the chain as `else { .. }`, or stands alone after the `let` when it
  is the only clause. Without a catch-all the chain ends with
  `else { return Err(e); }` in a function that throws, so it throws
  (P2R-211), or `else { e.uncaught(); }` in `fn main()`. The binding has
  the Rust name of the variable of the clauses; a clause without a variable
  takes the name of the others, and `_` is bound when there is no `is_a`
  test and no handler reads the variable. The variable of a `catch` is a
  new variable that holds only inside its handler, as a `foreach` value
  does (P2R-41): `$e->getMessage()` gives `e.message`, lent as a `String`
  field (P2R-134), and `$e->getCode()` gives `e.code`, of Rust type `i64`,
  where its value is printed, joined, computed with or compared; Mago types
  `getCode()` as `int|string`, so its assignment to a variable stops by
  P2R-22, and its return or its use as an `int` argument stops on Mago's
  finding (P2R-11). These stop the conversion: `finally`; a
  `try` inside another `try`, its catch bodies included, whose labels
  would shadow the outer ones (the problem is reported at the inner
  `try`); a `try` whose body holds no `throw` and no call that throws that
  converts (a `default` arm that P2R-121 leaves out does not count); a
  `break` or `continue` that leaves the body, and one that leaves a handler
  when a path reaches the end of the body, which Rust rejects inside a
  labelled block; a class in a `catch` that is
  neither in P2R-210, an exception class of the input (P2R-392) nor one
  of Laravel's classes of P2R-420
  (`Error`, `TypeError` and the input's other classes included); a clause after a catch-all, which PHP never runs; clauses of
  one `try` with different variable names; a clause that tests a class
  when no clause has a variable; a catch variable whose name the function
  already uses before the `try`, because PHP overwrites it; a catch
  variable named like a superglobal, because in a function a read of it
  gives PHP's superglobal, not the exception, and at the top level the
  catch overwrites the superglobal for every function; a catch variable
  named `$this`, which PHP rejects and Mago does not report; and a read of
  `$e` after its handler (by P2R-34), or any other use of `$e` than
  `getMessage()`, `getCode()` and `throw $e` (a built-in function such as
  `get_class($e)` stops by P2R-48).
- **P2R-215** A `throw` expression MUST convert in two more places: as the
  arm result of a `match` (P2R-120), where the arm result is
  `return Err(PhpException::new(..))` (or the forms of P2R-212 in
  `fn main()` and a `try` body), and as the right side of `$a ?? throw new
  C(..)` on an `$a` that P2R-105 accepts, where it gives
  `a.ok_or_else(|| PhpException::new(..))?` in a function that throws,
  with `$a` taken as `unwrap_or` takes it in P2R-105 (a parameter
  `Option<&T>` copied with `.map(ToOwned::to_owned)`, an array shape element
  copied with `.clone()`). Mago types a `throw` as `never`, so neither the arm type
  rules of P2R-122 nor the type rule of P2R-105 applies to it. These stop
  the conversion: a `match` whose every converted arm is a `throw`, because
  rustc finds the code around it unreachable (P2R-50); `?? throw` in
  `fn main()` or a `try` body; and a `throw` expression anywhere else (an
  operand, an argument, a `?:` arm).
- **P2R-216** A case folder whose PHP run exits with code 255 and writes
  only the fatal message of an uncaught exception to stderr (it holds
  `Uncaught `) MUST give a Rust run that exits with code 255 and the same
  stdout, and stdout written before the exception MUST reach the output;
  for such a case P2R-72's exit 0 and non-empty stdout are lifted, and
  stderr is not compared, because PHP's message names the input path and a
  stack trace. The README lists, under P2R-60, that a `catch (Throwable
  $e)` does not catch the errors that P2R-60 lists as panics.

## 23. ArrayAccess classes (milestone 2, slice 13)

A value object often implements `ArrayAccess` so old code can read it as
an array (`$result['valid']`). The interface's methods take `mixed`, and a
docblock `@implements ArrayAccess<K, V>` names the real key and value
types. This section converts such a class when K and V have Rust types.
Rust's `Index` trait is not used: `offsetGet` may throw (P2R-212), and
`Index` cannot return a `Result`. Element access on such an object and
inheritance are later slices; section 38 converts the input's own
interfaces.

- **P2R-220** A class, readonly or not, whose only interface is
  `ArrayAccess` (resolved as PHP resolves it, P2R-151) MUST convert as
  sections 14 and 15 say when Mago gives the template arguments K and V of
  its `@implements ArrayAccess<K, V>` docblock, and each maps to one Rust
  type (P2R-20) other than `f64` or `Option<f64>`. Its methods
  `offsetExists`, `offsetGet`, `offsetSet` and `offsetUnset`, which PHP
  requires, convert as other methods do, with these types where the
  declaration says `mixed`: the first parameter of each takes K, the second
  parameter of `offsetSet` takes V, and the return of `offsetGet` takes V,
  each in its parameter or owned form (`&str` and `String` for a string,
  `&[T]` and `Vec<T>` for a list, `&C` and `C` for a class or shape;
  P2R-24, P2R-132, P2R-201). A return type other than `mixed` keeps its
  own Rust type (P2R-20). A parameter of these four methods with a written
  type other than `mixed` stops: PHP rejects it when it loads the class,
  because a parameter may not narrow the interface's `mixed`, and Mago
  does not report it. A read of such a parameter in the body has its
  Rust type, also where Mago types it `mixed`, or `scalar` after
  `is_scalar()`, and P2R-24, P2R-34 and P2R-181 apply to it by that type.
  A float K or V stops, also a union with a `Float` variant (P2R-230),
  because a `mixed` slot keeps an int that P2R-25 would turn into a float. These also stop the conversion: `ArrayAccess`
  without template arguments, or with a K or V that has no Rust type
  (P2R-22); another interface beside it, and any other interface that
  P2R-371 does not convert (a later slice); a `mixed` type anywhere else; and `$o[k]` on an object of such a
  class, its write, `isset`, `unset` and `??` (a later slice; call
  `offsetGet()` instead).
- **P2R-221** The attribute `#[Override]` (any name PHP resolves to
  `\Override`), written once and with no arguments (`#[Override]` or
  `#[Override()]`), MUST be left out on a
  method of a class that P2R-220 converts when the method implements a
  method of `ArrayAccess`: PHP checks the attribute when it compiles the
  class, and the Rust output has nothing left to check. Every other
  attribute stops the conversion (P2R-07), `#[Override]` on any other
  method included, because no method that overrides a parent's converts
  (P2R-350 stops one with a name of `Data`'s) and PHP rejects an
  `#[Override]` on a method that overrides nothing.
- **P2R-222** `is_scalar($v)` MUST convert to `true` when `$v` has PHP type
  int, float, bool or string (Rust `i64`, `f64`, `bool`, `String` or
  `&str`), and to `false` for a list, an enum, a class or a shape. Mago
  reports each `false` case as an impossible type comparison, so in
  practice those stop by P2R-11 before this rule applies.
  `gettype($v)` MUST convert to the string literal PHP gives: `"integer"`,
  `"double"`, `"boolean"`, `"string"`, `"array"` for a list or a shape, and
  `"object"` for an enum or a class; it follows P2R-24 and P2R-26 as a
  literal does. `$v` MUST be a variable, a parameter or `$this`, whose type
  the tool knows; the call does not read it, because the result does not
  depend on its value, so a parameter read nowhere else gets the `_` of
  P2R-35, and a local read only there stops by P2R-35. A folded
  `is_scalar()` is the literal `true` or `false` for every rule, so
  `while (is_scalar($v))` becomes `loop` as `while (true)` does (P2R-40).
  These stop the conversion: any other argument (a call could have an
  effect), a value that may be null (P2R-106) other than a union value of
  P2R-233, and a second argument. A value of a union type converts by
  P2R-233.

## 24. Union types and the nullsafe operator (milestone 2, slice 14)

Real code declares a parameter, property or return value with a union of
types (`int|string $id`, `bool|array{...}|string|null`), and reads a
nullable object with `?->` (`$this->gender?->value`). Every member of such
a union is a type the tool already converts, so the union becomes a
generated Rust enum with one variant per member. A union that Mago only
infers inside a body (the type of `$c ? 1 : 'a'`) keeps P2R-22, a union
inside a list waits for a later slice, and section 44 converts a union as
a shape field.

- **P2R-230** A declared type MUST map to a generated enum, the union enum
  below, when its Mago type has two or more members, other than `null`,
  whose Rust types (P2R-20, P2R-21) differ. A declared type is the type of
  a function or method parameter, a return type, a promoted or declared
  property (P2R-140), and the K and V of P2R-220. Each member MUST be an
  int, float, bool or string, an enum, a readonly class (P2R-132), a shape
  (P2R-200), a map (P2R-270) or a list of a built type; `scalar` counts as
  `bool|float|int|string` and `array-key` as `int|string`. Members of one
  Rust type after P2R-21 (`1|2`, `true|string`) give one variant. Each
  variant is named `Int`, `Float`, `Bool`, `String`, `List`, or
  the Rust name of its enum, class or shape, and holds the member's
  owned Rust type. The Rust name of the enum is the variant names in byte
  order, joined with `Or`: `int|string` gives `IntOrString`, and
  `bool|array{year: string, month: string, day: string}|string` gives
  `BoolOrDayMonthYearShapeOrString`. A `null` member makes the type
  `Option<U>` (P2R-100), where `U` is the union enum. The output holds
  `#[derive(Clone, Debug)] pub enum U { .. }` with the variants in that
  order, `Bool(bool)` for example, once for each union enum a type of the
  input maps to, in byte order of their names, after the shape structs of
  P2R-200 and in the same place. These stop the conversion: a member with
  no Rust type (P2R-22), `mixed`, `numeric` and `object` included; two
  members of different Rust types that give one variant name
  (`list<int>|list<string>`); two union enums of one name with different
  variants; a member that is or holds a class that changes its objects
  (P2R-141); a member that is a list of a nullable struct type (P2R-204);
  a union enum whose name is the Rust name of an enum, class or shape of
  the input, whichever the tool meets first (P2R-131, P2R-200); a class that holds itself through
  a union enum other than through a list (P2R-130), because rustc rejects
  a type of infinite size; a union type as a list element type, a shape
  field type outside P2R-430 or the member of another union (a later
  slice; the value type of a declared map converts by P2R-250); a declared
  type whose written PHP type holds `float` and not `int` while its Mago
  type holds `int` (`@param int|string` on `float|string`), because PHP
  stores the int as a float; the stops of P2R-25; and a written type that
  PHP rejects and Mago does not report: a type written twice in any letter
  case (`int|string|INT`), where a class name counts by the name PHP
  resolves (`Card|\Card`, an alias) and `self` as the class that holds it;
  `true|false`; and `bool` beside `true` or `false`. P2R-21 keeps converting the widened forms of these, so the last
  stop applies to every declared type, not only to unions.
- **P2R-231** A value MUST convert into a place of type `U` or `Option<U>`
  (a return value, an argument for a parameter, of `new` too, a parameter
  default that a call leaves out (P2R-168), an assignment to a property, a
  property default (P2R-140), an assignment to a new variable, and an arm
  of a `match` or `?:` that stands in such a place) as follows. A value of a member type `T`, in the owned form
  P2R-26 gives it (`.to_string()`, `.clone()` or a move), becomes
  `U::V(value)`, with `V` the variant of `T`, inside `Some(..)` for
  `Option<U>` (P2R-102). A value of type `Option<T>`, in the owned form
  P2R-101 and P2R-204 give it (`.clone()` for a property or a variable
  read again, `.map(ToOwned::to_owned)` for an `Option<&str>`,
  `Option<&[T]>` or `Option<&S>` parameter, a move otherwise), becomes
  `value.map(U::V)`. A `U` value becomes `Some(..)` of its owned form where
  `Option<U>` is needed, and `null` becomes `None`. As an argument for a
  `&U` parameter a member value is lent as `&U::V(value)`, and for an
  `Option<&U>` parameter as `Some(&U::V(..))`, or `value.map(U::V).as_ref()`
  for an `Option<T>` value; a `U` or `Option<U>` value is lent by P2R-201
  and P2R-204 (`&u`, `u.as_ref()`), and a `&U` or `Option<&U>` parameter
  passes on as it is. A `?:` in such a place gives an owned value in each
  arm, wrapped as above, and the whole `if` is lent as P2R-205 lends a
  shape: `&(if ..)` for `&U` and `(if ..).as_ref()` for `Option<&U>`.
  These stop the conversion: an int value where `U` has a `Float` variant
  and no `Int` variant (P2R-25, P2R-230); an array literal where `U` or
  `Option<U>` is needed; and a value of another union type, or a union
  value where a type that is not its union is needed, also where every
  member fits, because Mago does not report it and PHP may convert a
  member there.
- **P2R-232** A value of type `U` MUST behave as a value of a shape type
  for copying and lending (P2R-201): a parameter becomes `&U` or
  `Option<&U>`, and P2R-26 copies with `.clone()`. A read of a variable,
  parameter or property of type `U` or `Option<U>` keeps that type also
  where Mago narrows it to one or more of its members (after
  `is_scalar()`, `is_string()` or a check against null) or types it
  `mixed` or `scalar` (P2R-220), so a use of such a read as a member type
  stops by the rules below; only the binding of P2R-252 reads a member
  value. `$v === null` and `$v !== null` on `Option<U>`
  convert by P2R-103 and P2R-104, whose binding has the type `&U`, and
  `$v ?? $w` with `$w` of type `U` converts by P2R-105. A union value
  converts into a place of its own union (P2R-231) and in the calls of
  P2R-233, P2R-251 and P2R-310. Every other use stops the conversion: `echo`, interpolation and
  joining, arithmetic, every comparison other than with the literal
  `null`, a condition, a cast other than `(string)`, the argument of a
  built-in function other than those of P2R-233 and P2R-251 and the
  search and replacement of P2R-310, `??` with a default of
  another type, a `foreach`, an element read, a property read or method
  call, and the subject of a `match`.
- **P2R-233** `is_scalar($v)`, `gettype($v)` and `(string) $v`, where `$v`
  is a variable or parameter of type `U` or `Option<U>`, MUST read `$v`
  and convert over its variants, with `None` for null, matching `&v`, or
  `v` itself for a `&U` or `Option<&U>` parameter and a binding of
  P2R-104. `is_scalar()` gives `matches!(v, ..)` with the `Int`, `Float`,
  `Bool` and `String` variants, so every other variant and `None` give
  `false`. `gettype()` gives a `match` with `"integer"`, `"double"`,
  `"boolean"`, `"string"`, `"array"` for `List`, `Map` and a shape, `"object"`
  for an enum or class, and `"NULL"` for `None`. `(string)` gives a
  `match` with `x.to_string()` for `Int`, `"1"` or `""` for `Bool`, the
  text for `String` and `""` for `None`; an arm that does not read its
  value binds `_`. Where `U` has an `Int` variant the `(string)` result is
  an owned `String`, which P2R-180 stops as the arm of a `?:`; otherwise
  it is a `&str` that borrows `$v`, as `gettype()` gives literals, and
  both may stand as an arm of a `?:`. `(string) $v` as the first arm of
  `is_scalar($v) ? .. : ..`, the same variable or parameter in both, is
  the one place that writes `unreachable!()`: for the `List`, `Map`,
  shape, enum and class variants and for `None`, which the check rules out and which
  no code between the check and the cast can change. These stop the
  conversion: `(string)` where `U` has a `Float` variant (P2R-46), because
  PHP prints a float its own way; `(string)` where `U` has a `List`,
  `Map`, shape, enum or class variant anywhere else, because PHP warns for an
  array and throws for an object; and these calls on a property read
  (P2R-222) and on any other value that may be null (P2R-106). After
  `is_scalar()`, Mago narrows a `scalar` member to `never` and reports
  the read (P2R-11).
- **P2R-234** `$o?->p` and `$o?->m(args)`, where `$o` is a variable,
  parameter, property read or array shape element (P2R-203) of type
  `Option<S>` for a class `S`, or
  `Option<E>` for an enum `E` with `->value` or `->name` (P2R-113), MUST
  convert to a `map` over the lent value, whose closure binds the snake_case
  name of `$o`, or of its last property: `self.date_components.as_ref()
  .map(|date_components| date_components.to_array())`, and
  `self.gender.map(|gender| gender.value().to_string())` for a Copy enum.
  The closure gives the owned value P2R-26 gives a returned value, so
  `$o?->name` of a string property gives `.map(|o| o.name.clone())`. When
  the property or the return type is itself `Option<T>`, `and_then`
  replaces `map`. The result has the type `Option<T>` and converts as any
  `Option` value, `??` included (P2R-105). The arguments of `m` are
  evaluated inside the closure, so they run only when `$o` is not null, as
  in PHP. These stop the conversion: `?->` on a value that cannot be null;
  a chain that goes on after `?->` (`$a?->b->c`, `$a?->b?->c`), whose
  object is then a value that may be null (P2R-106) or not a variable,
  parameter or property read; `?->` on a call result; `?->` on a class
  that changes its objects (P2R-141); a call of a method that throws
  (P2R-211), that changes its object (P2R-143), or that returns `void`,
  also as a statement (P2R-43); an argument that is not a literal, a
  constant, a variable or parameter, or a property read on one, because
  inside the closure a call could borrow what the closure borrows and a
  `?` would leave the closure instead of the function; an argument that
  reads a variable of the closure's binding name; and `?->` on the left of
  an assignment, in `isset`, `unset` or `empty`, or inside an interpolated
  string (P2R-45).

## 25. Maps (milestone 2, slice 15)

Real code keys arrays by strings or ints (`array<string, bool>`,
`array<int, Item>`). This section converts them to `IndexMap`, which keeps
insertion order as a PHP array does. PHP stores a string key that is a
canonical decimal integer in the int range (`'5'`, `'-5'`, not `'05'`,
`'-0'` or `'9223372036854775808'`) as an int key, and turns a lookup key
the same way, so a map keyed by the decimal text of
the key gives PHP's result for every insert, read and check; the int form
becomes visible only where a key leaves the map, and those forms stop. In
the validator this slice moves no problem line by itself: its
`batchValidate` also needs the union narrowing of the next slice, and its
`toArray` puts a shape where a map is declared, which stops. The Laravel
application's `array<string, mixed>` values keep P2R-22.

- **P2R-240** An array type with a key type and a value type and no known
  keys (`array<K, V>`, `non-empty-array<K, V>`) MUST map to a map type:
  `IndexMap<i64, V>` when `K` is `int`, and `IndexMap<String, V>` when `K`
  is `string`, `int|string` or `array-key`, where `V` is the Rust type of
  the value type (P2R-20). The type Mago gives a map after a write by a
  literal key, which names that key beside the key and value types
  (`array{'z': int, ...<string, int>}`), maps the same way, with the
  named keys and their values counted in. A variable that holds a map
  keeps its map type where Mago names only the keys that writes by
  literal keys gave it (`array{'z': int}` after `$m = []; $m['z'] = 2;`),
  when each key and value fits the map. A map behaves as a value of a shape type for
  copying and lending (P2R-201, P2R-204): a parameter becomes
  `&IndexMap<K, V>`, or `Option<&IndexMap<K, V>>` for a nullable map lent
  with `.as_ref()`, an owned map is returned, and P2R-26 copies a map with
  `.clone()`; a map may be a list element, a shape field, a property and a
  map value. Output that names `IndexMap` holds `use indexmap::IndexMap;`:
  after the P2R-52 line for one file, and for a directory input in each
  module that names it, in the `use` group of P2R-172 and P2R-174 after
  the `pub mod` lines, where rustfmt sorts it (P2R-190). These stop the
  conversion: a key type other than those; a value type with no Rust type
  (P2R-22, `array<string, mixed>` included); a value type that is or holds
  a class that changes its objects (P2R-141), because such a class derives
  no `Clone`; a union value type outside a declared type (P2R-250), and a
  map as a union member outside P2R-270; a value of a list type where a
  map type is needed, and of a shape type outside P2R-271, a
  map where a list or shape is needed, and a map where a map of another
  key or value type is needed (`array<string, int>` where
  `array<string, ?int>` or `array<string, float>` is declared, at any
  depth), because PHP keeps each value as it is, while an array literal of
  P2R-241 takes the map type it goes to; an enum or class named `IndexMap`
  (P2R-111, P2R-131); and a module named `indexmap` (P2R-172).
- **P2R-241** An array literal whose elements all have a key where a map
  type is needed (the places of P2R-202: a return value, an argument for a
  parameter, of `new` too, an assignment to a property, a property or
  parameter default, and an element of a list literal or an arm of a
  `match` or `?:` that stands in such a place) MUST convert to
  `IndexMap::from([(key, value), ..])` in source order, so a key that
  repeats at run time (`[$a => 1, $b => 2]` with `$a` equal to `$b`)
  keeps its first position and its last value, as in PHP; `[]` there
  converts to `IndexMap::new()`. A literal of plain values only
  (`['a', 'b']`) there takes the keys 0, 1, 2 in order, as PHP gives them:
  `IndexMap::from([(0, ..), (1, ..)])`, or with `String::from("0")` and on
  for a map keyed by text. `$m = []` on a variable takes its map
  type from the other reads and writes of the variable and the declared
  types it flows into, as P2R-21 gives a list type, where a declared map
  type it flows into wins over Mago's narrower types of its sites, and
  converts to `IndexMap::new()`. Each key and each value converts as a
  value assigned to a variable of `K` or `V` (P2R-25, P2R-26, P2R-102), so
  a key is owned: a string literal as `String::from("k")`, a `&str`
  parameter as `p.to_string()`. An int key of an `IndexMap<String, V>`
  becomes the decimal text of its value (P2R-49): `0x10` gives
  `String::from("16")`, `-0` gives `String::from("0")`, and any other int
  `(n).to_string()` with the parentheses P2R-43 needs. These stop the
  conversion: elements with and without a key in one literal, a spread
  element or a by-reference element; a key whose own expression type is float, bool or null, judged
  before Mago turns it into an int or string key in the array's type; a
  string key of an `IndexMap<i64, V>`, because PHP makes a canonical
  decimal string an int and keeps any other string; a key of a union type;
  an int value for a `float` or `?float` value type, because PHP keeps the
  int (P2R-202); and a literal key that appears twice, which Mago reports
  (P2R-11). An array literal whose type Mago gives as a shape where no map
  type is needed stays a shape (P2R-202).
- **P2R-242** These forms MUST convert on a map `$m`: the reads on a
  variable, parameter or property, and the writes (`$m[k] = v;` and
  `unset`) on a variable or parameter. A lookup takes the key `&k`
  borrowed: a string
  literal as it is (`"k"`), a `&str` parameter as it is, a `String`
  variable as `&s`, an int key of an `IndexMap<String, V>` as
  `&(n).to_string()` (a literal by P2R-241: `"16"`), and an int key of an
  `IndexMap<i64, V>` as `&n`. `$m[k]` converts to `m[&k]` (`m["k"]`,
  `m[&5]`), never to a bare or `as usize` index, which `IndexMap` reads as
  a position; the value is read as a property is (P2R-134): a Copy value
  as it is, any other in place where Rust takes a reference and copied
  with `.clone()` where P2R-26 copies. `$m[k] = v;` converts to
  `m.insert(k, v);`, with the key and the value owned and converted as in
  P2R-241. `unset($m[k]);` converts to `m.shift_remove(&k);`, which keeps
  the order of the other entries as PHP does. `isset($m[k])` converts to
  `m.contains_key(&k)`, or for an `Option` value
  `m.get(&k).is_some_and(Option::is_some)`, because PHP's `isset` is false
  for a null value; `array_key_exists(k, $m)` converts to
  `m.contains_key(&k)`. `$m[k] ?? $d` converts by P2R-105 on the owned
  option `m.get(&k).copied()` for a Copy value and `m.get(&k).cloned()`
  otherwise, followed by `.flatten()` for an `Option` value.
  `count($m)` converts to `m.len() as i64`. `foreach ($m as $v)` converts
  to `for &v in m.values()` for a Copy value and
  `for v in m.values().cloned()` otherwise, with the other rules of
  P2R-41; `foreach ($m as $k => $v)` over an `IndexMap<i64, V>` converts to
  `for (&k, &v) in &m` for a Copy value (`in m` for a `&IndexMap`
  parameter) and `for (k, v) in m.clone()` otherwise. A map parameter that
  the body changes is copied first, as P2R-181 copies a list. Reading a
  key the map does not have is a known difference (P2R-60): PHP warns and
  gives null, Rust panics. These stop the conversion: `$m[] = v`, because
  PHP's next index is the highest int key the array has ever had; a key
  leaving an `IndexMap<String, V>`, where PHP gives an int for a canonical
  decimal string (`foreach ($m as $k => $v)`, `array_keys`,
  `array_key_first`, `array_key_last`, `array_flip`, `array_search`,
  `key()`); `array_merge`, `+`, a sort and any output of a whole map
  (`json_encode`, `var_dump`, `print_r`, `echo`), which renumber or show
  the keys; a compound assignment, `++`, `--` or `??=` on an element
  (`$m[k] += 1`); a comparison of two maps; a key of another type, a
  string key of an `IndexMap<i64, V>` and a key of a union type (P2R-241);
  an element read or write on a map that may be null (P2R-106); a write
  or `unset` of an element of a map property (a later slice); `unset` of
  anything but a map element; and a `foreach` over a map whose body
  changes the map.
- **P2R-243** A map case MUST build with plain `rustc` given the
  `indexmap` crate (`--extern indexmap=<rlib>` and
  `-L dependency=<deps folder>`), and the tests MUST build that crate,
  pinned to version 2.14.2 with its dependencies fixed by a committed
  `Cargo.lock`, from a small cargo project in the repository, so P2R-50,
  P2R-72 and P2R-176 hold for output that names `IndexMap`. The README MUST say that such output needs the dependency
  `indexmap = "2"` in the `Cargo.toml` that builds it; the tool writes no
  `Cargo.toml`.

## 26. Unions as map values and narrowing by type tests (milestone 2, slice 16)

Real code takes a map of mixed scalar values and keeps the ones of one
type: the validator's `batchValidate(array $idNumbers)` declares
`array<int|string, scalar>` and skips each value for which
`!\is_string($idNumber)` holds. This section converts a union as the value
type of a declared map, the four scalar type tests on a union value, and
the narrowing those tests give, in the forms P2R-104 gives a check against
`null`. In the validator it moves the `batchValidate` line; its `toArray`
puts a shape where a union's map member is needed, and a map as a member
of a union still stops, because a shape converts to a map only in PHP's
key order, which Mago does not keep (P2R-200).

- **P2R-250** A declared type (P2R-230) that is a map (P2R-240) whose value
  type has two or more members, other than `null`, of different Rust types
  MUST map to `IndexMap<K, U>`, or `IndexMap<K, Option<U>>` with a `null`
  member, where `U` is the union enum that P2R-230 builds for the value type,
  `scalar` and `array-key` included (`array<int|string, scalar>` gives
  `IndexMap<String, BoolOrFloatOrIntOrString>`). A map type that Mago infers
  inside a body keeps P2R-22, as P2R-230 keeps an inferred union. A value
  goes into such a map, by a literal of P2R-241 or by `$m[k] = v;`, as into
  a place of type `U` (P2R-231), and a read gives a value of type `U`
  (P2R-232). A value of such a map type keeps that type where Mago types
  it more narrowly, each of its value types fitting a member of `U`
  (`array<string, int>` after `$m[$k] = 5`), as P2R-240 keeps a map type:
  in a variable or parameter that holds it, also when it may be null, in a
  new variable that a copy (`$x = $m;`) or a call of a declared return type
  starts, and as the value of another map, a `foreach` value included.
  Only a map type with no declared origin, such as the type of an array
  literal or of a `@var` docblock, keeps P2R-22. These stop the conversion: a union as a list element type, a
  union as a shape field type outside P2R-430, and a map as a member of a
  union outside P2R-270; a
  value of the union type that is not a member value (P2R-231).
- **P2R-251** `is_int($v)`, `is_float($v)`, `is_bool($v)` and `is_string($v)`
  (also written `\is_string`), with one positional argument whose Rust type
  is a union enum `U` or `Option<U>`, MUST convert to
  `matches!(v, U::Int(_))`, `U::Float(_)`, `U::Bool(_)` or `U::String(_)`,
  wrapped in `Some(..)` for `Option<U>`, because PHP's test is false for
  `null`. These stop the conversion: such a test whose union has no member
  of that type, and such a test on a value of any other type (a later
  slice).
- **P2R-252** A test of P2R-251 on a variable or parameter `$v` MUST bind
  the member value in the two forms P2R-104 gives `$v !== null` and
  `if ($v === null)`, with their rules for the reads after it, for writes to
  `$v` and for the step of a `for`:
  - `is_T($v)` as the condition of an `if` or `elseif`, or as an operand of
    the top-level `&&` chain of that condition, becomes
    `let P = S` in an `if let` or `else if let`;
  - `if (!is_T($v))` with no `elseif` or `else` and a body that always
    leaves becomes `let P = S else { ... };`.

  The pattern `P` is `U::V(v)`, or `Some(U::V(v))` for `Option<U>`, where
  `V` is the variant of the member. A Copy member is an int, float, bool or
  enum (P2R-110). The scrutinee `S` is `v` for a Copy member and `&v` for
  any other member when `$v` owns its value, and `*v` for a Copy member and
  `v` for any other member when `$v` is a `&U`: a parameter, or the binding
  of a P2R-104 check on an `Option<U>`. For an `Option<&U>` parameter, the
  pattern of a Copy member is `Some(&U::V(v))`, and `S` is `v` for every
  member. The binding has the member's Rust type, as a parameter of that
  type has it (P2R-24, P2R-26) for a string, list or shape member, so a
  value of the variable is never moved by the test. A string member binds a
  `&String`, so `let v = v.as_str();` follows the `let .. else`, and
  `&& let v = v.as_str()` follows the pattern in an `if let`, and the
  binding then reads as a `&str` parameter does. When nothing reads the binding,
  or the body of `if (!is_T($v))` does not always leave, the test converts
  by P2R-251 and nothing binds. A test in any other form (`!is_T($v) || ..`,
  inside `?:`) converts by P2R-251 only. A reference assignment
  (`$r = &$v`), which would change `$v` through another name, stops the
  conversion (P2R-07).

## 27. Regular expressions without the u flag (milestone 2, slice 17)

The validator cleans a number with
`\preg_replace(self::NON_DIGIT_REGEX, '', $number) ?? ''`, where the
constant is `'#\D#'`, and `StringManipulation` squeezes spaces twice with
`\preg_replace('# {2,}#', ' ', $text) ?? ''`. Without the `u` flag PHP's
PCRE matches bytes, and `\d`, `\w`, `\s` and `\b` are ASCII classes:
`#\D#` removes every byte of `"\u{0663}"`, an Arabic digit, where `#\D#u`
keeps it. Rust's `regex::bytes::Regex` in `(?-u)` mode matches the same
way, so this section converts such patterns on the `regex` crate. A
pattern with `u`, all of whose uses in `StringManipulation` also need
look-around, which `regex` lacks, waits for an engine chosen against
those patterns; section 33 rewrites the two of them for `regex`.

- **P2R-260** A pattern MUST convert when it is a string literal, or a
  constant of a class or enum of the file whose value is a string literal
  (P2R-156), read from that declaration because Mago's literal type
  follows a docblock `@var`, made of printable
  ASCII bytes (0x20 to 0x7E) only, with a delimiter PHP accepts (any byte
  other than a letter, a digit, a backslash, whitespace and NUL; `(`, `{`,
  `[` and `<` close with `)`, `}`, `]` and `>`, and nest as PHP counts
  them), flags from `i`, `m` and `s` only, and a body built only from:
  - literal characters, and escaped punctuation, where `\<` and `\>`
    become `<` and `>`, because `regex` reads them as word boundaries;
  - `.`; `\d`, `\D`, `\w`, `\W`, `\s`, `\S`, `\b` and `\B`; `\t`, `\n`,
    `\r`, `\f`, `\xhh`, and `\x{h..}` up to `7F`, because `regex` reads a
    higher `\x{..}` as a code point where PCRE reads a byte;
  - character classes, also negated, with ranges, the escapes above and
    the POSIX classes `[:alnum:]`, `[:alpha:]`, `[:ascii:]`, `[:blank:]`,
    `[:cntrl:]`, `[:digit:]`, `[:graph:]`, `[:lower:]`, `[:print:]`,
    `[:punct:]`, `[:space:]`, `[:upper:]`, `[:word:]` and `[:xdigit:]`,
    each also with `^`, inside a class;
  - groups `( )` and `(?: )`, and alternation;
  - the quantifiers `*`, `+` and `?` after a literal, `.`, an escape
    class, a character class or a group, and `{n}`, `{n,}` and `{n,m}`,
    with counts up to 65535, after any of these but a group, and their
    lazy forms;
  - `^`, `\A` and `\z`, and `$` with the `m` flag.

  It converts to the Rust pattern `(?-u)` followed by `(?i)`, `(?m)` and
  `(?s)` for its flags, in this order and each once however often PHP's
  flags name it, and by the body, as a raw string literal (`r"(?-u)\D"`,
  `r#".."#` when the body holds `"`), and the tool compiles each
  converted pattern with `regex::bytes::Regex` of `regex` 1.13.1 when it
  converts (`regex::Regex` rejects a `(?-u)` class that can match bytes
  that are not UTF-8). These stop the
  conversion: a pattern of any other form (a variable, a concatenation,
  interpolation); the `u` flag, except in a `preg_match` that P2R-320
  converts, and every flag other than `i`, `m` and `s`; a byte outside printable ASCII; a pattern longer than
  4096 bytes, because PCRE rejects a pattern whose compiled form passes 64
  KiB and a two-letter class of 4 bytes compiles to 33 (1990 copies of
  `[ab]` fail in PHP); an unbalanced bracket
  delimiter (`<a<>`); `$` without `m`, because PCRE's `$` also matches
  before a final newline; `^` with `m`, because PCRE's multiline `^` does
  not match after a newline that ends the subject; `\v`, which PCRE reads
  as every vertical space; `\b{`; inside a class, `[` other than a listed
  POSIX class, `&&`, `--` and `~~`, which `regex` reads as nested classes
  and set operators, and a range from a POSIX class; a class that starts
  with `[:`, `[.` or `[=` and has the form PCRE checks for a POSIX class
  (`[:alpha:]`, `[.a.]`, `[=a=]` outside a class), which PCRE rejects,
  while `[:]` and `[^:ab:]` are plain classes in both engines; a
  quantifier after another quantifier or an assertion (`a**`, `^*`), which
  PCRE rejects; a count after a group (`(?:a){2}`), because PCRE copies
  the group once per count and rejects the pattern past a size limit that
  depends on the group; a repeat that can run more than once of a part
  that can match both the empty string and a longer one (`(?:a?|b)*`),
  because PCRE ends the loop where that part matches empty and `regex`
  goes on; whitespace before the delimiter or among the flags, which PHP
  skips; `\Z`, `\G`, `\K`, `\Q..\E`,
  `\p` and look-around outside the form of P2R-320, backreferences,
  `\k`, atomic groups, possessive
  quantifiers, conditionals, recursion and octal escapes; and a pattern
  that does not compile.
- **P2R-261** `preg_replace(P, R, S) ?? $d`, where `P` converts by
  P2R-260 and can match no empty string, `R` is a string literal or a
  constant as in P2R-260, with no `$` and no `\`, `S` is a
  string, and `$d` is a string literal (a dropped variable would be left
  unread), MUST convert to `php_preg_replace(r"..", "R", s)`, a call of
  the helper below, and `$d` is dropped: PHP's `preg_replace` returns
  null only on a PCRE error, which these patterns raise only past PCRE's
  backtrack, recursion or JIT stack limit (P2R-60). The output then holds
  `fn php_preg_replace(pattern: &str, replacement: &str, subject: &str) -> String`,
  which compiles `pattern` with `::regex::bytes::Regex::new` and returns
  `String::from_utf8_lossy` of `replace_all` on the subject's bytes with
  the replacement taken as it is (`::regex::bytes::NoExpand`). A pattern
  that can match the empty string stops, because PCRE then also matches
  empty right after a match where `regex` does not
  (`preg_replace('#x*#', '-', 'xab')` gives `--a-b-` in PHP and `-a-b-`
  in Rust). These also stop: `preg_replace` without `??`, with a fourth
  or fifth argument, with an array, named or spread argument, and a
  replacement with `$` or `\`, which PHP reads as a group reference.
- **P2R-262** `preg_match(P, S)`, where `P` converts by P2R-260 or P2R-320 and `S` is
  a string, MUST convert to `php_preg_match(r"..", s)`, which gives a
  `bool`, where the result is read as one: the whole condition of an
  `if`, `elseif`, `while`, `for` or `?:`, an operand of `!`, `&&` and
  `||` wherever they stand, and an operand of `=== 1` or `!== 0` (true on a match) or
  `=== 0` or `!== 1` (true on no match), with the literal on either side.
  The output then holds
  `fn php_preg_match(pattern: &str, subject: &str) -> bool`, which
  returns `is_match` on the subject's bytes. These stop the conversion:
  a third argument or more, a named or spread argument, a comparison with
  any other value (`=== false` included), and every other use of the
  result, whose PHP value is the int `0` or `1` or `false`.
- **P2R-263** Output that calls either helper MUST build with plain
  `rustc` given the `regex` crate (`--extern regex=<rlib>` and
  `-L dependency=<deps folder>`). The helpers name the crate only through
  `::regex::` paths, so the output holds no `use` line for it and no
  module, function or type of the input can take its name over. The tests
  MUST build `regex` 1.13.1 from the small cargo project of P2R-243 with
  its dependencies fixed by its `Cargo.lock`, a test MUST check that this
  lock and the tool's own `Cargo.lock` hold the same `regex` version, and
  the README MUST say that
  such output needs `regex = "1.13"` in the `Cargo.toml` that builds it
  (the escapes of any ASCII punctuation the output keeps need 1.8 or
  later, and 1.13 is the version the tests run). A
  function, variable or parameter named `php_preg_replace` or
  `php_preg_match` stops by P2R-31.

## 28. Maps as union members, and shapes as maps (milestone 2, slice 18)

The validator's `IDValidationResult::toArray()` declares
`array<string, bool|array<string, string>|string|null>` and puts
`$this->dateComponents?->toArray()`, a value of the shape
`array{year: string, month: string, day: string}`, where the map member is
needed. This is its last problem line when it converts together with
`StringManipulation`, whose `strReplace` declares `array|string`
parameters typed `string|string[]`. A shape converts to a struct
(P2R-200), whose fields have no order, and Mago keeps no key order for a
shape, but P2R-200 lets nothing change a shape after an array literal
builds it, so the run-time key order of a shape value is the key order of
the literal that built it.

- **P2R-270** A declared type (P2R-230) MAY have one member that is a map
  (P2R-240, with a union value type by P2R-250), beside members of other
  Rust types and `null`. The map member MUST become the variant
  `Map(IndexMap<K, V>)` of the union enum, named `Map` in the enum's name
  (`bool|array<string, string>|string|null` gives
  `Option<BoolOrMapOrString>`). A map value goes into the union as
  `U::Map(..)` (P2R-231). `is_scalar()` gives `false` and `gettype()`
  gives `"array"` for the `Map` variant (P2R-233), and no type test of
  P2R-251 matches it. A map type that Mago infers inside a body keeps
  P2R-22, as P2R-230 keeps an inferred union. `(string)` of such a union
  value follows P2R-233 for the `Map` variant as for `List`: it writes
  `unreachable!()` as the first arm of `is_scalar($v) ? .. : ..` and
  stops anywhere else, because PHP gives `"Array"` with a warning. A
  union with both a shape member and a map member puts a value of that
  shape type into the shape's own variant (P2R-231). An array literal
  where such a union is needed still stops (P2R-231). These stop the
  conversion: two map members (P2R-230 names both `Map`); two unions that
  differ only in the key or value type of their map member
  (`array<string, string>|bool` and `array<string, int>|bool` both give
  `BoolOrMap`, P2R-230); and every other
  use of the `Map` variant's value (P2R-232), such as a condition,
  `is_array()`, `count()` or `foreach`.
- **P2R-271** A value of a shape type `S`, or `Option<S>`, MUST convert
  where a map with text keys is needed (`IndexMap<String, V>`: a map
  place of P2R-241, a parameter, a return value or a property of that
  type, a shape field or a map value of that type, a variable of that
  type, or the `Map` variant of P2R-270), when every field of `S` has the
  Rust type `V`, to `IndexMap::from(s)`, or `s.map(IndexMap::from)` for
  `Option<S>`, after the value is made owned as P2R-26 makes an assigned
  value owned: `.clone()` for a property read, a `&S` parameter or
  binding and a variable that is read again, `.map(ToOwned::to_owned)`
  for an `Option<&S>` parameter, and a move otherwise. The output then holds, after the struct `S`,
  `impl From<S> for IndexMap<String, V>`, whose `from` builds the map with
  `IndexMap::from([(String::from("key"), s.field), ..])`, where `key` is
  the PHP string value of the shape key, escapes decoded, and `field` the
  Rust name P2R-200 gives that key (`(String::from("dateOfBirth"),
  s.date_of_birth)`), in one key order: the order in which the array
  literals of the input that convert to a struct literal of `S`
  (P2R-202), in any place (a return value, an argument, a property, a
  local variable, an arm of `?:` or `match`, the default of `??`, a list
  element, a map value), list their keys, compared by those string
  values. Every such literal MUST list the keys in one order; two
  literals in different orders, and a shape that no literal of the input
  builds, stop the conversion at each place that needs the map, because
  the key order would not be PHP's. A PHP caller outside the input may
  build `S` in another order; that is a known difference (P2R-60). These
  also stop: a field whose Rust type is not `V` (an `int` field where `V`
  is `String`, an `int` field where `V` is the union enum `IntOrString`,
  while a union field of type `V` (P2R-430) converts); a map with int keys
  (`IndexMap<i64, V>`), because a shape key is text; and a shape where a
  list is needed (P2R-240).

## 29. Traits with constants, string map constants and strtr (milestone 2, slice 19)

`StringManipulation` keeps its mapping tables in two traits,
`AccentNormalization` and `UnicodeMappings`, whose only members are
`private const array` constants of string keys and string values, and
its class pulls them in with `use AccentNormalization;`. Four of its
functions run `\strtr($text, self::TABLE)`. PHP's `strtr` with an array
replaces, at each position, the longest key that matches there, whatever
the order of the array, does not look again at text it put in, and
ignores an empty key with a warning; it works on bytes. These three
forms unblock `searchWords`, `utf8Ansi` and `removeAccents`.

- **P2R-280** A trait declared at the top level MUST convert when its
  members are constants only, and a class or enum of the input that uses
  it (`use T;` or `use T1, T2;` in its body, the trait named as P2R-150
  names a class) MUST hold each constant of the trait as if its body
  declared it there, by P2R-156 and P2R-281, in the order of the `use`
  statements and then the trait's own order, before the constants the
  body declares. The trait itself writes nothing. A trait that no class
  or enum of the input uses stops, because the tool checks a trait's
  constants only where it copies them, while PHP checks them when it reads
  the file. For P2R-33, P2R-110, P2R-172 and P2R-175 a trait
  declaration counts as a declaration, not a statement, so a file with a
  trait may still be a library, and a namespace that declares only traits
  gets no module. These stop the conversion: a trait with a method, a
  property, an abstract member or a `use` of another trait (a later
  slice); a `use` with a block of conflict rules (`use T { a as b; }`); a
  `use` of a trait that the input does not declare; a trait named twice
  in one class or enum, which PHP takes once; a trait constant whose name
  the class or enum, or another trait it uses, declares too, which PHP
  rejects unless the two constants match, at the second declaration in
  that order (the class's own constant after a trait's); a trait whose
  fully qualified name equals, ignoring letter case, that of another
  trait, class or enum of the input (P2R-173), or of a class-like PHP has
  built in, and a trait with a name PHP reserves (`Never`, `Mixed`, a
  keyword) (P2R-111); a class that uses a trait after a statement outside
  functions, because PHP declares it only when execution reaches it
  (P2R-130); and a trait declared inside a function or block.
- **P2R-281** A class, enum or trait constant whose value is an array
  literal with a string literal key and a string literal value on every
  element, and whose declared type is `array` or none, MUST convert to
  `pub const NAME: &[(&str, &str)] = &[("key", "value"), ..];` with the
  pairs in the literal's order, each key and value the PHP string value
  of the literal, escapes decoded (P2R-49). Its uses MUST be the second
  argument of `strtr` (P2R-282) only; every other use stops (a read by
  key, a value passed, returned or stored, `foreach`, `count`,
  `isset`). PHP stores a key that is a canonical decimal integer string
  (`'1'`, `'-5'`) as an int key; `strtr` turns every key back to text, so
  the slice holds PHP's result here, and a later slice that reads such a
  constant by key or in a `foreach` must handle it. These stop the
  conversion: an element without a key, with an
  int, float or constant key, or with a key or value that is not a
  single- or double-quoted string literal (heredoc and nowdoc included);
  an empty key, which `strtr` ignores with a warning; a key that
  appears twice, compared by its string value (Mago reports a repeated key
  only in a literal inside a body); and an empty array.
- **P2R-282** `strtr($s, C)`, where `$s` is a string and `C` reads a
  constant of P2R-281 (`self::C`, `static::C` or `Name::C`, `Name` a class
  or enum, never a trait, whose trait constant PHP does not let be read
  through the trait), MUST convert
  to `php_strtr(s, Name::C)`, a call of the helper below with the
  constant's Rust path, giving an owned `String`. The output then holds
  `fn php_strtr(text: &str, pairs: &[(&str, &str)]) -> String`, which
  walks the text from the start: where one or more keys match, it appends
  the value of the longest of them, the last in the slice when several
  are equally long, and moves past that key; where none
  matches, it appends the next character and moves past it. Because every
  key is valid UTF-8 (P2R-49 stops a string literal that is not) and
  `text` is a `&str`, stepping by characters gives PHP's byte result.
  An array literal converts by P2R-311. These stop the conversion:
  `strtr` with three arguments, with any other array (a later slice), with a
  first argument that is not a string, and with a named or spread
  argument (P2R-169).
  A function, variable or parameter named `php_strtr` stops by P2R-31,
  and P2R-188 places `php_strtr` after `php_preg_match`.

## 30. Writes after a null check, trim with a character list, and checkdate (milestone 2, slice 20)

`StringManipulation::nameFix` checks its `?string` parameter against null
at the top of its body and then assigns it six times; its `trim` passes
a second argument to PHP's `trim`; and `isValidTimePart` calls
`checkdate`. These forms give eight of the problem lines of the combined
run of both packages at the start of this slice.

- **P2R-290** A check `if ($v === null)` (or `null === $v`) that P2R-104
  makes a `let ... else` MUST give an owned binding when all of these
  hold: the `if` is a top-level statement of the body of a function or
  method; the last statement of its body is a `return` or a `throw`; the
  inner type of `$v` is `string`, `list<T>` of a type that does not
  change its objects (P2R-143), a map (P2R-240), `int`, `float`, `bool`
  or an enum; and a
  later top-level statement of that body writes `$v` (`=`, a compound
  assignment such as `.=` or `+=`, `++`, `--`, an append, or an `unset`
  of a map element (P2R-242), at any depth). The line after the
  `let ... else` is then
  a copy line, as P2R-181 writes one for a parameter:
  `let mut v = v.to_string();`, `let mut v = v.to_vec();`,
  `let mut v = v.clone();` for a map, or `let mut v = v;` for an int,
  float, bool or enum. The check is the binding's first write for P2R-35,
  so a write of `$v` before any read of it after the check stops the
  conversion, as rustc would warn about the copy line. From the copy line on,
  `$v` is an owned local variable of the inner type for every rule
  (P2R-26, P2R-34, P2R-35, P2R-105, P2R-122, P2R-160), and each later
  write of `$v`, also in a nested block, writes it. A write after the
  check whose value may be null stops the conversion (P2R-34, P2R-106),
  because from the check on the variable has the inner type. For P2R-181
  such a write does not change the parameter, so a parameter that only
  such writes change gets no copy line of its own. The rule holds at the
  top level of the body only: the PHP variable outlives a nested block
  and keeps its value into the next pass of a loop, while the Rust
  binding ends with its block, so a write there would not reach the
  `Option` outside. These still stop: a write in the body of the `if let`
  form of P2R-104; a write after a `let ... else` that is not a top-level
  statement of the body, or whose body does not end with a `return` or a
  `throw`; a write after a type test of P2R-252 (a later slice); a write
  after a check of a shape or an object, which P2R-201 and P2R-143 do not
  let change; and a `foreach` key or value named `$v` after the check,
  whose loop variable would hide the binding (P2R-41).
- **P2R-291** `trim($s, $characters)`, `ltrim($s, $characters)` and
  `rtrim($s, $characters)` with two `string` arguments MUST convert to
  `php_trim_chars(s, characters)`, `php_ltrim_chars(s, characters)` and
  `php_rtrim_chars(s, characters)`, of type `String`. Each of these helpers
  gets PHP's byte mask from the helper
  `php_charmask(characters) -> [bool; 256]`, which the output then holds
  too, removes the bytes of the mask from the start, from the end or from
  both, as PHP does, and gives the rest through `String::from_utf8_lossy`.
  `php_charmask` walks the bytes as PHP's `php_charmask` does: a byte,
  `..` and a byte that is not lower than the first add the range from
  the first to the last, and the walk goes on after the last; where the
  byte at the walk position is `.`, the next byte is `.` and no such
  range starts there, that byte adds nothing and the walk goes on at the
  second `.` (PHP warns there), so `a..` adds `a` and `.`; every other
  byte adds itself. Where the second argument is a string literal, or a
  class constant whose value the tool reads from its declaration
  (P2R-260), the tool does the same walk when it converts, and these
  stop the conversion: a `..` that starts no range, which PHP warns
  about, and a byte above `7F` in the mask, which can cut a character of
  more than one byte. Any other second argument converts, and its
  differences at run time are listed by P2R-60. A named or spread argument stops the
  conversion (P2R-169). A function, variable or parameter named like one
  of the four helpers stops by P2R-31.
- **P2R-292** `checkdate($month, $day, $year)` with three `int` arguments
  MUST convert to `php_checkdate(month, day, year)`, of type `bool`, a
  helper that gives true when the year is 1 to 32767, the month 1 to 12,
  and the day 1 to the number of days of that month, with a 29th of
  February in a year that 4 divides and 100 does not, or that 400
  divides, as PHP does. The helper uses only comparisons and `%` by a
  positive number, so no argument value overflows. Another argument count,
  an argument of another type (PHP throws a `TypeError` under
  `strict_types`, and Mago does not report it), and a named or spread
  argument (P2R-169), stop the conversion. P2R-188
  places `php_charmask`, `php_trim_chars`, `php_ltrim_chars`,
  `php_rtrim_chars` and `php_checkdate` after `php_strtr`, in this order.

## 31. Closures as callbacks of array_map and preg_replace_callback, and ucwords (milestone 2, slice 21)

`StringManipulation::nameFix` capitalises the parts of a hyphenated name
with `array_map(ucwords(...), explode('-', $name))` and lowers name
prefixes with `preg_replace_callback(P, static fn(array $matches): string
=> strtolower($matches[1]), $name) ?? ''`. Closures are common in
application code (the Laravel `app/` holds about 70), and these two
built-ins run their callback at once, before the enclosing body goes on,
so a Rust closure that borrows what it reads sees what PHP's copy holds.
This section converts a closure only as such a callback.

- **P2R-300** An arrow function (`fn(T $x): R => expr`, with or without
  `static` and the return type) MUST convert to a Rust closure
  `|x: T| expr` where it is the callback argument of P2R-301 or P2R-302,
  and nowhere else. It takes exactly one parameter, a new variable of the
  parameter type of P2R-24 (`&str` for a string), which only its body
  reads, and which hides a variable of the same PHP name of the enclosing
  body there, as in PHP; for P2R-30 and P2R-31 it is a variable of the
  enclosing function, so a parameter whose Rust name another variable or
  a function has stops. A parameter the body does not read gets a
  leading `_`, as a foreach binding does, and is no P2R-35 problem. The
  body converts as the value of `return` in a function whose return type
  is the declared one, or the type Mago gives the body when none is
  declared (P2R-25, P2R-26, P2R-49, P2R-102), so a string becomes owned
  and a literal gets its type; P2R-301 stops a declared return type that
  differs from the body's type. The body may read a
  variable or parameter of the enclosing body whose Rust type is Copy (an
  int, float or bool, an enum of P2R-110, or an `Option` of one of these),
  which the closure borrows; such a read counts as a read at the place of
  the arrow function for P2R-34 and P2R-35. PHP copies these values when
  it creates the closure, before it evaluates the list of P2R-301, where
  the Rust chain evaluates the list first; both read the same value,
  because no form that could change a captured variable inside the list
  argument converts (P2R-07 for references and `global`, P2R-43, P2R-44).
  These stop the conversion: an arrow function anywhere else (stored,
  returned, or passed to a function of the input or to another built-in);
  a parameter without a declared type, by reference, variadic or with a
  default; `fn&`, which returns by reference; a read of a variable of the
  enclosing body of any other type, a union enum of P2R-230 included (a
  later slice); a write in the body (an assignment, `++` or `--`), which
  in PHP changes only the closure's copy; a call that throws (P2R-211) or
  `?? throw` in the body, whose `?` would leave the closure instead of
  the function; `$this` in the body; a `match` as the body (P2R-120); a
  nested arrow function, also as the callback inside the body of another
  (an `array_map` call as the list argument of another still converts,
  because the list converts before the callback); and a closure of the
  long form (`function () use (..) { .. }`), also with `static` (a later
  slice). As the callback of a `when` link (P2R-451), an arrow function
  or a closure of the long form converts to store conditions instead, and
  this paragraph does not apply to it.
- **P2R-301** `array_map(CB, $list)` with two positional arguments, where
  `$list` is any expression whose value is a list (`Vec<T>`, not a map)
  of ints, floats, bools, enums or strings, a temporary such as
  `explode(..)` included, MUST convert to
  `list.iter().copied().map(CB).collect::<Vec<_>>()` for a Copy element,
  and `list.iter().map(String::as_str).map(CB).collect::<Vec<_>>()` for a
  string element, a `Vec` of the callback's return type in the list's
  order, as PHP keeps a list's keys. `CB` is an arrow function of P2R-300
  with one parameter of the element type, or a first-class callable
  `f(...)` that converts as the closure `|x: T| f(x)` would: `f` a
  function of the input with one parameter of the element type, that
  returns a value and does not throw (P2R-211), or one of the built-ins
  `strtolower`, `strtoupper`, `ucfirst`, `lcfirst`, `ucwords`, `trim`,
  `ltrim` and `rtrim` on a string element. The closure's parameter gets a
  name that is not the Rust name of `f`. A first-class callable converts
  as this callback and nowhere else. These stop the conversion: a map,
  more than one list, a `null` callback, a callback of any other form (a
  string such as `'trim'`, a variable, a static method callable), a
  callback whose parameter type is not the element type (PHP passes
  `array_map` elements in weak mode, so `fn(string $x)` on ints would
  convert them), a list of any other element type, `Option` included (a
  later slice), a result whose Rust type is not that of the type Mago
  gives the call, where that type has one (where it has none, every
  place that takes the result stops by P2R-22 or checks the value's own
  type) (Mago types the result from the callback's body, not
  from its declared return type, so `fn(int $x): float => $x` or
  `fn(int $x): ?int => $x` stops, because the places that take the result
  go by Mago's type), and a named or spread argument (P2R-169).
- **P2R-302** `preg_replace_callback(P, CB, S) ?? 'literal'`, where `P` is
  a pattern of P2R-260 that cannot match the empty string, `S` is a
  string, and `CB` is an arrow function of P2R-300 with one parameter
  declared `array` and a declared `string` return type, MUST convert to
  `php_preg_replace_callback(r"..", |matches: &IndexMap<String, String>|
  body, s)`, with the `??` default dropped as P2R-261 drops it. The
  parameter has the type `&IndexMap<String, String>` whatever P2R-22 and
  P2R-23 give a bare `array`: a map of P2R-240 with text keys, which is
  what Mago types its reads (`array<array-key, string>`), so
  `$matches[1]` reads `matches["1"]` (P2R-242). The helper walks the
  matches from left to right; for each it calls the closure with the keys
  `"0"` (the whole match) to `"n"`, where `n` is the last group that took
  part in the match, and `""` for a group before it that did not, as PHP
  does, and it joins the text between the matches with the closure's
  results. The text is matched as bytes, as in P2R-261, and each group
  and the result go through `String::from_utf8_lossy`. These stop the
  conversion: a pattern that can match the empty string (P2R-261 gives the
  reason); a pattern that can match a byte above `7F` (`.`, `\D`, `\W`,
  `\S`, a negated class), because a group could then hold part of a
  character, which PHP passes as raw bytes and a Rust `String` cannot
  hold; a fourth or fifth argument; a callback of any other form; a
  parameter declared with another type; a missing or other return type;
  `preg_replace_callback` without `??`; a default that is not a string
  literal; and a named or spread argument (P2R-169).
- **P2R-303** `ucwords($s)` with one string argument MUST convert to
  `php_ucwords(s)`, of type `String`, a helper that makes the first byte,
  and each byte after a space, `\t`, `\r`, `\n`, `\f` or `\v`, an ASCII
  capital, as PHP does; it changes ASCII letters only. A second argument
  (the delimiters, a later slice) and a named or spread argument
  (P2R-169) stop the conversion. P2R-188 places
  `php_ucwords` and `php_preg_replace_callback` after `php_checkdate`, in
  this order, and P2R-31 covers their names.

## 32. Arrays in str_replace and strtr (milestone 2, slice 22)

`StringManipulation::strReplace` takes `array|string $search` and
`array|string $replace`, which its docblock types `string|string[]`, a
union of a string and a map of strings (P2R-230, P2R-270), and calls
`str_replace($search, $replace, $subject)`, and for a one-byte search
`strtr($subject, [$search => $replace])`. With arrays, PHP's `str_replace`
replaces in order: for each search, where a later search sees what the
earlier ones made, it puts in the replacement at the same position, `""`
where the replacements run out, or the one string replacement for every
search; it skips an empty search, whose replacement still counts, it
ignores keys, and it throws a `TypeError` for a string search with an
array replacement.

- **P2R-310** `str_replace($search, $replace, $subject)` with a string
  subject, where `$search` or `$replace` is not a string, MUST convert to
  `php_str_replace(searches, search_is_text, replaces, replace_is_text,
  subject)`, a helper with PHP's rule above on `&[&str]` slices. Each of
  `$search` and `$replace` gives its slice and its flag: a string `s`
  gives `&[s]` and `true`; a list literal without keys gives its
  elements, each a string, as `&["a", s]` and `false`; a list
  (`Vec<String>`) gives `&xs.iter().map(String::as_str).collect::<Vec<_>>()`
  and `false`; a map of strings (P2R-240) gives its values in order,
  `&m.values().map(String::as_str).collect::<Vec<_>>()`, and `false`; and
  a value `v` of a union `U` of P2R-230 whose members are a string and
  one list or map of strings gives `&match &v { U::String(text) =>
  vec![text.as_str()], U::List(list) => list.iter().map(String::as_str)
  .collect() }`, with `U::Map(map) => map.values()` in the second arm for
  a map, and `matches!(&v, U::String(_))`. A union argument is read
  twice, so it must be a literal, a constant, a variable, a parameter or
  a chain of property reads on one of them or on `$this` (P2R-186), and
  this is an exception to P2R-232. Three strings still convert by P2R-185. The helper panics,
  before it looks at the subject, where the search is a string at run
  time (a string, or a union that holds one) and the replacement is a
  list or map at run time (a list, a map, a list literal, or a union that
  holds one): PHP throws a `TypeError` there, also for an empty subject
  (P2R-60). These stop the conversion: a string search with a list, map
  or list literal replacement, which PHP always rejects with that
  `TypeError`; a subject that is not a string; an element that is not a
  string or a spread in a list literal; an array literal with keys, which
  P2R-202 stops, or this rule where P2R-202 reads it as a shape (passing
  its values in order would differ where two keys are equal at run
  time); a legacy `array(..)` literal, which P2R-07 stops; a union with
  other members; a union argument of any other form;
  and a fourth argument. P2R-188 places `php_str_replace` after
  `php_preg_replace_callback`, and P2R-31 covers its name.
- **P2R-311** `strtr($s, [k1 => v1, ..])`, with an array literal whose
  every element has a key and a value of type string, MUST convert to
  `php_strtr(s, &[(k1, v1), ..])` with each key and value lent as a
  `&str`, a call of the helper of P2R-282. `strtr($s, [])` converts to
  `php_strtr(s, &[])` and gives the text unchanged. A key that repeats at run time
  keeps its last value, as in PHP, because the helper takes the last of
  equal longest keys. These stop the conversion: an element without a
  key, a key of type int (a literal or not), a key or value of another
  type, a literal empty key (for P2R-281's reason), a spread, and a
  legacy `array(..)` literal, which P2R-282 stops. A key
  that is empty at run time, with a text that is not empty: PHP warns and
  ignores it, Rust ignores it without the warning (P2R-60).

## 33. Look-around in two u patterns read as a bool (milestone 2, slice 23)

`StringManipulation::nameFix` tests a surname with
`\preg_match('#(?<!\p{L})mc(?! )#u', $lowerLastName) === 1`, and the same
for `mac`. The `regex` crate has no look-around. Read as a bool, though,
`(?<!\p{L})mc(?! )` matches somewhere in a subject exactly when
`(?:^|\P{L})mc(?:[^ ]|$)` does: the look-behind tests one character
before the word, or the start, and the look-ahead one character after
it, or the end. A spike against PHP 8.5.9 with PCRE2 10.48 compared the
words `mc` and `mac` over every code point and over 100082 strings, and
a spec review compared 18 words over 1776152 strings; neither found a
difference other than the letters new in Unicode 17. A spike that added
`fancy-regex` or `pcre2` gave the same Unicode 16 answers, at the cost
of a new crate (and a C compiler for `pcre2`). This section converts the
one tested form; general look-around waits for an engine.

- **P2R-320** `preg_match(P, S)` read as a bool by P2R-262, where `P` is
  a string literal or a constant as in P2R-260, of printable ASCII bytes
  and 4096 bytes at most, with a delimiter as there, no flag but `u`
  (once or more, as PHP reads a repeated flag once), and the body
  `(?<!\p{L})`, then one or more ASCII letters and digits, then `(?! )`,
  MUST convert to
  `php_preg_match(r"(?:^|\P{L})mc(?:[^ ]|$)", s)` of P2R-262, with the
  letters and digits of the body in place of `mc`. The Rust pattern has
  no `(?-u)`, so `regex` reads the subject's bytes as UTF-8 and `\P{L}`
  and `[^ ]` each match one character, as PCRE's `u` does; a subject
  holds valid UTF-8 in Rust (section 5). `\p{L}` follows the Unicode 16.0
  tables of `regex-syntax` 0.8.11 in Rust and the tables of the PCRE2
  that PHP links in PHP (17.0 for PCRE2 10.48, 14.0 for 10.42), so a
  letter of only one of the two versions right before the word can
  change the answer (P2R-60). These stop the conversion: a pattern with
  `u` of any other form, in `preg_match` or elsewhere (another
  look-around or class, a word with other characters, a look-around in
  the middle, a missing look-around); a delimiter that ends the pattern
  early or leaves it unclosed, as `<(?<!\p{L})mc(?! )>u`, whose `<` nests
  so that it has no closing delimiter (P2R-260), and
  `!(?<!\p{L})mc(?! )!u`, which ends at the `!` of the look-behind (also
  `?`, `)` and `}`), as in PHP, where each gives `false` with a warning,
  while `(..)`, `{..}` and `[..]` nest as PHP counts them and convert;
  `u` with another flag; and a pattern with `u` in `preg_replace` and
  `preg_replace_callback`, which P2R-260 still stops for them.

## 34. DateTime from a format, and date_parse of its text (milestone 2, slice 24)

`StringManipulation::isValidDate(string $date, string $format)` reads a
date with `DateTime::createFromFormat($format, $date)`, rejects `false`
and a date whose `format($format)` differs from the text with
`if (!$dateTime instanceof DateTime || $dateTime->format($format) !==
$date) { return false; }`, and checks the parts of
`date_parse($dateTime->format('Y-m-d H:i:s'))`, typed by a docblock as
`array{year: int, month: int, day: int, hour: int, minute: int, second:
int}`. The validator calls it with the format `'Ymd'`. The format is a
parameter, so the conversion cannot know it, and a helper reads it at
run time. Probes of PHP 8.5.9 (`php -n`, time zone UTC) fixed the rules
below: a number reads digits up to its width, a day, month, hour,
minute or second out of range carries into the next field instead of
failing (`19800230` gives 1 March 1980), and a field the format does not
read takes the current date or time.

- **P2R-330** The class `DateTime` (`\DateTime`) as a type MUST map to
  the helper struct `pub struct PhpDateTime`, with the `pub` fields
  `year`, `month`, `day`, `hour`, `minute` and `second`, each an `i64`,
  and `#[derive(Clone, Copy, Debug)]`, and Mago's type `DateTime|false`
  (the return type of `DateTime::createFromFormat()`) MUST map to
  `Option<PhpDateTime>`, which P2R-230 does not make a union enum. The
  struct is `Copy`, so a value passes and copies as an enum value does
  (P2R-112). The output holds the struct once, after the shape structs
  and the union enums (P2R-200, P2R-230), when the output or a helper
  names it; a module of a directory input reads it through `use
  crate::PhpDateTime;` (P2R-174). `$v === false`, `false === $v`, `$v
  !== false` and `false !== $v` on `Option<PhpDateTime>` convert as
  P2R-103 converts the same forms with `null`, a `false` value where
  `DateTime|false` is needed (a return, an argument, a `match` arm)
  converts to `None`, and `gettype()` of a `DateTime` gives `"object"`
  (P2R-222).
  Only these forms and those of this section take a `DateTime`. These
  stop the conversion: `new DateTime(..)`, every other method or
  property of `DateTime`, `DateTimeImmutable`, `DateTimeInterface`, a
  union of `DateTime` with a type other than `false`, `?DateTime` and
  `DateTime|null` (whose `null` the `None` of `DateTime|false` would
  confuse with `false`), every comparison
  of two values of type `DateTime` or `DateTime|false` (PHP compares
  objects by identity or by their fields), `=== null`, `!== null`, `??`
  and `?->` on a `DateTime|false` value, whose `None` is PHP's `false`,
  not `null`, a `DateTime` in `echo`,
  interpolation or joining, and a class or enum whose Rust name is
  `PhpDateTime`, a name the output uses (P2R-111).
- **P2R-331** `DateTime::createFromFormat($format, $text)` with two
  `string` arguments MUST convert to
  `php_date_create_from_format(format, text)`, of type
  `Option<PhpDateTime>`, a helper that reads the format up to its first
  NUL byte, as PHP does, and walks it and the text's bytes with PHP's
  rules:
  - `d` and `j` read a day, `m` and `n` a month, `H` and `G` an hour,
    each of one or two digits; `Y` reads a year of one to four digits;
    `y` reads one or two digits and adds 2000 below 70 and 1900 from 70;
    `i` and `s` read a minute and a second of exactly two digits. Each
    number starts at a digit and reads as many digits as its width
    allows;
  - `!` sets every field to 1970-01-01 00:00:00, and `|` sets the fields
    not read yet to those values;
  - a space reads zero or more spaces, tabs, no-break spaces (U+00A0)
    and narrow no-break spaces (U+202F); `#` reads one byte of
    `;:/.,-()`; `?` reads any one byte; `\` makes the next format byte
    a literal;
  - every other byte that is not an ASCII letter, `*` or `+` reads the
    same byte.

  The helper gives `None` where a number does not start at a digit, `i`
  or `s` has one digit, a literal, `#` or escaped byte differs, the
  format ends with `\`, text is left after the format, or the format is
  left after the text other than `!` and `|`, which still apply.
  Otherwise the fields that no letter, `!` or `|` set take the current
  date in UTC for the year, month and day (each on its own), `0` for the
  hour, minute and second where a letter set one of these, and the
  current UTC time where none did; a clock before 1970 reads as
  1970-01-01 00:00:00. Then the fields carry as PHP's do: the second,
  minute and hour into the day, the month into the year, and then the
  day against the length of that month (second 60 is the next minute,
  hour 24 the next day, day 0 the last day of the month before, month
  13 January of the next year), which can give a year from -1 to 10007.
  The helper panics, before it reads the text, where the format holds
  an ASCII letter other than those above, `*` or `+` outside a `\`
  escape, and it panics where the text holds a NUL byte, where PHP
  throws a `ValueError` (P2R-60); where the format is a string literal
  or a constant as in P2R-260, such a byte stops the conversion
  instead.
- **P2R-332** `$d->format($f)` on a value of type `PhpDateTime` with a
  `string` argument MUST convert to `php_date_format(d, f)`, of type
  `String`, a helper that writes `d`, `m`, `H`, `i` and `s` with two
  digits, `j`, `n` and `G` without a leading zero, `Y` with at least four
  digits after a `-` for a year below 0 (`-0001`), `y` as the remainder
  of the year by 100 in two digits (`-1` for the year -1, as PHP gives),
  the byte after `\` as it is, a NUL byte for a `\` that ends the format,
  as PHP writes the end of its format string, and every other byte that
  is not an ASCII letter as it is (`*` and `+` too). It panics at another
  ASCII letter (P2R-60); where the
  format is a string literal or a constant as in P2R-260, such a letter
  stops the conversion instead. `format()` on `Option<PhpDateTime>`
  stops by P2R-106.
- **P2R-333** `$v instanceof DateTime` (`\DateTime`) on a value of type
  `Option<PhpDateTime>` MUST convert to `v.is_some()`, and
  `!$v instanceof DateTime`, which PHP reads as `!($v instanceof
  DateTime)`, to `v.is_none()`. P2R-104 MUST take `$v instanceof
  DateTime` and `$v !== false` as it takes `$v !== null`, and
  `!$v instanceof DateTime` and `$v === false` as it takes `$v ===
  null`, for its `if let` and `let ... else` bindings of type
  `PhpDateTime`. These stop the conversion: `instanceof` with another
  class (other than the `Optional` of P2R-362) or on a value of another
  type, and a class given by a variable or a string.
- **P2R-334** `if (X || B) { .. }`, where `X` is a check that P2R-104
  turns into `let Some(v) = .. else` (`$v === null`, `!$v instanceof
  DateTime`, `$v === false`, and their mirrored forms), it is the first
  operand of the top-level `||` chain of the condition, the `if` has no
  `elseif` or `else`, and its body ends with a `return`, `throw`,
  `break` or `continue` statement and, inside a `for` with a step,
  holds no `break` or `continue` that leaves it (the step goes in at
  each `continue` of one block only), MUST convert to
  `let Some(v) = v else { .. };`
  followed by `if B { .. }`, where `B` is the rest of the chain in order
  (`B || C` for three operands), reads the binding, and both blocks hold
  the body, converted once. PHP evaluates `X` first and reads `B` only
  where `X` is false, where Mago types `$v` as not null, which the
  binding is. Where neither `B` nor a later statement reads the binding,
  P2R-104 writes `if v.is_none() { .. }` for the let-else, still followed
  by `if B { .. }`. The rules of P2R-104 and P2R-290 for a write to `$v`
  and for the step of a `for` apply as to a `let ... else`. Where the
  body reads `$v`, which is not the binding in Rust, where the `if` has
  an `else`, or where the body does not end as above, the form does not
  apply, and the condition converts as any other: a read of `$v` in `B`
  then stops by P2R-106. `X` as a later operand converts by P2R-103 or
  P2R-333 without a binding.
- **P2R-335** `date_parse($d->format('Y-m-d H:i:s'))`, where `$d` has
  type `PhpDateTime` and the format is that string literal, MUST convert
  to a struct literal of the shape `array{day: int, hour: int, minute:
  int, month: int, second: int, year: int}` of P2R-200 with each field
  read from `d` (`DayHourMinuteMonthSecondYearShape { day: d.day, hour:
  d.hour, .. }`): PHP's `date_parse` reads that text back to the same
  numbers for every year from -9999 to 9999, and for a year above 9999
  it misreads the text (P2R-60). Mago types the call as `mixed`, so a
  variable that the call writes takes that shape, and a later read
  checks it against Mago's type of the read (P2R-34), which a docblock
  `@var` of the variable gives: another type stops. Every other use of
  `date_parse` stops by P2R-48.

## 35. Code that the input uses but does not convert (milestone 2, slice 25)

A Laravel application calls its framework and packages, whose code sits
in Composer's `vendor/` folder and is not part of the input. Mago then
knows none of those classes, types their values as `mixed`, and reports
an unknown class or a `mixed` access on about a third of the problem
lines of the Laravel `app/` folder (1000 of 3295 lines on 15 Oct 2026),
which stops every file whatever the converter supports. Mago has a file
kind for such code, `FileType::Vendored`: its symbols enter the codebase
so that the input's types resolve, and Mago keeps a built-in or an input
symbol over a vendored one of the same name. This section reads such
code for its types only; converting the framework's own forms waits for
later sections.

- **P2R-340** The option `--include <PATH>`, which may be given more than
  once, MUST make every regular file whose name ends in `.php` under each
  path, at any depth and without following a symbolic link below the
  path, known to Mago as a file of kind `FileType::Vendored` before the
  input is analysed: its classes, interfaces, traits, enums, functions
  and constants enter Mago's codebase, so a type or call of the input
  that names one resolves. A path that is a symbolic link is followed,
  and a path that names a file names that one file, whatever its name
  ends in. Mago knows an included file by a name that no input file can
  have and that is the same on every machine: the position of its option
  on the command line and its path below the option's path. The input
  files enter the codebase first, then the included files, in the order
  of the options and, below one path, in byte order of their paths. An
  included file is never analysed or converted, a Mago finding whose span
  lies in an included file is not printed, and a file that cannot be
  read, is not valid UTF-8, does not parse, or has a path below the
  option's path that is not valid UTF-8 (two such paths could share a
  name) is skipped without a problem line. Mago keeps a symbol of PHP's
  built-ins or of the input over an included symbol of the same name, so
  a polyfill in an included file changes no built-in function, and an
  input file that also lies under an include path is read as input only.
  The checks of a name against PHP's built-ins (P2R-31, P2R-111, P2R-131,
  P2R-151) see PHP's built-ins only, and an included declaration is not a
  declaration of the crate (P2R-173). A type or call of the input that
  names an included symbol converts by the rules of the earlier sections,
  so it stops (a class type by P2R-22, a call by P2R-48 or P2R-153) until
  a later section maps it. An unqualified call in a namespace whose
  function of that short name an included file that is read declares
  calls that function in PHP, not the built-in or a global function of
  the input, so it stops by P2R-48. A file that is not read (a symbolic
  link below the path, or a file skipped above) declares nothing, so
  such a call converts as the built-in or the global function, which
  P2R-60 lists as a difference. An attribute stops by P2R-07 whether or
  not an included file declares its class, on a function, a parameter or
  an arrow function as on a class or a member, other than the one of
  P2R-341, those of P2R-351 and P2R-381, the `#[Override]` of P2R-221 and
  the `#[ObservedBy]` of P2R-460. The option works for a file
  input and
  for a directory input.
  An include path that does not exist, that is neither a file nor a
  directory (a named pipe), or below which a directory cannot be listed
  exits 2 with its reason (P2R-04).
- **P2R-341** The attribute `#[SensitiveParameter]` (any name PHP
  resolves to `\SensitiveParameter`), written alone on a parameter that
  is not promoted, of a function, a method other than an interface's
  (P2R-370) or an arrow function, and
  with no arguments (`#[\SensitiveParameter]` or
  `#[\SensitiveParameter()]`), MUST be left out: PHP uses it only to
  hide the argument's value in a stack trace, and the Rust output prints
  no stack trace. `StringManipulation::nameFix` carries it, so that
  package converts in full only with this rule. Every other attribute on
  a parameter stops (P2R-07), other than those of P2R-351 and P2R-381.

## 36. Spatie Data classes as plain structs (milestone 2, slice 26)

A Laravel application passes its input and output through classes that
extend `Spatie\LaravelData\Data` of the package `spatie/laravel-data`: 60
of the 375 files of the `finance-accounting-api` app on 15 Oct 2026,
each a constructor of promoted properties, most with attributes such as
`#[MapInputName('data.name')]` or `#[Max(255)]` on the parameters. In
the installed version 4.18.0, `Data` is an abstract class with no
constructor, no `__get`, `__set`, `__call` or `__isset` (its one magic
method is `__sleep`), two protected properties of its own
(`$_additional` and `$_dataContext`) and the interfaces `Responsable`
and nine of the package's own, which bring `JsonSerializable`,
`Arrayable`, `Jsonable` and `Castable` with them. Its methods (`from()`,
`toArray()`, `validate()` and the rest), and Laravel when it builds
such a class from a request, are the only code that reads the
attributes or those two properties; the engine reads the properties
too for `var_dump`, `(array)`, `==`, `clone` and `serialize`, which
all stop in the converted code (P2R-07, P2R-48, P2R-136). This section
converts such a class as a plain struct and leaves every use of the
parent to a later section.

- **P2R-350** A class whose `extends` names one class, which PHP
  resolves in any letter case to `\Spatie\LaravelData\Data` declared in
  an included file, MUST convert as the same class without `extends`
  converts: nothing of the parent enters the Rust output. A `Data` that
  the input declares is a class of the input, so extending it stops by
  P2R-130. A list of parents stops, because PHP rejects it and Mago's
  parser accepts it. Only that direct parent
  qualifies; a class that extends such a class, or extends anything
  else other than the parents of P2R-390 and P2R-401, still stops
  (P2R-130), and so does a `readonly` one, which PHP
  rejects because `Data` is not readonly. Everything the parent brings
  stops: a call of a method that the class does not declare itself
  (`$dto->toArray()` by P2R-135, `Dto::from()` by P2R-153), a read of
  the parent's properties (`$this->_additional`, P2R-134), a type that
  names `Data` or one of its interfaces (P2R-22), an `instanceof` of
  them (P2R-333), and the class's object where a value of such a type
  is due. These stop too: a property or promoted parameter named
  `_additional` or `_dataContext`, because the parent declares one of
  that name; a method whose name, in any letter case, is a method of
  `Data` in Mago's codebase, because PHP checks an override against the
  parent and Mago does not catch every form (the names of its `@method`
  tags count too, other than the hooks that P2R-380 leaves out); a class
  whose parent, as Mago knows it, names a trait or
  interface that no included file declares (an `--include` of
  `Data.php` alone), because then the methods of `Data` are not all
  known; a class whose parent, as Mago knows it, extends another class,
  which version 4.18.0 does not; a class whose parent, as Mago knows it,
  has a method that PHP calls by itself (`__construct`, `__destruct`,
  `__get`, `__set`, `__isset`, `__unset`, `__call` or `__toString`),
  which version 4.18.0 does not have and the struct would drop; and
  such a class after a
  statement outside functions, because PHP declares it before any
  statement runs only when `Data` is loaded before the file compiles,
  which Composer's autoloader does not do. These checks see the `Data`
  that `--include` gives: a stand-in that declares fewer methods than
  the package (as the test folders use) hides an override of the
  others. Without `--include` of the package, no included file declares
  `Data`, so the class stops by P2R-130 (P2R-340).
- **P2R-351** On a class of P2R-350, an attribute whose class PHP
  resolves, in any letter case, to a name that starts with
  `\Spatie\LaravelData\Attributes\` (`MapInputName`, `MapOutputName`,
  `Validation\Max`, `Validation\Required`, `Validation\Unique` and the
  others), on the class or on a promoted parameter of its constructor,
  MUST be left out with its arguments: PHP builds an attribute and
  evaluates its arguments only when reflection reads them
  (`ReflectionAttribute::getArguments()` or `newInstance()`), and only
  the parent's methods and Laravel's resolving of the class do that,
  which never run in the converted code. Each argument of such an
  attribute must be a string or int literal or `Name::class` (not a
  string with an escape that PHP rejects, such as `\u{110000}`, or warns
  about, such as an octal escape above `\377`), with
  named arguments after the positional ones and each name once: PHP
  compiles the arguments as constant expressions and rejects other
  forms, such as a call or `static::class`, when it compiles the file,
  and Mago does not report them all; P2R-382 allows `new` too, and
  P2R-381 leaves them out on other classes. Every other attribute there,
  and one with another argument,
  stops (P2R-07), and an attribute group
  (`#[A, B]`) that holds an attribute that stops, stops as a whole.

## 37. Spatie's Optional in a Data class (milestone 2, slice 27)

A `Data` class of section 36 marks a property that a request may leave
out with the type `Spatie\LaravelData\Optional`: `string|Optional`, or
`string|Optional|null` where the request may also send `null`. 27 of the
60 `Data` classes of the Laravel app hold such a property, and code
fills one with `new Optional`, `Optional::create()` or a value, and
reads it back with `$dto->name instanceof Optional`. In version 4.18.0,
`Optional` is a class with one static method, `create()`, which returns
an `Optional`: no parent, interface, trait, property, constant,
constructor or other magic method, and no class of the package or the
app extends it. This section gives such a union a
Rust type and converts the forms above; reading the value inside, which
needs the check to narrow the type, waits for a later section.

- **P2R-360** A declared or analysed type whose members are a name that
  PHP resolves, in any letter case, to `\Spatie\LaravelData\Optional`,
  other members of one Rust type `T` after P2R-21, and possibly `null`,
  MUST become `PhpOptional<T>` for `T|Optional` and
  `PhpOptional<Option<T>>` for `T|Optional|null`, whatever the order of
  the members, where `PhpOptional` is the generic enum
  `pub enum PhpOptional<T> { Absent, Present(T) }` with
  `#[derive(Clone, Debug, PartialEq, Eq)]`, written once at the crate
  root after the shapes, the unions and the `DateTime` struct when an
  item names it, with the `use` line of every module that names it
  (P2R-174). A parameter of that type is borrowed as `&PhpOptional<T>`,
  its value cloned where it is stored, as a union is (P2R-232), and it
  cannot be assigned, as an object parameter cannot (P2R-132). A read of
  a property, variable or parameter of that type that Mago narrows
  (after `instanceof Optional`, a check against `null` or a write)
  stops: a property read with the problem of P2R-362, also as the
  operand of a second `instanceof Optional`, and a variable or parameter
  read because its type changes (P2R-34), as P2R-361 says for a write.
  This holds only
  when an included file declares `Optional` with the shape of 4.18.0: a
  class that is not abstract, with no parent, interface, trait, property
  or constant, none of the methods PHP calls by itself (P2R-350), and a
  public static `create()` without parameters; an `Optional` that the input
  declares is a class of the input (P2R-230), and one of another shape,
  or no included `Optional`, gives the union no Rust type (P2R-22). These
  stop too: `Optional` alone, also with `null` (`Optional|null`,
  `?Optional`); with members of more than one Rust type; with `true` or
  `false` as `T`, which P2R-21 would widen to a `bool` PHP's type does
  not take whole; with `T` a `DateTime` together with `false` or
  `null`, because P2R-330 makes `None` mean `false`; with `T` a class
  that changes its objects or holds one (P2R-141), which is never
  cloned; with `T` a list of a nullable class type (`list<?C>`, P2R-204);
  as a list or map element or a shape field; and a class that holds
  itself through `PhpOptional`, as through `?C` (P2R-130). A class or
  enum whose Rust name is `PhpOptional` stops (P2R-111, P2R-131),
  because the helper takes that name.
- **P2R-361** Where a value of type `PhpOptional<T>` is due (an argument,
  a return value or a property write), `new Optional`, `new Optional()` and
  `Optional::create()`, with a name that PHP resolves, in any letter
  case, to `\Spatie\LaravelData\Optional` and without arguments, MUST
  convert to `PhpOptional::Absent`, the call to `create()` only where
  Mago types it as `Optional` alone, as the declared return type of
  4.18.0 gives, with its body trusted to make a new `Optional` and do
  nothing else, as 4.18.0's does; a value that goes where `T` is
  declared to `PhpOptional::Present(v)`, converted as for a place of type
  `T` (P2R-25, P2R-26, P2R-102: an int is a float for `float|Optional`, a
  `?T` value goes into `Present` of `T|Optional|null` as it is); `null`
  for `PhpOptional<Option<T>>` to `PhpOptional::Present(None)`; and a
  value of the same `PhpOptional` type to itself. An argument passed to a
  `&PhpOptional<T>` parameter is lent: `&PhpOptional::Absent`,
  `&PhpOptional::Present(v)` with `v` owned, `&v` for a variable, and a
  parameter as it is. Each arm of a `match` in such a place converts as
  such a value (P2R-121). A variable keeps one Rust type (P2R-34), and
  Mago gives it the type of each value written, so a variable of type
  `PhpOptional<T>` takes only a value of that type: a later write of
  `new Optional`, a `T` value or `null` stops, as does a first write of
  a `match` whose arms give `Optional::create()` and a `T` value
  (P2R-122). These stop: `new Optional` or
  `Optional::create()` anywhere else, because its PHP type is `Optional`
  alone; `new Optional(...)` with an argument, which PHP runs; a value of
  another `PhpOptional` type; `new Optional` or `Optional::create()` as
  an arm of `?:` (P2R-180) or as the default of `??` in such a place,
  while a `?:` or `??` of `T` values goes into `Present` as one `T`
  value; and the default of a declared property of that type.
- **P2R-362** `$v instanceof Optional` on a value of type `PhpOptional<T>`,
  with a name that PHP resolves, in any letter case, to
  `\Spatie\LaravelData\Optional`, MUST convert to
  `matches!(v, PhpOptional::Absent)`, and `!$v instanceof Optional`,
  which PHP reads as `!($v instanceof Optional)`, to its negation. Every
  other use of such a value stops: `===`, `!==`, `==` and `!=`, `??`,
  `?->` that reads such a value, `echo` and concatenation, a cast, and the
  value where a `T` or another type is due, because PHP's type there
  depends on a check this section does not narrow (P2R-07, P2R-22).

## 38. Interfaces of the input as traits (milestone 2, slice 28)

The Laravel app declares 9 interfaces of its own, all in
`App\Interfaces`: `Labelable`, which an enum implements, and service,
processor and file-system interfaces, which classes implement. A PHP
interface lists methods that every class that implements it must
declare, and a Rust trait does the same. This section converts such an
interface to a trait, and the methods of a class or enum that implements
it to an `impl` block of that trait. A value whose type is an interface
needs `dyn` or a generic, and waits for a later section, as do an
interface that extends another, its constants and its static methods.

- **P2R-370** An interface declared at the top level, with no `extends`
  and no attribute, whose body holds only methods, each with at most one
  modifier, `public`, no body, a declared return type and a name that
  does not start with `__`, MUST convert to `pub trait Name { ... }` with
  one line `fn m(&self, p: T) -> R;` per method, in source order. The
  receiver is `self` instead when one or more enums and no class
  implement the interface, because an enum method takes its value
  (P2R-155), and an interface with a method that both an enum and a
  class implement stops. The trait takes the Rust name
  a class of that name takes, and its methods the names methods take
  (P2R-131); a parameter takes the type and form of a method parameter
  (P2R-24, P2R-132, P2R-201, P2R-232), a parameter of a changing type its
  shared form (`&C`, `&[C]`, P2R-143), the return the Rust type of a
  method's return, with none for `void`, and docblock types count as they
  do for a method (P2R-23). The trait stands where a class of that name
  would (P2R-33); in a directory input it stands in its file's module,
  which P2R-172 makes for a namespace of interfaces only. These stop:
  a constant, a property or a static method in the interface; `extends`;
  an attribute (P2R-07), also on a parameter; a method modifier other than one `public`, a
  method body, a by-reference return and a name that starts with `__`
  (P2R-130, P2R-155), which PHP rejects or treats in its own way and
  Mago's analyser does not report; a parameter with a default, by
  reference or variadic; a parameter or return type with no Rust type
  (P2R-22), also `self`, `static` and `never`; a method named like one
  that a trait of Rust's prelude gives the output's types (`to_owned`,
  `clone_into`, `into`, `try_into`, `eq`, `ne`), where only an inherent
  method wins, and `new` or `clone` (P2R-131); two methods whose names are
  equal ignoring letter case (P2R-131); an interface whose name PHP
  reserves or has as a class-like (P2R-111), or whose Rust name is that
  of another enum, class, interface or function of the output or a name
  the output uses (P2R-111, P2R-131, P2R-173); and an
  interface declared inside another statement. No array shape or union
  takes a trait's name (P2R-200, P2R-230). An interface declared in an
  included file (P2R-340) is not one of the input.
- **P2R-371** A class or enum whose `implements` list names one or more
  interfaces that P2R-370 converts, and no other interface, MUST convert
  as it would without `implements` (sections 14 to 16 and 36), except
  that each of its methods that one of those interfaces declares (by
  name, in any letter case) is written without `pub` in an
  `impl Name for C { ... }` block, one block per interface in the order
  of the `implements` list, after the class's or enum's own `impl` block;
  its body converts as before. A method without a docblock takes the
  types of the interface method's docblock, as Mago gives them. The
  method's Rust name MUST be the trait method's, and its Rust signature
  the trait's: the same number of parameters, each with the trait's type
  and form, and the trait's return type; parameter names, the `_` of an
  unused parameter (P2R-35), `mut` on a parameter the body writes and a
  default on the class side do not count. These stop: a method whose Rust
  name or Rust signature differs (PHP matches names ignoring letter case,
  and accepts a wider parameter type, a narrower return type or an added
  optional parameter); a method that is static, changes its object
  (`&mut self`, P2R-143) or throws (P2R-211); a method that two of the
  interfaces declare; a class or enum that declares no method of the name
  of a method of one of its interfaces, which the tool checks itself,
  because Mago does not report a missing `from`, `tryFrom` or `cases` on
  an enum; an interface that one `implements` list names twice, which PHP
  rejects and Mago does not report; `ArrayAccess` (P2R-220) or an
  interface not of the input beside them; and such a class or enum after
  a statement outside functions, which PHP declares only when execution
  reaches it (P2R-110, P2R-130).
- **P2R-372** A call of such a method on a value of the class or enum
  MUST convert as other method calls do (P2R-135, P2R-155). In a
  directory input, a module that does not declare the trait and whose
  Rust lines name it in an `impl` line or call a method of its `impl`
  blocks holds one `use` line for it, and no other module does (P2R-174);
  a call in a `default` arm that P2R-121 leaves out is no call. The
  interface's name anywhere else stops, as the name of a class that does
  not convert does: as the type of a parameter, property, return value or
  variable, also in a docblock (P2R-22), with `instanceof`, `::class` or a
  static call.

## 39. laravel-data's request hooks left out (milestone 2, slice 29)

When laravel-data builds a `Data` object from a request, it validates
and authorises it through static methods the class may declare: the
seven that its `ValidateableData` trait names in `@method` tags
(`rules`, `messages`, `attributes`, `stopOnFirstFailure`, `redirect`,
`redirectRoute` and `errorBag`), and `authorize`, which its
`AuthorizedDataPipe` looks for. None has a real method in `Data`, only
laravel-data calls them (through `app()->call`), and the methods that run
them (`from()`, `validate()`, `validateAndCreate()`,
`getValidationRules()`) stop (P2R-350). 12 of the 60 `Data` classes of
the Laravel app declare `rules()` or `authorize()`, whose bodies use
Laravel's validation rules and `Gate`. This section leaves them out, as
P2R-351 leaves out the validation attributes, and it leaves out two more
forms of those attributes that the app uses.

- **P2R-380** On a class of P2R-350, a method whose name, in any letter
  case, is one of those eight MUST be left out with its signature and
  body when it has this form: its modifiers are `static` and at most one
  `public` and one `final` (no visibility means public), it has a body
  and no attribute, no interface of the class declares a method of that
  name, and neither `Data`, as Mago knows it, nor a trait or interface
  of it has a real method of that name, also one that `Data` or one of
  its traits renames to that name in a `use` block, only an `@method`
  tag or nothing (Mago
  takes a tag of `Data` over a real method of its trait or interface).
  Nothing of it enters the Rust output; its docblock is not part of it.
  Its signature must keep to forms that PHP always compiles: each
  parameter or return type is a class name, `int`, `float`, `string`,
  `bool`, `array`, `iterable` or `object`, with or without `?`, or
  `mixed` alone, and a return type may also be `void`, `never`,
  `static` or `self` alone; a default is a string, int or float
  literal, `true`, `false`, or `null` on a type with `?` or no type,
  and defaults come last (P2R-168); no parameter is variadic or by
  reference or has an attribute. A local class name `integer`, `boolean`, `double` or
  `resource` stops, because PHP warns that it reads it as a class name.
  Other forms that PHP accepts, such as `callable` or `?self`, stop
  too. These stop with their own rules: a return by
  reference (P2R-135), a parameter named `$this` (P2R-135), `static` in
  a parameter (P2R-154), a promoted parameter, which for Mago is any
  parameter with a modifier (`abstract` and `final` too) (P2R-130), a parameter
  named like a superglobal (`$_GET`, `$GLOBALS` and the others) and a
  second parameter of one name. In its body these stop, because PHP
  rejects, deprecates or warns about them when it compiles the file and
  Mago reports nothing or a code that is dropped here: a function
  declared inside it, in which `self` names no class; `self`, `parent`
  or `static` after a backslash (`new \self`); a cast that PHP removed
  or deprecates (`(real)`, `(unset)`, `(integer)`, `(boolean)`,
  `(double)`, `(binary)`); `${...}`; `case 1;` with a semicolon;
  backticks; a string escape that PHP rejects (`\u{110000}`) or an
  octal escape above `\377`; `$http_response_header`; a class,
  interface, trait or enum declared inside it, and an anonymous class,
  whose members the tool does not check; `declare`; and in a closure, a
  `null` default on a type without null, a default before a required
  parameter or a parameter named like a superglobal. PHP accepts the function itself, `Foo\self`, `${'x'}`
  outside a string, an anonymous class, `declare(ticks=1)` and `\400`
  in single quotes, so this stops more than PHP rejects; the same holds
  for `\400` in single quotes in an attribute argument or a default. Of
  Mago's findings inside it (P2R-11), these are
  not reported: a
  class or method that does not exist, or a call on a mixed value
  (`non-existent-class`, `non-existent-method`, `mixed-method-access`),
  which PHP checks only when the line runs, and a parameter count that
  does not match the
  `@method` tag (`incompatible-parameter-count`), which PHP never
  checks. Every other Mago finding inside it still stops, such as
  `return;` in a method with a return type, which PHP rejects when it
  compiles the file. Some of those stop a file that PHP accepts: Mago
  checks a parameter or return type against the tag too, a parameter
  type that no file declares gives `non-existent-class-like`, and a
  value of a class that no file declares leads to other codes, such as
  `mixed-return-statement`, so a hook that uses Laravel converts only
  with the framework's sources in `--include`, and a value that Mago
  types as `mixed` there still stops it. A call of it stops, as a
  call of a method the class
  does not declare does (P2R-135, P2R-153). A method of one of those
  names in another form stops (P2R-380), and so does a second one whose
  name is equal ignoring letter case: laravel-data calls it as a public
  static method, PHP rejects an attribute such as `#[\Override]` when it
  loads the class, and a modifier such as `readonly` or one written
  twice and a second method of one name when it compiles the file, and
  Mago does not always report them. When an interface of the class
  declares the name, P2R-371 applies to the method instead, and so does
  P2R-350's stop for the name of an `@method` tag; when `Data` or its
  trait or interface has a real method of the name, P2R-350's stop
  applies (`withValidator` and
  `prepareForPipeline` are real methods of version 4.18.0, not hooks).
- **P2R-381** The attributes of P2R-351, with its argument forms, MUST
  be left out on every class of the input, not only on a class of
  P2R-350: on the class and on the promoted parameters of its
  constructor, of a `readonly` class too. laravel-data reads them only
  on a class that extends one of its own classes (it builds its
  `DataClass` only for `BaseData`), and any parent other than `Data`
  stops by P2R-130.
- **P2R-382** An argument of such an attribute MAY also be `new C`,
  `new C()` or `new C(` arguments `)`, where `C` is a class name (not
  `self`, `parent`, `static` or an expression), and each argument of
  `new` is a string or int literal or `Name::class`, with named
  arguments after the positional ones, each name once, and no `new`
  inside; the attribute is left out with it. PHP compiles such a `new`
  as a constant expression and runs it only when reflection reads the
  arguments (`getArguments()` or `newInstance()`), which never runs in
  the converted code, so whether `C` can be built does not change the
  output. A Mago finding on it still stops (P2R-11), and so does any
  other `new` (P2R-07).

## 40. Exception classes of the input (milestone 2, slice 30)

Section 22 throws and catches PHP's built-in exceptions only. The Laravel
app declares 13 exception classes of its own in `App\Exceptions`, and
throws them in about 40 places: some hold nothing
(`class RoleDoesNotExistException extends \Exception {}`), others only
a constructor that builds the message and passes it on with
`parent::__construct($message, $code)`. This section converts those two
forms. An exception object stays the one struct of P2R-210, so the
properties a constructor promotes are not kept, and a class with
methods, such as getters, still stops.

- **P2R-390** A class declared at the top level whose `extends` names
  one class, and whose chain of parents of the input ends at a class
  that PHP resolves to one of P2R-210, is an exception class of the
  input; one whose fully qualified name equals, in any letter case, one
  of Laravel's classes of P2R-420 is not, and a chain that ends at one of
  those classes does not count, so such a class stops as a class that
  extends another (P2R-130). It MUST convert to no struct: its fully qualified PHP name as
  declared, in that letter case and without the leading backslash
  (`App\Exceptions\RoleDoesNotExistException`), joins the class tree of
  `is_a` under its parent (P2R-210), after the built-in classes under
  the same parent and in source order, and `uncaught()` prints that
  name, as PHP does, whatever letter case a `throw` or `catch` writes.
  The output writes the name as a Rust string literal with each `\`
  doubled (`"App\\Exceptions\\RoleDoesNotExistException"`). P2R-111
  applies to its name, as to a class's. It may be `final`, and it may
  hold a constructor (P2R-391) and nothing else. These stop: an
  attribute on it or on its constructor (P2R-381 does not apply to
  it); `abstract` or `readonly`; a modifier written twice and a second
  constructor in any letter case, which PHP rejects (P2R-130);
  `implements`; any other member (a
  method, a property, a constant, a trait use); a parent of the input
  that is `final`; a parent of the input that the same file declares
  after it, because PHP then declares the class only when execution
  reaches it; and any use of the class other than `throw new` (P2R-212),
  a `catch` clause (P2R-392), the `extends` of another exception class,
  a `use` line (P2R-150) and a docblock: a type that names it stops
  (P2R-22), and so do `new` outside a `throw`, `instanceof` and a static
  access.
- **P2R-391** The constructor of an exception class of the input MUST
  convert to `pub fn <name>(<parameters>) -> PhpException`, placed
  where the class stands, where `<name>` is the class's short name in
  snake case (`FieldLengthInvalidException` gives
  `field_length_invalid_exception`). Its parameters follow the rules for
  the parameters of a function, defaults and their order included
  (P2R-168); a promoted parameter converts as a plain parameter, because
  only a read through the object (`$e->entity`) reads its property, and
  that stops (P2R-214). Its body converts as the body of a function and
  MUST end with its one `parent::__construct(..)` call, at the top level
  of the body, which converts to `return PhpException::new("<class>",
  message, code);` with the arguments of P2R-212. A class with a
  constructor makes the output hold the struct of P2R-210 even when no
  `throw` or `catch` converts. These stop: a constructor with a
  modifier other than `public` and `final` (`static` and `abstract`
  included), with a return type, or that returns by reference, which
  PHP rejects or gives a notice for and Mago does not report; a
  parameter with a modifier written twice, with `static` or `abstract`,
  or with a set visibility wider than its visibility, which PHP rejects
  (P2R-130); a promoted
  parameter with a property hook, which runs when the constructor sets
  the property (P2R-130); a body that does not end with that call, or
  that reads or writes `$this`, holds a `return`, holds another
  `parent::` access or holds a `throw` or a call that throws, also in the
  arguments of that call (P2R-211), or that changes an object it is
  given, because a `throw` would have to lend it and its callers' forms
  do not see that (P2R-143); a call with arguments other than the
  message and the code (P2R-212);
  a promoted parameter named `message`, `code`, `file`, `line`,
  `previous` or `trace`, names of properties of PHP's `Exception`, the
  first four of which change what `getMessage()`, `getCode()`, the
  uncaught text or `getLine()` give; a constructor in a class whose parent of
  the input has a constructor, and a class without a constructor whose
  parent of the input has one, because `parent::__construct(..)` or
  `new` would run the parent's; and a name that P2R-31 or P2R-173
  refuses for a function, other than P2R-31's rule on the names of PHP's
  built-in functions, which PHP applies to functions only, or that a
  function or another exception class of the input gives. For P2R-31 the
  function is a function of the file, so a variable, parameter or
  `catch` variable of its Rust name stops. The body is no function for
  P2R-50's test of a call to itself, so it may call a PHP function whose
  name equals the class's. A `throw` of a class whose constructor stops
  stops with no problem of its own (P2R-03).
- **P2R-392** `throw new C(..)`, where `C` is an exception class of the
  input, MUST convert as P2R-212 converts a class of P2R-210: for a class
  without a constructor with `PhpException::new("<class>", message,
  code)`, and for a class with one with `<name>(<arguments>)`, its
  arguments converted as for a call of that function (P2R-169). A
  `catch` clause MUST accept an exception class of the input as it
  accepts a class of P2R-210, testing `e.is_a("<class>")` (P2R-214),
  where `<class>` is the name P2R-390 gives. In
  a directory input, a module that names the function of P2R-391 holds a
  `use` line for it (P2R-174).

## 41. Eloquent models from migrations (milestone 2, slice 31)

The Laravel app keeps its data in 35 Eloquent models, 33 of which extend
`Illuminate\Database\Eloquent\Model` directly. A model declares no property: its
attributes are the columns of its table, and only the migrations in
`database/migrations` declare those, so Mago types every attribute read as
`mixed`. This section reads the migrations, tells Mago the type of each
column of a model's table, gives the model a struct of those columns, and
converts attribute reads. Creating, querying and saving models need a
store and wait for a later section; this section clears no file of the app
by itself, because the files that read a model also create or query one,
and its gain is typed attribute reads.

- **P2R-400** The option `--migrations <DIR>`, given at most once, MUST
  read as migrations the regular files and the symlinks to one directly
  in the directory whose
  names match `*_*.php`, as Laravel does (a name that starts with `.`
  does not match, as in PHP's `glob()`), in byte order of the names
  without `.php`. Migrations never enter Mago: the tool parses them and
  resolves their names only, and prints no Mago finding about them. A
  migration's last top-level statement is `return new class extends
  Migration { .. };`, with only `declare` and `use` statements and
  inline text (a byte order mark) before it, because Laravel runs any
  other code when it requires the file, and inline text only in a file
  without `declare`, which PHP refuses after such text;
  `Migration` resolves to
  `\Illuminate\Database\Migrations\Migration`, and its class has a public
  method `up()` without parameters, with or without the return type
  `void`, no method `shouldRun()` or `getConnection()`, which Laravel
  calls before it runs `up()`, and no magic method (a name that starts
  with `__`, such as `__construct` or `__destruct`), which PHP runs when
  Laravel builds, clones or drops the object, no two methods,
  properties or constants of one name and no
  modifier written twice, which PHP rejects, and no hooked property and
  no closure, arrow function or first-class callable (`drop(...)`) in a
  constant, a property default or a parameter default (PHP 8.5 allows
  them there), because a read in
  `up()` would run its code; its other members are not read (a `$connection` naming another
  database included), and `down()` is not read. `Schema` stands for
  `\Illuminate\Support\Facades\Schema` and `DB` for
  `\Illuminate\Support\Facades\DB`, each also by Laravel's global alias
  (`\DB`). A migration changes rows only, and is skipped, when every call
  in its class, `down()` aside, is one of these: `DB::table(..)`,
  `DB::raw(..)`, `now()` or `now()` of Carbon (`Carbon\Carbon`,
  `Carbon\CarbonImmutable` or `Illuminate\Support\Carbon`), or a chain
  of calls on one of them that calls neither `getConnection()`, which
  runs any statement, nor `getProcessor()`, which runs its SQL through
  the connection, nor a method named by an expression
  (`now()->toDateTimeString()` qualifies); `date(..)` or `count(..)`; or
  a call on `$this` of a method the class declares other than `down()`.
  It holds no `new`, partial
  application, `include`, `require`, `eval` or `exit`, and no `$this`
  other than as the receiver of a call or property read, because
  `[$this, 'down']` or an invokable `$this` passed to `chunk(..)` runs a
  method without a call. A macro that the app adds to the query builder
  or to Carbon is a call of the chain too, and a callable passed as a
  string or a class's array (`->each('drop')`, `[C::class, 'm']`) runs
  code without a call the tool sees; the tool cannot see into these
  (review concern). The statements of each other `up()` change a set
  of tables, file after file, in order:
  - `Schema::create('<t>', <closure>);` creates table `t`, and
    `Schema::table('<t>', <closure>);` changes it.
  - `if (! Schema::hasTable('<t>')) { Schema::create('<t>', <closure>); }`
    creates `t` when no table `t` exists, known or unknown, and otherwise
    does nothing; after a statement or file that made every table
    unknown (the last item below), a table that the tool has not seen
    may exist, so the guard makes `t` unknown.
  - A statement that is a chain on `DB::table('<t>')` that calls none of
    the three above, and whose arguments call only the forms above
    that a rows-only migration may make (`DB::raw(..)`, `now()`, a
    chain on them, `date(..)`, `count(..)`) and nothing on `$this`,
    changes rows, not columns, and is skipped; a `chunk(..)` callback
    that calls `Schema` makes it another statement.
  - Any other statement (`DB::statement(..)`, `DB::unprepared(..)`, a
    call of a helper method) makes every table that exists at that point
    unknown; a table created after it is known. A file that cannot be
    read, is not valid UTF-8, does not parse or does not have this shape
    does the same.

  The closure is `function (Blueprint $table) { .. }`, with or without
  `static` and the return type `void`, where `Blueprint` resolves to
  `\Illuminate\Database\Schema\Blueprint`, and each statement of its body
  is a call `$table->m(..)` on its parameter with modifiers chained after
  it (`->nullable()`); another closure form, or another statement in it,
  makes its table unknown. A column method names its column by its first
  argument, a string literal, and the arguments after the name are not
  read (`string('ip', 45)`, `enum('c', [..])`), nor is the argument of
  `timestamps(..)`. These declare a column
  of a Rust type: `id()` (column `id`, or the name of its argument),
  `foreignId`, `bigInteger`, `unsignedBigInteger`, `integer`,
  `unsignedInteger`, `smallInteger`, `unsignedSmallInteger`,
  `tinyInteger` and `unsignedTinyInteger` as `i64`; `string`, `char`,
  `text`, `mediumText`, `longText` and `enum` as `String`; `uuid()`
  (column `uuid`, or the name of its argument) as `String`;
  `rememberToken()` as a nullable `String` named `remember_token`; and
  `foreignIdFor(C::class)` as `i64`, named by its second argument or
  else by the snake case of P2R-401 of `C`'s short name followed by
  `_id`, when `C` resolves to a model of the input (P2R-401) whose body
  holds no `$primaryKey`, `$keyType` or `$incrementing`, no method
  `getKeyName`, `getKeyType` or `getForeignKey` and no trait other than
  `HasFactory` and `SoftDeletes`, because Laravel takes the column's name
  and type from `C`'s key; for any other `C` the table is unknown. These
  declare a column without a Rust type: `boolean` and `decimal`, which
  PHP reads as an int or a bool, and as a string, a float or an int,
  depending on the database driver; `float` and `double`; `timestamp`, `dateTime` and
  `date`; and `timestamps()` (`created_at` and `updated_at`) and
  `softDeletes()` (`deleted_at`, or the name of its argument), whose
  columns are nullable. The modifiers `nullable()` and `nullable(true)`
  (the column may hold null), `nullable(false)` (it may not),
  `default(..)`, `unsigned()`, `constrained(..)`, `cascadeOnDelete()`,
  `nullOnDelete()`, `restrictOnDelete()`, `cascadeOnUpdate()`,
  `onDelete(..)`, `onUpdate(..)`, `unique()`, `index()`, `primary()`, `after(..)`,
  `comment(..)`, `useCurrent()`, `useCurrent(true)`, `useCurrent(false)`
  and `useCurrentOnUpdate()` keep the column's type. After `foreignId` or `foreignIdFor`, a modifier after
  `constrained(..)` belongs to the foreign key, as in Laravel, so
  `nullable()` there leaves the column not nullable; `constrained(..)`
  after any other column method makes the table unknown, because Laravel
  then keeps the later modifiers on the column. `change()` replaces the column of that name with the new
  declaration, nullability included, as Laravel 11 and later do, and the
  column keeps its place. The table methods `unique(..)`, `index(..)`,
  `primary(..)`, `foreign(..)` with `references(..)`, `on(..)` and the
  delete and update modifiers chained after it, `dropForeign(..)`,
  `dropUnique(..)` and `dropIndex(..)` change no column, and
  `dropColumn('<c>')` or `dropColumn(['<c>', ..])` removes columns. The
  columns of a table keep the order in which they were first declared;
  `after()` does not move them. Any other form makes its table unknown:
  another method or modifier, a column name that is not a string
  literal, an unpacked argument to the method or to `nullable(..)`,
  `default(..)` or `useCurrent(..)`, a named argument `column`, `columns`
  or `model` to the method, which binds what the tool reads by position
  (`id(column: 'code_id')`), any named argument to `nullable(..)`,
  `default(..)` or `useCurrent(..)`, which Laravel takes as its first
  value by any name (`constrained('t', indexName: 'i')` and
  `timestamps(precision: 6)` keep the table known), `useCurrent(..)` with
  an argument other than `true` or `false`, a modifier after
  `timestamps()`, which gives a Collection that Laravel cannot chain, a
  column declared
  again without `change()`, a `change()` or
  drop of a column the table does not have, `Schema::create` of a table
  that exists, and `Schema::table` of one that does not. None of these is
  a problem of its own: a model of an unknown table stops (P2R-401), and
  its message names the form with the migration's path, the directory as
  given on the command line joined to the file name by one `/` (as
  P2R-171 forms an input path), and its line. A
  path that does not exist, is not a directory or cannot be listed exits 2 with its
  reason (P2R-04), as P2R-340 does for an include path, and so does a
  second `--migrations`. The option does not take its files out of a
  directory input, so a migrations directory belongs outside the input.
  The tool does not read a schema dump (`database/schema`) or the
  migrations of a package, which Laravel also runs (review concern).
- **P2R-401** A class declared at the top level is a model when its
  `extends` names one class that PHP resolves to
  `\Illuminate\Database\Eloquent\Model` and a file that `--include` reads
  declares that class (P2R-340); a `Model` that the input declares makes
  no class a model. The placement rules of P2R-130 apply to it, and so
  does P2R-280's rule on a class that uses a trait after a statement
  outside functions; P2R-280's stop of a trait that the input does not
  declare does not apply to the two trait uses below. Its table is the string literal of its property `protected $table
  = '<t>';`, or else Laravel's snake case of its short name (an `_`
  before each capital letter that follows another character, then lower
  case: `VATRate` gives `v_a_t_rate`) with the last word in the plural:
  `y` after a consonant becomes `ies`, a word ending in `s`, `x`, `ch`
  or `sh` gains `es`, and any other gains `s` (`waltz` gives `waltzs`, as
  in Laravel). Laravel's pluralizer differs on some words (`person`
  gives `people`, `software` and `data` stay, `quiz` gives `quizzes`),
  so a model whose table
  Laravel names otherwise finds no table of this rule's name and stops,
  unless `$table` names its table; it converts wrongly only when the
  migrations create both names, and Laravel then reads the other table.
  A model
  MUST convert to `#[derive(Clone, Debug)] pub struct <Name> { pub
  <field>: <type>, .. }`, with no `impl` block other than the store of
  P2R-410, `final` or not: one field
  per column of its table in the order of P2R-400, named by P2R-30 from
  the column name, of the column's Rust type after P2R-402, inside
  `Option` when the column is nullable. Left out of the struct are a
  column without a Rust type, a column whose cast is not one of P2R-402,
  a column named like a public instance property of `Model` or of a
  trait it uses at any depth (`exists`, `timestamps`), which PHP reads
  instead of the column, and a
  column whose Rust name P2R-30 refuses. Two fields with one Rust name
  stop the model (P2R-131). Its body may hold the properties `$table`,
  `$fillable`, `$guarded`, `$hidden` and `$casts`, each `protected` with
  a literal value (a string, or a list or map of strings and `C::class`),
  the method `protected function casts(): array { return [..]; }` with a
  literal map, which is read with `$casts`, its entries winning on a
  shared key as in Laravel, the trait uses `HasFactory` and
  `SoftDeletes` (resolving to
  `\Illuminate\Database\Eloquent\Factories\HasFactory` and
  `\Illuminate\Database\Eloquent\SoftDeletes`), and `public $timestamps
  = true;` or `false`, which decides whether `created_at` and
  `updated_at` read as dates (P2R-402); these are left out. A method stops the conversion and still leaves the struct to the
  code that reads the model, with or without `#[\Override]` on it,
  because it does not change what an attribute read gives, unless it is
  an accessor or mutator (a name `get<Name>Attribute` or
  `set<Name>Attribute`), has a declared return type that names
  `\Illuminate\Database\Eloquent\Casts\Attribute`, also inside `?` or a
  union, has a name that starts with `boot` or `initialize` (Laravel runs
  boot<Trait> and initialize<Trait> when it boots or builds the model),
  has any other attribute (such as `#[Boot]` or `#[Initialize]`, which
  Laravel runs the same way), or overrides a method of `Model` or of a
  trait it uses at any depth (`booted`, `getAttribute`), other than
  `getRouteKeyName`, each name compared in any letter case, as Laravel
  and PHP compare them. Everything else stops the
  model and leaves it without a struct, because PHP may read an attribute
  through it differently: such a method, a constant, another property or
  trait; an attribute on the class (P2R-07) other than the
  `#[ObservedBy]` of P2R-460, whose forms section 47 limits because an
  observer may change a model it reads; `abstract`, `readonly` or
  `implements`; a missing `--migrations`; a table that the migrations do
  not create or have made unknown; and a cast of a column its table does
  not have. The problem points at the member, attribute or modifier that
  takes the struct away, or at the model's name for a reason of its
  table, whose message names the form with its migration's path and
  line, or that no migration creates the table; each read of a model
  without a struct stops by P2R-404. A modifier written twice
  (P2R-130), a second property of one name (`$table` and `$Table` are
  two), and a second method of one name in any letter case, which PHP
  rejects and Mago does not report,
  stop the conversion. These checks see the `Model` that `--include`
  gives, as P2R-350's see `Data`: a stand-in that declares fewer
  properties or methods than the framework hides a column named like
  one of the others and an override of it. A Mago finding in the model's
  declaration, its name included, stops the conversion by P2R-11 and
  leaves the struct (`@use HasFactory<XFactory>` needs `--include` of
  the factory classes).
- **P2R-402** The casts `'integer'` and `'int'`, `'string'`, and
  `'boolean'` and `'bool'` in `$casts` or `casts()` MUST give the column
  the Rust type `i64`, `String` or `bool`, whatever its type in the
  migrations (a date or a boolean column too), keeping its nullability,
  because Laravel applies a cast before its date handling. Every
  other cast (a date cast, `'encrypted'`, `'decimal:2'`, a class such as
  an enum or a custom cast) leaves the column out of the struct. A
  column `id` without a cast of its own reads as cast `'int'`, because
  Laravel casts the key of an incrementing model to its key type, which
  is `int` for every model that keeps a struct (`$table->uuid('id')`
  gives `i64`). Without a cast of its own, `created_at` and `updated_at`
  (unless `$timestamps` is `false`) and, in a model that uses
  `SoftDeletes`, `deleted_at` read as dates whatever the column method
  (`unsignedInteger('created_at')`), so they are left out.
- **P2R-403** Before Mago analyses the input, the tool MUST tell Mago each
  field of a model as a property that PHP reads through `__get`, as a
  `@property` tag in the model's docblock would, of the PHP type of the
  field's Rust type (`int`, `string` or `bool`, with `?` when nullable),
  replacing a tag of the model of the same name. A read of such a
  property then has that type for Mago and for P2R-11. A column that is
  left out is not told, so its read keeps the type Mago gives it
  otherwise, and stops by P2R-404.
- **P2R-404** A read `$m->column`, where `$m` has a model type and the
  column is a field of the struct, MUST convert as a property read of a
  readonly class does (P2R-134), and a model type converts as a readonly
  class type does (P2R-132 to P2R-136). These stop the conversion: a read
  of a name that is no field (a column left out included), and each read
  of a model without a struct (P2R-134); a write to an attribute (`=`, a
  compound assignment, `++` or `--`, `$m->c[] = ..`); `new` of a model,
  which creates no model in this section; and every method call on a
  model or static call of a model class (`$m->save()`, `M::create(..)`,
  `M::query()`), by P2R-135 and P2R-153, other than the forms of sections
  42, 43 and 45.

## 42. Creating and finding models (milestone 2, slice 32)

After section 41, 14 files of the app report one problem only, at an
action's `M::create([..])` or a query's `find($id)`, `where(..)->first()`
or `where(..)->sole()`; P2R-03 hides what else stops inside that call.
This section converts `create`, `find` and `where(..)->first()` against a
store that the output keeps in memory: one list of rows per model and
thread, empty when the thread starts, as a database that holds no rows
yet. The output connects to no database. Of the 14 files, two clear
(`GetContactQuery` and `GetDimensionCodeQuery`); the others stop on a
model without a struct, a `uuid` key that `create` cannot fill, a seeded
table or `sole()`. The store answers as an empty SQLite table without
constraints would, and these are stated limits, not checked against a
real database: it holds no row that a seeder, another process or an
earlier run inserts; it enforces no unique, foreign key or other constraint,
where a database would refuse the row; it compares strings byte for byte,
where MySQL's collation `utf8mb4_unicode_ci`, which the app sets, also
ignores case, accents and trailing spaces; and it keeps rows in creation
order, which SQL does not promise without `orderBy` (an index can change
the order SQLite gives). The converted code makes the same calls with the
same values as the PHP code. `sole()`, `update()`, `save()`, `delete()`,
relations and every other query method still stop.

- **P2R-410** A model with a struct (P2R-401) whose rows the input creates
  or finds MUST convert with an `impl` block after its struct. The block
  holds the model's store, a `thread_local!` list of rows inside a private
  function `store_rows`, and those of the public functions `store_create`,
  `store_find` and `store_first` that the forms below call, as decided
  over the whole input (a directory input included), and those of
  P2R-422 and P2R-441; it appears only for a model whose rows the input
  creates or finds, a form of section 43 or 45 included. The store of a thread is
  empty when the thread starts. The forms of P2R-411 to P2R-413 stop on a
  model, with the reason, unless all of these hold:
  - The struct has a field `id` of type `i64`, not nullable, whose column
    `id()` declares (an auto-increment key) and no cast changes, because
    the store numbers the rows as the database numbers them.
  - No other model for which the first and the last condition of this
    list hold reads the same table, because Laravel gives both models the
    rows of one table and the store keeps one list per model. The forms on
    a model for which one of them fails stop, so its rows never reach a
    store.
  - No migration writes rows into the table: a `DB::table('<t>')` call in
    a migration, `down()` aside, names `t` in any case, as SQLite matches
    it (a seed row that the store, empty when it starts, does not hold),
    and one whose argument is not a string literal of only letters, digits
    and `_` (`'roles as r'`, `'main.roles'`) may name any table, so it
    stops the forms on every model.
  - A model that uses `SoftDeletes` has a column `deleted_at` without a
    default that is no field, because Laravel's queries read it to skip
    deleted rows, and a default would mark each new row deleted.

  Laravel's `Model` declares neither `create` nor `find`: it forwards
  them through `__callStatic`, which Mago types as `mixed`. Before Mago
  analyses the input, when the included `Model` declares none of
  `create`, `find` and `findOrFail` (P2R-421), the
  tool MUST give it, as a patch of that vendored class would, the static
  methods `create(array<string, mixed> $attributes = []): static` and
  `find(int $id): ?static`, as `@method static` tags declare them, so a
  read of the result has the model's type.
- **P2R-411** `M::create([..])` and `M::query()->create([..])`, where `M`
  is a model with a struct and `query()` has no arguments, with one
  positional argument that is a literal map written `[..]` (`array(..)`
  stops), MUST convert to
  `M::store_create(M { .. })`, which adds the row and gives back its
  struct, when all of these hold:
  - Each key is a string literal that names a field of the struct other
    than `id`, at most once.
  - Each value has the field's PHP type (`int` for `i64`, `string` for
    `String`, `bool` for `bool`; for a nullable field also `null` or the
    nullable type), because Laravel keeps a created model's attribute as
    given and casts it only on a read (a string given for an int column
    reads as a string). A `String` field of a column that is not text (a
    `'string'` cast of an int, boolean, decimal or date column) takes no
    value, because SQLite stores it by the column's type (`'1.0'` reads
    back as `'1'`); nor does an `i64` field of a `float(..)` or
    `double(..)` column, which SQLite stores as REAL (2^53 + 1 reads back
    as 2^53).
  - Each field other than `id` is named, except a nullable field whose
    column has no default, which gets null: for a column with a default
    the database stores the default while the created model reads null.
    `default(..)` with a value other than null, and `useCurrent()` or
    `useCurrent(true)` after `date(..)`, `dateTime(..)`, `timestamp(..)`
    or `softDeletes(..)`, also over a `default(..)`, give a default
    (Laravel writes no default for `useCurrent()` on another column);
    `default(null)` (also written `\null` or `(null)`), `useCurrent(false)`,
    `useCurrentOnUpdate()` and a
    `default(..)` after `constrained(..)`, which belongs to the foreign
    key, give none, and `change()` keeps only the default it writes again.
    The last `default(..)` and the last `useCurrent(..)` of a column
    count, and `useCurrent(false)` removes no `default(..)`.
  - The table has no column that is neither nullable nor has a default
    and is not a field (a date or decimal column), other than `id`, and
    `created_at` and `updated_at` while the model keeps its timestamps,
    because the database would refuse the row.
  - While the model keeps its timestamps, the table has `created_at` and
    `updated_at` and neither is a field, because Laravel writes both on
    every create, also into a field that a cast made of one.
  - Each key is fillable. A `$fillable` that is not a literal list of
    string literals (a keyed array, a constant) stops `create`. When
    `$fillable` holds at least one key, it holds the key; otherwise the
    model declares `$guarded = []` and the key starts with no `_` and holds
    no `.`. Laravel throws `MassAssignmentException` for a fully guarded
    model, and for another key in strict mode (`Model::shouldBeStrict()`,
    which the app sets), and drops that key otherwise.

  The values are computed in the order the map gives them. The new row's
  `id` is one more than the number of rows the store holds, as an empty
  table's auto-increment gives.
- **P2R-412** `M::find($id)` and `M::query()->find($id)`, where `query()`
  has no arguments, with one positional argument of type `int`, MUST
  convert to `M::store_find(<id>)`,
  which gives a copy of the row with that `id` as `Option<M>`, or `None`
  when the store holds none.
- **P2R-413** `M::query()` without arguments, followed by one or more
  `->where('<c>', $v)` and then `->first()` without arguments (with a list
  of columns Laravel loads only those), MUST convert to
  `M::store_first(|row: &M| ..)`, a closure that compares a row's fields
  with the values, which gives a copy of the first row in creation order
  that matches every `where`, as `Option<M>`. The closure's own names
  (`row`, and `text` for a nullable `String` field) take the first suffix
  `_1`, `_2` and so on that is no word of a value's Rust form (a
  variable, a field, or a word of a string literal), so a value never
  reads the row in place of a variable of the same name. Each `where` has two positional arguments: a string literal
  that names a field of the struct, and a value of the field's PHP type
  that is a variable, a literal (a negative number included) or a property
  read of one, so it reads the same each time the closure runs, once per
  row; for a nullable field the
  value may also be the literal `null`, which matches a row whose field is
  null, as Laravel turns `where('c', null)` into `whereNull('c')`. A value
  of a nullable type, and `null` for a field that is not nullable, stop.
  A `String` field of a column that is not text, and an `i64` field of a
  REAL column, stop, as in P2R-411. Review concern, not checked
  mechanically: the included `Model::query()` gives back Eloquent's
  builder itself, as Laravel's does; an included subclass with its own
  `where` would filter otherwise (P2R-451 checks this for `when()`).
- **P2R-414** These stop, by P2R-135 and P2R-153: any other call on a
  model, a model class or a query (`firstOrCreate(..)`,
  `orderBy(..)`, `where` with an operator or a closure, `update(..)`,
  `save()`); these forms on a model without a struct; `find` with an
  argument of another type (`find('2')`, which Laravel accepts as well);
  a named or unpacked argument (`where('c', value: $v)`, which Laravel
  reads as `whereNull('c')`); a `create` or `where` that P2R-411 or
  P2R-413 refuses, with a message that names the condition at the call,
  key or value it concerns; and a query held in a variable (`$q =
  M::query();`), because each form is one whole chain. Section 43 adds
  `sole()`, `firstOrFail()` and `findOrFail()`, section 45 `get()` and
  `all()`, and section 46 `when()` links.

## 43. A missing or doubled row as Laravel's exceptions (milestone 2, slice 33)

After section 42, two files of the app (`GetDimensionCodeByCodeQuery`
and `GetDimensionCodeByNameQuery`) stop only on `where(..)->sole()`, and
the app calls `sole()` 8 times, `firstOrFail()` 11 times and
`findOrFail()` 12 times.
Laravel 12.35.1 answers these from the rows it reads: Eloquent's `sole()`
reads at most two matching rows and throws `ModelNotFoundException` for
none and `MultipleRecordsFoundException` for two, and Eloquent's
`firstOrFail()` and `findOrFail()` throw `ModelNotFoundException` for
none (a `Collection`'s `firstOrFail()` throws another class and stops).
Section 42's limit on comparing strings byte for byte now also decides
which exception `sole()` throws: under MySQL's collation `'abc'` also
matches `'ABC'`, so the database can find two rows where the store finds
one. This section
converts the three forms against the store of section 42 and throws
these exceptions as section 22 throws its own (P2R-211 to P2R-214), so a
function that calls one throws, and a `catch` of those classes catches
them.

- **P2R-420** The output's exception struct (P2R-210) MUST hold three of
  Laravel's classes in the class tree of `is_a` when a form of P2R-421
  converts or a `catch` clause names one of them, and not otherwise:
  `Illuminate\Database\RecordsNotFoundException` under
  `RuntimeException`, `Illuminate\Database\Eloquent\ModelNotFoundException`
  under `Illuminate\Database\RecordsNotFoundException`, and
  `Illuminate\Database\MultipleRecordsFoundException` under
  `RuntimeException`, each written by its fully qualified name as P2R-390
  writes the name of an exception class of the input, after the classes
  of P2R-210 and the input's own under the same parent. A `catch` clause
  whose class PHP resolves to one of the three MUST test
  `e.is_a("<name>")` (P2R-214), and `$e->getMessage()` and
  `$e->getCode()` read its message and code (`0`). `throw new` of one of
  the three stops, as `new` of another class does (P2R-212), because
  `MultipleRecordsFoundException` builds its message from a count and
  Laravel names the model of a `ModelNotFoundException` with
  `setModel(..)`, which the struct does not hold; `RecordsNotFoundException`
  stops with them, although it takes `RuntimeException`'s arguments,
  because no form of P2R-421 throws it.
- **P2R-421** These forms on a model whose store applies (P2R-410) MUST
  convert to a call of the store that throws (P2R-211), converted by
  P2R-213 (`?` in a function that throws, `unwrap_or_else(|e|
  e.uncaught())` in `fn main()`, a `break 'throw` in a `try` body). For
  P2R-211, P2R-213 and P2R-214 each form on a model with a struct (the
  `where` chain, with its `when` links (P2R-451), ended by `sole()` or
  `firstOrFail()` without arguments,
  or `M::findOrFail(..)` and `M::query()->findOrFail(..)` with any
  arguments) is a call that throws, also where
  it then stops on the store, an argument or a `where` value:
  - The `where` chain of P2R-413 ended by `->sole()` without arguments
    converts to `M::store_sole(|row: &M| ..)`, with the closure of
    P2R-413, which gives a copy of the one matching row as `M`. With no
    matching row it throws `ModelNotFoundException` with the message
    `No query results for model [<M>].`, and with two or more it throws
    `MultipleRecordsFoundException` with the message
    `2 records were found.`, because Laravel reads at most two rows.
  - The same chain ended by `->firstOrFail()` without arguments converts
    to `M::store_first_or_fail(|row: &M| ..)`, which gives a copy of the
    first matching row in creation order as `M`, and with none throws
    `ModelNotFoundException` with the message
    `No query results for model [<M>].`.
  - `M::findOrFail($id)` and `M::query()->findOrFail($id)`, where
    `query()` has no arguments, with one positional argument of type
    `int`, convert to `M::store_find_or_fail(<id>)`, which gives a copy of
    the row with that `id` as `M`, and with none throws
    `ModelNotFoundException` with the message
    `No query results for model [<M>] <id>`, the `id` in decimal and with
    no full stop.

  `<M>` is the model's fully qualified PHP name as declared, in that
  letter case and without the leading backslash, as Laravel's
  `get_class()` gives it; every exception's code is `0`. The rules of
  P2R-413 on the `where` values, and of P2R-412 on the `id`, apply
  unchanged. Laravel's `Model` declares no `findOrFail`, so the patch of
  P2R-410 MUST also give it `findOrFail(int $id): static`, as a
  `@method static` tag declares it, when the included `Model` declares
  none of `create`, `find` and `findOrFail`.
- **P2R-422** The `impl` block of P2R-410 MUST hold `store_sole`,
  `store_first_or_fail` and `store_find_or_fail` only when the input's
  code calls them, each returning `Result<M, PhpException>` and reading
  the store as `store_first` and `store_find` do; a model's store whose
  only uses are these forms holds no other public function. A form of
  P2R-421 makes the output hold the struct of P2R-210, with the three
  classes of P2R-420, even where no `throw` or `catch` converts.
- **P2R-423** These stop, by P2R-135 and P2R-153, or by Mago's finding
  (P2R-11) where Laravel's `__callStatic` gives `mixed`: `sole()` or
  `firstOrFail()` with an argument (a list of columns, which Laravel
  loads alone); `findOrFail` with an argument of another type, a second
  argument, or a named or unpacked argument; `sole()` or `firstOrFail()`
  after `query()` without a `where` or `when` link, or after a `where`
  called on the model class without `query()`
  (`M::where(..)->firstOrFail()`), and `where(..)->findOrFail(..)` and
  `when(..)->findOrFail(..)`; these forms on a model whose store
  does not apply, with the reason of P2R-410; and these forms where a
  call that throws cannot stand (P2R-213), such as the default of `??`.

## 44. Unions as fields of array shapes (milestone 2, slice 34)

Two Data classes of the app (`SetAdministrationConfigurationValuesDto`
and `SetAccountingSoftwareEnvironmentConfigurationValuesDto`) declare
`array<array-key, array{key: string, value: string|bool|int}>`: a map of
shapes whose `value` field is a union, which P2R-230 and P2R-250 stop.
This section gives such a field the union enum of P2R-230, so a shape
holds a union as a class property does, and a read of the field narrows
by the tests of section 26. A union as a list element type, or as a
member of another union, still stops.

- **P2R-430** A field of a shape (P2R-200) whose value type has two or
  more members, other than `null`, of different Rust types MUST take the
  union enum `U` of P2R-230 for that value type as its Rust type, or
  `Option<U>` with a `null` member, when a declared type (P2R-230) names
  the shape: as the whole type, as a member of a union, as the value type
  of a declared map (P2R-250), as the element type of a declared list, or
  at any depth inside such a type (a field, the element of a list field,
  the value of a map field). The members, the variant names,
  the name of `U` and the stops of P2R-230 apply to the field's value
  type, and so does P2R-200's stop of a value type that is or holds a
  class, for each member (`array{at: Point|int}` stops); a field whose
  type is a map with union values, or a list, a map or a `null` union
  that holds one, stops (P2R-250), while a union field may hold such a
  map as its map member (P2R-270). A
  shape that a declared type names takes its own field types, so two
  declared shapes of one Rust name with different field types stop by
  P2R-200, whichever the tool meets first. Any other shape type with
  the keys of such a shape, each of whose field types fits the struct's
  field (for a union field, the Rust type (P2R-21) of each member of its
  Mago type a member of `U`, `null` only for `Option<U>`; for any other
  field, the same Rust type after P2R-21, or `T` or `null` for an
  `Option<T>` field, as in P2R-200's exception for `?T`), maps to the
  same struct: the type Mago gives a read or a copy of it (a variable, a
  parameter, a `foreach` value, a list element) and the type it gives an
  array literal in a local variable, so a field type that Mago narrows
  keeps `U`, as a read of a union property keeps it (P2R-232), and a
  literal's value goes into the field by P2R-431. A shape with a union
  field whose keys no such shape has keeps P2R-22.
- **P2R-431** A value for such a field in a shape literal (P2R-202) MUST
  convert as a value into a place of type `U` or `Option<U>` (P2R-231),
  `U::V(value)` for a member value, and a read `$s['k']` of the field
  (P2R-203) gives a value of type `U` or `Option<U>` that behaves as a
  read of a union property (P2R-232): it copies with `.clone()`, lends as
  `&U` (an `Option<U>` field as `Option<&U>` with `.as_ref()`, P2R-204),
  goes into a place of its own union, and takes the tests of
  P2R-251; a variable that holds it takes the binding of P2R-252 and the
  calls of P2R-233, which stop on the read itself as on a property read.
  Every other use stops by P2R-232, as for any union value.

## 45. Laravel's collections as lists (milestone 2, slice 35)

42 files of the app import `Illuminate\Support\Collection` or
`Illuminate\Database\Eloquent\Collection`, and 27 call a query's
`get()` (26 without arguments), which gives an Eloquent collection of
models. Parameters,
returns and interface methods carry such types, and each stops by P2R-22
today. This section maps a collection to a Rust `Vec` of its elements,
gives `get()` and `M::all()` the store of section 42, and converts the
uses that read the elements in order and nothing else. A collection is an
object that a caller and a callee share, which a `Vec` copy is not, so
every use that could change it stops, and so does every use that reads
its keys or treats it as a PHP array, because PHP reads an object
otherwise than an array (`empty($c)` is false for an empty collection).

- **P2R-440** `Illuminate\Support\Collection<K, T>` and
  `Illuminate\Database\Eloquent\Collection<K, T>`, as a declared type
  or as the type Mago gives a value, where the class is the vendored one
  (P2R-340) and, for Eloquent's class, so is its parent
  `Illuminate\Support\Collection`, from which it inherits the methods of
  P2R-442, MUST map to `Vec<T>` when `list<T>` has a Rust type
  (P2R-21) and `T` is neither nullable, nor `DateTime|false` (whose Rust
  type `Option<PhpDateTime>` would make `first()` give two `Option`s,
  P2R-330), nor a changing type (P2R-141), whatever `K` is, because no
  use below reads a key. Such a type is a
  collection type below, and a value of it a collection value. A
  collection type is no list type for any rule but these: a parameter of
  a collection type becomes `&[T]`, as a list parameter does (P2R-24),
  and works on a copy when the function assigns to it (P2R-181); an
  assignment, a return, an argument to a parameter of a collection type
  and a property copy or lend a collection value as they do a list
  (P2R-26); and the forms of P2R-442. These stop by P2R-22: a collection
  type with another element type; one without type arguments in its
  docblock, whose elements Mago gives as `mixed` (or as `Model` for the
  Eloquent class); one with a single type argument (`Collection<Item>`,
  which Mago reads as the key type); a collection type that is nullable,
  in a union, or inside a list, a map, a shape or another collection; and
  either class without the vendored class.
- **P2R-441** `M::query()` without arguments, followed by zero or more
  `->where('<c>', $v)` and `when` links (P2R-451) and then `->get()`
  without arguments, and `M::all()` without arguments, where `M` is a
  model whose store applies (P2R-410), MUST convert to
  `M::store_get(|row: &M| ..)`, with the closure and the conditions of
  P2R-413 for each `where` and of P2R-452 for each `when` link, or to
  `M::store_all()` when no `where` or `when` link comes before `get()`. Both give copies
  of every matching row in creation order, as `Vec<M>`, and the `impl`
  block of P2R-410 holds each only when a form calls it. The order is the
  limit that section 42 states: an index whose columns start with a
  `where` column can make SQLite give another order. `get([..])` and
  `all([..])` with a list of columns stop, because Laravel then loads only
  those columns, and so does a `where` that P2R-413 refuses. These forms
  also stop when Eloquent's collection class maps to no `Vec` by P2R-440
  (the input declares it or its parent), and on a model whose store does
  not apply, with the reason of P2R-410. `M::get()` and `M::where(..)->get()` stop, by P2R-153 or on
  Mago's finding (P2R-11), because `__callStatic` forwards them and Mago
  gives `mixed`.
- **P2R-442** On a collection value `$c`, these MUST convert, in the
  collection's order:
  - `foreach ($c as $x)` without a key, as a `foreach` over a list
    (P2R-41);
  - `count($c)` and `$c->count()` as `c.len() as i64`, the forms of
    `count()` on a list (P2R-48);
  - `$c->isEmpty()` as `c.is_empty()`, and `$c->isNotEmpty()` as
    `!c.is_empty()`;
  - `$c->first()` without arguments as a copy of the first element, or
    `None` for an empty collection, of type `Option<T>` whatever Mago
    narrows it to (P2R-102) (`c.first().cloned()`), as Laravel's
    `first()` gives `null` then; for a collection of `DateTime` it stops,
    because `Option<PhpDateTime>` stands for `DateTime|false` (P2R-330).
- **P2R-443** Every other use of a collection value stops, by P2R-135,
  P2R-153 or the rule that names it: any other method (`map`, `filter`,
  `pluck`, `push`, `each`, `toArray`, `all`, `keys`, `where`, `first`
  with a callback or a default), `foreach` with a key (Laravel keeps the
  keys a collection had) or by reference, a condition on it and `empty()`
  (PHP reads an object as true), a comparison, `gettype()` and
  `is_scalar()` (P2R-222; PHP gives `"object"`), `$c[..]` and
  `$c[] = ..` (Laravel's `ArrayAccess`), a property read on it (`$c->x`,
  or a Laravel proxy such as `$c->sum`), a collection value where a PHP
  array is needed and an array where a collection is needed (Mago reports
  most of these), `isEmpty()`, `isNotEmpty()` and `first()` as an
  expression statement (P2R-43), whose unused Rust result rustc refuses,
  and the forms that make a collection other than P2R-441: `collect(..)`,
  `new Collection(..)` and the static methods of both classes. A
  collection value therefore comes only from P2R-441, a parameter, a
  property, a variable that holds one, or a call of the input that
  returns one.

## 46. Conditional filters with when() (milestone 2, slice 36)

Seven list queries of the app build their filters as
`M::query()->when($value, function (Builder $query, $value): void {
$query->where('c', $value); })->get()`, and Mago types each chain as
`mixed`: Laravel's `when()` returns `$this|TWhenReturnType`, and a
callback without a return value gives that type no meaning. A throwaway
spike that told Mago `when()` returns `$this` and let such links through
the store chain cleared four of the seven files (the other three read
models without a store) and made no other file stop. With the checks
of this section two of the four still stop: `ListAccountQuery` compares
an int with its string column `number` (P2R-413), and
`ListJournalLineDimensionQuery` reads `JournalLine`, which takes no
struct (P2R-401). This section tells Mago that type and converts the
links to conditions of the store.

- **P2R-450** Before Mago analyses the input, when the input declares a
  model (P2R-401) and an included file, never the input (P2R-340),
  declares Laravel's trait
  `Illuminate\Support\Traits\Conditionable` with a method `when`, the
  tool MUST give Mago, as a patch of that trait would, a `when()` that
  keeps Laravel's parameters and its template `TWhenParameter`, so a
  callback's second parameter takes the value's type, and that returns
  `$this`. Laravel's `when()` returns the callback's result, or `$this`
  when that result is `null`; for the callbacks of P2R-451 (no return
  value, or the builder that `$query->where(..)` gives back) that is the
  same builder. Every other use of `when()` stops (P2R-135, P2R-153, the
  rules of section 45 on a collection, or a finding Mago gives, P2R-11,
  such as on Spatie's `QueryBuilder`, whose `@mixin` makes `$this` the
  `QueryBuilder`), so the told type lets no other form convert.
- **P2R-451** In the `where` chain of P2R-413, of P2R-421 (ended by
  `sole()` or `firstOrFail()`) or of P2R-441, a link
  `->when($value, <callback>)` MAY stand anywhere between `M::query()` and
  the last call, among the `where` links; `first()`, `sole()` and
  `firstOrFail()` then need one or more `where` or `when` links. These
  links convert only where P2R-450 told Mago the type of `when()` and
  Eloquent's builder and every subclass of it take `when()` from that
  trait, not from a method one of them declares (which may treat the
  value otherwise; an included `query()` may give back a subclass where
  Mago sees the builder). Such a
  link converts when all of these hold:
  - It has two positional arguments, neither named nor unpacked.
  - `$value` is a variable, a parameter or a property read of such a
    value (`$filter->name`, `$this->filter->name`; it is read once per
    row, as a `where` value is, P2R-413), of a type that
    P2R-452 gives a truthiness rule; a closure as `$value` stops,
    because Laravel calls it.
  - The callback is a closure `function (Builder $query, T $value): void
    { .. }`, with or without `static` and with `: void` or no return
    type, whose body is one or more statements, each a chain
    `$query->where('<c>', <v>)` of one or more `where` calls on its first
    parameter, or an arrow function
    `fn (Builder $query, T $value) => $query->where('<c>', <v>)`, with or
    without `static` and without a return type, whose body is one such
    chain. The callback has one or two parameters, none by reference,
    variadic, with a default, a modifier or an attribute, no two of one
    name and none named `$this` or like a superglobal (PHP rejects
    these, and Mago does not report them), and no `use` list; the first
    parameter is untyped or Eloquent's `Builder`, and the second, which
    may be left out, is untyped or written as the PHP type of `$value`
    without `null`: `int`, `string`, `bool`, the class or the enum (not
    `?string` or `true`, which this form leaves out, nor `false`, which
    PHP refuses with a `TypeError` for every value Laravel passes).
  - Each `where` follows P2R-413, except that its value is a literal, the
    callback's second parameter, or, when that parameter is a model or a
    struct, a property read of it (`$administration->id`), which stands
    for `$value` without `null` (for an `Option`, the binding of
    P2R-452). An enum case's `->value` or `->name` stops (a later
    slice), and so does a property read of a model that takes no struct
    (P2R-401).
- **P2R-452** Each `when` link MUST convert to one more condition of the
  store closure of P2R-413, joined with `&&` in source order: a row
  passes when `$value` is falsy by PHP's rules, or when every `where` of
  the callback holds with the second parameter bound to `$value`. Falsy
  are `null`, `false`, `0`, `""`, `"0"` and an empty list; any other int,
  string, list, model, struct or enum case is truthy. An `Option<T>`
  value converts to `v.is_none_or(|b| <falsy> || <conditions>)`, with
  `v` lent as an argument is (P2R-101, P2R-204), and any other value to
  `<falsy> || <conditions>` with the value in place of `b`; for a model,
  struct or enum case `<falsy>` and its `||` are left out. The binding
  `b` has the Rust name of the callback's second parameter (P2R-30, with
  `r#` for a keyword), or `value` when the callback has one parameter or
  P2R-30 gives the second no Rust name (`$self`, `$_`), which PHP
  accepts; it takes the first suffix `_1`, `_2` and so on (after the
  name without `r#`) that is no word of a value's Rust form (`r#type`
  is the word `type`) and neither of the closure's names `row` and
  `text` of P2R-413; and it is `_` when
  nothing reads it (a model, struct or enum case with no `where` whose
  value reads the second parameter). The callback's parameters are no
  variables of the enclosing function for P2R-30, P2R-31 and P2R-160:
  the first has no Rust form, and the binding takes its name by this
  rule alone. Because the binding avoids every word the conditions read,
  a function they call included, it can hide only a variable or function
  that nothing inside the closure reads; the idiom
  `->when($name, fn ($q, $name) => ..)` names the binding after the
  value on purpose.
- **P2R-453** These stop the conversion: `when()` with one or three
  arguments (Laravel then gives a proxy or calls a default callback), a
  named or unpacked argument, `unless()`, a value of another type (a
  float, because PHP 8.5 warns when it tests `NAN` for truth, a map, a
  shape, a union, a collection) or another form, a model, struct or
  enum case without `null` as the value with no `where` whose value
  reads the second parameter (it is always truthy, and Rust would read
  it nowhere), a
  `when` link before `find`, `findOrFail` or `create` (P2R-153), and a
  callback of any other form: another statement or expression in the
  body, a `return`, another return type, a `where` with an operator or
  a closure, a call on another variable, a value that reads any other
  variable (`$this` included), a parameter of another form, or a
  written parameter type of another class.

## 47. Observed models (milestone 2, slice 37)

Six models of the app (`Journal`, `JournalLine`, `JournalFileLine`,
`SupplierDetail`, `CustomerDetail` and `FileDetail`) carry
`#[ObservedBy([XObserver::class])]`, so P2R-401 gives them no struct and
every read of them stops (P2R-404). Their observers declare only
`created`, `updated` and `deleted`. Laravel 12.35.1 registers the
observers when it boots the model on its first use, an empty table
included: it calls `class_exists` on each, which loads the observer's
file with its parents, interfaces and traits and throws on the first use
when one does not exist, and for each event that `getObservableEvents()`
lists it hooks the observer's method of that name when `method_exists`
finds one, in any letter case and also when the observer inherits it or
takes it from a trait. Of these events only `retrieved` fires when a
model is read, once per loaded row, and `saving`, `creating`, `created`
and `saved` fire on `create()`. Laravel merges a parent's attribute only for a grandchild
of `Model`, which P2R-401 does not admit. A throwaway spike that let the
attribute through cleared five files and made no other file stop. This
section gives such a model its struct and keeps every form that would
run an observer method stopped.

- **P2R-460** A model (P2R-401) MAY carry one or more attributes
  `#[ObservedBy(<arg>)]` whose name PHP resolves to
  `\Illuminate\Database\Eloquent\Attributes\ObservedBy`, a class that a
  file `--include` reads declares (P2R-340), where `<arg>` is one
  positional argument: `X::class`, or an array literal `[..]` of one or
  more `X::class` without keys, each naming a class (not an interface,
  trait or enum) that the input or an included file declares together
  with its parent classes, interfaces and traits at any depth, none of
  them built into PHP, a class, interface or trait alike (Mago's stubs also name classes of
  extensions that PHP may not load, such as `\Redis`, and Laravel then
  throws on the first use), as PHP
  loads them when Laravel boots the model (Mago reports a missing one in
  the input, but not in an included file). The named classes are the
  model's observers. Such a model takes its struct by P2R-401 as if the
  attributes were absent. Any other attribute on the model, an
  `ObservedBy` that the input declares, another argument form (a string,
  a named or unpacked argument, `self::class`, `parent::class`,
  `static::class`, which PHP rejects in an attribute, a keyed entry,
  `array(..)`, an empty list, a variable, a second argument) or a class
  that fails the rule keeps P2R-401's stop, whose message names the
  admitted form.
- **P2R-461** On a model with observers, `create` (P2R-411) MUST stop,
  because Laravel runs the observers' `creating`, `created`, `saving`
  and `saved` methods, which the store does not run. Every read form of
  sections 42 to 46 (`find`, `first`, `sole`, `firstOrFail`,
  `findOrFail`, `get` and `all`, with their `where` and `when` links)
  MUST stop when an observer has a method named `retrieved` in any letter
  case, declared, inherited or taken from a trait, at any visibility, as
  Mago's method lookup finds it, because Laravel runs it on every row it
  reads. That lookup also counts a `@method` tag and a private method of
  a parent, which PHP's `method_exists` does not, so the tool stops there
  too, which gives no wrong output. The stop names the observer.
  Otherwise these reads convert as on a model without observers,
  and a read of a field of such a model's struct is a plain field read
  (no event fires).
- **P2R-462** These stop the conversion, each with a message that names
  the reason: `create` on a model with observers, a read form on a model
  whose observer has `retrieved`, and the forms of `#[ObservedBy]` that
  P2R-460 does not admit (by P2R-401's stop of an attribute). Observers
  that a service provider registers (`M::observe(..)`) remain the review
  concern of P2R-410; the top-level code of an included observer's file,
  and an included observer that PHP refuses to declare for a reason
  other than a missing name, are review concerns of P2R-460.

## 48. Relation methods on models (milestone 2, slice 38)

Most models of the app declare relation methods such as
`public function journalLines(): HasMany { return
$this->hasMany(JournalLine::class); }`, and P2R-401 stops each one with
"a method of a model is not supported yet", although the method leaves
the struct. Laravel 12.35.1's `getAttribute` returns a column only when
the model holds it (or casts it); after `create`, a nullable column the
map leaves out is not held, and `getAttribute` then calls a method of
that name in any letter case as a relation. Only a relation method named
like a column can therefore change a column read. A throwaway spike that let such
methods through cleared twelve model files when the factories are
included (`--include database/factories`) and made no other file stop;
under the one `return` of P2R-470 nine clear, because three chain a call
such as `->withTimestamps()`. This section leaves a relation method out of the output, as P2R-401
leaves out `casts()`, while every use of the relation keeps stopping.

- **P2R-470** A method of a model (P2R-401) that P2R-401 lets keep the
  struct (no `get..Attribute` or `set..Attribute` name, no return type
  that names `Casts\Attribute`, no `boot` or `initialize` prefix, no
  attribute other than `#[\Override]` and no override of a method of
  `Model` or of a trait it uses, each name in any letter case) MAY be a
  relation method and MUST then be left out of the output when it meets
  each of these conditions, which are checked in this order:
  1. its name does not start with `__` (PHP gives such names their own
     rules, `__clone(): HasMany` fails to compile, and Mago does not
     report it);
  2. its name equals no column of the model's table in any letter case;
  3. it has no parameters;
  4. its only modifier is `public` (a method without a modifier fails;
     PHP rejects `readonly`, a set visibility or a doubled modifier on a
     method, and Mago does not report them);
  5. it has no attribute (`#[\Override]` included);
  6. it does not return by reference;
  7. its return type is not nullable, a union or an intersection;
  8. it declares a return type that PHP resolves to `BelongsTo`,
     `HasMany`, `HasOne`, `BelongsToMany`, `HasManyThrough`,
     `HasOneThrough`, `MorphTo`, `MorphMany` or `MorphOne` of
     `\Illuminate\Database\Eloquent\Relations`, declared in a file that
     `--include` reads (P2R-340);
  9. its body is one statement `return $this-><name>(..);`;
  10. `<name>` is the helper of that type, written in exactly that letter
      case: `belongsTo` for `BelongsTo`, `hasMany` for `HasMany`, `hasOne`
      for `HasOne`, `belongsToMany` for `BelongsToMany`, `hasManyThrough`
      for `HasManyThrough`, `hasOneThrough` for `HasOneThrough`, `morphTo`
      for `MorphTo`, `morphMany` for `MorphMany` and `morphOne` for
      `MorphOne`;
  11. its arguments are positional and not unpacked, each a string
      literal or `X::class` naming a class (`self::class`,
      `static::class`, `parent::class`, a constant and a call stop).
- **P2R-471** A call of a relation method (`$m->journalLines()`), a read
  of a relation as a property (`$m->journalLines`, which has no field of
  the struct, P2R-134) and every other use of the relation MUST still
  stop. A method of another form keeps P2R-401's stop.
- **P2R-472** A method of a model that P2R-401 lets keep the struct,
  other than `getRouteKeyName`, whose declared return type names a class
  of `\Illuminate\Database\Eloquent\Relations` (also inside `?`, a union
  or an intersection) or whose body is one `return $this-><name>(..);`
  (a plain call such as `return $this->formatName();` counts), and that
  P2R-470 does not admit, is a relation method that stops with a message
  that names the first condition of P2R-470 it fails: for example a
  relation named like a column (after `create` Laravel reads the
  relation in place of a column the map left out), a method without a
  modifier (condition 4), without a return type or with the return type
  `MorphToMany`, `PendingHasThroughRelationship` or a class the input
  declares (condition 8), a chained call such as `->withTimestamps()`
  (condition 9), or the helper `morphToMany`, `morphedByMany` or
  `through` (condition 10). A method that P2R-401 does not let keep the
  struct, `getRouteKeyName` and every other method keep P2R-401's rule
  and message.

## Spec checklist

- [x] No `[NEEDS CLARIFICATION]` markers left. Decided in review: problems
  inside or equal to a reported span are not reported (P2R-03); exit 2 wins
  over exit 1 (P2R-04); comments are not copied in version 1 (P2R-52); `[]`
  takes its element type from the same variable or the declared type it
  flows into (P2R-21); a file of only functions becomes a library (P2R-33).
- [x] Every requirement is testable: P2R-07 to P2R-49 and P2R-100 to
  P2R-472 by case and stop folders (P2R-75 enforces this), P2R-01 to P2R-06
  by error folders and tests, P2R-50 to P2R-52 by P2R-71, the rest by CI or
  a file in the
  repository. Stated review concerns (P2R-05 partial write, P2R-72 case
  coverage, P2R-176 calls between files checked by rustc only, and in
  P2R-340 the dropped Mago findings in an included file, which no known
  input reaches, and which of two included declarations of one name
  wins, which no output shows, and in P2R-361 that the included
  `create()` returns a new `Optional` and does nothing else, as 4.18.0's
  does, which Mago's metadata does not show, and in P2R-440 that the
  included collection classes behave as Laravel 12.35.1's do in
  `count()`, `isEmpty()`, `isNotEmpty()`, `first()` and iteration, with
  the element type as their second template parameter, which the tool
  does not check, and in P2R-380 a PHP
  compile-time error, deprecation or warning in the body of a left-out
  method that Mago does not report and P2R-380 does not list, such as
  `yield` in a method that returns `array`, `unset($this)`, a closure
  parameter named `$this` or with a default that names `static`, a
  closure parameter named like a variable of its `use` list, `continue`
  that targets a `switch`, `goto` into a loop, `[] = [1]`, an
  assignment to `$GLOBALS`, a static variable
  declared twice or an attribute that PHP
  rejects there, because the tool does not read that body, and in P2R-400
  a macro of the query builder in a chain on `DB::table(..)`, whose body
  the tool does not read, a Carbon macro on a `now()` chain, a callable
  passed as a string or a class's array into such a chain, in P2R-410
  observers, listeners or global scopes that a service provider
  registers on a model (`M::observe(..)`, `addGlobalScope(..)`) and
  `Model::unguard()` there, which change what `create`, `find`,
  `first`, `sole`, `firstOrFail`, `findOrFail`, `get` and `all` give, in
  P2R-460 the top-level code of an included observer's file, which PHP
  runs when Laravel boots the model and loads it, and an included
  observer that PHP refuses to declare for a reason other than a missing
  name (a class that extends an interface or a final class, leaves a
  method abstract or inherits in a cycle), which Mago reports only in
  the input, a relation that
  `Model::resolveRelationUsing(..)` registers under one of these names,
  which `Model::__call` tries before it forwards to the query, and a schema
  dump or package migrations that
  create tables the tool does not see, and in P2R-413 that the included
  `Model::query()` gives back Eloquent's builder itself, as Laravel's
  does, whose `where` a subclass could declare again) are the only
  parts not checked mechanically.
- [x] No speculative features: later milestones carry no requirements yet.
