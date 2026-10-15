mod convert;
mod names;
mod printer;
mod problem;
mod types;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use clap::{ColorChoice, Parser};

use crate::convert::Failure;

/// Convert typed PHP into normal, readable Rust, without AI.
#[derive(Parser)]
#[command(version, color = ColorChoice::Never)]
struct Cli {
    /// The PHP file to convert, or a directory of PHP files to convert into one crate.
    input: PathBuf,
    /// Write the Rust source to this file instead of stdout. For a directory input, the
    /// directory to create for the crate; it must not exist yet.
    #[arg(short = 'o', value_name = "OUTPUT")]
    output: Option<PathBuf>,
    /// Read the PHP files under this path for their classes, functions and constants only,
    /// such as Composer's vendor folder; they are not converted. May be given more than once.
    #[arg(long = "include", value_name = "PATH")]
    include: Vec<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let included = match included_files(&cli.include) {
        Ok(included) => included,
        Err(reason) => return invalid(reason),
    };
    if cli.input.is_dir() {
        return convert_directory(&cli.input, cli.output.as_deref(), included);
    }
    let path = cli.input.display().to_string();
    let source = match fs::read(&cli.input) {
        Ok(bytes) => bytes,
        Err(err) => return invalid(format!("{path}: cannot read input: {err}")),
    };
    if std::str::from_utf8(&source).is_err() {
        return invalid(format!("{path}: input is not valid UTF-8"));
    }
    if let Some(output) = &cli.output
        && let Err(reason) = check_output(&cli.input, output)
    {
        return invalid(reason);
    }
    let rust = match convert::convert(&path, source, included) {
        Ok(rust) => laid_out(&path, vec![("out.rs".to_owned(), rust)], "out.rs"),
        Err(failure) => Err(failure),
    };
    match rust {
        Ok(files) => {
            let rust = &files[0].1;
            match &cli.output {
                None => {
                    print!("{rust}");
                    ExitCode::SUCCESS
                }
                Some(output) => match write_atomically(output, rust) {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(err) => {
                        invalid(format!("{}: cannot write output: {err}", output.display()))
                    }
                },
            }
        }
        Err(Failure::Refused(lines)) => {
            eprint!("{lines}");
            ExitCode::from(1)
        }
        Err(Failure::Invalid(text)) => {
            eprint!("{text}");
            ExitCode::from(2)
        }
    }
}

/// Converts every `.php` file under `input` into one crate in the new directory `output`
/// (P2R-170).
fn convert_directory(
    input: &Path,
    output: Option<&Path>,
    included: Vec<convert::Included>,
) -> ExitCode {
    let Some(output) = output else {
        return invalid(format!(
            "{}: a directory input needs -o with the directory to create for the crate",
            input.display()
        ));
    };
    // `out/` and `out/.` name the directory `out`, so its temporary sits beside it.
    let output: PathBuf = output.components().collect();
    let output = output.as_path();
    if let Err(reason) = check_crate_output(input, output) {
        return invalid(reason);
    }
    let shown = input.display().to_string();
    let mut relative = Vec::new();
    if let Err(err) = php_files(input, Path::new(""), &mut relative) {
        return invalid(format!("{shown}: cannot read the directory: {err}"));
    }
    // Byte order of the relative path (P2R-170).
    relative.sort_by(|a, b| {
        a.as_os_str()
            .as_encoded_bytes()
            .cmp(b.as_os_str().as_encoded_bytes())
    });
    if relative.is_empty() {
        return invalid(format!("{shown}: the directory holds no .php file"));
    }
    let mut inputs = Vec::new();
    for path in relative {
        // Joined from the path's parts, so `x\y.php`, a valid name outside Windows, stays
        // apart from `x/y.php`: the name is also the file's identity in Mago (P2R-171).
        let name = path
            .components()
            .map(|part| part.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");
        let shown = if shown.ends_with('/') {
            format!("{shown}{name}")
        } else {
            format!("{shown}/{name}")
        };
        let source = match fs::read(input.join(&path)) {
            Ok(bytes) => bytes,
            Err(err) => return invalid(format!("{shown}: cannot read input: {err}")),
        };
        if std::str::from_utf8(&source).is_err() {
            return invalid(format!("{shown}: input is not valid UTF-8"));
        }
        inputs.push(convert::Input {
            shown,
            name,
            source,
        });
    }
    match convert::convert_crate(inputs, included)
        .and_then(|files| laid_out(&shown, files, "lib.rs"))
    {
        Ok(files) => match write_crate(output, &files) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => invalid(format!("{}: cannot write output: {err}", output.display())),
        },
        Err(Failure::Refused(lines)) => {
            eprint!("{lines}");
            ExitCode::from(1)
        }
        Err(Failure::Invalid(text)) => {
            eprint!("{text}");
            ExitCode::from(2)
        }
    }
}

