use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Format {
    B2,
    B8,
    B10,
    B16,
    Ascii,
    Utf8,
    Hex,
    Base64,
    Binary,
    Raw,
    Inferred,
}

impl Format {
    pub fn all_variants() -> Vec<&'static str> {
        vec![
            "b2", "b8", "b10", "b16", "utf8", "hex", "base64", "binary", "raw",
        ]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "b2" => Some(Self::B2),
            "b8" => Some(Self::B8),
            "b10" => Some(Self::B10),
            "b16" => Some(Self::B16),
            "ascii" => Some(Self::Ascii),
            "utf8" => Some(Self::Utf8),
            "hex" => Some(Self::Hex),
            "base64" => Some(Self::Base64),
            "binary" => Some(Self::Binary),
            "raw" => Some(Self::Raw),
            "__infer" => Some(Self::Inferred),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::B2 => "base 2",
            Self::B8 => "base 8",
            Self::B10 => "base 10",
            Self::B16 => "base 16",
            Self::Ascii => "ascii",
            Self::Utf8 => "utf8",
            Self::Hex => "hex",
            Self::Base64 => "base 64",
            Self::Binary => "binary",
            Self::Raw => "raw bytes",
            // This shouldn't really happen, since if we ever have an inferred enum
            // we should be converting it to a relevant one
            Self::Inferred => "inferred",
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.to_str();
        write!(f, "{}", s)
    }
}
