use super::image::Image;
use super::transformation::{convert_hdf5_datetime, pixel_to_mm_hr};
use crate::errors::DatasetError;
use crate::nowcast_precipitation::models::{PrecipitationForecast, PrecipitationForecastValue};
use hdf5::types::FixedAscii;
use hdf5::File;
use hdf5::Group;
use hdf5::Result;
use ndarray::{ArrayBase, Ix2, OwnedRepr};

/// Represents a dataset containing multiple images.
#[derive(Debug)]
pub struct Dataset {
    pub filepath: String,
    pub hdf5_file: File,
    pub image_count: u32,
}

impl Dataset {
    /// Constructs a new Dataset from the given file path.
    /// The file path should point to a valid HDF5 file.
    ///
    /// # Errors
    /// - `DatasetError::FileNotFound`: If the file does not exist.
    /// - `DatasetError::ReadError`: If an error occurs while reading the file.
    pub fn new(filepath: String) -> Result<Dataset, DatasetError> {
        if filepath.is_empty() || !std::path::Path::new(&filepath).exists() {
            return Err(DatasetError::FileNotFound(format!(
                "File not found: {}",
                filepath
            )));
        }

        let hdf5_file = File::open(filepath.clone());
        if let Err(e) = hdf5_file {
            return Err(DatasetError::ReadError(e.to_string()));
        }

        Ok(Dataset {
            filepath,
            hdf5_file: hdf5_file.unwrap(),
            image_count: 25,
        })
    }

    /// Reads the image with the given index from the dataset.
    /// The index should be in the range [1, 25].
    /// The image contains the pixel data and the datetime of the image.
    ///
    /// # Errors
    /// - `DatasetError::ReadError`: If an error occurs while reading the image.
    pub fn read_image(&self, image_index: u32) -> Result<Image, DatasetError> {
        if image_index < 1 || image_index > self.image_count {
            return Err(DatasetError::ImageIndexOutOfBounds(format!(
                "Image index out of bounds, should be between 1 and 25: {}",
                image_index
            )));
        }

        let group_img = self
            .hdf5_file
            .group(format!("image{}", image_index).as_str());
        if group_img.is_err() {
            return Err(DatasetError::ReadError(
                "Error reading image group".to_string(),
            ));
        }

        let group_img = group_img.unwrap();
        let datetime = self.get_image_datetime(&group_img)?;
        let img_data = self.get_image_data(&group_img)?;

        let datetime = convert_hdf5_datetime(datetime.as_str().to_string()).unwrap();
        // create the Image struct
        let image = Image::new(img_data, datetime);
        Ok(image)
    }

    /// Gets the precipitation forecast (2 hours) from the dataset for the given longitude and latitude.
    /// The forecast contains the datetime and the precipitation value in mm/h for the next 25 images.
    /// The longitude and latitude should be in the range of the dataset.
    ///
    /// # Errors
    /// - `DatasetError::ReadError`: If an error occurs while reading the image.
    ///
    /// # Example
    /// ```
    /// use kanemi::nowcast_precipitation::dataset::Dataset;
    /// let dataset = Dataset::new("../example_data/example.hdf5".to_string()).unwrap();
    /// let forecast = dataset.get_forecast(5.0, 52.0).unwrap();
    /// ```
    pub fn get_forecast(
        &self,
        longitude: f64,
        latitude: f64,
    ) -> Result<PrecipitationForecast, DatasetError> {
        let mut forecast = PrecipitationForecast {
            datetime: "".to_string(),
            values: Vec::new(),
        };

        for i in 1..26 {
            let image = self.read_image(i);
            if image.is_err() {
                return Err(DatasetError::ReadError("Error reading image".to_string()));
            }

            let image = image.unwrap();

            // ToDo: Get the real file datetime
            if i == 1 {
                forecast.datetime = image.datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string();
            }

            let value = image.get_value_at_lon_lat(longitude, latitude).unwrap();
            let mm_per_hour = pixel_to_mm_hr(value.unwrap());
            let iso_datetime = image.datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string();

            forecast.values.push(PrecipitationForecastValue {
                datetime: iso_datetime,
                value: mm_per_hour,
            });
        }

        Ok(forecast)
    }

