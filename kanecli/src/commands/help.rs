pub const SHORT_HELP_Q: &str = r#"Search query. Example: `museumplein 1 amsterdam`"#;
pub const LONG_HELP_Q: &str = r#"This is where the search terms are specified.
The Solr syntax for search terms can be applied here, e.g.,
combining terms with 'and' and using double quotes for consecutive
search terms. Search terms can be incomplete. Synonyms are also utilized.

Example: -q "museumplein amsterdam""#;

pub const SHORT_HELP_BEST_MATCH: &str = r#"Get and return only the best match"#;
pub const LONG_HELP_BEST_MATCH: &str = r#"By adding --best-match, only the best match will be returned.
This will only return 1 single entry without the rest of the document result.
This option overwrites possible set values: start, row and sort.

Example: --best-match"#;

pub const SHORT_HELP_FL: &str = r#"Comma separated list of fields to return"#;
pub const LONG_HELP_FL: &str = r#"Use this option to specify a list of comma separated fields to return.

Example: --fl id,weergavenaam,straatnaam
Example: --fl "id, bron, weergavenaam, straatnaam" (add quotes if you have spaces in the field names)"#;

pub const SHORT_HELP_FQ: &str =
    r#"Query to filter results see --help for more information and examples"#;
pub const LONG_HELP_FQ: &str = r#"Use this option to filter the results.

default: type:(gemeente OR woonplaats OR weg OR postcode OR adres).

Example filter source: --fq "bron:BAG"
Example filter multiple types: --fq "type:(gemeente OR woonplaats OR weg OR postcode OR adres)"
Example only retrieving woonplaats: --fq "type:woonplaats"

Possible type: provincie, gemeente, woonplaats, weg, postcode, adres, perceel, hectometerpaal, wijk, buurt, waterschapsgrens, appartementsrecht
Possible bron: BAG, NWB, BAG/NWB, DKK, Bestuurlijke Grenzen, CBS, HWH"#;

pub const SHORT_HELP_START: &str = r#"Start row, default 0 used for pagination"#;
pub const LONG_HELP_START: &str = r#"Using this option you can specify the start row this can be used for pagination.
the max value is 10000.

Example: --start 100"#;

pub const SHORT_HELP_ROWS: &str = r#"Amount of rows to return, default 10, max 100"#;
pub const LONG_HELP_ROWS: &str = r#"Amount of rows to return, default 10, max 100

Example: --rows 100"#;

pub const SHORT_HELP_LONLAT_DIST: &str =
    r#"Sorting will done on distance for given coordinates, format --lonlat "5.12, 52.09""#;
pub const LONG_HELP_LONLAT_DIST: &str = r#"If you want to sort the results on the distance from a given
coordinate, you can use the --lonlat option.

Example: --lonlat 5.12,52.09
Example: --lonlat "5.12, 52.09"
Example: --lonlat "5.12 52.09""#;

pub const SHORT_HELP_DF: &str = r#"Default search field"#;
pub const LONG_HELP_DF: &str = r#"If the field is not retrieved, the default search field set witch --df will be used.

Example: --df "tekst""#;

pub const SHORT_HELP_SORT: &str = r#"Sort order of the results, default: score desc"#;
pub const LONG_HELP_SORT: &str = r#"Sorting the results can be done by adding the --sort option.

Default: score desc

Example: --sort "score desc,sortering asc,weergavenaam asc""#;

pub const SHORT_HELP_BQ: &str = r#"Boost fields for score calculation, see --info for more info"#;
pub const LONG_HELP_BQ: &str = r#"Boost fields for score calculation, supply a list of fields with a boost value comma separated.

Example: --bq "type:provincie^1.5,type:gemeente^1.5,type:postcode^0.5,type:adres^1""#;

pub const SHORT_HELP_LONLAT_REV: &str =
    r#"Longitude and latitude coordinates to reverse geocode, format --lonlat "5.12, 52.09""#;