/// The `.php` files under each `--include` path, in byte order of their paths, read for
/// their symbols only; a file that cannot be read, is not UTF-8 or has a path below the
/// include path that is not UTF-8 is skipped. A path that
/// does not exist, or is neither a file nor a directory, is refused (P2R-340).
fn included_files(paths: &[PathBuf]) -> Result<Vec<convert::Included>, String> {
    let mut included = Vec::new();
    for (index, root) in paths.iter().enumerate() {
        let shown = root.display().to_string();
        let mut relative = Vec::new();
        if root.is_dir() {
            php_files(root, Path::new(""), &mut relative)
                .map_err(|err| format!("{shown}: cannot read the include path: {err}"))?;
        } else if root.is_file() {
            relative.push(PathBuf::new());
        } else if root.exists() {
            return Err(format!(
                "{shown}: the include path is neither a file nor a directory"
            ));
        } else {
            return Err(format!("{shown}: the include path does not exist"));
        }
        relative.sort_by(|a, b| {
            a.as_os_str()
                .as_encoded_bytes()
                .cmp(b.as_os_str().as_encoded_bytes())
        });
        for path in relative {
            // Mago knows the file by a name no input can have, the same on every machine:
            // the option's position and the path below it. A path that is not UTF-8 is
            // skipped, because a lossy name could equal another file's.
            let Some(below) = path
                .components()
                .map(|part| part.as_os_str().to_str())
                .collect::<Option<Vec<_>>>()
            else {
                continue;
            };
            let name = format!("\0include {index}/{}", below.join("/"));
            let file = if path.as_os_str().is_empty() {
                root.clone()
            } else {
                root.join(&path)
            };
            let Ok(source) = fs::read(&file) else {
                continue;
            };
            if std::str::from_utf8(&source).is_err() {
                continue;
            }
            included.push(convert::Included { name, source });
        }
    }
    Ok(included)
}

/// Adds the path below `dir` of every regular `.php` file under it to `found`, without
/// following symbolic links (P2R-170).
fn php_files(root: &Path, dir: &Path, found: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(root.join(dir))? {
        let entry = entry?;
        let kind = entry.file_type()?;
        let path = dir.join(entry.file_name());
        if kind.is_dir() {
            php_files(root, &path, found)?;
        } else if kind.is_file() && entry.file_name().as_encoded_bytes().ends_with(b".php") {
            found.push(path);
        }
    }
    Ok(())
}

/// Refuses a crate output that exists, whose parent directory does not exist, or that is
/// inside the input directory (P2R-170).
fn check_crate_output(input: &Path, output: &Path) -> Result<(), String> {
    let shown = output.display();
    if output.symlink_metadata().is_ok() {
        return Err(format!("{shown}: the output directory already exists"));
    }
    let parent = match output.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    };
    // A regular file as the parent fails here too, before any conversion.
    let (Ok(parent), Ok(input)) = (fs::canonicalize(parent), fs::canonicalize(input)) else {
        return Err(format!(
            "{shown}: the output's parent directory does not exist"
        ));
    };
    if !parent.is_dir() {
        return Err(format!(
            "{shown}: the output's parent directory does not exist"
        ));
    }
    if parent.starts_with(&input) {
        return Err(format!(
            "{shown}: the output directory is inside the input directory"
        ));
    }
    Ok(())
}

/// The output files, each a path in the crate and its text, laid out by rustfmt from
/// `root`, the file that names the others (P2R-190). rustfmt runs in a new directory under
/// the system's temporary directory, with an empty configuration file, so no
/// `rustfmt.toml` near the output or in the user's configuration changes the layout; a
/// second run with `--check` proves the layout settled.
fn laid_out(
    shown: &str,
    files: Vec<(String, String)>,
    root: &str,
) -> Result<Vec<(String, String)>, Failure> {
    let result = private_dir().and_then(|dir| {
        let result = format_in(&dir, files, root);
        let _ = fs::remove_dir_all(&dir);
        result
    });
    result.map_err(|reason| {
        Failure::Invalid(format!("{shown}: cannot lay out the output: {reason}\n"))
    })
}

