use encoding::all::UTF_8;
use encoding::{EncoderTrap, Encoding};
use std::str::from_utf8;

pub fn encode_replace(string: &str) -> Result<String, String> {
    match UTF_8.encode(string, EncoderTrap::Replace) {
        Ok(bytes) => match from_utf8(&bytes) {
            Ok(data) => Ok(String::from(data)),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
