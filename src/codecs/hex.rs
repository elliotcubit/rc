use super::codec::Codec;
use super::error::Error;
use crate::Format;

pub struct HexCodec {}

impl Codec for HexCodec {
    fn format(&self) -> Format {
        Format::Hex
    }

    fn decode(&self, s: Vec<u8>) -> Result<Vec<u8>, Error> {
        if s.len() % 2 == 1 {
            return Err(Error::new(
                "Invalid number of characters for hex string".to_string(),
            ));
        }
        s.into_iter()
            .map(|v| v as char)
            .map(Self::char_to_nibble)
            .collect::<Result<Vec<u8>, Error>>()
            .map(|nibbles| {
                nibbles
                    .chunks(2)
                    .map(|ns| (ns[0] << 4) | (ns[1] & 0b00001111))
                    .collect::<Vec<u8>>()
            })
    }

    fn encode(&self, data: Vec<u8>) -> Result<String, Error> {
        Ok(data
            .into_iter()
            .map(|byte| {
                format!(
                    "{}{}",
                    Self::nibble_to_char(byte >> 4),
                    Self::nibble_to_char(byte & 0b00001111)
                )
            })
            .collect::<Vec<String>>()
            .join(""))
    }
}

impl HexCodec {
    fn char_to_nibble(c: char) -> Result<u8, Error> {
        Ok(match c {
            c if c >= '0' && c <= '9' => (c as u8 - 48),
            c if c >= 'A' && c <= 'F' => (c as u8 - 55),
            c if c >= 'a' && c <= 'f' => (c as u8 - 87),
            _ => return Err(Error::new(format!("Invalid hex character {}", c))),
        })
    }

    fn nibble_to_char(n: u8) -> char {
        (match n & 0b00001111 {
            v if v <= 9 => 48 + v,
            v if v <= 15 => 87 + v,
            _ => panic!("Hex character encoding was passed more than 4 bits"),
        }) as char
    }
}

#[test]
fn encode() {
    use std::collections::HashMap;

    let tests: HashMap<&str, Vec<u8>> = [(
        "6162636465666768696a6b6c6d6e6f707172737475767778797a",
        "abcdefghijklmnopqrstuvwxyz".as_bytes().to_vec(),
    )]
    .iter()
    .cloned()
    .collect();

    let codec = HexCodec {};
    for (expected, bytes) in tests {
        assert_eq!(expected, codec.encode(bytes).unwrap());
    }
}

#[test]
fn decode() {
    use std::collections::HashMap;

    let tests: HashMap<Vec<u8>, Result<Vec<u8>, Error>> = [
        (
            "6162636465666768696a6b6c6d6e6f707172737475767778797a"
                .as_bytes()
                .to_vec(),
            Ok("abcdefghijklmnopqrstuvwxyz".as_bytes().to_vec()),
        ),
        (
            "ffgg".as_bytes().to_vec(),
            Err(Error::new("Invalid hex character g".to_string())),
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let codec = HexCodec {};
    for (data, expected) in tests {
        assert_eq!(expected, codec.decode(data));
    }
}
