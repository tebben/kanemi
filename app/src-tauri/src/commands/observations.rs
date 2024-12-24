use kanemi::{
    actual_synoptic_observations::{dataset, models::Station},
    dataplatform::{api::OpenDataAPI, models::config::DatasetConfig},
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Observation {
    pub distance: u64,
    pub datetime: String,
    pub station: Station,
}

#[tauri::command]
pub async fn get_closest_observation(
    api_key: String,
    longitude: f64,
    latitude: f64,
) -> Result<String, String> {
    let dataset_config =
        DatasetConfig::new("Actuele10mindataKNMIstations".to_string(), "2".to_string());
    let path = "./output/observations";
    let oda = OpenDataAPI::new(api_key, dataset_config, None);
    let download_result = oda.download_latest_file(path, None, Some(false)).await;
    if let Err(e) = download_result {
        return Err(e.to_string());
    }

    let (_, latest_file) = download_result.unwrap();
    let dataset = dataset::Dataset::new(latest_file.clone());
    if let Err(e) = dataset {
        return Err(e.to_string());
    }

    // remove all files in folder except for latest file
    remove_files_except(&latest_file, path);

    let dataset = dataset.unwrap();
    let (station, distance) = dataset.get_closest_station(longitude, latitude);
    let datetime = dataset
        .datetime
        .unwrap()
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string();
    let observation = Observation {
        distance,
        datetime,
        station,
    };

    Ok(serde_json::to_string_pretty(&observation).unwrap())
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
