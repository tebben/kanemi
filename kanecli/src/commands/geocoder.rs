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
    #[arg(short, long)]
    #[clap(
        help = "Search query. Example: `amsterdam`",
        long_help = "This is where the search terms are specified.
The Solr syntax for search terms can be applied here, e.g.,
combining terms with \'and\' and using double quotes for consecutive
search terms. Search terms can be incomplete. Synonyms are also utilized.
\nExample: -q \"amsterdam\""
    )]
    pub q: String,

    #[arg(long)]
    #[clap(
        help = "Get and return only the best match",
        long_help = "By adding --best-match, only the best match will be returned.
This will only return 1 single entry without the rest of the document result.
This option overwrites possible set values: start, row and sort.
\nExample: --best-match"
    )]
    pub best_match: bool,

    #[arg(long, required = false, value_parser = string_to_vec)]
    #[clap(
        help = "Comma separated list of fields to return",
        long_help = "Use this option to specify a list of comma separated fields to return.
    \nExample: --fl \"id,bron,weergavenaam,straatnaam\""
    )]
    pub fl: Option<Vec<String>>,

    /// filter query, example: bron:BAG, example 2: type:(gemeente OR woonplaats OR weg)
    #[arg(long, required = false)]
    #[clap(
        help = "Query to filter results see --help for more information and examples",
        long_help = "Use this option to filter the results, default: type:(gemeente OR woonplaats OR weg OR postcode OR adres).
    \nExample filter source: --fq \"bron:BAG\"
    \nExample filter multiple types: --fq \"type:(gemeente OR woonplaats OR weg OR postcode OR adres)\"
    \nExample only retrieving woonplaats: --fq \"type:woonplaats\"
    \nPossible type: provincie, gemeente, woonplaats, weg, postcode, adres, perceel, hectometerpaal, wijk, buurt, waterschapsgrens, appartementsrecht
    \nPossible bron: BAG, NWB, BAG/NWB, DKK, Bestuurlijke Grenzen, CBS, HWH"
    )]
    pub fq: Option<String>,

    #[arg(long, required = false)]
    #[clap(
        help = "Start row, default 0 used for pagination",
        long_help = "Using this option you can specify the start row this can be used for pagination.
the max value is 10000.
    \nExample: --start 100"
    )]
    pub start: Option<i32>,

    #[arg(long, required = false)]
    #[clap(
        help = "Amount of rows to return, default 10, max 100",
        long_help = "Amount of rows to return, default 10, max 100
    \nExample: --rows 100"
    )]
    pub rows: Option<i32>,

    #[arg(long, required = false, value_parser = parse_coordinates)]
    #[clap(
        help = "Sorting will be done on the distance from the given coordinates, format --lonlat \"5.12, 52.09\"",
        long_help = "If you want to sort the results on the distance from a given coordinate, you can use the --lonlat option.
    \nExample: --lonlat \"5.12, 52.09\"
    \nExample: --lonlat \"5.12 52.09\""
    )]
    pub lonlat: Option<(f64, f64)>,

    #[arg(long, required = false)]
    #[clap(
        help = "Default search field",
        long_help = "If the field is not retrieved, the default search field set witch --df will be used.
    \nExample: --df \"tekst\""
    )]
    pub df: Option<String>,

    /// sort results on field, example: score desc,sortering asc,weergavenaam asc
    #[arg(long, required = false)]
    #[clap(
        help = "Sort order of the results, default: score desc",
        long_help = "Sorting the results can be done by adding the --sort option.
    \nExample: --sort \"score desc,sortering asc,weergavenaam asc\""
    )]
    pub sort: Option<String>,

    #[arg(long, required = false, value_parser = parse_bq)]
    #[clap(
        help = "Boost fields for score calculation, example: --bq \"weergavenaam^0.5,straatnaam^1.5\"",
        long_help = "Boost fields for score calculation, supply a list of fields with a boost value comma separated.
    \nExample: --bq \"type:provincie^1.5,type:gemeente^1.5,type:postcode^0.5,type:adres^1\""
    )]
    pub bq: Option<Vec<String>>,
}

