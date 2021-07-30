use super::codec::Codec;
use super::error::Error;
use crate::Format;

pub struct Utf8Codec {}

impl Codec for Utf8Codec {
    fn format(&self) -> Format {
        Format::Utf8
    }

    fn decode(&self, s: Vec<u8>) -> Result<Vec<u8>, Error> {
        match String::from_utf8(s.clone()) {
            Ok(_) => Ok(s),
            Err(_) => Err(Error::new("invalid utf8".to_string())),
        }
    }

    fn encode(&self, data: Vec<u8>) -> Result<String, Error> {
        String::from_utf8(data).map_err(|_| Error::new("invalid utf8".to_string()))
    }
}
