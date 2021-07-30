use super::codec::Codec;
use super::error::Error;
use crate::Format;

pub struct Rot13Codec {}

impl Codec for Rot13Codec {
    fn format(&self) -> Format {
        Format::Rot13
    }

    fn decode(&self, s: Vec<u8>) -> Result<Vec<u8>, Error> {
        String::from_utf8(s.clone())
            .map(|s| {
                s.to_lowercase()
                    .chars()
                    .map(Self::cnv)
                    .collect::<Result<String, Error>>()
                    .map(|s| s.into_bytes())
            })
            .unwrap_or(Err(Error::new("input data is not utf8".to_string())))
    }

    fn encode(&self, data: Vec<u8>) -> Result<String, Error> {
        String::from_utf8(data.clone())
            .map(|s| {
                s.to_lowercase()
                    .chars()
                    .map(Self::cnv)
                    .collect::<Result<String, Error>>()
            })
            .unwrap_or(Err(Error::new("input data is not utf8".to_string())))
    }

    fn inferrable(&self) -> bool {
        false
    }
}

impl Rot13Codec {
    fn cnv(c: char) -> Result<char, Error> {
        let val = c as u8;
        if val >= 'a' as u8 && val <= ('z' as u8 - 13) {
            Ok((val + 13) as char)
        } else if val > ('z' as u8 - 13) && val <= 'z' as u8 {
            Ok((((val + 12) % ('z' as u8)) + 'a' as u8) as char)
        } else {
            Ok(val as char)
        }
    }
}
