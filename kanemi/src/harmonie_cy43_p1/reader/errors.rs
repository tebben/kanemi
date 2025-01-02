use std::fmt;

#[derive(Debug)]
pub enum GribError {
    /// The input coordinates are outside the valid grid boundaries.
    OutOfBounds(String),
    /// An error occurred during reading the HDF5 gile.
    FileNotFound(String),
    /// An error occurred during reading the dataset.
    ReadError(String),
    /// A given parameter is not found
    ParameterNotFound(String),
}

impl fmt::Display for GribError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GribError::OutOfBounds(s) => write!(f, "Out of bounds: {}", s),
            GribError::FileNotFound(s) => write!(f, "File not found: {}", s),
            GribError::ReadError(s) => write!(f, "Read error: {}", s),
            GribError::ParameterNotFound(s) => write!(f, "Parameter not found: {}", s),
        }
    }
}
