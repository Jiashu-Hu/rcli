use std::{fmt, path::Path, str::FromStr};

use clap::Parser;

use super::verify_input_file;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

/// Options for the csv subcommand, parsed to [`SubCommand::Csv(CsvOpts)`](SubCommand::Csv)
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    
    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_parser = parse_format ,default_value = "json")]
    pub format: OutputFormat,
    
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

/// parse_format is a value parser for the [`CsvOpts::format`] argument, it will parse the format string to [`OutputFormat`].
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // parse() may turn the string to an different type, but this type has to implement FromStr
    format.parse::<OutputFormat>()
}

// From is a trait that can be implemented for a type to convert it to another type
// in here, we implement From<OutputFormat> for &'static str, which means we can convert OutputFormat to &'static str
/// Implement the conversion from [`OutputFormat`] to `&'static str`, this will be used to convert the format to a string.
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

// FromStr is a trait that can be implemented for a type to parse a string to the type
// in here, we implement FromStr for OutputFormat, which means we can parse a string to OutputFormat
/// Implement the conversion from `&str` to [`OutputFormat`], this will be used to parse the format from a string.
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
    
}