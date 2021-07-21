#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    err: String,
}

impl Error {
    pub fn new(e: String) -> Self {
        Self { err: e }
    }
}
