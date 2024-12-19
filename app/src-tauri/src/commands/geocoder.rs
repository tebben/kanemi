use pdok_geocoder::location_server::LocationServer;
use pdok_geocoder::options::free::FreeOptions;

#[tauri::command]
pub async fn geocode(location: &str) -> Result<String, String> {
    let location_server = LocationServer::default();
    let free_options = FreeOptions {
        q: location.to_string(),
        best_match: Some(false),
        fl: None,
        fq: None,
        df: None,
        bq: None,
        start: None,
        rows: None,
        sort: None,
        lonlat: None,
    };

    let data = location_server
        .get_free(free_options.clone())
        .await
        .unwrap();

    Ok(serde_json::to_string_pretty(&data.response).unwrap())
}
