//! Command-line interface: argument definitions and command bodies.

use std::io::Write;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::greeting::greet;

/// A Rust CLI project template.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// Log at debug level (`RUST_LOG` overrides).
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print a greeting.
    Greet {
        /// Who to greet.
        #[arg(default_value = "world")]
        name: String,
    },
}

impl Cli {
    /// Run the selected command, writing its output to `out`.
    pub fn run(&self, out: &mut impl Write) -> Result<()> {
        match &self.command {
            Command::Greet { name } => writeln!(out, "{}", greet(name))?,
        }
        Ok(())
    }
}
