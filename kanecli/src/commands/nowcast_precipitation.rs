use super::help::*;
use clap::Args;

#[derive(Args, Debug)]
pub struct NowcastPrecipitationOptions {
    #[arg(short, long, env = "KNMI_API_KEY_OPEN_DATA", help = SHORT_HELP_API_KEY_ODA, long_help = LONG_HELP_API_KEY_ODA)]
    pub api_key: String,

    #[arg(short, long, env = "KNMI_LOCATION", value_parser = parse_location, help = SHORT_HELP_LOCATION, long_help = LONG_HELP_LOCATION)]
    pub location: (f64, f64),

    #[arg(short, long, required = false, default_value = "./output", help = SHORT_HELP_OUTPUT_DIR, long_help = LONG_HELP_OUTPUT_DIR)]
    pub output_dir: String,

    #[arg(short, long, required = false, help = SHORT_HELP_INPUT_FILE, long_help = LONG_HELP_INPUT_FILE)]
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
