use std::{fmt, io, path::PathBuf};

#[derive(Debug)]
pub enum BigFileError {
    Io {
        file: Option<PathBuf>,
        offset: Option<usize>,
        err: io::Error,
    },
    EntryNotFound(PathBuf),
}

pub type Result<T> = core::result::Result<T, BigFileError>;

impl fmt::Display for BigFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            BigFileError::Io { file, offset, err } => {
                if let Some(file) = file {
                    write!(f, "{}", file.display())?;
                }

                if let Some(offset) = offset {
                    write!(f, " at offset {offset}")?;
                }

                write!(f, ": {}", err)
            }
            BigFileError::EntryNotFound(p) => write!(f, "Couldn't find the entry {}", p.display()),
        };
    }
}

impl From<io::Error> for BigFileError {
    fn from(value: io::Error) -> Self {
        BigFileError::Io {
            file: None,
            err: value,
            offset: None,
        }
    }
}

impl std::error::Error for BigFileError {}

pub(crate) trait IoErrorExt {
    fn with_file(self, file: PathBuf) -> BigFileError;
    fn with_offset(self, file: Option<PathBuf>, offset: Option<usize>) -> BigFileError;
}

impl IoErrorExt for io::Error {
    fn with_file(self, file: PathBuf) -> BigFileError {
        BigFileError::Io {
            file: Some(file),
            offset: None,
            err: self,
        }
    }

    fn with_offset(self, file: Option<PathBuf>, offset: Option<usize>) -> BigFileError {
        BigFileError::Io {
            file,
            offset,
            err: self,
        }
    }
}

pub(crate) trait IoResultExt<T> {
    fn with_file(self, file: PathBuf) -> Result<T>;
    fn with_offset(self, file: Option<PathBuf>, offset: Option<usize>) -> Result<T>;
}

impl<T> IoResultExt<T> for io::Result<T> {
    fn with_file(self, file: PathBuf) -> Result<T> {
        self.map_err(|e| e.with_file(file))
    }

    fn with_offset(self, file: Option<PathBuf>, offset: Option<usize>) -> Result<T> {
        self.map_err(|e| e.with_offset(file, offset))
    }
}
