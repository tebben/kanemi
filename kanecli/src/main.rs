mod commands;
mod handlers;

use clap::Parser;
use commands::{Cli, CliCommands};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        CliCommands::Download(download) => {
            handlers::download::handle_command(download).await;
        }
        CliCommands::NowcastPrecipitation(forecast) => {
            handlers::nowcast_precipitation::handle_command(forecast).await;
        }
        CliCommands::Notifications(notification) => {
            handlers::notification::handle_command(notification).await;
        }
        CliCommands::Geocoder { command } => {
            handlers::geocoder::handle_command(command).await;
        }
        CliCommands::HarmonieCY43P1 { command } => {
            handlers::cy43p1::handle_command(command).await;
        }
    };
}
