use std::fmt;

/// Errors that can occur during the coordinate projection process.
#[derive(Debug)]
pub enum ProjectionError {
    /// The input coordinates are outside the valid grid boundaries.
    OutOfBounds(String),
    /// An error occurred during the coordinate transformation.
    CoordinateError(String),
}

impl fmt::Display for ProjectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProjectionError::OutOfBounds(s) => write!(f, "Out of bounds: {}", s),
            ProjectionError::CoordinateError(s) => write!(f, "Coordinate error: {}", s),
        }
    }
}

/// Errors that can occur during the coordinate projection process.
#[derive(Debug)]
pub enum DatasetError {
    /// The input coordinates are outside the valid grid boundaries.
    OutOfBounds(String),
    /// An error occurred during reading the HDF5 gile.
    FileNotFound(String),
    /// An error occurred during reading the dataset.
    ReadError(String),
}

impl fmt::Display for DatasetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatasetError::OutOfBounds(s) => write!(f, "Out of bounds: {}", s),
            DatasetError::FileNotFound(s) => write!(f, "File not found: {}", s),
            DatasetError::ReadError(s) => write!(f, "Read error: {}", s),
        }
    }
}
