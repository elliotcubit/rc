use super::error::Error;

pub trait Codec {
    fn decode(s: Vec<u8>) -> Result<Vec<u8>, Error>;
    fn encode(data: Vec<u8>) -> String;
    // Would decode() succeed?
    // fn infer(s: &str) -> bool;
}
