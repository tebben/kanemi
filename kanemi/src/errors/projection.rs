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
