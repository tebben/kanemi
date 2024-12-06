mod knmi_nowcast;

use dotenv::dotenv;
use knmi_nowcast::dataplatform::api::OpenDataAPI;
use knmi_nowcast::nowcast::projection::lon_lat_to_grid;
use knmi_nowcast::nowcast::transformation::pixel_to_mm_hr;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("KNMI_API_KEY").expect("API_KEY must be set");
    let oda = OpenDataAPI::new(
        String::from("https://api.dataplatform.knmi.nl/open-data/v1"),
        String::from("radar_forecast"),
        String::from("2.0"),
        api_key,
    );

    let value_mm_hr = pixel_to_mm_hr(113);
    println!("Value in mm/hr: {}", value_mm_hr);

    let result = lon_lat_to_grid(12.2, 13.3);
    match result {
        Ok((col, row)) => println!("Grid coordinates: ({}, {})", col, row),
        Err(e) => println!("Error: {:?}", e),
    }

    match oda.get_latest_files(1).await {
        Ok(response) => {
            println!("Files: {:?}", response.files);

            if response.files.len() > 0 {
                let filename = &response.files[0].filename;
                get_download_url(&oda, filename).await;
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

async fn get_download_url(oda: &OpenDataAPI, filename: &str) {
    match oda.get_download_url(filename).await {
        Ok(response) => {
            if let Err(e) = oda
                .download_file(&response.temporary_download_url, "test.hdf5")
                .await
            {
                eprintln!("Error: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    }
}
