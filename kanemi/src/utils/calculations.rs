use std::f64::consts::PI;

/// Converts degrees to radians
pub fn to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Calculate the haversine distance between two points (lon1, lat1) and (lon2, lat2)
/// in decimal degrees. Returns the distance in meters.
pub fn haversine_distance(lon1: f64, lat1: f64, lon2: f64, lat2: f64) -> f64 {
    let earth_radius_km = 6371.0; // Earth's radius in kilometers

    let dlat = to_radians(lat2 - lat1);
    let dlon = to_radians(lon2 - lon1);

    let a = (dlat / 2.0).sin().powi(2)
        + to_radians(lat1).cos() * to_radians(lat2).cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    // Convert to meters
    earth_radius_km * c * 1000.0
}
