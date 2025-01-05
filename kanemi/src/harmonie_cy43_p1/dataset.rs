use crate::errors::CY43P1Error;
use crate::harmonie_cy43_p1::reader::CY43P1Reader;
use crate::harmonie_cy43_p1::reader::{GRIBInfo, GribMetadata, GribResponse};
use crate::harmonie_cy43_p1::wind_image::{create_image, ColorStep};
use chrono::{DateTime, Duration, SecondsFormat, Utc};
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;
use tar::Archive;
use tempfile::TempDir;

#[derive(Debug)]
pub struct Dataset {
    pub filepaths: Vec<String>,
    pub readers: Vec<Reader>,
    temp_dir: Option<TempDir>,
}

#[derive(Debug)]
pub struct Reader {
    pub hour: u16,
    pub cy43p1_reader: CY43P1Reader,
}

#[derive(Debug, Serialize)]
pub struct Forecast {
    results: Vec<ForecastResult>,
}

#[derive(Debug, Serialize)]
pub struct ForecastResult {
    location: (f32, f32),
    parameters: Vec<ForecastParameter>,
}

#[derive(Debug, Serialize)]
pub struct ForecastParameter {
    name: String,
    level: u16,
    values: Vec<ForecastValue>,
}

#[derive(Debug, Serialize)]
pub struct ForecastValue {
    datetime: String,
    value: f64,
}

/// Get all available parameters
pub fn get_available_parameters() -> Vec<GribMetadata> {
    let grib_info = GRIBInfo::new();
    grib_info.get_all_parameters_copy()
}

/// Check if filename is valid, filename should be in the format HA43_N20_202412221800_00000_GB
fn is_valid_filename(filename: &str) -> bool {
    let re = Regex::new(r"^HA43_N20_\d{12}_\d{5}_GB$").unwrap();
    re.is_match(filename)
}

/// Extract hour from filename
fn hour_from_filename(filename: &str) -> Result<u16, CY43P1Error> {
    let re = Regex::new(r"_(\d{5})_GB$").unwrap();
    if let Some(caps) = re.captures(filename) {
        if let Some(hour_str) = caps.get(1) {
            let hour: u16 = hour_str.as_str().parse().map_err(|_| {
                CY43P1Error::InvalidFilename("Unable to parse hour from filename".to_string())
            })?;
            return Ok(hour / 100);
        }
    }
    Err(CY43P1Error::InvalidFilename(
        "No hour found in filename".to_string(),
    ))
}

impl Dataset {
    /// Load a dataset from a list of filepaths. The hours parameter can be used to filter
    /// the files by the hour in the filename.
    pub fn from_files(filepaths: Vec<String>, hours: Option<u16>) -> Result<Dataset, CY43P1Error> {
        let mut filtered_files = vec![];
        let mut readers = vec![];

        for filepath in &filepaths {
            let filename = Path::new(filepath).file_name().unwrap().to_str().unwrap();
            if !is_valid_filename(filename) {
                return Err(CY43P1Error::InvalidFilename(format!(
                    "Invalid filename: {}",
                    filename
                )));
            }

            // Do not add the reader if the file hour is greater than the request hours
            let hour = hour_from_filename(filename)?;
            if let Some(h) = hours {
                if hour > h {
                    continue;
                }
            }

            let cy43p1_reader = CY43P1Reader::open(filepath)?;
            let reader = Reader {
                hour,
                cy43p1_reader,
            };
            readers.push(reader);
            filtered_files.push(filepath.clone());
        }

        Ok(Dataset {
            filepaths: filtered_files,
            readers,
            temp_dir: None,
        })
    }

    /// Load a dataset from a directory containing the files. The hours parameter can be used to filter
    /// the files by the hour in the filename. The directory should contain only valid files.
    pub fn from_directory(path: &str, hours: Option<u16>) -> Result<Dataset, CY43P1Error> {
        let mut filepaths = vec![];
        let entries = fs::read_dir(path).map_err(|_| {
            CY43P1Error::InvalidDirectory(format!("Unable to read directory {}", path))
        })?;

        for entry in entries {
            let entry = entry.map_err(|_| {
                CY43P1Error::InvalidFilename(format!("Unable to read entry in directory {}", path))
            })?;
            let path = entry.path();
            if path.is_file() {
                let filename = path.file_name().unwrap().to_str().unwrap();
                if is_valid_filename(filename) {
                    filepaths.push(path.to_str().unwrap().to_string());
                }
            }
        }

        if filepaths.is_empty() {
            return Err(CY43P1Error::InvalidDirectory(format!(
                "No valid files found in directory: {}",
                path
            )));
        }

        Dataset::from_files(filepaths, hours)
    }

    /// Load a dataset directly from a .tar file which is the file that is downloaded
    /// from KNMI. The tar file will be extracted to a temporary directory which will be
    /// removed automatically when the Dataset is dropped.
    pub fn from_tar(tar_path: &str, hours: Option<u16>) -> Result<Dataset, CY43P1Error> {
        let file = File::open(tar_path)
            .map_err(|_| CY43P1Error::TarError(format!("Unable to open tar file: {}", tar_path)))?;
        let mut archive = Archive::new(file);

        let temp_dir = TempDir::new().map_err(|_| {
            CY43P1Error::TarError("Unable to create temporary directory".to_string())
        })?;
        let temp_path = temp_dir.path();

        archive
            .unpack(temp_path)
            .map_err(|e| CY43P1Error::TarError(format!("Unable to extract tar file: {}", e)))?;

        let mut dataset = Dataset::from_directory(temp_path.to_str().unwrap(), hours)?;
        dataset.temp_dir = Some(temp_dir);

        Ok(dataset)
    }

