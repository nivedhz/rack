#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("fatal: {src}")]
    IO { src: String },
    #[error("fatal: {path} has no parent")]
    NoParent { path: String },
    #[error("fatal: not a rack repository (or any parent up to mount point /)")]
    NotInRackRepo,
    #[error("fatal: path '{path}' did not match any files")]
    PathNotFound { path: String },
    #[error("fatal: path '{path}' is a dir and hence didn't match any files")]
    PathIsDir { path: String },
    #[error("fatal: {message}\nvalid keys: user.name, user.email")]
    InvalidConfig { message: String },
    #[error("fatal: config not found\n{message}")]
    ConfigNotFound { message: String },
    #[error("fatal: no changes added to commit! ({message})")]
    NoFileAdded { message: String },
    #[error("fatal: no commits found! ({message})")]
    NoCommitsFound { message: String },
    #[error("fatal: {message}")]
    Convert { message: String },
}
impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Convert {
            message: err.to_string(),
        }
    }
}
impl From<std::convert::Infallible> for Error {
    fn from(err: std::convert::Infallible) -> Self {
        Error::IO {
            src: err.to_string(),
        }
    }
}
impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::IO {
            src: err.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO {
            src: err.to_string(),
        }
    }
}
impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::IO {
            src: err.to_string(),
        }
    }
}
impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::IO {
            src: err.to_string(),
        }
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::IO {
            src: err.to_string(),
        }
    }
}
impl From<std::path::StripPrefixError> for Error {
    fn from(err: std::path::StripPrefixError) -> Self {
        Error::IO {
            src: err.to_string(),
        }
    }
}
impl From<tempfile::PersistError> for Error {
    fn from(err: tempfile::PersistError) -> Self {
        Error::IO {
            src: err.to_string(),
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;
