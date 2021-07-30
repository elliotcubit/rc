use super::codec::Codec;
use super::error::Error;
use crate::Format;

pub struct Base64Codec {}

impl Codec for Base64Codec {
    fn format(&self) -> Format {
        Format::Base64
    }

    fn decode(&self, s: Vec<u8>) -> Result<Vec<u8>, Error> {
        if s.len() % 4 != 0 {
            Err(Error::new(
                "Invalid number of characters for base64 string".to_string(),
            ))
        } else {
            let mut tail_pd = 0;
            s.into_iter()
                .map(|c| match Self::char_to_val(c as char) {
                    // If we have a valid character come after a padding, error out
                    // Otherwise just pass values along
                    Ok(Some(_)) if tail_pd > 0 => {
                        Err(Error::new("Non-tailing padding".to_string()))
                    }
                    Ok(None) => {
                        tail_pd += 1;
                        if tail_pd > 2 {
                            Err(Error::new(
                                "Only two padding bytes are allowed for base64".to_string(),
                            ))
                        } else {
                            Ok(None)
                        }
                    }
                    pass => pass,
                })
                .collect::<Result<Vec<Option<u8>>, Error>>()
                .map(|bytes| {
                    bytes
                        .chunks(4)
                        .flat_map(|v| {
                            match (v[0], v[1], v[2], v[3]) {
                                (Some(v0), Some(v1), None, None) => {
                                    vec![(v0 << 2) | (v1 >> 4)].into_iter()
                                }
                                (Some(v0), Some(v1), Some(v2), None) => {
                                    vec![(v0 << 2) | (v1 >> 4), (v1 << 4) | (v2 >> 2)].into_iter()
                                }
                                (Some(v0), Some(v1), Some(v2), Some(v3)) => vec![
                                    (v0 << 2) | (v1 >> 4),
                                    (v1 << 4) | (v2 >> 2),
                                    (v2 << 6) | v3,
                                ]
                                .into_iter(),
                                // If this happens, the at-most-two-and-only-tailing padding checks
                                // above failed, so something is wrong enough to fail ungracefully
                                _ => panic!("Base64 format validation failed!"),
                            }
                        })
                        .collect()
                })
        }
    }

    fn encode(&self, data: Vec<u8>) -> Result<String, Error> {
        Ok(data
            .chunks(3)
            .flat_map(|group| match group.len() {
                1 => vec![
                    Some((group[0] & 0b11111100) >> 2),
                    Some((group[0] & 0b00000011) << 4),
                    None,
                    None,
                ]
                .into_iter(),
                2 => vec![
                    Some((group[0] & 0b11111100) >> 2),
                    Some(((group[0] & 0b00000011) << 4) | (group[1] & 0b11110000) >> 4),
                    Some((group[1] & 0b00001111) << 2),
                    None,
                ]
                .into_iter(),
                3 => vec![
                    Some((group[0] & 0b11111100) >> 2),
                    Some(((group[0] & 0b00000011) << 4) | (group[1] & 0b11110000) >> 4),
                    Some(((group[1] & 0b00001111) << 2) | (group[2] & 0b11000000) >> 6),
                    Some(group[2] & 0b00111111),
                ]
                .into_iter(),
                // chunks() is guaranteed to return between length 1 and 3
                _ => panic!("Error while encoding base64"),
            })
            .map(|ch| match ch {
                Some(v) => Self::val_to_char(v),
                None => '=',
            })
            .collect())
    }
}

impl Base64Codec {
    pub fn char_to_val(c: char) -> Result<Option<u8>, Error> {
        if c == '=' {
            Ok(None)
        } else {
            Ok(Some(match c {
                c if c >= 'A' && c <= 'Z' => c as u8 - 65,
                c if c >= 'a' && c <= 'z' => c as u8 - 71,
                c if c >= '0' && c <= '9' => c as u8 + 4,
                c if c == '+' => 62,
                c if c == '/' => 63,
                _ => return Err(Error::new(format!("Invalid char: {}", c))),
            }))
        }
    }

    pub fn val_to_char(v: u8) -> char {
        (match v & 0b00111111 {
            z if z <= 25 => 'A' as u8 + z,
            z if z <= 51 => 'a' as u8 + (z - 26),
            z if z <= 61 => '0' as u8 + (z - 52),
            z if z == 62 => '+' as u8,
            z if z == 63 => '/' as u8,
            _ => panic!("Base64 character encoding was passed more than 6 bits"),
        }) as char
    }
}

