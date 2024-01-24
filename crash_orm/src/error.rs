use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Postgres(tokio_postgres::Error),
    String(String),
}

impl Error {
    pub fn from_str(error: &str) -> Self {
        Self::String(String::from(error))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Postgres(error) => std::fmt::Display::fmt(&error, f),
            Error::String(error) => std::fmt::Display::fmt(&error, f),
        }
    }
}

impl std::error::Error for Error {}

impl From<tokio_postgres::Error> for Error {
    fn from(value: tokio_postgres::Error) -> Self {
        Self::Postgres(value)
    }
}
