use super::help::*;
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum CY43P1Options {
    /// Get forecast data for a location
    Forecast(CY43P1OptionsForecast),

    /// Get all available parameters
    Parameters(CY43P1OptionsParameters),
}

#[derive(Args, Debug)]
pub struct CY43P1OptionsForecast {
    #[arg(
        short,
        long,
        required = true,
        value_delimiter = ' ',
        help = SHORT_HELP_CY43P1_FORECAST_INPUT,
        long_help = LONG_HELP_CY43P1_FORECAST_INPUT
    )]
    pub input: Vec<String>,

    #[arg(
        short,
        long,
        required = true,
        value_parser = parse_locations,
        value_delimiter = ' ',
        help = SHORT_HELP_CY43P1_FORECAST_LOCATIONS,
        long_help = LONG_HELP_CY43P1_FORECAST_LOCATIONS
    )]
    pub locations: Vec<(f32, f32)>,

    #[arg(
        short,
        long,
        required = false,
        value_parser = parse_parameters,
        value_delimiter = ' ',
        help = SHORT_HELP_CY43P1_FORECAST_PARAMETERS,
        long_help = LONG_HELP_CY43P1_FORECAST_PARAMETERS
    )]
    pub parameters: Option<Vec<(String, u16)>>,

    #[arg(
        short('H'),
        long,
        required = false,
        help = SHORT_HELP_CY43P1_FORECAST_HOURS,
        long_help = LONG_HELP_CY43P1_FORECAST_HOURS
    )]
    pub hours: Option<u16>,
}

#[derive(Args, Debug)]
pub struct CY43P1OptionsParameters {}

fn parse_parameters(s: &str) -> Result<(String, u16), String> {
    let mut split = s.split(',');
    let name = split
        .next()
        .ok_or_else(|| "Missing parameter name".to_string())?
        .to_string();
    let level = split
        .next()
        .ok_or_else(|| "Missing parameter level".to_string())?
        .parse::<u16>()
        .map_err(|e| e.to_string())?;
    Ok((name, level))
}

fn parse_locations(s: &str) -> Result<(f32, f32), String> {
    let mut split = s.split(',');
    let lon = split
        .next()
        .ok_or_else(|| "Missing longitude".to_string())?
        .parse::<f32>()
        .map_err(|e| e.to_string())?;
    let lat = split
        .next()
        .ok_or_else(|| "Missing latitude".to_string())?
        .parse::<f32>()
        .map_err(|e| e.to_string())?;
    Ok((lon, lat))
}
