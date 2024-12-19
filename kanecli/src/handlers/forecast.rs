use crate::commands::forecast::ForecastOptions;
use kanemi::{
    dataplatform::{api::OpenDataAPI, models::config::DatasetConfig},
    nowcast::{dataset, transformation::pixel_to_mm_hr},
};

pub async fn handle_command(options: ForecastOptions) {
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

    // loop over images 1 to 25 and store datetime and mm/h values to print later
    let mut values = Vec::new();
    for i in 1..26 {
        let image = dataset.read_image(i);
        if let Err(e) = image {
            eprintln!("Error: {}", e);
            return;
        }

        let image = image.unwrap();
        let value = image.get_value_at_lon_lat(longitude, latitude).unwrap();
        let mm_per_hour = pixel_to_mm_hr(value.unwrap());
        values.push((image.datetime, mm_per_hour));
    }

    let date_time_first = values.first().unwrap().0;
    println!(
        "\x1b[45m\x1b[37m\x1b[1m{} - Precipitation @ {}, {}\x1b[0m",
        date_time_first.format("%d-%m-%Y"),
        longitude,
        latitude
    );

    println!("\x1b[92m-----------------------------------\x1b[0m");
    print!("\x1b[33m{:<10}\x1b[0m", "Time");
    print!("\x1b[33m{:<10}\x1b[0m", "Precipitation (mm/h)");
    println!();
    println!("\x1b[92m-----------------------------------\x1b[0m");

    for (datetime, value) in values {
        print!("\x1b[33m{:<10}\x1b[0m", datetime.format("%H:%M"));
        println!("\x1b[33m{:<10}\x1b[0m", value);
    }
}
