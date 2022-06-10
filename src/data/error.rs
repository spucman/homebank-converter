use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("unable to initialize csv reader: {0}")]
    UnableToInitializeCSVReader(#[from] csv::Error),
}