pub const LONG_HELP_LONLAT_REV: &str = r#"Reverse geocode the given location, use the --lonlat option for longitude and latitude coordinates
or the --rd option for RD coordinates.

Example: --lonlat 5.12,52.09
Example: --lonlat "5.12, 52.09"
Example: --lonlat "5.12 52.09""#;

pub const SHORT_HELP_RD: &str =
    r#"The RD coordinates to reverse geocode, format --rd "122000, 487000""#;
pub const LONG_HELP_RD: &str = r#"Reverse geocode the given location, use the --rd option for RD coordinates
or the --lonlat option for longitude and latitude coordinates.

Example: --rd 122000,487000
Example: --rd "122000, 487000"
Example: --rd "122000 487000""#;

pub const SHORT_HELP_TYPES: &str = r#"Types to return, see --help for more information"#;
pub const LONG_HELP_TYPES: &str = r#"Types to return,

Default: adres

Example: --types "adres, wijk, buurt"

Possible types: adres, perceel, hectometerpaal, wijk, buurt, waterschapsgrens, appartementsrecht"#;

pub const SHORT_HELP_DISTANCE: &str = r#"Max search distance in meters"#;
pub const LONG_HELP_DISTANCE: &str = r#"Max search distance in meters (integer)

Example: --distance 100"#;

pub const SHORT_HELP_API_KEY_ODA: &str = r#"API key for the KNMI Open Data API"#;
pub const LONG_HELP_API_KEY_ODA: &str = r#"API key for the KNMI Open Data API.
A free API key can be obtained from KNMI, for more information
browse to https://developer.dataplatform.knmi.nl/open-data-api#token

Key can be set trough env var KNMI_API_KEY_OPEN_DATA"#;

pub const SHORT_HELP_LOCATION: &str =
    r#"Location as a comma-separated string "longitude,latitude""#;
pub const LONG_HELP_LOCATION: &str = r#"Location as a comma-separated string "longitude,latitude

Example: --location 5.12,52.09
Example: --location "5.12, 52.09""#;

pub const SHORT_HELP_OUTPUT_DIR: &str = r#"Output directory for the forecast data"#;
pub const LONG_HELP_OUTPUT_DIR: &str = r#"Output directory for the forecast data

Default: ./output

Example: --output-dir ./output"#;

pub const SHORT_HELP_INPUT_FILE: &str =
    r#"Input file to load, new file will be downloaded if not provided"#;
pub const LONG_HELP_INPUT_FILE: &str = r#"Input file to load, new file will be downloaded if not provided

Example: --input-file "./data.hdf5""#;

pub const SHORT_HELP_API_KEY_NOTI: &str = r#"API key for the KNMI Notification Service"#;
pub const LONG_HELP_API_KEY_NOTI: &str = r#"API key for the KNMI Notification Service.
A free API key can be obtained from KNMI, for more information, make sure you get the
key for the Notification Service and not for the Open Data API.
browse to https://developer.dataplatform.knmi.nl/open-data-api#token

Key can be set trough env var KNMI_API_KEY_OPEN_DATA"#;

pub const SHORT_HELP_DATASET_NAME_NOTI: &str = "The name of the dataset to subscribe to";
pub const SHORT_HELP_DATASET_VERSION_NOTI: &str = "The version of the dataset to subscribe to";
pub const SHORT_HELP_CLIENT_ID_NOTI: &str = "Unique client id for the notification service";
pub const LONG_HELP_CLIENT_ID_NOTI: &str = r#"Unique client id for the notification service
The client identifier is a string that identifies each MQTT client that connects to an MQTT server.
Each client that connects to the MQTT server must have a unique client id as the server uses it to
identify the state of the MQTT session between the client and the server. If you try to connect
with a client id that already has a session present, the old one will be terminated. After a
disconnect for whatever reason, by reconnecting with the same client ID,
the session will resume seamlessly and receive missed messages"#;
