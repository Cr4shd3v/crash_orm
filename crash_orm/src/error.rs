//! Contains an [error enum](Error) for all types of errors in the ORM.
//!
//! Also contains a [result](Result) with that error type.

use std::fmt::{Debug, Display, Formatter};

/// Type alias for Result with the custom [Error]
pub type Result<T> = std::result::Result<T, Error>;

/// Error enum holding all possible errors that can be thrown by the ORM.
///
/// This does implement [Error](std::error::Error) and contains tokio-postgres [Error](tokio_postgres::Error) if any occur.
#[derive(Debug)]
pub enum Error {
    /// Variant for [tokio_postgres::Error]
    Postgres(tokio_postgres::Error),
    /// Variant for custom error message
    String(String),
}

impl Error {
    /// Shortcut method to create an error with custom error message.
    #[doc(hidden)]
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