    fn get_image_datetime(&self, group: &Group) -> Result<String, DatasetError> {
        let attribute_datetime = group.attr("image_datetime_valid");
        if attribute_datetime.is_err() {
            return Err(DatasetError::ReadError(
                "Error reading datetime attribute".to_string(),
            ));
        }

        let attribute_datetime = attribute_datetime.unwrap();
        let datetime = attribute_datetime.read_scalar::<FixedAscii<25>>();
        if datetime.is_err() {
            return Err(DatasetError::ReadError(
                "Error reading datetime attribute".to_string(),
            ));
        }

        Ok(datetime.unwrap().as_str().to_string())
    }

    fn get_image_data(
        &self,
        group: &Group,
    ) -> Result<ArrayBase<OwnedRepr<u16>, Ix2>, DatasetError> {
        let img = group.dataset("image_data");
        if img.is_err() {
            return Err(DatasetError::ReadError(
                "Error reading image data".to_string(),
            ));
        }

        let img = img.unwrap();
        let img_data = img.read_2d::<u16>();
        if img_data.is_err() {
            return Err(DatasetError::ReadError(
                "Error reading image data".to_string(),
            ));
        }

        Ok(img_data.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nowcast_precipitation::projection;
    use chrono::*;

    #[test]
    fn test_file_not_found() {
        assert!(matches!(
            Dataset::new("".to_string()).unwrap_err(),
            DatasetError::FileNotFound(_)
        ));

        assert!(matches!(
            Dataset::new("./doesnotexist.1".to_string()).unwrap_err(),
            DatasetError::FileNotFound(_)
        ));
    }

    #[test]
    fn test_image_out_of_bounds() {
        let dataset = Dataset::new("../example_data/example.hdf5".to_string()).unwrap();
        assert!(matches!(
            dataset.read_image(0).unwrap_err(),
            DatasetError::ImageIndexOutOfBounds(_)
        ));

        assert!(matches!(
            dataset.read_image(26).unwrap_err(),
            DatasetError::ImageIndexOutOfBounds(_)
        ));
    }

    #[test]
    fn test_read_image_1() {
        let dataset = Dataset::new("../example_data/example.hdf5".to_string()).unwrap();
        let image = dataset.read_image(1).unwrap();

        // Known by inspection this holds value 5
        let grid_x = 20;
        let grid_y = 430;

        // get lon and lat from the grid position
        let (lon, lat) = projection::grid_to_lon_lat(grid_x, grid_y).unwrap();

        // Check if the date time is correct
        assert_eq!(
            image.datetime,
            NaiveDate::from_ymd_opt(2024, 12, 4)
                .unwrap()
                .and_hms_opt(20, 15, 0)
                .unwrap()
        );

        // Check if the value at the grid position is correct
        assert_eq!(
            image
                .get_value_at_position(grid_x as usize, grid_y as usize)
                .unwrap(),
            5
        );

        // Check if the same value is returned when using lon and lat
        assert_eq!(image.get_value_at_lon_lat(lon, lat).unwrap(), Some(5));
    }

    #[test]
    fn test_forecast() {
        let dataset = Dataset::new("../example_data/example.hdf5".to_string()).unwrap();
        let grid_x = 111;
        let grid_y = 527;

        // get lon and lat from the grid position
        let (lon, lat) = projection::grid_to_lon_lat(grid_x, grid_y).unwrap();

        let forecast = dataset.get_forecast(lon, lat).unwrap();

        // Check if the datetime is correct
        assert_eq!(forecast.datetime, "2024-12-04T20:15:00Z");

        // Check if the values are correct
        assert_eq!(forecast.values.len(), 25);
        assert_eq!(forecast.values[0].value, 0.0);
        assert_eq!(forecast.values[24].value, 0.36);
    }
}
