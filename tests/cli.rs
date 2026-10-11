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
