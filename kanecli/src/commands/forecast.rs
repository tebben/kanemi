use clap::Args;

#[derive(Args, Debug)]
pub struct ForecastOptions {
    /// API key for the Open Data API
    #[arg(short, long, env = "KNMI_API_KEY_OPEN_DATA")]
    pub api_key: String,

    /// Location as a comma-separated string "longitude,latitude"
    #[arg(short, long, env = "KNMI_LOCATION", value_parser = parse_location)]
    pub location: (f64, f64),

    /// Output directory for storing downloaded files
    #[arg(
        short,
        long,
        env = "KNMI_OUTPUT_DIR",
        required = false,
        default_value = "./output"
    )]
    pub output_dir: Option<String>,

    /// Input file to load, new file will be downloaded if not provided
    #[arg(short, long, env = "KNMI_INPUT_FILE", required = false)]
    pub input_file: Option<String>,
}

fn parse_location(s: &str) -> Result<(f64, f64), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err("Location must be in the format 'longitude,latitude'".to_string());
    }
    let lon: f64 = parts[0]
        .parse()
        .map_err(|_| "Invalid longitude".to_string())?;
    let lat: f64 = parts[1]
        .parse()
        .map_err(|_| "Invalid latitude".to_string())?;
    Ok((lon, lat))
}
