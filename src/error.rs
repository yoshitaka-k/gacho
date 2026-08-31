use std::{path, error, fmt, io};

pub type Result<T> = std::result::Result<T, GachoError>;

#[derive(Debug)]
pub enum GachoError {
    FileNotFound(path::PathBuf),
    FileError(String, path::PathBuf),
    LockPoisoned,
    InvalidVersion,
    IconLoadFailed,
    Io(io::Error),
}

/// GachoError を表示
impl fmt::Display for GachoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GachoError::FileNotFound(path) => write!(f, "File does not exist: {}", path.display()),
            GachoError::FileError(e, path) => write!(f, "File error: {} \n\n{}", e, path.display()),
            GachoError::LockPoisoned => write!(f, "Lock poisoned"),
            GachoError::InvalidVersion => write!(f, "Invalid version"),
            GachoError::IconLoadFailed => write!(f, "Icon load failed"),
            GachoError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

/// GachoError を表示
impl error::Error for GachoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            GachoError::FileNotFound(_) => None,
            GachoError::FileError(_, _) => None,
            GachoError::LockPoisoned => None,
            GachoError::InvalidVersion => None,
            GachoError::IconLoadFailed => None,
            GachoError::Io(ref e) => Some(e),
        }
    }
}

/// std::io::Error を GachoError に変換
impl From<io::Error> for GachoError {
    fn from(e: io::Error) -> Self {
        GachoError::Io(e)
    }
}
