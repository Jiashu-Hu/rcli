// we are using Serde, it can Cast the input data to internal data structure
// and then cast to output.
// Serilize is a trait that can be derived to make a serde data structure serializable to JSON, XML, etc.
// Deserialize is a trait that can be derived to make a struct deserializable to serde data structure.
use anyhow;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

use crate::cli::OutputFormat;

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

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    // Reader::from_path is a function from the csv module
    // ? is to do a try, if the result is an error, it will return the error
    /* it just like the following code
     * match reader {
     *   Ok(reader) => reader,
     *   Err(e) => return Err(e),}
     */
    // a benefit of anyhow is it can convert any error to anyhow::Error
    // so error in here can be converted to anyhow::Error
    let mut reader = Reader::from_path(input)?;
    // ret is a Vec<Player> that will be used to store the parsed CSV data
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    //diffrence between deserialize and records is deserialize will return a single record
    // records will return an iterator of records
    for result in reader.records() {
        let record = result?;
        // headers.iter() -> it will return an iterator of headers
        // zip(record.iter()) -> it will return an iterator of tuple that combine the headers and record iterator [(header, record), ...]
        // collect::<Value>() -> it will convert the iterator of tuple to a JSON Value
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value);
    }

    // content is a string that will be used to store the serialized data
    // serde_json, serde_yaml, and toml are used to serialize the data, it will convert the data to a string
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}
