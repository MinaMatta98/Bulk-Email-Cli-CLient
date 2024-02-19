use std::env::VarError;

use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum EmailingError {
    #[error("Error: {source}")]
    EnvError {
        #[from]
        source: VarError,
    },
    #[error("Error sending email: {source}")]
    LettreError {
        #[from]
        source: lettre::error::Error,
    },
    #[error("I/O Error: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error("Error dealing with Content Type: {source}")]
    ContentType {
        #[from]
        source: lettre::message::header::ContentTypeErr,
    },
    #[error("Error Parsing Email Address: {source}")]
    AddressError {
        #[from]
        source: lettre::address::AddressError,
    },
    #[error("Error joining tokio tasks: {source}")]
    JoinError {
        #[from]
        source: JoinError,
    },
    #[error("Error handling CSV file: {source}")]
    CsvError {
        #[from]
        source: csv::Error,
    },
}
