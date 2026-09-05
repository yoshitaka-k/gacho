use std::{path, error, fmt};

pub type Result<T> = std::result::Result<T, GachoError>;

#[derive(Debug)]
pub enum GachoError {
    FileNotFound(String, path::PathBuf),
    FileError(String, path::PathBuf),
    ArchiveError(String),
    InvalidVersion,
}

/// GachoError を表示
impl fmt::Display for GachoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GachoError::FileNotFound(e, path) => write!(f, "File does not exist: {}\n\n{}", e, path.display()),
            GachoError::FileError(e, path) => write!(f, "File error: {}\n\n{}", e, path.display()),
            GachoError::ArchiveError(e) => write!(f, "Archive error: {}", e),
            GachoError::InvalidVersion => write!(f, "Invalid version"),
        }
    }
}

/// GachoError を表示
impl error::Error for GachoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            GachoError::FileNotFound(_, _) => None,
            GachoError::FileError(_, _) => None,
            GachoError::ArchiveError(_) => None,
            GachoError::InvalidVersion => None,
        }
    }
}
