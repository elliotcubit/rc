use super::codec::Codec;
use super::error::Error;
use crate::Format;

pub struct RawCodec {}

impl Codec for RawCodec {
    fn format(&self) -> Format {
        Format::Raw
    }

    fn decode(&self, s: Vec<u8>) -> Result<Vec<u8>, Error> {
        Ok(s)
    }

    // TODO this aint it
    fn encode(&self, data: Vec<u8>) -> Result<String, Error> {
        Ok(data.into_iter().map(|v| v as char).collect())
    }
}
