//! Basic error handling mechanisms
use std::error::Error;
use std::{fmt, io};

/// The result type for the Parsing
pub type PeelResult<'a, T> = Result<T, PeelError>;

/// Representation for an error of the library
pub struct PeelError {
    /// The error variant
    pub code: ErrorType,

    /// Additional description for the error
    pub description: String,

    /// The cause for this error
    pub cause: Option<Box<Error>>,
}

impl fmt::Display for PeelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Code: {:?}, Description: {}",
               self.code,
               self.description)
    }
}

impl fmt::Debug for PeelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for PeelError {
    fn description(&self) -> &str {
        &self.description
    }
}

// Error conversion
macro_rules! from_error {
    ($($p:ty,)*) => (
        $(impl From<$p> for PeelError {
            fn from(err: $p) -> PeelError {
                PeelError {
                    code: ErrorType::Other,
                    description: err.description().to_owned(),
                    cause: Some(Box::new(err)),
                }
            }
        })*
    )
}

from_error! {
    io::Error,
}

#[derive(Debug, PartialEq, Eq)]
/// Error codes as indicator what happened
pub enum ErrorType {
    /// New nodes have to be added before traversing
    NoTreeRoot,

    /// The root parser failed already
    RootParserFailed,

    /// The error originates from another error
    Other,
}

/// Throw an internal error
pub fn bail(code: ErrorType, description: &fmt::Display) -> PeelError {
    PeelError {
        code: code,
        description: description.to_string(),
        cause: None,
    }
}

macro_rules! bail {($code:expr, $($fmt:tt)*) => (
    return Err(::error::bail($code, &format_args!($($fmt)*)))
)}
