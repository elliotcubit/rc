use super::codec::Codec;
use super::error::Error;

pub struct Base64Codec {}

impl Codec for Base64Codec {
    // grrrrrr bark bark woiof woof
    fn decode(s: &str) -> Result<Vec<u8>, Error> {
        if s.len() % 4 != 0 {
            Err(Error::new(
                "Invalid number of characters for base64 string".to_string(),
            ))
        } else {
            let mut tail_pd = 0;
            s.chars()
                .map(|c| match Self::char_to_val(c) {
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

    fn encode(data: Vec<u8>) -> String {
        data.chunks(3)
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
            .collect()
    }
}

// TODO This could be way better, since most of them are
// sequential, but I went the lazy route for now
impl Base64Codec {
    pub fn char_to_val(c: char) -> Result<Option<u8>, Error> {
        if c == '=' {
            Ok(None)
        } else {
            Ok(Some(match c {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                'D' => 3,
                'E' => 4,
                'F' => 5,
                'G' => 6,
                'H' => 7,
                'I' => 8,
                'J' => 9,
                'K' => 10,
                'L' => 11,
                'M' => 12,
                'N' => 13,
                'O' => 14,
                'P' => 15,
                'Q' => 16,
                'R' => 17,
                'S' => 18,
                'T' => 19,
                'U' => 20,
                'V' => 21,
                'W' => 22,
                'X' => 23,
                'Y' => 24,
                'Z' => 25,
                'a' => 26,
                'b' => 27,
                'c' => 28,
                'd' => 29,
                'e' => 30,
                'f' => 31,
                'g' => 32,
                'h' => 33,
                'i' => 34,
                'j' => 35,
                'k' => 36,
                'l' => 37,
                'm' => 38,
                'n' => 39,
                'o' => 40,
                'p' => 41,
                'q' => 42,
                'r' => 43,
                's' => 44,
                't' => 45,
                'u' => 46,
                'v' => 47,
                'w' => 48,
                'x' => 49,
                'y' => 50,
                'z' => 51,
                '0' => 52,
                '1' => 53,
                '2' => 54,
                '3' => 55,
                '4' => 56,
                '5' => 57,
                '6' => 58,
                '7' => 59,
                '8' => 60,
                '9' => 61,
                '+' => 62,
                '/' => 63,
                _ => return Err(Error::new(format!("Invalid char: {}", c))),
            }))
        }
    }

    pub fn val_to_char(v: u8) -> char {
        match v & 0b00111111 {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            8 => 'I',
            9 => 'J',
            10 => 'K',
            11 => 'L',
            12 => 'M',
            13 => 'N',
            14 => 'O',
            15 => 'P',
            16 => 'Q',
            17 => 'R',
            18 => 'S',
            19 => 'T',
            20 => 'U',
            21 => 'V',
            22 => 'W',
            23 => 'X',
            24 => 'Y',
            25 => 'Z',
            26 => 'a',
            27 => 'b',
            28 => 'c',
            29 => 'd',
            30 => 'e',
            31 => 'f',
            32 => 'g',
            33 => 'h',
            34 => 'i',
            35 => 'j',
            36 => 'k',
            37 => 'l',
            38 => 'm',
            39 => 'n',
            40 => 'o',
            41 => 'p',
            42 => 'q',
            43 => 'r',
            44 => 's',
            45 => 't',
            46 => 'u',
            47 => 'v',
            48 => 'w',
            49 => 'x',
            50 => 'y',
            51 => 'z',
            52 => '0',
            53 => '1',
            54 => '2',
            55 => '3',
            56 => '4',
            57 => '5',
            58 => '6',
            59 => '7',
            60 => '8',
            61 => '9',
            62 => '+',
            63 => '/',
            _ => panic!("Base64 character encoding was passed more than 6 bits"),
        }
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

    for (expected, bytes) in tests {
        assert_eq!(expected, Base64Codec::encode(bytes));
    }
}

#[test]
fn decode() {
    use std::collections::HashMap;

    let tests: HashMap<&str, Result<Vec<u8>, Error>> = [
        ("aGVsbG8gd29ybGQ=", Ok("hello world".as_bytes().to_vec())),
        ("dHdvcGFkcw==", Ok("twopads".as_bytes().to_vec())),
        ("dGhyZWVwYWRz", Ok("threepads".as_bytes().to_vec())),
        ("", Ok("".as_bytes().to_vec())),
        ("Zg==", Ok("f".as_bytes().to_vec())),
        ("Zm8=", Ok("fo".as_bytes().to_vec())),
        ("Zm9v", Ok("foo".as_bytes().to_vec())),
        ("Zm9vYmFy", Ok("foobar".as_bytes().to_vec())),
        // "A test string that includes all 64 possible Base64 symbols"
        // Thank you David Cary for your 2011 StackOverflow answer :)
        (
            "U28/PHA+VGhpcyA0LCA1LCA2LCA3LCA4LCA5LCB6LCB7LCB8LCB9IHRlc3RzIEJhc2U2NCBlbmNvZGVyLiBTaG93IG1lOiBALCBBLCBCLCBDLCBELCBFLCBGLCBHLCBILCBJLCBKLCBLLCBMLCBNLCBOLCBPLCBQLCBRLCBSLCBTLCBULCBVLCBWLCBXLCBYLCBZLCBaLCBbLCBcLCBdLCBeLCBfLCBgLCBhLCBiLCBjLCBkLCBlLCBmLCBnLCBoLCBpLCBqLCBrLCBsLCBtLCBuLCBvLCBwLCBxLCByLCBzLg==",
            Ok("So?<p>This 4, 5, 6, 7, 8, 9, z, {, |, } tests Base64 encoder. Show me: @, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, [, \\, ], ^, _, `, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s.".as_bytes().to_vec())
        ),
        ("Z====", Err(Error::new("Invalid number of characters for base64 string".to_string()))),
        ("Z===", Err(Error::new("Only two padding bytes are allowed for base64".to_string()))),
        ("=ZZZ", Err(Error::new("Non-tailing padding".to_string())))
    ]
    .iter()
    .cloned()
    .collect();

    for (bytes, expected) in tests {
        assert_eq!(expected, Base64Codec::decode(bytes));
    }
}
