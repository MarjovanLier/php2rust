mod convert;
mod printer;
mod problem;

use std::fs;
use std::path::PathBuf;
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
    match convert::convert(&path, source) {
        Ok(rust) => match &cli.output {
            None => {
                print!("{rust}");
                ExitCode::SUCCESS
            }
            Some(output) => match fs::write(output, rust) {
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

fn invalid(reason: String) -> ExitCode {
    eprintln!("{reason}");
    ExitCode::from(2)
}
