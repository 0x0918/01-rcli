use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
    str::FromStr,
};

use anyhow::{anyhow, Error, Ok, Result};
use clap::Parser;

use super::{verify_file, verify_path};
use crate::{process_text_generate, process_text_sign, process_text_verify, CmdExecutor};
use std::fs::*;

#[derive(Debug, Parser)]
pub enum TextSubcommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,

    #[arg(short, long)]
    pub sig: String,
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow!("invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExecutor for TextSubcommand {
    async fn execute(&self) -> Result<()> {
        match self {
            TextSubcommand::Sign(opts) => opts.execute().await,

            TextSubcommand::Verify(opts) => opts.execute().await,
            TextSubcommand::Generate(opts) => opts.execute().await,
        }
    }
}

impl CmdExecutor for TextSignOpts {
    async fn execute(&self) -> Result<()> {
        let sign = process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", sign);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(&self) -> Result<()> {
        let verified = process_text_verify(&self.input, &self.key, self.format, &self.sig)?;
        println!("{}", verified);
        Ok(())
    }
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(&self) -> Result<()> {
        let key = process_text_generate(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                write(name, &key[0])?;
            }
            TextSignFormat::Ed25519 => {
                let name = &self.output;
                write(name.join("ed25519.sk"), &key[0])?;
                write(name.join("ed25519.pk"), &key[1])?;
            }
        }
        Ok(())
    }
}
