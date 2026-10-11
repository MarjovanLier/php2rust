use std::path::PathBuf;
use std::process::ExitCode;

use clap::{ColorChoice, Parser};

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
    // The conversion comes in the next task; reading the fields keeps rustc quiet until then.
    let _ = (&cli.input, &cli.output);
    ExitCode::SUCCESS
}
