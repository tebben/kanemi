mod cli;
mod functions;

use clap::Parser;
use cli::{Cli, Commands};
use dotenv::dotenv;
use kanemi::dataplatform::models::config::DatasetConfig;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Forecast {
            api_key,
            location,
            output_dir,
            input_file,
        } => {
            let (longitude, latitude) = location;
            functions::forecast::get_forecast(
                api_key,
                get_dataset_config(),
                input_file,
                output_dir,
                longitude,
                latitude,
            )
            .await;
        }
        Commands::Notifications { api_key } => {
            if let Err(e) =
                functions::notification::run_notification_test(api_key, get_dataset_config()).await
            {
                eprintln!("Error: {}", e);
            }
        }
    };
}

fn get_dataset_config() -> DatasetConfig {
    DatasetConfig::new("radar_forecast".to_string(), "2.0".to_string())
}
