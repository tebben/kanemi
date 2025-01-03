use crate::commands::cy43p1::{CY43P1Options, CY43P1OptionsForecast, CY43P1OptionsParameters};
use kanemi::harmonie_cy43_p1::dataset::get_available_parameters;
use kanemi::harmonie_cy43_p1::dataset::Dataset;
use std::path::Path;

pub async fn handle_command(command: CY43P1Options) {
    match command {
        CY43P1Options::Forecast(options) => {
            handle_forecast(options).await;
        }
        CY43P1Options::Parameters(options) => {
            handle_parameters(options).await;
        }
    }
}

async fn handle_forecast(options: CY43P1OptionsForecast) {
    let input = &options.input[0];
    let locations = options.locations;
    let parameters = options.parameters;
    let hours = options.hours;

    let dataset = if is_directory(input) {
        Dataset::from_directory(input, hours)
    } else if is_tar_file(input) {
        Dataset::from_tar(input, hours)
    } else {
        Dataset::from_files(options.input, hours)
    };

    match dataset {
        Ok(dataset) => {
            let data = dataset.get_forecast(locations, parameters).unwrap();
            let pretty_data = serde_json::to_string_pretty(&data).unwrap();
            println!("{}", pretty_data);
        }
        Err(e) => eprintln!("Error loading dataset: {}", e),
    }
}

async fn handle_parameters(_options: CY43P1OptionsParameters) {
    let parameters = get_available_parameters();
    let pretty_data = serde_json::to_string_pretty(&parameters).unwrap();
    println!("{}", pretty_data);
}

fn is_directory(path: &str) -> bool {
    Path::new(path).is_dir()
}

fn is_tar_file(path: &str) -> bool {
    path.ends_with(".tar")
}
