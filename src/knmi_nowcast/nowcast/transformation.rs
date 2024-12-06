//! Transformation functions for the nowcast data.

/// The calibration factor used to convert pixel values to millimeters per hour (mm/hr).
const CALIBRATION_FACTOR: f64 = 0.01;

/// Converts a pixel value to millimeters per hour (mm/hr).
///
/// This function takes an 8-bit unsigned integer (`u8`) representing a pixel value,
/// applies a calibration factor, and converts it to a precipitation rate in millimeters per hour (mm/hr).
///
/// # Arguments
///
/// * `value` - An 8-bit unsigned integer representing the pixel value.
///
/// # Returns
///
/// * `f64` - The precipitation rate in millimeters per hour (mm/hr), rounded to two decimal places.
///
/// # Examples
///
/// ```rust,ignore
/// use knmi_nowcast::nowcast::transformation::pixel_to_mm_hr;
///
/// let mm_hr = pixel_to_mm_hr(113);
/// let mm_hr = pixel_to_mm_hr(31);
/// let mm_hr = pixel_to_mm_hr(0);
/// ```
pub fn pixel_to_mm_hr(value: u8) -> f64 {
    let result = (value as f64 * CALIBRATION_FACTOR) * 12.0;
    (result * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_value_to_mm_hr() {
        assert_eq!(pixel_to_mm_hr(113), 13.56);
        assert_eq!(pixel_to_mm_hr(31), 3.72);
        assert_eq!(pixel_to_mm_hr(0), 0.00);
    }
}
