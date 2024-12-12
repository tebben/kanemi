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
        CliCommands::Forecast(forecast) => {
            handlers::forecast::handle_command(forecast).await;
        }
        CliCommands::Notifications(notification) => {
            handlers::notification::handle_command(notification).await;
        }
        CliCommands::Geocoder { command } => {
            handlers::geocoder::handle_command(command).await;
        }
    };
}
