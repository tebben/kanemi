use super::errors::DatasetError;
use super::image::Image;
use hdf5::types::FixedAscii;
use hdf5::File;
use hdf5::Result;

pub struct Dataset {
    pub filepath: String,
    pub hdf5_file: File,
    pub image_count: u32,
}

impl Dataset {
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
            filepath: filepath,
            hdf5_file: hdf5_file.unwrap(),
            image_count: 25,
        })
    }

    pub fn read_image(&self, image_index: u32) -> Result<Image, DatasetError> {
        let group_img = self
            .hdf5_file
            .group(format!("image{}", image_index).as_str());
        if let Err(_) = group_img {
            return Err(DatasetError::ReadError(
                "Error reading image group".to_string(),
            ));
        }

        let group_img = group_img.unwrap();
        let img = group_img.dataset("image_data");
        if let Err(_) = img {
            return Err(DatasetError::ReadError(
                "Error reading image data".to_string(),
            ));
        }

        let img = img.unwrap();

        // get the datetime attribute for the image
        let attribute_datetime = group_img.attr("image_datetime_valid");
        if let Err(_) = attribute_datetime {
            return Err(DatasetError::ReadError(
                "Error reading datetime attribute".to_string(),
            ));
        }

        let attribute_datetime = attribute_datetime.unwrap();
        let datetime = attribute_datetime.read_scalar::<FixedAscii<25>>();
        if let Err(_) = datetime {
            return Err(DatasetError::ReadError(
                "Error reading datetime attribute".to_string(),
            ));
        }

        let datetime = datetime.unwrap();

        // read the image data
        let img_data = img.read_2d::<u16>();
        if let Err(_) = img_data {
            return Err(DatasetError::ReadError(
                "Error reading image data".to_string(),
            ));
        }

        let img_data = img_data.unwrap();

        // create the Image struct
        let image = Image::new(img_data, datetime.as_str().to_string());
        Ok(image)
    }
}

pub fn read_hdf5(file_path: String) -> Result<Image, DatasetError> {
    let dataset = Dataset::new(file_path)?;
    let image = dataset.read_image(1)?;

    let val = image.get_value_at_position(20, 430);
    if let Some(val) = val {
        println!("Value at position (20, 430): {}", val);
    }

    Ok(image)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_hdf5() {
        let img = read_hdf5("../example_data/example.hdf5".to_string()).unwrap();
        assert_eq!(img.datetime, "04-DEC-2024;20:15:00.000");

        let data = img.data;
        assert_eq!(data[[20, 430]], 0); //5
    }
}
