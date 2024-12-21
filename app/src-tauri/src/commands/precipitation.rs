use kanemi::{
    dataplatform::{api::OpenDataAPI, models::config::DatasetConfig},
    nowcast_precipitation::dataset,
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Forecast {
    pub datetime: String,
    pub values: Vec<ForecastValue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastValue {
    pub datetime: String,
    pub value: f64,
}

#[tauri::command]
pub async fn get_nowcast_forecast(
    api_key: String,
    longitude: f64,
    latitude: f64,
) -> Result<String, String> {
    let dataset_config = DatasetConfig::new("radar_forecast".to_string(), "2.0".to_string());
    let path = "./output/precipitation";
    let oda = OpenDataAPI::new(api_key, dataset_config, None);
    let download_result = oda.download_latest_file(&path, None, Some(false)).await;
    if let Err(e) = download_result {
        return Err(e.to_string());
    }

    let (_, latest_file) = download_result.unwrap();
    let dataset = dataset::Dataset::new(latest_file.clone());
    if let Err(e) = dataset {
        return Err(e.to_string());
    }

    // remove all files in folder except for latest file
    remove_files_except(&latest_file, &path);

    let dataset = dataset.unwrap();
    let forecast = dataset.get_forecast(longitude, latitude).unwrap();

    Ok(serde_json::to_string_pretty(&forecast).unwrap())
}

// remove all files in folder except for given file
fn remove_files_except(file: &str, folder: &str) {
    let paths = fs::read_dir(folder).unwrap();
    for path in paths {
        let path = path.unwrap().path();

        // Check if the path is a file and not the specified file
        if path.is_file() && path.to_str().unwrap() != file {
            fs::remove_file(path).unwrap();
        }
    }
}
