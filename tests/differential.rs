//! The differential harness (P2R-600 to P2R-609, ADR 0010): each fixture under
//! tests/differential runs once on PHP and once on the Rust that php2rust writes, from one
//! JSON file, and PHP is the oracle.
//!
//! `PHP2RUST_FIXTURE=<text>` keeps the fixtures whose file name holds the text, and
//! `PHP2RUST_RECORD=1 cargo test --test differential golden` writes the golden files from
//! PHP, for review before they are committed.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::OnceLock;

use regex::Regex;
use serde_json::{Map, Value};

// No `float`: php2rust converts no float to text, so the Rust driver could not print one
// (P2R-602).
const TYPES: [&str; 7] = [
    "int",
    "string",
    "bool",
    "?int",
    "?string",
    "list<int>",
    "list<string>",
];

const VERSIONS: [&str; 4] = ["8.3", "8.4", "8.5", "8.6"];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn harness() -> PathBuf {
    root().join("tests/differential")
}

fn vendor() -> PathBuf {
    harness().join("laravel/vendor")
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// Writes to stderr past the test runner, which keeps `eprintln!` of a passing test to
/// itself, so record mode's lines and the skip line show without `--nocapture`.
fn note(line: &str) {
    let _ = std::io::stderr().write_all(line.as_bytes());
}

/// The PHP binary: `PHP2RUST_PHP`, or `php` on PATH (P2R-72).
fn php() -> String {
    std::env::var("PHP2RUST_PHP").unwrap_or_else(|_| "php".to_owned())
}

/// The version of that binary as `--php` names it, `8.5` for PHP 8.5.13.
fn php_version() -> &'static str {
    static VERSION: OnceLock<String> = OnceLock::new();
    VERSION.get_or_init(|| {
        let full = text(&run(&php(), &["-n", "-r", "echo PHP_VERSION;"], &root()).stdout);
        full.splitn(3, '.').take(2).collect::<Vec<_>>().join(".")
    })
}

/// The flags of every PHP run, those of the folders' P2R-72 run.
fn php_flags() -> Vec<String> {
    static FLAGS: OnceLock<Vec<String>> = OnceLock::new();
    FLAGS
        .get_or_init(|| {
            let mut flags: Vec<String> = [
                "-n",
                "-d",
                "display_errors=stderr",
                "-d",
                "error_reporting=E_ALL",
            ]
            .map(str::to_owned)
            .to_vec();
            let probe = [
                "-n",
                "-r",
                "echo function_exists('ctype_digit') ? 'yes' : 'no';",
            ];
            if run(&php(), &probe, &root()).stdout != b"yes" {
                flags.extend(["-d".to_owned(), "extension=ctype".to_owned()]);
            }
            flags
        })
        .clone()
}

fn run(program: &str, args: &[&str], dir: &Path) -> Output {
    Command::new(program)
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap_or_else(|err| panic!("cannot run {program}: {err}"))
}

/// The requirement IDs that docs/spec.md defines, as a fixture name starts with them
/// (`p2r-10`, `p2r-410`).
fn requirement_ids() -> &'static BTreeSet<String> {
    static IDS: OnceLock<BTreeSet<String>> = OnceLock::new();
    IDS.get_or_init(|| {
        let spec = fs::read_to_string(root().join("docs/spec.md")).expect("docs/spec.md");
        spec.split("**P2R-")
            .skip(1)
            .filter_map(|rest| rest.split_once("**"))
            .filter_map(|(number, _)| number.parse::<u32>().ok())
            .map(|number| format!("p2r-{number:02}"))
            .collect()
    })
}

/// One fixture file, checked against P2R-600 to P2R-602 and P2R-605 to P2R-607.
struct Fixture {
    tier: String,
    name: String,
    path: PathBuf,
    subject: PathBuf,
    php: Vec<String>,
    laravel: bool,
    masks: Vec<(Regex, String)>,
    cases: Vec<Map<String, Value>>,
}

impl Fixture {
    /// The directory of a directory subject, which may hold `migrations/`.
    fn migrations(&self) -> Option<PathBuf> {
        Some(self.subject.join("migrations")).filter(|dir| dir.is_dir())
    }

    /// The golden file of a case (P2R-605).
    fn golden(&self, case: &str) -> PathBuf {
        self.path.with_extension("").join(format!("{case}.txt"))
    }

    fn mask(&self, output: &str) -> String {
        self.masks
            .iter()
            .fold(output.to_owned(), |text, (pattern, with)| {
                pattern.replace_all(&text, with.as_str()).into_owned()
            })
    }
}

fn keys(object: &Map<String, Value>, allowed: &[&str], what: &str) -> Result<(), String> {
    match object.keys().find(|key| !allowed.contains(&key.as_str())) {
        Some(key) => Err(format!("{what} has the unknown key {key:?} (P2R-601)")),
        None => Ok(()),
    }
}

