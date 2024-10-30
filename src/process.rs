// we are using Serde, it can Cast the input data to internal data structure
// and then cast to output.  
// Serilize is a trait that can be derived to make a serde data structure serializable to JSON, XML, etc.
// Deserialize is a trait that can be derived to make a struct deserializable to serde data structure.
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::fs;
use csv::Reader;
use anyhow::Result;

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

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
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
                let json_value = headers.iter().zip(record.iter()).
                    collect::<Value>();
                ret.push(json_value);
            }
            
            // ret in here is a Vec<Player> that will be used to store the parsed CSV data
            // let json = serde_json::to_string(&ret)?;
            // fs is used to write the json to the output file
            // fs::write(output, json)?; // it will return an ()
            Ok(())
}