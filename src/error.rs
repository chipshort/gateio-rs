use std::fmt::Display;

use thiserror::Error;
use serde::Deserialize;

#[derive(Error, Debug)]
pub enum Error {
    #[error("http error {0}")]
    Http(reqwest::Error),

    #[error("{0}")]
    GateIO(GateIOError),

    #[error("invalid server response")]
    ParseError,

    #[error("endpoint requires authentication")]
    AuthRequired,

    #[error("invalid secret key")]
    InvalidKey,
    
    #[error("unknown error")]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub struct GateIOError {
    label: String,
    message: String
}

impl Display for GateIOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.label, self.message)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Http(e)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(_: reqwest::header::InvalidHeaderValue) -> Self {
        Self::Unknown
    }
}