use super::projection::lon_lat_to_grid;
use super::transformation::pixel_to_mm_hr;
use crate::errors::ProjectionError;
use chrono::NaiveDateTime;
use ndarray::{ArrayBase, Ix2, OwnedRepr};

/// Represents a single image in the nowcast dataset.
/// The image contains the pixel data and the datetime of the image.
/// The pixel data is stored as a 2D array of u16 values.
#[derive(Debug)]
pub struct Image {
    pub data: ArrayBase<OwnedRepr<u16>, Ix2>,
    pub datetime: NaiveDateTime,
}

impl Image {
    /// Constructs a new Image with the given pixel data and datetime.
    pub fn new(data: ArrayBase<OwnedRepr<u16>, Ix2>, datetime: NaiveDateTime) -> Image {
        Image { data, datetime }
    }

    /// Returns the raw pixel value at the specified x, y grid position, none if the position is out of bounds.
    pub fn get_value_at_position(&self, x: usize, y: usize) -> Option<u16> {
        if y < self.data.shape()[0] && x < self.data.shape()[1] {
            Some(self.data[[y, x]])
        } else {
            None
        }
    }

    pub fn get_mmhhr_at_position(&self, x: usize, y: usize) -> Option<f64> {
        self.get_value_at_position(x, y).map(pixel_to_mm_hr)
    }

    /// Returns the raw pixel value at a specific longitude and latitude position, none if the position is out of bounds.
    ///
    /// # Errors
    /// - `ProjectionError`: If the coordinate transformation fails or the coordinates are out of bounds.
    pub fn get_value_at_lon_lat(
        &self,
        longitude: f64,
        latitude: f64,
    ) -> Result<Option<u16>, ProjectionError> {
        let (x, y) = lon_lat_to_grid(longitude, latitude)?;
        Ok(self.get_value_at_position(x as usize, y as usize))
    }

    pub fn get_mmhhr_at_lon_lat(
        &self,
        longitude: f64,
        latitude: f64,
    ) -> Result<Option<f64>, ProjectionError> {
        self.get_value_at_lon_lat(longitude, latitude)
            .map(|value| value.map(pixel_to_mm_hr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nowcast::transformation;

    #[test]
    fn test_get_value_at_position() {
        // generate test data to create an Image
        let mut data = Vec::new();
        for _i in 0..765 {
            for j in 0..700 {
                data.push((j) as u16);
            }
        }

        let img_data = ArrayBase::from_shape_vec((765, 700), data).unwrap();
        let datetime = "01-JAN-2021;20:15:00.000".to_string();

        let datetime = transformation::convert_hdf5_datetime(datetime).unwrap();
        let image = Image::new(img_data, datetime);

        // test the corners of the image
        assert_eq!(image.get_value_at_position(0, 0).unwrap(), 0);
        assert_eq!(image.get_value_at_position(0, 764).unwrap(), 0);
        assert_eq!(image.get_value_at_position(699, 0).unwrap(), 699);
        assert_eq!(image.get_value_at_position(699, 764).unwrap(), 699);

        // test out of bounds
        assert!(image.get_value_at_position(700, 0).is_none());
        assert!(image.get_value_at_position(0, 765).is_none());
        assert!(image.get_value_at_position(700, 765).is_none());

        // test mm/h conversion
        assert_eq!(image.get_mmhhr_at_position(0, 0).unwrap(), 0.0);
        assert_eq!(image.get_mmhhr_at_position(699, 764).unwrap(), 83.88);

        let lon = 0.0;
        let lat = 55.9736;
        assert_eq!(image.get_value_at_lon_lat(lon, lat).unwrap().unwrap(), 0);
        assert_eq!(image.get_mmhhr_at_lon_lat(lon, lat).unwrap().unwrap(), 0.0);
    }
}
