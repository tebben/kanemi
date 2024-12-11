use clap::Args;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum GeocoderOptions {
    /// Geocoding using free text
    Free(GeocoderOptionsFree),

    /// Reverse geocoding
    Reverse(GeocoderOptionsReverse),
}

#[derive(Args, Debug)]
pub struct GeocoderOptionsFree {
    /// Query location
    #[arg(short, long)]
    pub q: String,

    /// Get and return only the best match
    #[arg(long)]
    pub best_match: bool,

    /// fields to return, example: bron,weergavenaam
    #[arg(long, required = false)]
    pub fl: Option<String>,

    /// filter query, example: bron:BAG, example 2: type:(gemeente OR woonplaats OR weg)
    #[arg(long, required = false)]
    pub fq: Option<String>,

    /// default search field, example: weergavenaam
    #[arg(long, required = false)]
    pub df: Option<String>,

    /// start index, default: 0
    #[arg(long, required = false)]
    pub start: Option<i32>,

    /// number of rows to return, default: 10, max: 100
    #[arg(long, required = false)]
    pub rows: Option<i32>,

    /// sort results on field, example: score desc,sortering asc,weergavenaam asc
    #[arg(long, required = false)]
    pub sort: Option<String>,
}

#[derive(Args, Debug)]
pub struct GeocoderOptionsReverse {
    /// query
    #[arg(short, long)]
    pub q: String,
}