fn load(tier: &str, path: &Path) -> Result<Fixture, String> {
    let name = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_default()
        .to_owned();
    if !requirement_ids()
        .iter()
        .any(|id| name.starts_with(&format!("{id}-")))
    {
        return Err("the name starts with no requirement ID of docs/spec.md (P2R-600)".into());
    }
    let source = fs::read_to_string(path).map_err(|err| err.to_string())?;
    let json: Value = serde_json::from_str(&source).map_err(|err| format!("not JSON: {err}"))?;
    let object = json.as_object().ok_or("not a JSON object")?;
    let mut top = vec!["subject", "php", "laravel", "cases"];
    if tier == "golden" {
        top.push("masks");
    }
    keys(object, &top, "the fixture")?;
    let subject = object
        .get("subject")
        .and_then(Value::as_str)
        .ok_or("the fixture has no subject")?;
    let file = harness().join("subjects").join(format!("{subject}.php"));
    let dir = harness().join("subjects").join(subject);
    let subject = if file.is_file() {
        file
    } else if dir.is_dir() {
        dir
    } else {
        return Err(format!(
            "no subject {subject} under tests/differential/subjects"
        ));
    };
    let strings = |key: &str| -> Result<Vec<String>, String> {
        match object.get(key) {
            None => Ok(Vec::new()),
            Some(list) => list
                .as_array()
                .ok_or(format!("{key} is not a list"))?
                .iter()
                .map(|item| {
                    item.as_str()
                        .map(str::to_owned)
                        .ok_or(format!("{key} holds a non-string"))
                })
                .collect(),
        }
    };
    let php = strings("php")?;
    if let Some(version) = php
        .iter()
        .find(|version| !VERSIONS.contains(&version.as_str()))
    {
        return Err(format!("php names {version}, which is no target of P2R-10"));
    }
    let laravel = match object.get("laravel") {
        None => false,
        Some(flag) => flag.as_bool().ok_or("laravel is not a boolean")?,
    };
    let masks = masks(object.get("masks"))?;
    let cases = cases(tier, object.get("cases"))?;
    Ok(Fixture {
        tier: tier.to_owned(),
        name,
        path: path.to_owned(),
        subject,
        php,
        laravel,
        masks,
        cases,
    })
}

fn masks(value: Option<&Value>) -> Result<Vec<(Regex, String)>, String> {
    let Some(list) = value else {
        return Ok(Vec::new());
    };
    let list = list.as_array().ok_or("masks is not a list")?;
    list.iter()
        .map(|mask| {
            let mask = mask.as_object().ok_or("a mask is not an object")?;
            keys(mask, &["pattern", "with"], "a mask")?;
            let pattern = mask
                .get("pattern")
                .and_then(Value::as_str)
                .ok_or("a mask has no pattern")?;
            let with = mask
                .get("with")
                .and_then(Value::as_str)
                .ok_or("a mask has no with")?;
            let pattern = Regex::new(pattern).map_err(|err| format!("a mask pattern: {err}"))?;
            Ok((pattern, with.to_owned()))
        })
        .collect()
}

fn cases(tier: &str, value: Option<&Value>) -> Result<Vec<Map<String, Value>>, String> {
    let list = value
        .and_then(Value::as_array)
        .ok_or("the fixture has no list of cases")?;
    if list.is_empty() {
        return Err("the fixture has no cases (P2R-601)".into());
    }
    let (allowed, required): (&[&str], &[&str]) = match tier {
        "e2e" => (
            &[
                "name",
                "stdin",
                "arguments",
                "files",
                "stdout",
                "exit",
                "written",
            ],
            &["name", "stdout", "exit"],
        ),
        "golden" => (
            &["name", "function", "arguments", "type"],
            &["name", "function", "arguments", "type"],
        ),
        _ => (
            &["name", "function", "arguments", "type", "returns", "stdout"],
            &["name", "function", "arguments", "type", "returns"],
        ),
    };
    let mut names = BTreeSet::new();
    let mut functions = BTreeSet::new();
    let mut found = Vec::new();
    for case in list {
        let case = case.as_object().ok_or("a case is not an object")?;
        let name = case.get("name").and_then(Value::as_str).unwrap_or("?");
        keys(case, allowed, &format!("case {name}"))?;
        if let Some(key) = required.iter().find(|key| !case.contains_key(**key)) {
            return Err(format!("case {name} has no {key}"));
        }
        if !names.insert(name.to_owned()) {
            return Err(format!("two cases are named {name} (P2R-601)"));
        }
        if let Some(ty) = case.get("type").and_then(Value::as_str)
            && !TYPES.contains(&ty)
        {
            return Err(format!(
                "case {name}: type {ty} is not one of {TYPES:?} (P2R-602)"
            ));
        }
        if let Some(function) = case.get("function") {
            functions.insert(function.to_string());
        }
        found.push(case.clone());
    }
    if tier == "unit" && functions.len() > 1 {
        return Err("the cases of a unit fixture call one function (P2R-602)".into());
    }
    Ok(found)
}

