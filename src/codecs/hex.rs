use super::codec::Codec;
use super::error::Error;
use hex::{decode, encode};

pub struct HexCodec {}

impl Codec for HexCodec {
    fn decode(s: &str) -> Result<Vec<u8>, Error> {
        let mut g = s.to_string();
        if s.len() % 2 == 1 {
            g = "0".to_string();
            g.push_str(s)
        }
        Ok(decode(g)?)
    }

    fn encode(data: Vec<u8>) -> String {
        encode(data)
    }
}
