use kanemi::{
    dataplatform::{api::OpenDataAPI, models::config::DatasetConfig},
    nowcast::{dataset, transformation::pixel_to_mm_hr},
};
use serde::{Deserialize, Serialize};

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

    let oda = OpenDataAPI::new(api_key, dataset_config, None);
    let download_result = oda
        .download_latest_file("./output", None, Some(false))
        .await;
    if let Err(e) = download_result {
        return Err(e.to_string());
    }

    let (file, latest_file) = download_result.unwrap();
    let dataset = dataset::Dataset::new(latest_file.clone());
    if let Err(e) = dataset {
        return Err(e.to_string());
    }

    // loop over images 1 to 25 and store datetime and mm/h values to print later
    let mut forecast = Forecast {
        datetime: file.created,
        values: Vec::new(),
    };

    {
        let dataset = dataset.unwrap();
        for i in 1..26 {
            let image = dataset.read_image(i);
            if let Err(e) = image {
                return Err(e.to_string());
            }

            let image = image.unwrap();
            let value = image.get_value_at_lon_lat(longitude, latitude).unwrap();
            let mm_per_hour = pixel_to_mm_hr(value.unwrap());
            let iso_datetime = image.datetime.format("%Y-%m-%dT%H:%M:%S").to_string();
            let forecast_value = ForecastValue {
                datetime: iso_datetime,
                value: mm_per_hour,
            };

            forecast.values.push(forecast_value);
        }
    }

    Ok(serde_json::to_string_pretty(&forecast).unwrap())
}
