use super::{verify_file, verify_path};
use clap::Parser;
use std::{
    fmt::{self, write},
    path::PathBuf,
    str::FromStr,
};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "sign a message with a private key/shared key")]
    Sign(TextSignOpts),
    #[command(about = "verify a signed message with a public key/shared key")]
    Verify(TextVerifyOpts),
    #[command(about = "generate a key")]
    Generate(TextKeyGenerateOpts),
}

/// TextSignOpts is the options for the `TextSubCommand::Sign` subcommand
/// it has two arguments, `input` and `key`, input is the message to sign, and key is the key to sign the message with
/// both arguments are optional, and default to `-` which means stdin
#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

/// TextVerifyOpts is the options for the `TextSubCommand::Verify` subcommand
/// it has two arguments, `input` and `key`, input is the signed message to verify, and key is the key to verify the message with
/// both arguments are optional, and default to `-` which means stdin
/// it also has a `sig` argument which is the signature to verify the message with
/// this argument is required, it will come from the user
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

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

/// parse_format is a value parser for the [`TextSignOpts::format`] argument, it will parse the format string to [`TextSignFormat`].
/// it will return an error if the format is invalid
fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

/// when we create a enum (especially for those who need interact with string),
/// we may implement FromStr trait for it, so that we can parse a string to the enum
impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

/// when we want to convert an enum to a string, we can implement From trait for it
impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
