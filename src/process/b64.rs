use crate::{get_reader, Base64Format};
use anyhow::{Ok, Result};

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use serde::de;

/// process_encode will encode the input file to base64 and print the result to stdout.
/// input is a file path, if input is "-", it will read from stdin.
/// we use Box::new to create a Box<dyn Read> because we can't use File::open("-") to read from stdin.
/// to make return type same for both if and else, we use Box::new to make it a Box<dyn Read>
pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    println!("input: {}, format: {}", input, format);
    let mut reader = get_reader(input)?;

    // we use buf to store the read data is because we can't use reader.read_to_end(&mut Vec::new())
    // because it will return an error, because Vec::new() is a temporary value
    // ! because the return type of reader might be different, we use Box::new to make it a Box<dyn Read>
    // ! buf is from Read trait, so it can be used to store the data
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim(); // remove trailing newline
    println!("{:?}", buf);
    println!("{:?}", buf);

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "Cargo.toml";
        let format = Base64Format::UrlSafe;
        process_decode(input, format).unwrap();
    }
}
