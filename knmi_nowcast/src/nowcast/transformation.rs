//! Transformation functions for the nowcast data.

use chrono::NaiveDateTime;

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
/// ```rust
/// use knmi_nowcast::nowcast::transformation::pixel_to_mm_hr;
///
/// let mm_hr = pixel_to_mm_hr(113);
/// let mm_hr = pixel_to_mm_hr(31);
/// let mm_hr = pixel_to_mm_hr(0);
/// ```
pub fn pixel_to_mm_hr(value: u16) -> f64 {
    let result = (value as f64 * CALIBRATION_FACTOR) * 12.0;
    (result * 100.0).round() / 100.0
}

/// Converts a HDF5 formatted datetime string to a chrono `NaiveDateTime`.
pub fn convert_hdf5_datetime(datetime_str: String) -> Option<NaiveDateTime> {
    let format = "%d-%b-%Y;%H:%M:%S%.3f";
    NaiveDateTime::parse_from_str(&datetime_str, format).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::*;

    #[test]
    fn test_pixel_value_to_mm_hr() {
        assert_eq!(pixel_to_mm_hr(113), 13.56);
        assert_eq!(pixel_to_mm_hr(31), 3.72);
        assert_eq!(pixel_to_mm_hr(0), 0.00);
    }

    #[test]
    fn test_convert_hdf5_datetime() {
        test_date(1, "JAN", 1, 2024, 1, 0, 15);
        test_date(1, "FEB", 2, 2024, 2, 59, 0);
        test_date(1, "MAR", 3, 2024, 3, 15, 0);
        test_date(1, "APR", 4, 2024, 4, 15, 0);
        test_date(1, "MAY", 5, 2024, 5, 15, 0);
        test_date(1, "JUN", 6, 2024, 0, 15, 0);
        test_date(1, "JUL", 7, 2024, 6, 15, 0);
        test_date(1, "AUG", 8, 2024, 7, 15, 0);
        test_date(1, "SEP", 9, 2024, 8, 15, 0);
        test_date(1, "OCT", 10, 2024, 15, 15, 0);
        test_date(1, "NOV", 11, 2024, 23, 15, 0);
        test_date(1, "DEC", 12, 2024, 23, 59, 59);
    }

    fn test_date(
        day: u32,
        month: &str,
        month_chrono: u32,
        year: i32,
        hour: u32,
        minute: u32,
        second: u32,
    ) {
        let datetime_str = format!(
            "{:02}-{}-{};{:02}:{:02}:{:02}.000",
            day, month, year, hour, minute, second
        );

        assert_eq!(
            convert_hdf5_datetime(datetime_str.to_string())
                .unwrap()
                .and_utc(),
            NaiveDate::from_ymd_opt(year, month_chrono, day)
                .unwrap()
                .and_hms_opt(hour, minute, second)
                .unwrap()
                .and_utc()
        );
    }
}
