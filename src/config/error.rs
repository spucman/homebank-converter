use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ConfigurationError {
    #[error("Unable to find file: {0}")]
    FileNotFound(String),
    #[error("Unalbe to load config file: {0}")]
    Hocon(#[from] hocon::Error),
}