#[test]
fn encode() {
    use std::collections::HashMap;

    let tests: HashMap<&str, Vec<u8>> = [
        ("aGVsbG8gd29ybGQ=", "hello world".as_bytes().to_vec()),
        ("dHdvcGFkcw==", "twopads".as_bytes().to_vec()),
        ("dGhyZWVwYWRz", "threepads".as_bytes().to_vec()),
        ("", "".as_bytes().to_vec()),
        ("Zg==", "f".as_bytes().to_vec()),
        ("Zm8=", "fo".as_bytes().to_vec()),
        ("Zm9v", "foo".as_bytes().to_vec()),
        ("Zm9vYmFy", "foobar".as_bytes().to_vec()),
        // "A test string that includes all 64 possible Base64 symbols"
        // Thank you David Cary for your 2011 StackOverflow answer :)
        (
            "U28/PHA+VGhpcyA0LCA1LCA2LCA3LCA4LCA5LCB6LCB7LCB8LCB9IHRlc3RzIEJhc2U2NCBlbmNvZGVyLiBTaG93IG1lOiBALCBBLCBCLCBDLCBELCBFLCBGLCBHLCBILCBJLCBKLCBLLCBMLCBNLCBOLCBPLCBQLCBRLCBSLCBTLCBULCBVLCBWLCBXLCBYLCBZLCBaLCBbLCBcLCBdLCBeLCBfLCBgLCBhLCBiLCBjLCBkLCBlLCBmLCBnLCBoLCBpLCBqLCBrLCBsLCBtLCBuLCBvLCBwLCBxLCByLCBzLg==",
            "So?<p>This 4, 5, 6, 7, 8, 9, z, {, |, } tests Base64 encoder. Show me: @, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, [, \\, ], ^, _, `, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s.".as_bytes().to_vec()
        )
    ]
    .iter()
    .cloned()
    .collect();

    let codec = Base64Codec {};
    for (expected, bytes) in tests {
        assert_eq!(expected, codec.encode(bytes).unwrap());
    }
}

#[test]
fn decode() {
    use std::collections::HashMap;

    let tests: HashMap<Vec<u8>, Result<Vec<u8>, Error>> = [
        ("aGVsbG8gd29ybGQ=".as_bytes().to_vec(), Ok("hello world".as_bytes().to_vec())),
        ("dHdvcGFkcw==".as_bytes().to_vec(), Ok("twopads".as_bytes().to_vec())),
        ("dGhyZWVwYWRz".as_bytes().to_vec(), Ok("threepads".as_bytes().to_vec())),
        ("".as_bytes().to_vec(), Ok("".as_bytes().to_vec())),
        ("Zg==".as_bytes().to_vec(), Ok("f".as_bytes().to_vec())),
        ("Zm8=".as_bytes().to_vec(), Ok("fo".as_bytes().to_vec())),
        ("Zm9v".as_bytes().to_vec(), Ok("foo".as_bytes().to_vec())),
        ("Zm9vYmFy".as_bytes().to_vec(), Ok("foobar".as_bytes().to_vec())),
        // "A test string that includes all 64 possible Base64 symbols"
        // Thank you David Cary for your 2011 StackOverflow answer :)
        (
            "U28/PHA+VGhpcyA0LCA1LCA2LCA3LCA4LCA5LCB6LCB7LCB8LCB9IHRlc3RzIEJhc2U2NCBlbmNvZGVyLiBTaG93IG1lOiBALCBBLCBCLCBDLCBELCBFLCBGLCBHLCBILCBJLCBKLCBLLCBMLCBNLCBOLCBPLCBQLCBRLCBSLCBTLCBULCBVLCBWLCBXLCBYLCBZLCBaLCBbLCBcLCBdLCBeLCBfLCBgLCBhLCBiLCBjLCBkLCBlLCBmLCBnLCBoLCBpLCBqLCBrLCBsLCBtLCBuLCBvLCBwLCBxLCByLCBzLg==".as_bytes().to_vec(),
            Ok("So?<p>This 4, 5, 6, 7, 8, 9, z, {, |, } tests Base64 encoder. Show me: @, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, [, \\, ], ^, _, `, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s.".as_bytes().to_vec())
        ),
        ("Z====".as_bytes().to_vec(), Err(Error::new("Invalid number of characters for base64 string".to_string()))),
        ("Z===".as_bytes().to_vec(), Err(Error::new("Only two padding bytes are allowed for base64".to_string()))),
        ("=ZZZ".as_bytes().to_vec(), Err(Error::new("Non-tailing padding".to_string())))
    ]
    .iter()
    .cloned()
    .collect();

    let codec = Base64Codec {};
    for (bytes, expected) in tests {
        assert_eq!(expected, codec.decode(bytes));
    }
}
