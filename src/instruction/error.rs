

#[derive(Debug)]
pub enum Error {
    InsufficientBytes,
    InvalidEncoding,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::InsufficientBytes => f.write_str("insufficient bytes"),
            Error::InvalidEncoding => f.write_str("invalid encoding"),
        }
    }
}

impl std::error::Error for Error {}


