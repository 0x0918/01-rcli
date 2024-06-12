mod base64;
mod csv;
mod genpass;
mod text;

use std::path::{Path, PathBuf};

pub use base64::{Base64Format, Base64Subcommand};
use clap::Parser;
use csv::CsvOpts;
use genpass::GenpassOpts;
pub use text::{TextSignFormat, TextSubcommand};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenpassOpts),
    #[command(subcommand)]
    Base64(Base64Subcommand),
    #[command(subcommand)]
    Text(TextSubcommand),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

fn verify_file(filename: &str) -> Result<PathBuf, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn to_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}
