use super::error::Error;
use crate::options::Format;

pub trait Codec {
    fn decode(&self, s: Vec<u8>) -> Result<Vec<u8>, Error>;
    fn encode(&self, data: Vec<u8>) -> String;
    fn format(&self) -> Format;
}
