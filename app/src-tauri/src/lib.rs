mod commands;

use commands::geocoder;
use commands::observations;
use commands::precipitation;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            geocoder::geocode,
            precipitation::get_nowcast_forecast,
            observations::get_closest_observation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
