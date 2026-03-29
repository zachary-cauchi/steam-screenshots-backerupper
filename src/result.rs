use std::{error::Error, fmt::Display, num::ParseIntError};

/// Errors tied to this crates runtime.
#[derive(Debug)]
pub enum CrateError {
    /// The expected directory was not found.
    DirNotFound(String),
    FilePathing(&'static str),
    IntParsing(ParseIntError),
    IoError(std::io::Error),
}

impl CrateError {
    pub fn dir_not_found(dir: impl ToString) -> Self {
        Self::DirNotFound(dir.to_string())
    }
}

impl Display for CrateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirNotFound(dir) => write!(f, "Directory not found (\"{dir}\")"),
            Self::FilePathing(msg) => write!(f, "File path parsing error (\"{msg}\")"),
            Self::IntParsing(e) => write!(f, "Integer parsing error (\"{e}\")"),
            Self::IoError(e) => write!(f, "General IO Error ({e})"),
        }
    }
}

impl Error for CrateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DirNotFound(_) | Self::FilePathing(_) => None,
            Self::IntParsing(e) => Some(e),
            Self::IoError(e) => Some(e),
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

/// The crate-standard [Result] type.
pub type CrateResult<T> = Result<T, CrateError>;
