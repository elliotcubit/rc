use super::codec::Codec;
use super::error::Error;

pub struct Utf8Codec {}

impl Codec for Utf8Codec {
    fn decode(s: Vec<u8>) -> Result<Vec<u8>, Error> {
        match String::from_utf8(s.clone()) {
            Ok(_) => Ok(s),
            Err(_) => Err(Error::new("invalid utf8".to_string())),
        }
    }

    // TODO well, this can fail... or at least be invalid
    fn encode(data: Vec<u8>) -> String {
        data.into_iter().map(|v| v as char).collect()
    }
}
