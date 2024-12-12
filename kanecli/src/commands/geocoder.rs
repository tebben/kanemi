use super::help::*;
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum GeocoderOptions {
    /// Classic geocoding using free text, check kanecli geocoder free --help for more information
    Free(GeocoderOptionsFree),

    /// Reverse geocoding
    Reverse(GeocoderOptionsReverse),
}

#[derive(Args, Debug)]
pub struct GeocoderOptionsFree {
    #[arg(short, long, help = SHORT_HELP_Q, long_help = LONG_HELP_Q)]
    pub q: String,

    #[arg(long, help = SHORT_HELP_BEST_MATCH, long_help = LONG_HELP_BEST_MATCH)]
    pub best_match: bool,

    #[arg(long, required = false, value_delimiter = ',', value_parser = vec_handle_whitespace, help = SHORT_HELP_FL, long_help = LONG_HELP_FL)]
    pub fl: Option<Vec<String>>,

    #[arg(long, required = false, help = SHORT_HELP_FQ, long_help = LONG_HELP_FQ)]
    pub fq: Option<String>,

    #[arg(long, required = false, help = SHORT_HELP_START, long_help = LONG_HELP_START)]
    pub start: Option<i32>,

    #[arg(long, required = false, help = SHORT_HELP_ROWS, long_help = LONG_HELP_ROWS)]
    pub rows: Option<i32>,

    #[arg(long, required = false, value_parser = parse_coordinates,  help = SHORT_HELP_LONLAT_DIST, long_help = LONG_HELP_LONLAT_DIST)]
    pub lonlat: Option<(f64, f64)>,

    #[arg(long, required = false, help = SHORT_HELP_DF, long_help = LONG_HELP_DF)]
    pub df: Option<String>,

    #[arg(long, required = false, help = SHORT_HELP_SORT, long_help = LONG_HELP_SORT)]
    pub sort: Option<String>,

    #[arg(long, required = false, value_parser = vec_handle_whitespace, help = SHORT_HELP_BQ, long_help = LONG_HELP_BQ)]
    pub bq: Option<Vec<String>>,
}

#[derive(Args, Debug)]
pub struct GeocoderOptionsReverse {
    #[arg(long, required = false, value_parser = parse_coordinates, help = SHORT_HELP_LONLAT_REV, long_help = LONG_HELP_LONLAT_REV)]
    pub lonlat: Option<(f64, f64)>,

    #[arg(long, required = false, value_parser = parse_coordinates, help = SHORT_HELP_RD, long_help = LONG_HELP_RD)]
    pub rd: Option<(f64, f64)>,

    #[arg(long, help = SHORT_HELP_BEST_MATCH, long_help = LONG_HELP_BEST_MATCH)]
    pub best_match: bool,

    #[arg(long, required = false, value_delimiter = ',', value_parser = vec_handle_whitespace, help = SHORT_HELP_FL, long_help = LONG_HELP_FL)]
    pub fl: Option<Vec<String>>,

    #[arg(long, required = false, help = SHORT_HELP_FQ, long_help = LONG_HELP_FQ)]
    pub fq: Option<String>,

    #[arg(long, required = false, help = SHORT_HELP_START, long_help = LONG_HELP_START)]
    pub start: Option<i32>,

    #[arg(long, required = false, help = SHORT_HELP_ROWS, long_help = LONG_HELP_ROWS)]
    pub rows: Option<i32>,

    #[arg(long, required = false,  value_parser = vec_handle_whitespace, help = SHORT_HELP_TYPES, long_help = LONG_HELP_TYPES)]
    pub types: Option<Vec<String>>,

    #[arg(long, required = false, help = SHORT_HELP_DISTANCE, long_help = LONG_HELP_DISTANCE)]
    #[arg(long, required = false)]
    pub distance: Option<u32>,
}

fn parse_coordinates(s: &str) -> Result<(f64, f64), std::num::ParseFloatError> {
    let mut s = s.to_string();

    if s.contains(",") {
        s = s.replace(" ", "");
    } else {
        s = s.replace(" ", ",");
    }

    let coords: Vec<&str> = s.split(',').collect();
    let lon = coords[0].parse::<f64>()?;
    let lat = coords[1].parse::<f64>()?;

    Ok((lon, lat))
}

fn vec_handle_whitespace(s: &str) -> Result<String, String> {
    Ok(s.trim().to_string())
}
