#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    // use this to initialize a progress bar to the application
    // let pb = indicatif::ProgressBar::new(100);

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;

    grrs::find_matches(
        &content,
        &args.pattern,
        &mut io::BufWriter::new(io::stdout().lock()),
    );
    Ok(())
}
