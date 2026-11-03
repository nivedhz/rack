#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("fatal: IO error in {path} {src}")]
    IO { path: String, src: String },
    #[error("fatal: not a directory")]
    NotADir,
    #[error("fatal: path has no parent")]
    NoParent,
    #[error("fatal: not a rack repository (or any parent up to mount point /)")]
    NotInRackRepo,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO {
            path: err.to_string(),
            src: err.to_string(),
        }
    }
}
impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::IO {
            path: err.to_string(),
            src: err.to_string(),
        }
    }
}
impl From<tempfile::PersistError> for Error {
    fn from(err: tempfile::PersistError) -> Self {
        Error::IO {
            path: err.to_string(),
            src: err.to_string(),
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;
