use std::fmt;

use crate::harmonie_cy43_p1::reader::GribError;

/// Errors that can occur within the API
#[derive(Debug)]
pub enum CY43P1Error {
    /// Filename is not valied, should be in format provided by KNMI
    InvalidFilename(String),
    InvalidDirectory(String),
    ReaderError(GribError),
    TarError(String),
}

impl fmt::Display for CY43P1Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CY43P1Error::InvalidFilename(s) => write!(f, "Invalid filename: {}", s),
            CY43P1Error::InvalidDirectory(s) => write!(f, "Invalid folder: {}", s),
            CY43P1Error::ReaderError(e) => write!(f, "Reader error: {}", e),
            CY43P1Error::TarError(s) => write!(f, "Tar error: {}", s),
        }
    }
}

impl std::error::Error for CY43P1Error {}

impl From<GribError> for CY43P1Error {
    fn from(e: GribError) -> Self {
        CY43P1Error::ReaderError(e)
    }
}
