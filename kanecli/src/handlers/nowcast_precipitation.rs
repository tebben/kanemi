use crate::commands::nowcast_precipitation::NowcastPrecipitationOptions;
use kanemi::{
    dataplatform::{api::OpenDataAPI, models::config::DatasetConfig},
    nowcast_precipitation::dataset,
};

pub async fn handle_command(options: NowcastPrecipitationOptions) {
    let dataset_config = DatasetConfig::new("radar_forecast".to_string(), "2.0".to_string());
    let (longitude, latitude) = options.location;
    get_forecast(
        options.api_key,
        dataset_config,
        options.input_file,
        options.output_dir,
        longitude,
        latitude,
    )
    .await;
}

pub async fn get_forecast(
    api_key: String,
    dataset_config: DatasetConfig,
    input_file: Option<String>,
    output_dir: String,
    longitude: f64,
    latitude: f64,
) {
    // if input file is provided, directly load and print data
    if let Some(input_file) = input_file {
        load_and_print_data(input_file, longitude, latitude);
        return;
    }

    print_from_download(api_key, dataset_config, output_dir, longitude, latitude).await;
}

async fn print_from_download(
    api_key: String,
    dataset_config: DatasetConfig,
    output_dir: String,
    longitude: f64,
    latitude: f64,
) {
    let oda = OpenDataAPI::new(api_key, dataset_config, None);
    let download_result = oda
        .download_latest_file(&output_dir, None, Some(false))
        .await;
    if let Err(e) = download_result {
        eprintln!("Error: {}", e);
        return;
    }

    let (_, latest_download_url) = download_result.unwrap();

    load_and_print_data(latest_download_url, longitude, latitude);
}

fn load_and_print_data(filename: String, longitude: f64, latitude: f64) {
    let dataset = dataset::Dataset::new(filename);
    if let Err(e) = dataset {
        eprintln!("Error: {}", e);
        return;
    }

    let dataset = dataset.unwrap();
    let forecast = dataset.get_forecast(longitude, latitude);
    if let Err(e) = forecast {
        eprintln!("Error: {}", e);
        return;
    }

    let forecast = forecast.unwrap();
    let date_time_first = forecast.datetime;
    let values = forecast.values;

    println!(
        "\x1b[45m\x1b[37m\x1b[1m{} - Precipitation @ {}, {}\x1b[0m",
        date_time_first, longitude, latitude
    );

    println!("\x1b[92m-----------------------------------\x1b[0m");
    print!("\x1b[33m{:<10}\x1b[0m", "Time");
    print!("\x1b[33m{:<10}\x1b[0m", "Precipitation (mm/h)");
    println!();
    println!("\x1b[92m-----------------------------------\x1b[0m");

    for value in values {
        let hhmm = value.datetime.split('T').collect::<Vec<&str>>()[1];
        print!("\x1b[33m{:<10}\x1b[0m", hhmm);
        println!("\x1b[33m{:<10}\x1b[0m", value.value);
    }
}
