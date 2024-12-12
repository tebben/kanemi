pub mod forecast;
pub mod geocoder;
pub mod notifications;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "kanecli")]
#[command(version = "1.0")]
#[command(about = "A CLI tool to work with KNMI data and maybe some other stuff", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    /// Print the precipitation forecast from input file or a newly downloaded KNMI dataset
    Forecast(forecast::ForecastOptions),

    /// Test the notification service
    Notifications(notifications::NotificationOptions),

    /// Geocode or reverse geocode a location using the PDOK Locatieserver
    Geocoder {
        #[command(subcommand)]
        command: geocoder::GeocoderOptions,
    },
}
