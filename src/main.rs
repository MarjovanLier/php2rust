mod convert;
mod names;
mod printer;
mod problem;
mod types;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

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
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    if cli.input.is_dir() {
        return convert_directory(&cli.input, cli.output.as_deref());
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
    match convert::convert(&path, source) {
        Ok(rust) => match &cli.output {
            None => {
                print!("{rust}");
                ExitCode::SUCCESS
            }
            Some(output) => match write_atomically(output, &rust) {
                Ok(()) => ExitCode::SUCCESS,
                Err(err) => invalid(format!("{}: cannot write output: {err}", output.display())),
            },
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

/// Converts every `.php` file under `input` into one crate in the new directory `output`
/// (P2R-170).
fn convert_directory(input: &Path, output: Option<&Path>) -> ExitCode {
    let Some(output) = output else {
        return invalid(format!(
            "{}: a directory input needs -o with the directory to create for the crate",
            input.display()
        ));
    };
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
        let name = path.to_string_lossy().replace('\\', "/");
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
    match convert::convert_crate(inputs) {
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

/// Adds the path below `dir` of every regular `.php` file under it to `found`, without
/// following symbolic links (P2R-170).
fn php_files(root: &Path, dir: &Path, found: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(root.join(dir))? {
        let entry = entry?;
        let kind = entry.file_type()?;
        let path = dir.join(entry.file_name());
        if kind.is_dir() {
            php_files(root, &path, found)?;
        } else if kind.is_file() && path.extension().is_some_and(|e| e == "php") {
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
    let (Ok(parent), Ok(input)) = (fs::canonicalize(parent), fs::canonicalize(input)) else {
        return Err(format!(
            "{shown}: the output's parent directory does not exist"
        ));
    };
    if parent.starts_with(&input) {
        return Err(format!(
            "{shown}: the output directory is inside the input directory"
        ));
    }
    Ok(())
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
