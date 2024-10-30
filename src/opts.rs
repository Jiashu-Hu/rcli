use std::path::Path;

use clap::Parser;
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
}

/// Options for the csv subcommand, parsed to [`SubCommand::Csv(CsvOpts)`](SubCommand::Csv)
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

/// verify_input_file is a value parser for the [`CsvOpts::input`] argument, it will check if the file exists.  
/// Please note that this function is only check for file name, not the content of the file, it will
/// return ok if the file exists, otherwise it will return an error.

// static: if there any thing has the same lifetime with process, it can be static
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // path::new is a function from the std::path module
    // it will create a Path object from the filename
    // exists() is a method of the Path object, it will return true if the file exists
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}