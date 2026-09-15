use anyhow::{Context, Result};
use clap::Parser;
use std::io::{BufRead, BufReader};
use std::process::Command;
// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct CLI {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

pub fn execute() -> Result<()> {
    let args = CLI::parse();

    let file = std::fs::File::open(&args.path)
        .with_context(|| format!("could not open file '{}'", args.path.display()))?;
    let content = BufReader::new(file);

    let installer = Command::new("installer")
        .arg("-pkg")
        .arg(&args.path)
        .arg("-target")
        .arg("/")
        .status()?;

    Ok(())
}
