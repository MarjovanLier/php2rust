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
    let temporary = dir.join("tmp");
    std::fs::create_dir_all(&temporary).unwrap();
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_php2rust"))
            .args(args)
            .current_dir(&dir)
            .env("PATH", &empty)
            .env("TMPDIR", &temporary)
            .output()
            .unwrap()
    };
    for (args, shown) in [
        (&["in.php", "-o", "out.rs"][..], "in.php"),
        (&["src", "-o", "crate"], "src"),
    ] {
        let out = run(args);
        assert_eq!(out.status.code(), Some(2), "{args:?}");
        assert!(out.stdout.is_empty(), "{args:?}");
        assert_eq!(
            String::from_utf8_lossy(&out.stderr),
            format!(
                "{shown}: cannot lay out the output: cannot run rustfmt, which must be on PATH\n"
            ),
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
    // The private directory of the layout is gone too (P2R-170).
    assert_eq!(std::fs::read_dir(&temporary).unwrap().count(), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

/// P2R-190: a `rustfmt.toml` in the working directory or a `.rustfmt.toml` in the home
/// directory does not change the output, and the layout leaves nothing in the temporary
/// directory.
#[test]
fn rustfmt_configuration_around_the_tool_is_ignored() {
    let dir = scratch("rustfmt-config");
    let home = dir.join("home");
    let temporary = dir.join("tmp");
    std::fs::create_dir_all(&home).unwrap();
    std::fs::create_dir_all(&temporary).unwrap();
    std::fs::write(
        dir.join("in.php"),
        "<?php\n\ndeclare(strict_types=1);\n\nfunction pick(int $n): int\n{\n    if ($n > 1) {\n        return 2;\n    }\n    return 1;\n}\n",
    )
    .unwrap();
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_php2rust"))
            .arg("in.php")
            .current_dir(&dir)
            .env("HOME", &home)
            .env("TMPDIR", &temporary)
            .output()
            .unwrap()
    };
    let plain = run();
    assert_eq!(plain.status.code(), Some(0));
    let settings = "hard_tabs = true\nmax_width = 40\n";
    std::fs::write(home.join(".rustfmt.toml"), settings).unwrap();
    std::fs::write(dir.join("rustfmt.toml"), settings).unwrap();
    let configured = run();
    assert_eq!(configured.status.code(), Some(0));
    assert_eq!(configured.stdout, plain.stdout);
    assert!(!plain.stdout.contains(&b'\t'));
    assert_eq!(std::fs::read_dir(&temporary).unwrap().count(), 0);
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
        let temporary = dir.join(format!("{name}-tmp"));
        std::fs::create_dir_all(&temporary).unwrap();
        let out = Command::new(env!("CARGO_BIN_EXE_php2rust"))
            .args(["in.php", "-o", "out.rs"])
            .current_dir(&dir)
            .env("PATH", &bin)
            .env("TMPDIR", &temporary)
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(2), "{name}");
        assert_eq!(
            String::from_utf8_lossy(&out.stderr),
            format!("in.php: cannot lay out the output: {message}\n"),
            "{name}"
        );
        assert!(!dir.join("out.rs").exists(), "{name}");
        assert_eq!(std::fs::read_dir(&temporary).unwrap().count(), 0, "{name}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// An input that names the class `Lib\Money`, and the file that declares it (P2R-340).
fn money(dir: &std::path::Path, library: &std::path::Path) {
    std::fs::write(
        dir.join("in.php"),
        "<?php\n\ndeclare(strict_types=1);\n\nfunction one(\\Lib\\Money $m): int\n{\n    return 1;\n}\n",
    )
    .unwrap();
    std::fs::create_dir_all(library.parent().unwrap()).unwrap();
    std::fs::write(
        library,
        "<?php\n\nnamespace Lib;\n\nfinal class Money\n{\n}\n",
    )
    .unwrap();
}

fn stderr_of(dir: &std::path::Path, args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_php2rust"))
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// P2R-340: an include path may name one file, whatever its place.
#[test]
fn an_include_path_may_be_a_file() {
    let dir = scratch("include-file");
    money(&dir, &dir.join("lib/money.php"));
    let known = "in.php:5:14: the type Lib\\Money has no Rust type (P2R-22)\n";
    assert_eq!(
        stderr_of(&dir, &["in.php", "--include", "lib/money.php"]),
        known
    );
    assert!(stderr_of(&dir, &["in.php"]).contains("unknown-ref(Lib\\Money)"));
    let _ = std::fs::remove_dir_all(&dir);
}

/// P2R-340: how an include path is written changes nothing, a symbolic link as the path is
/// followed, and a named file is read whatever its name ends in.
#[cfg(unix)]
#[test]
fn an_include_path_reads_the_same_however_written() {
    let dir = scratch("include-spelling");
    money(&dir, &dir.join("lib/money.inc"));
    std::os::unix::fs::symlink(dir.join("lib"), dir.join("linked")).unwrap();
    let known = "in.php:5:14: the type Lib\\Money has no Rust type (P2R-22)\n";
    let absolute = dir.join("lib/money.inc");
    for include in [
        "lib/money.inc",
        "./lib/../lib/money.inc",
        absolute.to_str().unwrap(),
        "linked/money.inc",
    ] {
        assert_eq!(
            stderr_of(&dir, &["in.php", "--include", include]),
            known,
            "{include}"
        );
    }
    // Under a directory only names ending in .php are read.
    assert!(
        stderr_of(&dir, &["in.php", "--include", "linked"]).contains("unknown-ref(Lib\\Money)")
    );
    // A symbolic link to a directory as the path is walked.
    money(&dir, &dir.join("php/money.php"));
    std::os::unix::fs::symlink(dir.join("php"), dir.join("walked")).unwrap();
    assert_eq!(stderr_of(&dir, &["in.php", "--include", "walked"]), known);
    // A file that does not parse, or is not UTF-8, gives no symbols.
    std::fs::write(
        dir.join("lib/broken.php"),
        "<?php\n\nnamespace Lib;\n\nfinal class Money\n{\n}\n\nfunction (\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("lib/latin.php"),
        b"<?php\n\nnamespace Lib;\n\n// caf\xe9\nfinal class Money\n{\n}\n",
    )
    .unwrap();
    for broken in ["lib/broken.php", "lib/latin.php"] {
        assert!(
            stderr_of(&dir, &["in.php", "--include", broken]).contains("unknown-ref(Lib\\Money)"),
            "{broken}"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// P2R-340: an input file under an include path is read as input only.
#[test]
fn an_input_under_an_include_path_stays_input() {
    let dir = scratch("include-input");
    // The function is declared twice, once as input and once as included code.
    std::fs::write(
        dir.join("in.php"),
        "<?php\n\ndeclare(strict_types=1);\n\nfunction one(): int\n{\n    return 1;\n}\n\necho one(), \"\\n\";\n",
    )
    .unwrap();
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_php2rust"))
            .args(args)
            .current_dir(&dir)
            .output()
            .unwrap()
    };
    let alone = run(&["in.php"]);
    let included = run(&["in.php", "--include", "."]);
    assert_eq!(alone.status.code(), Some(0));
    assert_eq!(included.status.code(), Some(0));
    assert_eq!(alone.stdout, included.stdout);
    let _ = std::fs::remove_dir_all(&dir);
}

/// P2R-340: a file whose path below the include path is not UTF-8 is skipped. macOS
/// refuses such names, so Linux runs it.
#[cfg(target_os = "linux")]
#[test]
fn an_include_skips_a_path_that_is_not_utf8() {
    use std::os::unix::ffi::OsStrExt;
    let dir = scratch("include-bytes");
    let name = std::ffi::OsStr::from_bytes(b"m\xffoney.php");
    money(&dir, &dir.join("inc").join(name));
    assert!(stderr_of(&dir, &["in.php", "--include", "inc"]).contains("unknown-ref(Lib\\Money)"));
    let _ = std::fs::remove_dir_all(&dir);
}

/// P2R-340: an include path that is neither a file nor a directory, and one below which a
/// directory cannot be listed, exit 2 with the reason and write nothing.
#[cfg(unix)]
#[test]
fn an_include_path_that_cannot_be_walked_exits_2() {
    use std::os::unix::fs::PermissionsExt;
    let dir = scratch("include-unwalkable");
    money(&dir, &dir.join("inc/locked/money.php"));
    let made = Command::new("mkfifo")
        .arg("pipe")
        .current_dir(&dir)
        .status();
    assert!(made.unwrap().success());
    let run = |include: &str| {
        Command::new(env!("CARGO_BIN_EXE_php2rust"))
            .args(["in.php", "-o", "out.rs", "--include", include])
            .current_dir(&dir)
            .output()
            .unwrap()
    };
    let pipe = run("pipe");
    assert_eq!(pipe.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&pipe.stderr),
        "pipe: the include path is neither a file nor a directory\n"
    );
    let locked = dir.join("inc/locked");
    std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o000)).unwrap();
    // Root lists the directory anyway, so the refusal cannot happen there.
    if std::fs::read_dir(&locked).is_err() {
        let out = run("inc");
        assert_eq!(out.status.code(), Some(2));
        assert!(
            String::from_utf8_lossy(&out.stderr).starts_with("inc: cannot read the include path: ")
        );
    }
    std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert!(!dir.join("out.rs").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

/// P2R-340: the walk follows no symbolic link below an include path.
#[cfg(unix)]
#[test]
fn an_include_walk_follows_no_symbolic_link() {
    let dir = scratch("include-link");
    money(&dir, &dir.join("elsewhere/money.php"));
    std::fs::create_dir_all(dir.join("inc")).unwrap();
    std::os::unix::fs::symlink(dir.join("elsewhere"), dir.join("inc/linked")).unwrap();
    assert!(stderr_of(&dir, &["in.php", "--include", "inc"]).contains("unknown-ref(Lib\\Money)"));
    let _ = std::fs::remove_dir_all(&dir);
}
