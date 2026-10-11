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
    /// The PHP file to convert.
    input: PathBuf,
    /// Write the Rust source to this file instead of stdout.
    #[arg(short = 'o', value_name = "OUTPUT")]
    output: Option<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
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
