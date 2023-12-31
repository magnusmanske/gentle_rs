use std::error::Error;
use std::fmt;

use gb_io::reader::GbParserError;

#[derive(Debug)]
pub enum GENtleError {
    String(String),
    Io(std::io::Error),
    Serde(serde_json::Error),
    GbParser(GbParserError),
}

impl Error for GENtleError {}

impl fmt::Display for GENtleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self) // user-facing output
    }
}

impl From<String> for GENtleError {
    fn from(err: String) -> Self {
        GENtleError::String(err)
    }
}


impl From<std::io::Error> for GENtleError {
    fn from(err: std::io::Error) -> Self {
        GENtleError::Io(err)
    }
}

impl From<serde_json::Error> for GENtleError {
    fn from(err: serde_json::Error) -> Self {
        GENtleError::Serde(err)
    }
}

impl From<GbParserError> for GENtleError {
    fn from(err: GbParserError) -> Self {
        GENtleError::GbParser(err)
    }
}

