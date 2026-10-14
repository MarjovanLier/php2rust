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

/// P2R-190: without rustfmt an input that converts exits 2 and writes nothing, for a file
/// and for a directory; an input that stops still exits 1.
#[test]
fn missing_rustfmt_exits_2_and_writes_nothing() {
    let dir = scratch("no-rustfmt");
    let empty = dir.join("empty-path");
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::create_dir_all(&empty).unwrap();
    let source = "<?php\n\ndeclare(strict_types=1);\n\nfunction one(): int\n{\n    return 1;\n}\n";
    std::fs::write(dir.join("in.php"), source).unwrap();
    std::fs::write(dir.join("src").join("a.php"), source).unwrap();
    std::fs::write(dir.join("stops.php"), "<?php\n\necho \"x\\n\";\n").unwrap();
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_php2rust"))
            .args(args)
            .current_dir(&dir)
            .env("PATH", &empty)
            .output()
            .unwrap()
    };
    for args in [&["in.php", "-o", "out.rs"][..], &["src", "-o", "crate"]] {
        let out = run(args);
        assert_eq!(out.status.code(), Some(2), "{args:?}");
        assert!(out.stdout.is_empty(), "{args:?}");
        assert!(
            String::from_utf8_lossy(&out.stderr).contains("rustfmt"),
            "{args:?}"
        );
    }
    for left in [
        "out.rs",
        "out.rs.php2rust-tmp",
        "crate",
        "crate.php2rust-tmp",
    ] {
        assert!(!dir.join(left).exists(), "{left} exists");
    }
    assert_eq!(run(&["stops.php"]).status.code(), Some(1));
    let _ = std::fs::remove_dir_all(&dir);
}

/// P2R-190: a rustfmt that fails, and one whose `--check` finds a change, each give exit 2
/// with their own fixed message and no output file.
#[cfg(unix)]
#[test]
fn failing_rustfmt_exits_2_with_a_fixed_message() {
    use std::os::unix::fs::PermissionsExt;
    let dir = scratch("bad-rustfmt");
    std::fs::write(
        dir.join("in.php"),
        "<?php\n\ndeclare(strict_types=1);\n\necho \"x\\n\";\n",
    )
    .unwrap();
    let cases = [
        (
            "fails",
            "#!/bin/sh\nexit 1\n",
            "rustfmt failed on the output",
        ),
        (
            "unsettled",
            "#!/bin/sh\ncase \"$*\" in *--check*) exit 1;; esac\nexit 0\n",
            "rustfmt changes its own layout when it runs again",
        ),
    ];
    for (name, script, message) in cases {
        let bin = dir.join(name);
        std::fs::create_dir_all(&bin).unwrap();
        let rustfmt = bin.join("rustfmt");
        std::fs::write(&rustfmt, script).unwrap();
        std::fs::set_permissions(&rustfmt, std::fs::Permissions::from_mode(0o755)).unwrap();
        let out = Command::new(env!("CARGO_BIN_EXE_php2rust"))
            .args(["in.php", "-o", "out.rs"])
            .current_dir(&dir)
            .env("PATH", &bin)
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(2), "{name}");
        assert_eq!(
            String::from_utf8_lossy(&out.stderr),
            format!("in.php: cannot lay out the output: {message}\n"),
            "{name}"
        );
        assert!(!dir.join("out.rs").exists(), "{name}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}
