#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("fatal: not inside a rack repository")]
    NotARepository,
    #[error("fatal: {path} {src}")]
    IO { path: String, src: String },
    #[error("fatal: missing config\nuse 'rack config <key> <value>' to set a config value")]
    MissingConfig,
}
pub type Result<T> = std::result::Result<T, Error>;