#[derive(Args, Debug)]
pub struct GeocoderOptionsReverse {
    #[arg(long, required = false, value_parser = parse_coordinates)]
    #[clap(
        help = "The longitude latitude position to reverse geocode, format --lonlat \"5.12, 52.09\"",
        long_help = "Reverse geocode the given location, use the --lonlat option for longitude and latitude coordinates
or the --rd option for RD coordinates.
    \nExample: --lonlat \"5.12, 52.09\"
    \nExample: --lonlat \"5.12 52.09\""
    )]
    pub lonlat: Option<(f64, f64)>,

    #[arg(long, required = false, value_parser = parse_coordinates)]
    #[clap(
        help = "The RD coordinates to reverse geocode, format --rd \"122000, 487000\"",
        long_help = "Reverse geocode the given location, use the --rd option for RD coordinates or the --lonlat option for longitude and latitude coordinates.
    \nExample: --rd \"122000, 487000\"
    \nExample: --rd \"122000 487000\""
    )]
    pub rd: Option<(f64, f64)>,

    #[arg(long)]
    #[clap(
        help = "Get and return only the best match",
        long_help = "By adding --best-match, only the best match will be returned.
This will only return 1 single entry without the rest of the document result.
This option overwrites possible set values: start, row and sort.
\nExample: --best-match"
    )]
    pub best_match: bool,

    #[arg(long, required = false, value_parser = string_to_vec)]
    #[clap(
        help = "Comma separated list of fields to return",
        long_help = "Use this option to specify a list of comma separated fields to return.
    \nExample: --fl \"id,bron,weergavenaam,straatnaam\""
    )]
    pub fl: Option<Vec<String>>,

    /// filter query, example: bron:BAG, example 2: type:(gemeente OR woonplaats OR weg)
    #[arg(long, required = false)]
    #[clap(
        help = "Query to filter results see --help for more information and examples",
        long_help = "Use this option to filter the results, default: type:(gemeente OR woonplaats OR weg OR postcode OR adres).
    \nExample filter source: --fq \"bron:BAG\"
    \nExample filter multiple types: --fq \"type:(gemeente OR woonplaats OR weg OR postcode OR adres)\"
    \nExample only retrieving woonplaats: --fq \"type:woonplaats\"
    \nPossible type: provincie, gemeente, woonplaats, weg, postcode, adres, perceel, hectometerpaal, wijk, buurt, waterschapsgrens, appartementsrecht
    \nPossible bron: BAG, NWB, BAG/NWB, DKK, Bestuurlijke Grenzen, CBS, HWH"
    )]
    pub fq: Option<String>,

    #[arg(long, required = false)]
    #[clap(
        help = "Start row, default 0 used for pagination",
        long_help = "Using this option you can specify the start row this can be used for pagination.
the max value is 10000.
    \nExample: --start 100"
    )]
    pub start: Option<i32>,

    #[arg(long, required = false)]
    #[clap(
        help = "Amount of rows to return, default 10, max 100",
        long_help = "Amount of rows to return, default 10, max 100
    \nExample: --rows 100"
    )]
    pub rows: Option<i32>,

    #[arg(long, required = false,  value_parser = string_to_vec)]
    #[clap(
        help = "Types to return, see --help for more information",
        long_help = "Types to return, default: adres
    \nExample: --types \"adres, wijk, buurt\"
    \nPossible types: adres, perceel, hectometerpaal, wijk, buurt, waterschapsgrens, appartementsrecht"
    )]
    pub types: Option<Vec<String>>,

    #[arg(long, required = false)]
    #[clap(
        help = "Max search distance in meters",
        long_help = "Max search distance in meters (integer)
    \nExample: --distance 100"
    )]
    #[arg(long, required = false)]
    pub distance: Option<u32>,
}

fn parse_coordinates(s: &str) -> Result<(f64, f64), std::num::ParseFloatError> {
    let mut s = s.to_string();

    // if there is a comma, remove all whitespace
    if s.contains(",") {
        s = s.replace(" ", "");
    } else {
        // if there is no comma, replace whitespace with ,
        s = s.replace(" ", ",");
    }

    let coords: Vec<&str> = s.split(',').collect();
    let lon = coords[0].parse::<f64>()?;
    let lat = coords[1].parse::<f64>()?;

    Ok((lon, lat))
}

fn string_to_vec(s: &str) -> Result<Vec<String>, std::io::Error> {
    let fl = if s.contains(',') {
        s.split(',').map(|s| s.to_string()).collect::<Vec<String>>()
    } else {
        s.split(" ").map(|s| s.to_string()).collect::<Vec<String>>()
    };
    Ok(fl)
}

fn parse_bq(s: &str) -> Result<Vec<String>, std::io::Error> {
    let bq = s.split(',').map(|s| s.to_string()).collect::<Vec<String>>();
    Ok(bq)
}
