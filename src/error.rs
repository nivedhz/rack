#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("not inside a rack repository")]
    NotARepository,
    #[error("{path} {src}")]
    IO { path: String, src: String },
    #[error("missing config\nuse 'rack config <key> <value>' to set a config value")]
    MissingConfig,
}
pub type Result<T> = std::result::Result<T, Error>;
