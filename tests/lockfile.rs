//! Every Mago crate, direct and transitive, resolves to the one pinned version (P2R-81), and
//! the tool and the tests lock one `regex` version (P2R-263).

const PIN: &str = "1.57.0";

#[test]
fn every_mago_crate_uses_the_pinned_version() {
    let lock = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.lock"))
        .expect("Cargo.lock is committed");
    let mut name = "";
    let mut mago = 0;
    let mut wrong = Vec::new();
    for line in lock.lines() {
        if let Some(value) = line.strip_prefix("name = ") {
            name = value.trim_matches('"');
        }
        if let Some(value) = line.strip_prefix("version = ")
            && name.starts_with("mago-")
        {
            mago += 1;
            if value.trim_matches('"') != PIN {
                wrong.push(format!("{name} {value}"));
            }
        }
    }
    assert!(
        mago >= 10,
        "found only {mago} mago crates in Cargo.lock; the parser may be wrong"
    );
    assert!(wrong.is_empty(), "Mago crates off the {PIN} pin: {wrong:?}");
}

/// The version of the package `wanted` in the lockfile text `lock`.
fn locked_version(lock: &str, wanted: &str) -> Option<String> {
    let mut name = "";
    for line in lock.lines() {
        if let Some(value) = line.strip_prefix("name = ") {
            name = value.trim_matches('"');
        }
        if let Some(value) = line.strip_prefix("version = ")
            && name == wanted
        {
            return Some(value.trim_matches('"').to_owned());
        }
    }
    None
}

/// The tool checks patterns with the `regex` version the tests build output with (P2R-263).
#[test]
fn the_tool_and_the_tests_use_one_regex_version() {
    let read = |path: &str| {
        std::fs::read_to_string(format!("{}/{path}", env!("CARGO_MANIFEST_DIR")))
            .unwrap_or_else(|err| panic!("{path}: {err}"))
    };
    let tool = locked_version(&read("Cargo.lock"), "regex");
    let tests = locked_version(&read("tests/support/crates/Cargo.lock"), "regex");
    assert!(tool.is_some(), "no regex in Cargo.lock");
    assert_eq!(tool, tests);
}
