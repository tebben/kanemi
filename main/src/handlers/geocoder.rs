use crate::commands::geocoder::{GeocoderOptions, GeocoderOptionsFree};
use pdok_location::location_server::LocationServer;
use pdok_location::models::options::FreeOptions;

pub async fn handle_command(command: GeocoderOptions) {
    match command {
        GeocoderOptions::Free(options) => {
            get_free(options).await;
        }
        GeocoderOptions::Reverse(_) => {
            println!("Reverse geocoding not implemented yet");
        }
    }
}

pub async fn get_free(options: GeocoderOptionsFree) {
    let location_server = LocationServer::default();
    let free_options = FreeOptions {
        q: options.q,
        best_match: Some(options.best_match),
        fl: options.fl,
        fq: options.fq,
        df: options.df,
        start: options.start,
        rows: options.rows,
        sort: options.sort,
        ..Default::default()
    };

    if let Some(start) = free_options.start {
        if start > 10000 {
            eprintln!("Error: start cannot be higher than 10000");
            return;
        }
    }

    if let Some(rows) = free_options.rows {
        if rows > 100 {
            eprintln!("Error: rows cannot be higher than 100");
            return;
        }
    }

    let data = location_server
        .get_free(free_options.clone())
        .await
        .unwrap();

    // if best_match is true, only return the first result
    if free_options.best_match.unwrap_or(false) {
        if let Some(first) = data.response.docs.first() {
            println!("{}", serde_json::to_string_pretty(first).unwrap());
        } else {
            println!("No results found");
        }
        return;
    }

    println!("{}", serde_json::to_string_pretty(&data).unwrap());
}
