mod csv;
mod genpass;
mod base64;

use std::path::Path;

use clap::Parser;
use csv::CsvOpts;
use genpass::GenPassOpts;

pub use self::{csv::OutputFormat, base64::{Base64SubCommand, Base64Format}};

#[derive(Debug, Parser)]
// name = "rcli" is the name of the command,
// it will get version and author from the Cargo.toml
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

/// Subcommands of the rcli command, typed to [`Opts::cmd`]
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show csv or convert csv to other formats")]
    // CsvOpts is the struct that will be used to parse the arguments
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate a random password")]
    GenPass(GenPassOpts),
    // reason why we use subcommand here is because we have two subcommands for base64
    #[command(subcommand)]
    Base64(Base64SubCommand),
}


/// verify_input_file is a value parser for the [`CsvOpts::input`] argument, it will check if the file exists.  
/// Please note that this function is only check for file name, not the content of the file, it will
/// return ok if the file exists, otherwise it will return an error.

// static: if there any thing has the same lifetime with process, it can be static
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    //if  input is "-" or file exists
    if filename == "-" || Path::new(filename).exists(){
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("nonexistent"), Err("File does not exist"));
    }
}