/// A JSON argument as a PHP literal; an integer and a float stay distinct (P2R-602).
fn literal(value: &Value) -> Result<String, String> {
    Ok(match value {
        Value::Null => "null".to_owned(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => match number.as_i64() {
            Some(integer) => integer.to_string(),
            // Debug keeps the `.0` of a whole float, so PHP reads a float.
            None => format!("{:?}", number.as_f64().ok_or("a number out of range")?),
        },
        Value::String(text) => format!("'{}'", text.replace('\\', "\\\\").replace('\'', "\\'")),
        Value::Array(items) => {
            let items: Vec<String> = items.iter().map(literal).collect::<Result<_, _>>()?;
            format!("[{}]", items.join(", "))
        }
        Value::Object(_) => return Err("a map argument is not supported yet (ADR 0010)".into()),
    })
}

/// What the driver's helper of `ty` prints for the expected value (P2R-603).
fn rendered(ty: &str, value: &Value) -> Result<String, String> {
    let wrong = || format!("returns {value} is not a {ty}");
    Ok(match ty {
        "int" => format!(
            "int {}\n",
            value
                .as_i64()
                .filter(|_| !value.is_f64())
                .ok_or_else(wrong)?
        ),
        "string" => {
            let text = value.as_str().ok_or_else(wrong)?;
            format!("string {} {text}\n", text.len())
        }
        "bool" => format!("bool {}\n", value.as_bool().ok_or_else(wrong)?),
        "?int" | "?string" if value.is_null() => "null\n".to_owned(),
        "?int" => rendered("int", value)?,
        "?string" => rendered("string", value)?,
        _ => {
            let element = &ty[5..ty.len() - 1];
            let items = value.as_array().ok_or_else(wrong)?;
            let mut out = format!("{ty} {}\n", items.len());
            for item in items {
                out.push_str(&rendered(element, item)?);
            }
            out
        }
    })
}

/// The name of the driver's helper for `ty`.
fn helper(ty: &str) -> String {
    match ty {
        "?int" => "print_nullable_int".to_owned(),
        "?string" => "print_nullable_string".to_owned(),
        "list<int>" => "print_int_list".to_owned(),
        "list<string>" => "print_string_list".to_owned(),
        _ => format!("print_{ty}"),
    }
}

/// The source of each typed print helper (P2R-603).
fn helper_source(ty: &str) -> String {
    match ty {
        "int" => {
            "function print_int(int $v): void\n{\n    echo 'int ', $v, \"\\n\";\n}\n".to_owned()
        }
        "string" => "function print_string(string $v): void\n{\n    echo 'string ', strlen($v), ' ', $v, \"\\n\";\n}\n".to_owned(),
        "bool" => "function print_bool(bool $v): void\n{\n    echo 'bool ', $v ? 'true' : 'false', \"\\n\";\n}\n".to_owned(),
        "?int" | "?string" => {
            let inner = &ty[1..];
            format!(
                // An early return, because php2rust narrows `?T` after it (P2R-106).
                "function print_nullable_{inner}(?{inner} $v): void\n{{\n    if ($v === null) {{\n        echo 'null', \"\\n\";\n        return;\n    }}\n    print_{inner}($v);\n}}\n"
            )
        }
        _ => {
            let inner = &ty[5..ty.len() - 1];
            format!(
                "/**\n * @param {ty} $v\n */\nfunction print_{inner}_list(array $v): void\n{{\n    echo '{ty} ', count($v), \"\\n\";\n    foreach ($v as $item) {{\n        print_{inner}($item);\n    }}\n}}\n"
            )
        }
    }
}

/// The types a type's helper needs, itself included.
fn needed(ty: &str) -> Vec<String> {
    match ty {
        "?int" | "?string" => vec![ty[1..].to_owned(), ty.to_owned()],
        "list<int>" | "list<string>" => vec![ty[5..ty.len() - 1].to_owned(), ty.to_owned()],
        _ => vec![ty.to_owned()],
    }
}

/// The PHP driver of a call fixture (P2R-603).
fn driver(fixture: &Fixture) -> Result<String, String> {
    let mut types: BTreeSet<String> = BTreeSet::new();
    let mut body = String::new();
    for case in &fixture.cases {
        let name = case["name"].as_str().unwrap_or_default();
        let ty = case["type"].as_str().unwrap_or_default();
        let function = case["function"]
            .as_str()
            .ok_or("function is not a string")?;
        let arguments = case["arguments"]
            .as_array()
            .ok_or("arguments is not a list")?;
        let arguments: Vec<String> = arguments.iter().map(literal).collect::<Result<_, _>>()?;
        types.extend(needed(ty));
        body.push_str(&format!(
            "    echo {}, \"\\n\";\n    {}(\\{}({}));\n",
            literal(&Value::String(format!("case {name}")))?,
            helper(ty),
            function.trim_start_matches('\\'),
            arguments.join(", ")
        ));
    }
    let helpers: Vec<String> = types.iter().map(|ty| helper_source(ty)).collect();
    Ok(format!(
        "<?php\n\ndeclare(strict_types=1);\n\nnamespace Php2rustFixture;\n\n{}\nfunction php2rust_fixture(): void\n{{\n{body}}}\n",
        helpers.join("\n")
    ))
}

/// Splits the driver's output into each case's part, by the `case <name>` lines.
fn parts(output: &str) -> BTreeMap<String, String> {
    let mut found = BTreeMap::new();
    let mut current: Option<String> = None;
    for line in output.split_inclusive('\n') {
        if let Some(name) = line
            .strip_prefix("case ")
            .map(|rest| rest.trim_end_matches('\n'))
        {
            current = Some(name.to_owned());
            found.insert(name.to_owned(), String::new());
        } else {
            let key = current.clone().unwrap_or_default();
            found.entry(key).or_insert_with(String::new).push_str(line);
        }
    }
    found
}

/// A new, empty directory for one fixture's files.
fn scratch(fixture: &Fixture) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "php2rust-differential-{}/{}-{}",
        std::process::id(),
        fixture.tier,
        fixture.name
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch directory");
    dir
}

