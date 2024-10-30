use std::{fs, path::Path};
// we are using Serde, it can Cast the input data to internal data structure
// and then cast to output.  
// Serilize is a trait that can be derived to make a serde data structure serializable to JSON, XML, etc.
// Deserialize is a trait that can be derived to make a struct deserializable to serde data structure.
use serde::{Serialize, Deserialize};
// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use csv::Reader;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Player struct that will be used to parse the CSV file. 
/// The field name is the same as the CSV header. 
/// we use serde to rename the field name to match the CSV header
struct Player {
    // the reason why i can define different name here is because of the serde(rename = "Name")
    // it will rename the field to "Name" when serializing and deserializing
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

#[derive(Debug, Parser)]

// name = "rcli" is the name of the command,
// it will get version and author from the Cargo.toml
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

/// Subcommands of the rcli command, typed to [`Opts::cmd`]
#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "show csv or convert csv to other formats")]
    // CsvOpts is the struct that will be used to parse the arguments
    Csv(CsvOpts),
}

/// Options for the csv subcommand, parsed to [`SubCommand::Csv(CsvOpts)`](SubCommand::Csv)
#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,
    
    #[arg(short, long, default_value = "output.json")]
    output: String,
    
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() -> anyhow::Result<()> {
    // opts will finally receive Opts::cmd which is SubCommand::Csv(CsvOpts)
    // which is the struct of CsvOpts
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            // Reader::from_path is a function from the csv module
            // ? is to do a try, if the result is an error, it will return the error
            /* it just like the following code
             * match reader {
             *   Ok(reader) => reader,
             *   Err(e) => return Err(e),}
             */
            // a benefit of anyhow is it can convert any error to anyhow::Error
            // so error in here can be converted to anyhow::Error
            let mut reader = Reader::from_path(opts.input)?;
            // ret is a Vec<Player> that will be used to store the parsed CSV data
            let mut ret = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let record: Player = result?;
                println!("{:?}", record);
                ret.push(record);
            }
            
            // ret in here is a Vec<Player> that will be used to store the parsed CSV data
            let json = serde_json::to_string(&ret)?;
            // fs is used to write the json to the output file
            fs::write(opts.output, json)?;
        }
    }

    Ok(())
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