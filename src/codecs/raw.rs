use super::codec::Codec;
use super::error::Error;

pub struct RawCodec {}

impl Codec for RawCodec {
    fn decode(s: Vec<u8>) -> Result<Vec<u8>, Error> {
        Ok(s)
    }

    fn encode(data: Vec<u8>) -> String {
        data.into_iter().map(|v| v as char).collect()
    }
}
