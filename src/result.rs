use std::{error::Error, fmt::Display, num::ParseIntError};

/// Errors tied to this crates runtime.
#[derive(Debug)]
pub enum CrateError {
    /// The expected directory was not found.
    DirNotFound(String),
    DataNotFound(String),
    WrongType((&'static str, &'static str)),
    GeneralError(String),
    FilePathing(&'static str),
    IntParsing(ParseIntError),
    IoError(std::io::Error),
    NetworkError(ureq::Error),
}

impl CrateError {
    pub fn general(msg: impl ToString) -> Self {
        Self::GeneralError(msg.to_string())
    }

    pub fn dir_not_found(dir: impl ToString) -> Self {
        Self::DirNotFound(dir.to_string())
    }

    pub fn wrong_type(expected: &'static str, actual: &'static str) -> Self {
        Self::WrongType((expected, actual))
    }
}

impl Display for CrateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirNotFound(dir) => write!(f, "Directory not found (\"{dir}\")"),
            Self::DataNotFound(data) => write!(f, "Missing data '{data}'"),
            Self::FilePathing(msg) => write!(f, "File path parsing error (\"{msg}\")"),
            Self::GeneralError(msg) => write!(f, "General error: '{msg}'"),
            Self::IntParsing(e) => write!(f, "Integer parsing error (\"{e}\")"),
            Self::IoError(e) => write!(f, "General IO Error ({e})"),
            Self::NetworkError(e) => write!(f, "Network/HTTP error ({e})"),
            Self::WrongType((e, a)) => write!(f, "Wrong data type. Expected: {e}, Actual: {a}"),
        }
    }
}

impl Error for CrateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IntParsing(e) => Some(e),
            Self::IoError(e) => Some(e),
            Self::NetworkError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for CrateError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<std::num::ParseIntError> for CrateError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::IntParsing(value)
    }
}

impl From<ureq::Error> for CrateError {
    fn from(value: ureq::Error) -> Self {
        Self::NetworkError(value)
    }
}

/// The crate-standard [Result] type.
pub type CrateResult<T> = Result<T, CrateError>;