/// The PHP command of a fixture's subject: the flags, and for Laravel the bootstrap and
/// its migrations (P2R-606).
fn php_command(fixture: &Fixture, dir: &Path, extra: &[String]) -> Command {
    let mut command = Command::new(php());
    command.args(php_flags()).args(extra).current_dir(dir);
    if fixture.laravel {
        let boot = harness().join("laravel/boot.php");
        command
            .arg("-d")
            .arg(format!("auto_prepend_file={}", boot.display()));
        if let Some(migrations) = fixture.migrations() {
            command.env("PHP2RUST_MIGRATIONS", migrations);
        }
    }
    command
}

/// php2rust on `input`, with the subject's migrations, Laravel's `vendor/` and the
/// fixture's targets.
fn convert(fixture: &Fixture, input: &Path, output: &Path) -> Result<(), String> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_php2rust"));
    command.arg(input).arg("-o").arg(output);
    if fixture.laravel {
        command.arg("--include").arg(vendor());
    }
    if let Some(migrations) = fixture.migrations() {
        command.arg("--migrations").arg(migrations);
    }
    for version in &fixture.php {
        command.args(["--php", version]);
    }
    let result = command.output().map_err(|err| err.to_string())?;
    if result.status.success() {
        Ok(())
    } else {
        Err(format!(
            "php2rust refuses the fixture (P2R-604):\n{}",
            text(&result.stderr)
        ))
    }
}

fn rustc(args: &[&str], dir: &Path) -> Result<(), String> {
    let mut all = vec!["--edition", "2024"];
    all.extend(args);
    let build = run("rustc", &all, dir);
    if build.status.success() {
        Ok(())
    } else {
        Err(format!(
            "rustc fails on the Rust (P2R-604):\n{}",
            text(&build.stderr)
        ))
    }
}

/// A run of either side of a call fixture, which must exit 0 with an empty stderr.
fn clean(side: &str, output: &Output) -> Result<String, String> {
    if output.status.success() && output.stderr.is_empty() {
        Ok(text(&output.stdout))
    } else {
        Err(format!(
            "the {side} run fails, exit {:?} (P2R-604):\n{}",
            output.status.code(),
            text(&output.stderr)
        ))
    }
}

/// The extra PHP flags that load Xdebug with code coverage, when the binary has it
/// (P2R-605).
fn xdebug() -> Vec<String> {
    let dir = text(&run(&php(), &["-r", "echo ini_get('extension_dir');"], &root()).stdout);
    let library = Path::new(&dir).join("xdebug.so");
    if !library.is_file() {
        return Vec::new();
    }
    vec![
        "-d".to_owned(),
        format!("zend_extension={}", library.display()),
        "-d".to_owned(),
        "xdebug.mode=coverage".to_owned(),
    ]
}

