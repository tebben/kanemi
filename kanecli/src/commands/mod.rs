mod help;

pub mod cy43p1;
pub mod download;
pub mod geocoder;
pub mod notifications;
pub mod nowcast_precipitation;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "kanecli")]
#[command(version = "1.0")]
#[command(about = "A CLI tool to work with KNMI api's and some datasets may contain some other stuff aswell", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    /// Download KNMI data from the Open Data API
    Download(download::DownloadOptions),

    /// Dataset: Nowcast precipitation (2 hour precipitation forecast)
    NowcastPrecipitation(nowcast_precipitation::NowcastPrecipitationOptions),

    /// Dataset: Harmonie CY43 P1 (60 hours weather forecast for the Netherlands)
    HarmonieCY43P1 {
        #[command(subcommand)]
        command: cy43p1::CY43P1Options,
    },

    /// Receive messages from the KNMI notification service on new data availability
    Notifications(notifications::NotificationOptions),

    /// Geocode or reverse geocode a location using the PDOK Locatieserver
    Geocoder {
        #[command(subcommand)]
        command: geocoder::GeocoderOptions,
    },
}
