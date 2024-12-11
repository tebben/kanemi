use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "kanemi")]
#[command(version = "1.0")]
#[command(about = "A CLI tool to work with KNMI data", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Print the precipitation forecast from input file or a newly downloaded KNMI dataset
    Forecast {
        /// API key for the Open Data API
        #[arg(short, long, env = "KNMI_API_KEY_OPEN_DATA")]
        api_key: String,

        /// Location as a comma-separated string "longitude,latitude"
        #[arg(short, long, env = "KNMI_LOCATION", value_parser = parse_location)]
        location: (f64, f64),

        /// Output directory for storing downloaded files
        #[arg(
            short,
            long,
            env = "KNMI_OUTPUT_DIR",
            required = false,
            default_value = "./output"
        )]
        output_dir: Option<String>,

        /// Input file to load, new file will be downloaded if not provided
        #[arg(short, long, env = "KNMI_INPUT_FILE", required = false)]
        input_file: Option<String>,
    },

    /// Test the notification service
    Notifications {
        /// API key for the notification service
        #[arg(short, long, env = "KNMI_API_KEY_NOTIFICATION")]
        api_key: String,
    },
}

fn parse_location(value: &str) -> Result<(f64, f64), String> {
    let parts: Vec<&str> = value.split(',').map(str::trim).collect();

    if parts.len() != 2 {
        return Err("Location must be in the format 'longitude,latitude'".to_string());
    }

    let longitude = parts[0]
        .parse::<f64>()
        .map_err(|_| "Invalid longitude value")?;
    let latitude = parts[1]
        .parse::<f64>()
        .map_err(|_| "Invalid latitude value")?;

    if (-180.0..=180.0).contains(&longitude) && (-90.0..=90.0).contains(&latitude) {
        Ok((longitude, latitude))
    } else {
        Err(
            "Longitude must be between -180.0 and 180.0, and latitude between -90.0 and 90.0."
                .to_string(),
        )
    }
}