/// A new directory under the system's temporary directory that only this user can enter.
/// It must not exist yet: on a shared temporary directory another user could have made
/// a directory or a symbolic link of a guessed name.
fn private_dir() -> Result<PathBuf, String> {
    let base = std::env::temp_dir();
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |time| time.as_nanos());
    for attempt in 0..100 {
        let dir = base.join(format!("php2rust-{}-{stamp}-{attempt}", std::process::id()));
        let mut builder = fs::DirBuilder::new();
        #[cfg(unix)]
        std::os::unix::fs::DirBuilderExt::mode(&mut builder, 0o700);
        match builder.create(&dir) {
            Ok(()) => return Ok(dir),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(format!("cannot create a temporary directory: {err}")),
        }
    }
    Err("cannot create a temporary directory".to_owned())
}

fn format_in(
    dir: &Path,
    files: Vec<(String, String)>,
    root: &str,
) -> Result<Vec<(String, String)>, String> {
    let config = dir.join("rustfmt.toml");
    let tree = dir.join("crate");
    let io = |err: std::io::Error| format!("cannot write a temporary file: {err}");
    fs::create_dir_all(&tree).map_err(io)?;
    fs::write(&config, "").map_err(io)?;
    for (path, text) in &files {
        let target = tree.join(path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(io)?;
        }
        fs::write(target, text).map_err(io)?;
    }
    // One fixed message for each failure, so stderr never holds a temporary path (P2R-06).
    let rustfmt = |check: bool| {
        let mut command = Command::new("rustfmt");
        // A rustup proxy would otherwise download a missing toolchain.
        command.env("RUSTUP_AUTO_INSTALL", "0");
        command
            .args(["--edition", "2024", "--config-path"])
            .arg(&config);
        if check {
            command.arg("--check");
        }
        command
            .arg(tree.join(root))
            .output()
            .map_err(|_| "cannot run rustfmt, which must be on PATH".to_owned())
    };
    if !rustfmt(false)?.status.success() {
        return Err("rustfmt failed on the output".to_owned());
    }
    if !rustfmt(true)?.status.success() {
        return Err("rustfmt changes its own layout when it runs again".to_owned());
    }
    files
        .into_iter()
        .map(|(path, _)| {
            let text = fs::read_to_string(tree.join(&path))
                .map_err(|err| format!("cannot read a temporary file: {err}"))?;
            Ok((path, text))
        })
        .collect()
}

/// Writes the crate into a temporary directory beside `output` and renames it into place,
/// so a failed write never leaves a partial crate (P2R-170).
fn write_crate(output: &Path, files: &[(String, String)]) -> std::io::Result<()> {
    let mut temporary = output.as_os_str().to_owned();
    temporary.push(".php2rust-tmp");
    let temporary = PathBuf::from(temporary);
    // A temporary directory left by another run fails here, so its files never join the
    // crate, and it is not removed.
    fs::create_dir(&temporary)?;
    let result = files
        .iter()
        .try_for_each(|(path, text)| {
            let target = temporary.join(path);
            if let Some(dir) = target.parent() {
                fs::create_dir_all(dir)?;
            }
            fs::write(target, text)
        })
        .and_then(|()| fs::rename(&temporary, output));
    if result.is_err() {
        let _ = fs::remove_dir_all(&temporary);
    }
    result
}

/// Refuses an output path that resolves to the input file, or whose directory does not
/// exist (P2R-04).
fn check_output(input: &Path, output: &Path) -> Result<(), String> {
    let shown = output.display();
    if let (Ok(a), Ok(b)) = (fs::canonicalize(input), fs::canonicalize(output))
        && a == b
    {
        return Err(format!("{shown}: the output file is the input file"));
    }
    let parent = match output.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    };
    if !parent.is_dir() {
        return Err(format!("{shown}: the output directory does not exist"));
    }
    Ok(())
}

/// Writes through a temporary file in the same directory and renames it into place, so a
/// failed write never leaves a partial output file (P2R-05).
fn write_atomically(output: &Path, text: &str) -> std::io::Result<()> {
    let mut temporary = output.as_os_str().to_owned();
    temporary.push(".php2rust-tmp");
    let temporary = PathBuf::from(temporary);
    let result = fs::write(&temporary, text).and_then(|()| fs::rename(&temporary, output));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

fn invalid(reason: String) -> ExitCode {
    eprintln!("{reason}");
    ExitCode::from(2)
}
