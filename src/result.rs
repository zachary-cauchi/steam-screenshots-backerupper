use std::{error::Error, fmt::Display};

/// Errors tied to this crates runtime.
#[derive(Debug)]
pub enum CrateError {
    /// The expected directory was not found.
    DirNotFound(&'static str),
}

/// The crate-standard [Result] type.
pub type CrateResult<T> = Result<T, CrateError>;

impl Display for CrateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirNotFound(dir) => write!(f, "Directory not found (\"{dir}\")"),
        }
    }
}

impl Error for CrateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DirNotFound(_) => None,
        }
    }
}
