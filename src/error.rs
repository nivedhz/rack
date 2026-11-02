#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{path} {src}")]
    IO { path: String, src: String },
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
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
