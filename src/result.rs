use std::{error::Error, fmt::Display};

/// Errors tied to this crates runtime.
#[derive(Debug)]
pub enum CrateError {
    /// The expected directory was not found.
    DirNotFound(String),
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
            Self::IoError(e) => write!(f, "General IO Error ({e})"),
        }
    }
}

impl Error for CrateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DirNotFound(_) => None,
            Self::IoError(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for CrateError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

/// The crate-standard [Result] type.
pub type CrateResult<T> = Result<T, CrateError>;
