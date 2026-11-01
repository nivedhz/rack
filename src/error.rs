#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("fatal: not inside a rack repository")]
    NotARepository,
    #[error("fatal: {path} {src}")]
    IO { path: String, src: String },
}
pub type Result<T> = std::result::Result<T, Error>;
