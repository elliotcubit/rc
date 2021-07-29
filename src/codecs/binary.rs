use super::codec::Codec;
use super::error::Error;
use crate::Format;

pub struct BinaryCodec {}

impl Codec for BinaryCodec {
    fn format(&self) -> Format {
        Format::Binary
    }

    fn decode(&self, s: Vec<u8>) -> Result<Vec<u8>, Error> {
        match String::from_utf8(s.clone())
            .map(|v| {
                let mut new = v.clone();
                new.retain(|c| !c.is_whitespace());
                new
            })
            .map(|v| {
                v.into_bytes()
                    .into_iter()
                    .map(|c| {
                        if c == 0x30 || c == 0x31 {
                            Ok(c - 0x30)
                        } else {
                            Err(Error::new(format!(
                                "Invalid character for binary {}",
                                c as char
                            )))
                        }
                    })
                    .collect::<Result<Vec<u8>, Error>>()
                    .map(|bits| {
                        if bits.len() % 8 != 0 {
                            Err(Error::new(
                                "Invalid number of characters for binary".to_string(),
                            ))
                        } else {
                            Ok(bits
                                .chunks(8)
                                .map(|byte| {
                                    byte[0] << 7
                                        | byte[1] << 6
                                        | byte[2] << 5
                                        | byte[3] << 4
                                        | byte[4] << 3
                                        | byte[5] << 2
                                        | byte[6] << 1
                                        | byte[7]
                                })
                                .collect::<Vec<u8>>())
                        }
                    })
            }) {
            // There has to be an easier way to flatten these results
            Ok(v) => match v {
                Ok(v) => v,
                Err(e) => Err(e),
            },
            Err(_) => Err(Error::new("Input to binary was invalid utf8".to_string())),
        }
    }

    fn encode(&self, data: Vec<u8>) -> String {
        data.into_iter()
            .map(|byte| {
                format!(
                    "{}{}{}{}{}{}{}{}",
                    byte >> 7,
                    byte >> 6 & 1,
                    byte >> 5 & 1,
                    byte >> 4 & 1,
                    byte >> 3 & 1,
                    byte >> 2 & 1,
                    byte >> 1 & 1,
                    byte & 1
                )
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}