/// A `unit`, `feature` or `golden` fixture (P2R-602 to P2R-605).
fn calls(fixture: &Fixture, record: bool) -> Result<(), String> {
    let work = scratch(fixture);
    let tree = work.join("crate");
    fs::create_dir_all(&tree).map_err(|err| err.to_string())?;
    let mut sources: Vec<PathBuf> = if fixture.subject.is_file() {
        vec![fixture.subject.clone()]
    } else {
        fs::read_dir(&fixture.subject)
            .map_err(|err| err.to_string())?
            .map(|entry| entry.expect("readable subject").path())
            .filter(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "php"))
            .collect()
    };
    sources.sort();
    let mut entry = String::from("<?php\n\ndeclare(strict_types=1);\n\n");
    let mut copies = Vec::new();
    for source in &sources {
        let target = tree.join(source.file_name().expect("a file"));
        fs::copy(source, &target).map_err(|err| err.to_string())?;
        entry.push_str(&format!(
            "require {};\n",
            literal(&Value::String(target.display().to_string()))?
        ));
        copies.push(target);
    }
    fs::write(tree.join("driver.php"), driver(fixture)?).map_err(|err| err.to_string())?;
    entry.push_str("require __DIR__ . '/crate/driver.php';\n");
    let extra = if record { xdebug() } else { Vec::new() };
    entry.push_str(&coverage_entry(record, &copies)?);
    fs::write(work.join("entry.php"), entry).map_err(|err| err.to_string())?;
    let output = php_command(fixture, &work, &extra)
        .arg("entry.php")
        .output()
        .map_err(|err| err.to_string())?;
    let php = if record {
        note(&text(&output.stderr));
        if !output.status.success() {
            return Err(format!(
                "the PHP run fails, exit {:?}",
                output.status.code()
            ));
        }
        text(&output.stdout)
    } else {
        clean("PHP", &output)?
    };
    let php: BTreeMap<String, String> = parts(&php)
        .into_iter()
        .map(|(case, part)| (case, fixture.mask(&part)))
        .collect();
    if record {
        return recorded(fixture, &php);
    }
    expected_by_php(fixture, &php)?;
    convert(fixture, &tree, &work.join("out"))?;
    let library = work.join("libfixture.rlib");
    rustc(
        &[
            "--crate-type",
            "lib",
            "--crate-name",
            "fixture",
            "-o",
            library.to_str().expect("UTF-8"),
            "out/lib.rs",
        ],
        &work,
    )?;
    let module = driver_module(&work.join("out"))?;
    fs::write(
        work.join("main.rs"),
        format!("fn main() {{\n    fixture::{module}::php2rust_fixture();\n}}\n"),
    )
    .map_err(|err| err.to_string())?;
    let extern_arg = format!("fixture={}", library.display());
    rustc(
        &["--extern", &extern_arg, "-o", "fixture", "main.rs"],
        &work,
    )?;
    let rust = clean(
        "Rust",
        &run(work.join("fixture").to_str().expect("UTF-8"), &[], &work),
    )?;
    let rust: BTreeMap<String, String> = parts(&rust)
        .into_iter()
        .map(|(case, part)| (case, fixture.mask(&part)))
        .collect();
    compare("Rust", &rust, "PHP", &php)
}

/// The PHP lines that run the fixture, with Xdebug's coverage around it in record mode.
fn coverage_entry(record: bool, subjects: &[PathBuf]) -> Result<String, String> {
    if !record {
        return Ok("\\Php2rustFixture\\php2rust_fixture();\n".to_owned());
    }
    let files: Vec<String> = subjects
        .iter()
        .map(|path| literal(&Value::String(path.display().to_string())))
        .collect::<Result<_, _>>()?;
    Ok(format!(
        "$covered = function_exists('xdebug_start_code_coverage');\n\
         if ($covered) {{\n    xdebug_start_code_coverage(XDEBUG_CC_UNUSED | XDEBUG_CC_DEAD_CODE);\n}} else {{\n    fwrite(STDERR, \"coverage: this PHP loads no Xdebug, so no line is checked\\n\");\n}}\n\
         \\Php2rustFixture\\php2rust_fixture();\n\
         if ($covered) {{\n    $subjects = array_map('realpath', [{}]);\n    foreach (xdebug_get_code_coverage() as $file => $lines) {{\n        if (in_array(realpath($file), $subjects, true)) {{\n            foreach ($lines as $line => $hit) {{\n                if ($hit === -1) {{\n                    fwrite(STDERR, \"coverage: no input reached {{$file}}:{{$line}}\\n\");\n                }}\n            }}\n        }}\n    }}\n}}\n",
        files.join(", ")
    ))
}

/// Record mode writes each golden case's file from the PHP output (P2R-605).
fn recorded(fixture: &Fixture, php: &BTreeMap<String, String>) -> Result<(), String> {
    for case in &fixture.cases {
        let name = case["name"].as_str().unwrap_or_default();
        let file = fixture.golden(name);
        fs::create_dir_all(file.parent().expect("a folder")).map_err(|err| err.to_string())?;
        fs::write(&file, php.get(name).cloned().unwrap_or_default())
            .map_err(|err| err.to_string())?;
        note(&format!("recorded {}\n", file.display()));
    }
    Ok(())
}

/// Checks the PHP output of every case against what the fixture expects, first (P2R-604).
fn expected_by_php(fixture: &Fixture, php: &BTreeMap<String, String>) -> Result<(), String> {
    let mut expected = BTreeMap::new();
    for case in &fixture.cases {
        let name = case["name"].as_str().unwrap_or_default().to_owned();
        let wanted = if fixture.tier == "golden" {
            let file = fixture.golden(&name);
            let recorded = fs::read_to_string(&file).map_err(|_| {
                format!(
                    "case {name}: no golden file {}; record it with PHP2RUST_RECORD=1 (P2R-605)",
                    file.display()
                )
            })?;
            fixture.mask(&recorded)
        } else {
            let stdout = case
                .get("stdout")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let ty = case["type"].as_str().unwrap_or_default();
            format!("{stdout}{}", rendered(ty, &case["returns"])?)
        };
        expected.insert(name, wanted);
    }
    compare("PHP", php, "the fixture", &expected)
}