    pub fn get_raw(
        &self,
        parameters: Option<Vec<(String, u16)>>,
    ) -> Result<GribResponse, CY43P1Error> {
        let reader = self.readers.first().unwrap();
        let data = reader.cy43p1_reader.get(parameters, None)?;

        Ok(data)
    }

    pub fn create_wind_image(
        &self,
        output_path: &str,
        upscale_factor: u32,
        density: f32,
        antialiasing: bool,
        line_multiplier: Option<f32>,
        color_steps: Option<&[ColorStep]>,
    ) {
        let parameters = vec![("ugrd".to_string(), 10), ("vgrd".to_string(), 10)];
        let data = self.get_raw(Some(parameters)).unwrap();
        let u_vec = data
            .results
            .iter()
            .find(|r| r.name == "ugrd" && r.level == 10)
            .unwrap()
            .values
            .clone();
        let v_vec = data
            .results
            .iter()
            .find(|r| r.name == "vgrd" && r.level == 10)
            .unwrap()
            .values
            .clone();

        create_image(
            output_path,
            &u_vec,
            &v_vec,
            upscale_factor,
            density,
            antialiasing,
            line_multiplier,
            color_steps,
        );
    }

    pub fn get_forecast(
        &self,
        locations: Vec<(f32, f32)>,
        parameters: Option<Vec<(String, u16)>>,
    ) -> Result<Forecast, CY43P1Error> {
        let mut forecast_map: HashMap<String, ForecastResult> = HashMap::new();

        for reader in self.readers.iter() {
            let p = parameters.clone();
            let l = locations.clone();
            let reader_result = reader.cy43p1_reader.get(p, Some(l))?;

            // Parse the base time from the GribResponse
            let base_time = DateTime::parse_from_rfc3339(&reader_result.time)
                .map_err(|_| CY43P1Error::InvalidFilename("Invalid time format".to_string()))?
                .with_timezone(&Utc);

            // Adjust the base time by adding the hours
            let forecast_time = base_time + Duration::hours(reader.hour.into());

            // Transform the GribResponse into the Forecast structure
            if let Some(reader_locations) = reader_result.locations {
                for (i, location) in reader_locations.iter().enumerate() {
                    let location_key = format!("{:.6},{:.6}", location.0, location.1);

                    let forecast_result =
                        forecast_map.entry(location_key.clone()).or_insert_with(|| {
                            ForecastResult {
                                location: *location,
                                parameters: vec![],
                            }
                        });

                    for grib_result in &reader_result.results {
                        // Check if the parameter already exists
                        if let Some(existing_parameter) = forecast_result
                            .parameters
                            .iter_mut()
                            .find(|p| p.name == grib_result.name && p.level == grib_result.level)
                        {
                            // Append the new values to the existing parameter
                            if let Some(value) = grib_result.values.get(i) {
                                existing_parameter.values.push(ForecastValue {
                                    datetime: forecast_time
                                        .to_rfc3339_opts(SecondsFormat::Secs, true),
                                    value: *value as f64,
                                });
                            }
                        } else {
                            // Create a new parameter and add it to the parameters vector
                            let mut forecast_parameter = ForecastParameter {
                                name: grib_result.name.clone(),
                                level: grib_result.level,
                                values: vec![],
                            };

                            if let Some(value) = grib_result.values.get(i) {
                                forecast_parameter.values.push(ForecastValue {
                                    datetime: forecast_time
                                        .to_rfc3339_opts(SecondsFormat::Secs, true),
                                    value: *value as f64,
                                });
                            }

                            forecast_result.parameters.push(forecast_parameter);
                        }
                    }
                }
            }
        }

        // Sort the values in each ForecastParameter by datetime
        for forecast_result in forecast_map.values_mut() {
            for parameter in &mut forecast_result.parameters {
                parameter.values.sort_by(|a, b| a.datetime.cmp(&b.datetime));
            }
        }

        let forecast = Forecast {
            results: forecast_map.into_values().collect(),
        };

        Ok(forecast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmonie_cy43_p1::{get_palette, Palette};

    const FILE_PATH1: &str = "../example_data/HA43_N20_202412221800_00000_GB";

    #[test]
    fn test_dataset_new() {
        let parameters = vec![
            ("tmp".to_string(), 0),
            ("tmp".to_string(), 50),
            ("isba".to_string(), 802),
        ];
        let locations = vec![(5.351926, 51.716_8), (4.913082420058467, 52.3422859189378)];
        let filepaths = vec![FILE_PATH1.to_string()];
        let dataset = Dataset::from_files(filepaths, None).unwrap();

        let data = dataset.get_forecast(locations, Some(parameters)).unwrap();

        let pretty_data = serde_json::to_string_pretty(&data).unwrap();
        println!("{}", pretty_data);
    }

    #[test]
    fn test_get_available_parameters() {
        let parameters = get_available_parameters();
        let pretty_data = serde_json::to_string_pretty(&parameters).unwrap();
        println!("{}", pretty_data);
    }

    #[test]
    fn test_create_wind_flow_field() {
        let filepaths = vec![FILE_PATH1.to_string()];
        let dataset = Dataset::from_files(filepaths, None).unwrap();

        // Create a cheap flow field image (cheap as in drawing of lines instead of vector grid with particles and iterations)
        let upscale_factor = 4;
        let density = 0.1;
        let antialiasing = false;
        let line_multiplier = Some(1.8);
        let palette = get_palette(Palette::Default);

        dataset.create_wind_image(
            "flow_field_test.png",
            upscale_factor,
            density,
            antialiasing,
            line_multiplier,
            Some(&palette),
        );
    }
}
