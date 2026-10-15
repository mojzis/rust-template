//! Thin entry point: parse arguments, set up logging, delegate to
//! [`rust_template::cli::Cli::run`], and map its outcome onto an exit code.

use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use rust_template::cli::Cli;

/// Exit code for a run that could not complete.
const EXIT_ERROR: u8 = 2;

fn main() -> ExitCode {
    let cli = Cli::parse();
    init_tracing(cli.verbose);

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    match cli.run(&mut out).and_then(|()| Ok(out.flush()?)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("rust-template: {err:#}");
            ExitCode::from(EXIT_ERROR)
        }
    }
}

/// `RUST_LOG` wins when set; `--verbose` only raises the default level.
fn init_tracing(verbose: bool) {
    let default = if verbose { "debug" } else { "warn" };
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default));
    tracing_subscriber::fmt().with_env_filter(filter).with_writer(io::stderr).init();
}
