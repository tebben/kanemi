use once_cell::sync::Lazy;
use proj4rs;
use proj4rs::proj::Proj;

const GEO_ROW_OFFSET: f64 = 3649.98193359375;
const GEO_NUMBER_OF_ROWS: u16 = 765;
const GEO_NUMBER_OF_COLUMNS: u16 = 700;
const PROJECTION_PROJ4_KNMI: &str =
    "+proj=stere +lat_0=90 +lon_0=0 +lat_ts=60 +a=6378.14 +b=6356.75 +x_0=0 y_0=0 +type=crs";
const PROJECTION_PROJ4_4326: &str = "+proj=longlat +ellps=WGS84 +datum=WGS84 +no_defs";
static PROJ_4326: Lazy<Proj> = Lazy::new(|| Proj::from_proj_string(PROJECTION_PROJ4_4326).unwrap());
static PROJ_KNMI: Lazy<Proj> = Lazy::new(|| Proj::from_proj_string(PROJECTION_PROJ4_KNMI).unwrap());

#[derive(Debug)]
pub enum ProjectionError {
    OutOfBounds,
    CoordinateError,
}

pub fn lon_lat_to_grid(lon: f64, lat: f64) -> Result<(u16, u16), ProjectionError> {
    let mut coordinate = (lon.to_radians(), lat.to_radians(), 0.0);

    let test = proj4rs::transform::transform(&PROJ_4326, &PROJ_KNMI, &mut coordinate);
    if let Err(_) = test {
        return Err(ProjectionError::CoordinateError);
    }

    // Calculate and round to the nearest integer
    let col = coordinate.0.round();
    let row = (-GEO_ROW_OFFSET - coordinate.1).round();

    // Check bounds
    if col < 0.0 || col > 700.0 || row < 0.0 || row > 765.0 {
        return Err(ProjectionError::OutOfBounds);
    }

    Ok((col as u16, row as u16))
}

#[test]
fn test_lon_lat_to_grid() {
    assert_eq!(
        lon_lat_to_grid(0.0, 49.362064361572266).unwrap(),
        (0, GEO_NUMBER_OF_ROWS)
    ); // bottom left
    assert_eq!(lon_lat_to_grid(0.0, 55.973602294921875).unwrap(), (0, 0)); // top left
    assert_eq!(
        lon_lat_to_grid(10.856452941894531, 55.388973236083984).unwrap(),
        (GEO_NUMBER_OF_COLUMNS, 0)
    ); // top right
    assert_eq!(
        lon_lat_to_grid(9.009300231933594, 48.895301818847656).unwrap(),
        (700, 765)
    ); // bottom right
    assert!(matches!(
        lon_lat_to_grid(-1.0, 49.3620).unwrap_err(),
        ProjectionError::OutOfBounds
    ));
    assert!(matches!(
        lon_lat_to_grid(0.0, 48.362064).unwrap_err(),
        ProjectionError::OutOfBounds
    ));
    assert!(matches!(
        lon_lat_to_grid(0.0, 56.973602).unwrap_err(),
        ProjectionError::OutOfBounds
    ));
    assert!(matches!(
        lon_lat_to_grid(11.856452941, 55.388973236).unwrap_err(),
        ProjectionError::OutOfBounds
    ));
    assert!(matches!(
        lon_lat_to_grid(-10000000.01, 5000000000.0).unwrap_err(),
        ProjectionError::CoordinateError
    ));
}
