use hex::FromHexError;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    err: String,
}

impl From<FromHexError> for Error {
    fn from(e: FromHexError) -> Error {
        Error::new(e.to_string())
    }
}

impl Error {
    pub fn new(e: String) -> Self {
        Self { err: e }
    }
}
