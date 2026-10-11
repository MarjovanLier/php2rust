//! Command-line behaviour that no single input file covers (P2R-01, P2R-04).

use std::process::{Command, Output};

fn php2rust(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_php2rust"))
        .args(args)
        .output()
        .expect("the php2rust binary runs")
}

#[test]
fn version_prints_the_crate_version_to_stdout() {
    let out = php2rust(&["--version"]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        format!("php2rust {}\n", env!("CARGO_PKG_VERSION"))
    );
    assert!(out.stderr.is_empty());
}

#[test]
fn help_prints_usage_to_stdout() {
    let out = php2rust(&["--help"]);
    assert_eq!(out.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&out.stdout).contains("Usage: php2rust [OPTIONS] <INPUT>"));
    assert!(out.stderr.is_empty());
}

#[test]
fn missing_input_prints_usage_to_stderr_and_exits_2() {
    let out = php2rust(&[]);
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    assert!(String::from_utf8_lossy(&out.stderr).contains("Usage: php2rust <INPUT>"));
}

fn scratch(test: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("php2rust-cli-{test}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch directory");
    dir
}

#[test]
fn success_with_output_writes_the_file_and_nothing_to_stdout() {
    let dir = scratch("success");
    std::fs::write(
        dir.join("in.php"),
        "<?php\n\ndeclare(strict_types=1);\n\necho \"x\\n\";\n",
    )
    .unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_php2rust"))
        .args(["in.php", "-o", "out.rs"])
        .current_dir(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stdout.is_empty() && out.stderr.is_empty());
    assert!(
        std::fs::read_to_string(dir.join("out.rs"))
            .unwrap()
            .contains("print!(\"x\\n\");")
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn refusal_keeps_an_existing_output_file() {
    let dir = scratch("refusal");
    std::fs::write(dir.join("in.php"), "<?php\n\necho \"x\\n\";\n").unwrap();
    std::fs::write(dir.join("out.rs"), "keep me\n").unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_php2rust"))
        .args(["in.php", "-o", "out.rs"])
        .current_dir(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        std::fs::read_to_string(dir.join("out.rs")).unwrap(),
        "keep me\n"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
