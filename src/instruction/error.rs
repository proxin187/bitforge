

#[derive(Debug)]
pub enum Error {
    InsufficientBytes,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::InsufficientBytes => f.write_str("insufficient bytes"),
        }
    }
}

impl std::error::Error for Error {}


