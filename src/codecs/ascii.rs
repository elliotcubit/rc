use super::codec::Codec;
use super::error::Error;

pub struct AsciiCodec {}

impl Codec for AsciiCodec {
    fn decode(s: &str) -> Result<Vec<u8>, Error> {
        Ok(s.as_bytes().to_vec())
    }

    fn encode(data: Vec<u8>) -> String {
        data.into_iter().map(|v| v as char).collect()
    }
}
