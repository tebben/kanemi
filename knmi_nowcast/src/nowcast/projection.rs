//! This module contains functions to convert longitude and latitude coordinates to the HDF5 nowcast grid.
//!
//! It uses the `proj4rs` crate to convert coordinates from WGS84 to the Polar stereographic projection
//! using the proj4 string provided in the HDF5 file.
//!
//! ## Grid Details
//! The grid is a 700x765 grid with the following boundaries:
//! - Bottom left: (0.0, 49.362064361572266)
//! - Top left: (0.0, 55.973602294921875)
//! - Top right: (10.856452941894531, 55.388973236083984)
//! - Bottom right: (9.009300231933594, 48.895301818847656)
//!
//! ## HDF5 Parameters
//! - `GEO_ROW_OFFSET`: 3649.98193359375
//! - `GEO_COLUMN_OFFSET`: 0.0
//! - `GEO_NUMBER_OF_ROWS`: 765
//! - `GEO_NUMBER_OF_COLUMNS`: 700
//! - `GEO_PIXEL_SIZE_X`: 1.000003457069397
//! - `GEO_PIXEL_SIZE_Y`: 1.000004768371582

use super::errors::ProjectionError;
use once_cell::sync::Lazy;
use proj4rs;
use proj4rs::proj::Proj;

/// Offset for the rows in the HDF5 nowcast grid.
pub const GEO_ROW_OFFSET: f64 = 3649.98193359375;

/// Total number of rows in the grid.
pub const GEO_NUMBER_OF_ROWS: f64 = 765.0;

/// Total number of columns in the grid.
pub const GEO_NUMBER_OF_COLUMNS: f64 = 700.0;

/// Bottom left corner of the grid.
pub const GEO_BOTTOM_LEFT: (f64, f64) = (0.0, 49.362064361572266);

/// Top left corner of the grid.
pub const GEO_TOP_LEFT: (f64, f64) = (0.0, 55.973602294921875);

/// Top right corner of the grid.
pub const GEO_TOP_RIGHT: (f64, f64) = (10.856452941894531, 55.388973236083984);

/// Bottom right corner of the grid.
pub const GEO_BOTTOM_RIGHT: (f64, f64) = (9.009300231933594, 48.895301818847656);

/// WGS84 projection used as the source coordinate system.
static PROJ_4326: Lazy<Proj> = Lazy::new(|| {
    Proj::from_proj_string("+proj=longlat +ellps=WGS84 +datum=WGS84 +no_defs").unwrap()
});

/// Polar stereographic projection used for the HDF5 grid.
static PROJ_KNMI: Lazy<Proj> = Lazy::new(|| {
    Proj::from_proj_string(
        "+proj=stere +lat_0=90 +lon_0=0 +lat_ts=60 +a=6378.14 +b=6356.75 +x_0=0 y_0=0 +type=crs",
    )
    .unwrap()
});

/// Converts longitude and latitude coordinates to the HDF5 nowcast grid and returns the grid indices.
///
/// # Arguments
/// - `longitude`: Longitude in degrees.
/// - `latitude`: Latitude in degrees.
///
/// # Errors
/// - `ProjectionError::CoordinateError`: If the coordinate transformation fails.
/// - `ProjectionError::OutOfBounds`: If the resulting grid indices are outside the valid range.
///
/// # Examples
/// ```rust
/// use knmi_nowcast::nowcast::projection::lon_lat_to_grid;
///
/// let result = lon_lat_to_grid(4.9, 52.3);
/// match result {
///     Ok((col, row)) => println!("Grid coordinates: ({}, {})", col, row),
///     Err(e) => println!("Error: {:?}", e),
/// }
/// ```
pub fn lon_lat_to_grid(longitude: f64, latitude: f64) -> Result<(u16, u16), ProjectionError> {
    let mut coordinate = (longitude.to_radians(), latitude.to_radians(), 0.0);

    let proj_result = proj4rs::transform::transform(&PROJ_4326, &PROJ_KNMI, &mut coordinate);
    if let Err(_) = proj_result {
        return Err(ProjectionError::CoordinateError(
            "Coordinate transformation failed".to_string(),
        ));
    }

    // Calculate and round to the nearest integer
    let col = coordinate.0.round();
    let row = (-GEO_ROW_OFFSET - coordinate.1).round();

    // Check bounds
    if col < 0.0 || col > GEO_NUMBER_OF_COLUMNS || row < 0.0 || row > GEO_NUMBER_OF_ROWS {
        return Err(ProjectionError::OutOfBounds(
            "Coordinates are outside the valid grid boundaries".to_string(),
        ));
    }

    Ok((col as u16, row as u16))
}