/// Every case whose output differs between two sides, named.
fn compare(
    side: &str,
    got: &BTreeMap<String, String>,
    other: &str,
    wanted: &BTreeMap<String, String>,
) -> Result<(), String> {
    let differ: Vec<String> = wanted
        .iter()
        .filter(|(case, text)| got.get(*case) != Some(text))
        .map(|(case, text)| {
            format!(
                "case {case}: {side} gives\n{}-- {other} expects\n{text}",
                got.get(case).map_or("(nothing)\n", String::as_str)
            )
        })
        .collect();
    if differ.is_empty() {
        Ok(())
    } else {
        Err(differ.join("\n"))
    }
}

/// The module of the converted crate that holds `php2rust_fixture()`.
fn driver_module(out: &Path) -> Result<String, String> {
    for entry in fs::read_dir(out).map_err(|err| err.to_string())? {
        let path = entry.map_err(|err| err.to_string())?.path();
        let source = fs::read_to_string(&path).unwrap_or_default();
        if source.contains("pub fn php2rust_fixture()") {
            return Ok(path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_owned());
        }
    }
    Err("the converted crate holds no php2rust_fixture()".into())
}

/// What one e2e run gives: stdout, exit code, and each file it created or changed.
#[derive(PartialEq, Debug)]
struct Ran {
    stdout: String,
    exit: Option<i32>,
    written: BTreeMap<String, String>,
}

