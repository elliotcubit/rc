use super::codec::Codec;
use super::error::Error;

pub struct AsciiCodec {}

impl Codec for AsciiCodec {
    fn decode(s: &str) -> Result<Vec<u8>, Error> {
        s.bytes()
            .into_iter()
            .map(|v| {
                if v & 0b10000000 > 0 {
                    Err(Error::new("Invalid character for ascii".to_string()))
                } else {
                    Ok(v)
                }
            })
            .collect::<Result<Vec<u8>, Error>>()
    }

    // TODO well, this can fail... or at least be invalid
    fn encode(data: Vec<u8>) -> String {
        data.into_iter().map(|v| v as char).collect()
    }
}