/// Returns the longitude and latitude coordinates for the given grid indices.
/// There maybe some offset in the resulting coordinates due to the projection.
/// The offset is not yet investigated.
/// ToDo: Investigate the offset and adjust the result accordingly, use rust-decimal?
///
/// # Errors
/// - `ProjectionError::CoordinateError`: If the coordinate transformation fails.
/// - `ProjectionError::OutOfBounds`: If the grid coordinates are outside the valid range.
pub fn grid_to_lon_lat(col: u16, row: u16) -> Result<(f64, f64), ProjectionError> {
    if col > GEO_NUMBER_OF_COLUMNS as u16 || row > GEO_NUMBER_OF_ROWS as u16 {
        return Err(ProjectionError::OutOfBounds(
            "Grid coordinates are outside the valid grid boundaries".to_string(),
        ));
    }

    let row = -GEO_ROW_OFFSET - row as f64;
    let mut coordinate = (col as f64, row as f64, 0.0);

    let proj_result = proj4rs::transform::transform(&PROJ_KNMI, &PROJ_4326, &mut coordinate);
    if let Err(_) = proj_result {
        return Err(ProjectionError::CoordinateError(
            "Coordinate transformation failed".to_string(),
        ));
    }

    Ok((coordinate.0.to_degrees(), coordinate.1.to_degrees()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_to_lon_lat() {
        // there maybe some float precision error test with a margin of 0.001 error
        let margin = 0.0005;

        let (lon, lat) = grid_to_lon_lat(0, 0).unwrap();
        assert!((lon - GEO_TOP_LEFT.0).abs() < margin);
        assert!((lat - GEO_TOP_LEFT.1).abs() < margin);

        let (lon, lat) = grid_to_lon_lat(0, GEO_NUMBER_OF_ROWS as u16).unwrap();
        assert!((lon - GEO_BOTTOM_LEFT.0).abs() < margin);
        assert!((lat - GEO_BOTTOM_LEFT.1).abs() < margin);

        let (lon, lat) = grid_to_lon_lat(GEO_NUMBER_OF_COLUMNS as u16, 0).unwrap();
        assert!((lon - GEO_TOP_RIGHT.0).abs() < margin);
        assert!((lat - GEO_TOP_RIGHT.1).abs() < margin);

        let (lon, lat) =
            grid_to_lon_lat(GEO_NUMBER_OF_COLUMNS as u16, GEO_NUMBER_OF_ROWS as u16).unwrap();
        assert!((lon - GEO_BOTTOM_RIGHT.0).abs() < margin);
        assert!((lat - GEO_BOTTOM_RIGHT.1).abs() < margin);
    }

    #[test]
    fn test_lon_lat_to_grid() {
        // Test the corners of the grid
        assert_eq!(
            lon_lat_to_grid(GEO_BOTTOM_LEFT.0, GEO_BOTTOM_LEFT.1).unwrap(),
            (0, GEO_NUMBER_OF_ROWS as u16)
        );
        assert_eq!(
            lon_lat_to_grid(GEO_TOP_LEFT.0, GEO_TOP_LEFT.1).unwrap(),
            (0, 0)
        );
        assert_eq!(
            lon_lat_to_grid(GEO_TOP_RIGHT.0, GEO_TOP_RIGHT.1).unwrap(),
            (GEO_NUMBER_OF_COLUMNS as u16, 0)
        );
        assert_eq!(
            lon_lat_to_grid(GEO_BOTTOM_RIGHT.0, GEO_BOTTOM_RIGHT.1).unwrap(),
            (GEO_NUMBER_OF_COLUMNS as u16, GEO_NUMBER_OF_ROWS as u16)
        );

        // Test going out of bounds by supplying coordinates outside the bbox
        assert!(matches!(
            lon_lat_to_grid(-1.0, 49.3620).unwrap_err(),
            ProjectionError::OutOfBounds(_)
        ));
        assert!(matches!(
            lon_lat_to_grid(0.0, 48.362064).unwrap_err(),
            ProjectionError::OutOfBounds(_)
        ));
        assert!(matches!(
            lon_lat_to_grid(0.0, 56.973602).unwrap_err(),
            ProjectionError::OutOfBounds(_)
        ));
        assert!(matches!(
            lon_lat_to_grid(11.856452941, 55.388973236).unwrap_err(),
            ProjectionError::OutOfBounds(_)
        ));

        // Test going out of bounds by supplying invalid EPSG:4326 coordinates
        assert!(matches!(
            lon_lat_to_grid(-10000000.01, 5000000000.0).unwrap_err(),
            ProjectionError::CoordinateError(_)
        ));
    }
}