/// Runs `command` in a new directory that holds only the case's starting files (P2R-607).
fn ran(mut command: Command, case: &Map<String, Value>, dir: &Path) -> Result<Ran, String> {
    let _ = fs::remove_dir_all(dir);
    fs::create_dir_all(dir).map_err(|err| err.to_string())?;
    let files = case
        .get("files")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    for (path, content) in &files {
        let target = dir.join(path);
        fs::create_dir_all(target.parent().expect("a folder")).map_err(|err| err.to_string())?;
        fs::write(&target, content.as_str().unwrap_or_default()).map_err(|err| err.to_string())?;
    }
    let arguments: Vec<String> = case
        .get("arguments")
        .and_then(Value::as_array)
        .map(|list| {
            list.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();
    let mut child = command
        .args(&arguments)
        .current_dir(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| err.to_string())?;
    let stdin = case
        .get("stdin")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();
    if let Some(mut pipe) = child.stdin.take() {
        let _ = pipe.write_all(stdin.as_bytes());
    }
    let output = child.wait_with_output().map_err(|err| err.to_string())?;
    let mut written = BTreeMap::new();
    for entry in walk(dir)? {
        let relative = entry
            .strip_prefix(dir)
            .expect("below")
            .to_string_lossy()
            .replace('\\', "/");
        let content = text(&fs::read(&entry).map_err(|err| err.to_string())?);
        if files.get(&relative).and_then(Value::as_str) != Some(content.as_str()) {
            written.insert(relative, content);
        }
    }
    Ok(Ran {
        stdout: text(&output.stdout),
        exit: output.status.code(),
        written,
    })
}

fn walk(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut found = Vec::new();
    for entry in fs::read_dir(dir).map_err(|err| err.to_string())? {
        let path = entry.map_err(|err| err.to_string())?.path();
        if path.is_dir() {
            found.extend(walk(&path)?);
        } else {
            found.push(path);
        }
    }
    Ok(found)
}

/// An `e2e` fixture (P2R-607).
fn e2e(fixture: &Fixture) -> Result<(), String> {
    let work = scratch(fixture);
    let program = if fixture.subject.is_file() {
        fixture.subject.clone()
    } else {
        fixture.subject.join("program.php")
    };
    let mut errors = Vec::new();
    let mut php_runs = Vec::new();
    for case in &fixture.cases {
        let name = case["name"].as_str().unwrap_or_default();
        let mut command = php_command(fixture, &work, &[]);
        command.arg(&program);
        let php = ran(command, case, &work.join(format!("php-{name}")))?;
        let expected = Ran {
            stdout: case["stdout"].as_str().unwrap_or_default().to_owned(),
            exit: case["exit"]
                .as_i64()
                .and_then(|code| i32::try_from(code).ok()),
            written: case
                .get("written")
                .and_then(Value::as_object)
                .map(|files| {
                    files
                        .iter()
                        .map(|(path, content)| {
                            (
                                path.clone(),
                                content.as_str().unwrap_or_default().to_owned(),
                            )
                        })
                        .collect()
                })
                .unwrap_or_default(),
        };
        if php != expected {
            errors.push(format!(
                "case {name}: PHP gives {php:?}\n-- the fixture expects {expected:?}"
            ));
        }
        php_runs.push((name.to_owned(), php));
    }
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    let rust_source = work.join("program.rs");
    convert(fixture, &program, &rust_source)?;
    rustc(&["-o", "program", "program.rs"], &work)?;
    for (case, (name, php)) in fixture.cases.iter().zip(&php_runs) {
        let command = Command::new(work.join("program"));
        let rust = ran(command, case, &work.join(format!("rust-{name}")))?;
        if &rust != php {
            errors.push(format!(
                "case {name}: Rust gives {rust:?}\n-- PHP gives {php:?}"
            ));
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

/// Why a fixture does not run under this PHP binary, if it does not (P2R-604, P2R-606).
fn skipped(fixture: &Fixture) -> Option<String> {
    if !fixture.php.is_empty() && !fixture.php.iter().any(|version| version == php_version()) {
        return Some(format!("PHP {} is not in its php list", php_version()));
    }
    if fixture.laravel && !vendor().join("autoload.php").is_file() {
        return Some("Laravel".to_owned());
    }
    None
}

fn check(tier: &str, path: &Path, record: bool) -> Result<(), String> {
    let fixture = load(tier, path)?;
    if skipped(&fixture).is_some() {
        return Ok(());
    }
    if tier == "e2e" {
        e2e(&fixture)
    } else {
        calls(&fixture, record && tier == "golden")
    }
}

/// Runs every fixture of `tier` that `PHP2RUST_FIXTURE` keeps; every one runs when one
/// fails (P2R-600).
fn check_tier(tier: &str) {
    let version = php_version();
    assert!(
        VERSIONS.contains(&version),
        "the harness needs PHP 8.3, 8.4, 8.5 or 8.6 as {} (PHP2RUST_PHP), found: {version}",
        php()
    );
    let record = std::env::var_os("PHP2RUST_RECORD").is_some();
    assert!(
        !(record && std::env::var_os("CI").is_some()),
        "PHP2RUST_RECORD refuses to record in CI (P2R-605)"
    );
    let only = std::env::var("PHP2RUST_FIXTURE").unwrap_or_default();
    let mut paths: Vec<PathBuf> = fs::read_dir(harness().join(tier))
        .map(|entries| {
            entries
                .map(|entry| entry.expect("readable fixture").path())
                .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
                .collect()
        })
        .unwrap_or_default();
    paths.sort();
    let mut failures = Vec::new();
    let mut laravel = 0;
    for path in paths {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_owned();
        if !name.contains(&only) {
            continue;
        }
        if let Ok(fixture) = load(tier, &path)
            && skipped(&fixture).as_deref() == Some("Laravel")
        {
            laravel += 1;
        }
        if let Err(why) = check(tier, &path, record) {
            failures.push(format!("{tier}/{name}: {why}"));
        }
    }
    if laravel > 0 {
        note(&format!(
            "skipped {laravel} Laravel fixture(s) of {tier}: run composer install in tests/differential/laravel (P2R-606)\n"
        ));
    }
    assert!(
        failures.is_empty(),
        "{} fixture(s) failed:\n\n{}",
        failures.len(),
        failures.join("\n\n")
    );
}

#[test]
fn unit() {
    check_tier("unit");
}

#[test]
fn feature() {
    check_tier("feature");
}

#[test]
fn golden() {
    check_tier("golden");
}

#[test]
fn e2e_tier() {
    check_tier("e2e");
}

/// A fixture file in a new directory, for the harness's own tests (P2R-609).
fn written(tier: &str, name: &str, json: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "php2rust-harness-test-{}/{tier}",
        std::process::id()
    ));
    fs::create_dir_all(&dir).expect("scratch directory");
    let path = dir.join(format!("{name}.json"));
    fs::write(&path, json).expect("fixture file");
    path
}

/// P2R-609: each tier fails a fixture whose expectation is wrong, and passes it once the
/// expectation is right, on subjects without Laravel so it runs in CI.
#[test]
fn the_harness_fails_a_wrong_expectation() {
    let twice = |returns: &str| {
        format!(
            r#"{{"subject": "words", "cases": [{{"name": "two", "function": "Words\\twice", "arguments": [2], "type": "int", "returns": {returns}}}]}}"#
        )
    };
    let unit =
        |returns: &str, name: &str| check("unit", &written("unit", name, &twice(returns)), false);
    let wrong = unit("5", "p2r-183-wrong-twice").expect_err("a wrong return fails");
    assert!(wrong.contains("case two: PHP gives"), "{wrong}");
    unit("4", "p2r-183-right-twice").expect("a right return passes");

    let shout = |stdout: &str| {
        format!(
            r#"{{"subject": "words", "cases": [{{"name": "a", "function": "Words\\shout", "arguments": ["a"], "type": "string", "returns": "A!", "stdout": "{stdout}"}}]}}"#
        )
    };
    let wrong = check(
        "feature",
        &written("feature", "p2r-183-wrong-shout", &shout("wrong\\n")),
        false,
    )
    .expect_err("a wrong stdout fails");
    assert!(wrong.contains("case a: PHP gives"), "{wrong}");
    check(
        "feature",
        &written("feature", "p2r-183-right-shout", &shout("shouting a\\n")),
        false,
    )
    .expect("a right stdout passes");

    let golden = written(
        "golden",
        "p2r-183-golden-twice",
        r#"{"subject": "words", "cases": [{"name": "three", "function": "Words\\twice", "arguments": [3], "type": "int"}]}"#,
    );
    let file = golden.with_extension("").join("three.txt");
    fs::create_dir_all(file.parent().expect("a folder")).expect("golden folder");
    fs::write(&file, "int 7\n").expect("golden file");
    let wrong = check("golden", &golden, false).expect_err("a wrong golden file fails");
    assert!(wrong.contains("case three: PHP gives"), "{wrong}");
    fs::write(&file, "int 6\n").expect("golden file");
    check("golden", &golden, false).expect("a right golden file passes");

    let hello = |stdout: &str| {
        format!(
            r#"{{"subject": "hello", "cases": [{{"name": "run", "stdout": "{stdout}", "exit": 0}}]}}"#
        )
    };
    let wrong = check(
        "e2e",
        &written("e2e", "p2r-72-wrong-hello", &hello("bye\\n")),
        false,
    )
    .expect_err("a wrong e2e stdout fails");
    assert!(wrong.contains("case run: PHP gives"), "{wrong}");
    check(
        "e2e",
        &written("e2e", "p2r-72-right-hello", &hello("hello\\n")),
        false,
    )
    .expect("a right e2e stdout passes");
}

/// P2R-602, P2R-603: a float stays a float, and `list<int>` and `?string` print through
/// their helpers, on PHP and on the Rust.
#[test]
fn the_harness_prints_each_type_on_both_sides() {
    let fixture = written(
        "feature",
        "p2r-183-each-type",
        r#"{"subject": "words", "cases": [
            {"name": "float", "function": "Words\\bigger", "arguments": [2.5], "type": "bool", "returns": true},
            {"name": "whole-float", "function": "Words\\bigger", "arguments": [1.0], "type": "bool", "returns": false},
            {"name": "list", "function": "Words\\upTo", "arguments": [2], "type": "list<int>", "returns": [1, 2]},
            {"name": "empty", "function": "Words\\upTo", "arguments": [0], "type": "list<int>", "returns": []},
            {"name": "named", "function": "Words\\nameOrNull", "arguments": ["Ann"], "type": "?string", "returns": "Ann"},
            {"name": "unnamed", "function": "Words\\nameOrNull", "arguments": [""], "type": "?string", "returns": null}
        ]}"#,
    );
    check("feature", &fixture, false).expect("every type passes on both sides");
    let float_result = written(
        "unit",
        "p2r-183-float-result",
        r#"{"subject": "words", "cases": [{"name": "f", "function": "Words\\bigger", "arguments": [3.0], "type": "float", "returns": 1.5}]}"#,
    );
    let why = check("unit", &float_result, false).expect_err("a float result is no type");
    assert!(why.contains("type float is not one of"), "{why}");
}

/// P2R-605: a mask applies to both sides and the file, and a golden case without a file
/// fails.
#[test]
fn the_harness_masks_and_needs_golden_files() {
    let json = r#"{"subject": "words", "masks": [{"pattern": "[0-9]+", "with": "N"}], "cases": [{"name": "nine", "function": "Words\\twice", "arguments": [9], "type": "int"}]}"#;
    let golden = written("golden", "p2r-183-masked", json);
    let file = golden.with_extension("").join("nine.txt");
    let _ = fs::remove_file(&file);
    let why = check("golden", &golden, false).expect_err("a missing golden file fails");
    assert!(why.contains("no golden file"), "{why}");
    fs::create_dir_all(file.parent().expect("a folder")).expect("golden folder");
    fs::write(&file, "int 5\n").expect("golden file");
    check("golden", &golden, false).expect("int 18 and int 5 both mask to int N");
}

/// P2R-604: when the Rust differs from PHP, the message says so, not that the fixture is
/// wrong.
#[test]
fn a_rust_difference_names_rust() {
    let php = BTreeMap::from([("a".to_owned(), "int 1\n".to_owned())]);
    let rust = BTreeMap::from([("a".to_owned(), "int 2\n".to_owned())]);
    let why = compare("Rust", &rust, "PHP", &php).expect_err("a difference fails");
    assert!(
        why.starts_with("case a: Rust gives\nint 2\n-- PHP expects\nint 1\n"),
        "{why}"
    );
}

/// P2R-609: an unknown key and a name without a requirement ID fail the fixture.
#[test]
fn the_harness_refuses_a_malformed_fixture() {
    let unknown = written(
        "unit",
        "p2r-183-unknown-key",
        r#"{"subject": "words", "cases": [{"name": "two", "function": "Words\\twice", "arguments": [2], "type": "int", "returns": 4, "retruns": 4}]}"#,
    );
    let why = check("unit", &unknown, false).expect_err("an unknown key fails");
    assert!(why.contains("unknown key \"retruns\""), "{why}");
    let unnamed = written(
        "unit",
        "twice-without-id",
        r#"{"subject": "words", "cases": [{"name": "two", "function": "Words\\twice", "arguments": [2], "type": "int", "returns": 4}]}"#,
    );
    let why = check("unit", &unnamed, false).expect_err("a name without an ID fails");
    assert!(why.contains("no requirement ID"), "{why}");
}
