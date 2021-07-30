use super::codec::Codec;
use super::error::Error;
use crate::Format;

pub struct SpellingCodec {}

impl Codec for SpellingCodec {
    fn format(&self) -> Format {
        Format::Spelling
    }

    fn decode(&self, s: Vec<u8>) -> Result<Vec<u8>, Error> {
        String::from_utf8(s.clone())
            .map(|s| {
                s.to_lowercase()
                    .split_whitespace()
                    .map(Self::word_to_char)
                    .collect::<Result<String, Error>>()
            })
            .unwrap_or(Err(Error::new("input data is not utf8".to_string())))
            .map(|s| s.into_bytes())
    }

    fn encode(&self, data: Vec<u8>) -> Result<String, Error> {
        match String::from_utf8(data.clone()) {
            Ok(s)
                if s.chars()
                    .all(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace()) =>
            {
                Ok(s.trim()
                    .to_ascii_lowercase()
                    .split_ascii_whitespace()
                    .map(|word| {
                        word.chars()
                            .map(Self::char_to_word)
                            .collect::<Vec<String>>()
                            .join(" ")
                    })
                    .collect::<Vec<String>>()
                    .join(" "))
            }
            Ok(_) => Err(Error::new("input data is not ascii".to_string())),
            Err(_) => Err(Error::new("input data is not utf8".to_string())),
        }
    }
}

impl SpellingCodec {
    fn char_to_word(c: char) -> String {
        match c {
            'a' => "Alpha",
            'b' => "Bravo",
            'c' => "Charlie",
            'd' => "Delta",
            'e' => "Echo",
            'f' => "Foxtrot",
            'g' => "Golf",
            'h' => "Hotel",
            'i' => "India",
            'j' => "Juliet",
            'k' => "Kilo",
            'l' => "Lima",
            'm' => "Mike",
            'n' => "November",
            'o' => "Oscar",
            'p' => "Papa",
            'q' => "Quebec",
            'r' => "Romeo",
            's' => "Sierra",
            't' => "Tango",
            'u' => "Uniform",
            'v' => "Victor",
            'w' => "Whiskey",
            'x' => "X-Ray",
            'y' => "Yankee",
            'z' => "Zulu",
            // We verify this before using the method
            _ => panic!("input to char_to_word was not ascii"),
        }
        .to_string()
    }

    fn word_to_char(s: &str) -> Result<char, Error> {
        match s {
            "alpha" => Ok('a'),
            "bravo" => Ok('b'),
            "charlie" => Ok('c'),
            "delta" => Ok('d'),
            "echo" => Ok('e'),
            "foxtrot" => Ok('f'),
            "golf" => Ok('g'),
            "hotel" => Ok('h'),
            "india" => Ok('i'),
            "juliet" => Ok('j'),
            "kilo" => Ok('k'),
            "lima" => Ok('l'),
            "mike" => Ok('m'),
            "november" => Ok('n'),
            "oscar" => Ok('o'),
            "papa" => Ok('p'),
            "quebec" => Ok('q'),
            "romeo" => Ok('r'),
            "sierra" => Ok('s'),
            "tango" => Ok('t'),
            "uniform" => Ok('u'),
            "victor" => Ok('v'),
            "whiskey" => Ok('w'),
            "x-ray" => Ok('x'),
            "yankee" => Ok('y'),
            "zulu" => Ok('z'),
            _ => Err(Error::new(format!(
                "{} is not part of the spelling alphabet",
                s
            ))),
        }
    }
}
