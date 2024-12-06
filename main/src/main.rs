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

    test_lon_lat_to_grid();
    test_pixel_to_mm_hr();
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

fn test_lon_lat_to_grid() {
    // Test the corners of the grid
    assert_eq!(lon_lat_to_grid(4.9, 52.3).unwrap(), (0, 0));
    assert_eq!(lon_lat_to_grid(5.0, 52.3).unwrap(), (1, 0));
    assert_eq!(lon_lat_to_grid(4.9, 52.4).unwrap(), (0, 1));
    assert_eq!(lon_lat_to_grid(5.0, 52.4).unwrap(), (1, 1));
}

fn test_pixel_to_mm_hr() {
    assert_eq!(pixel_to_mm_hr(0), 0.0);
    assert_eq!(pixel_to_mm_hr(113), 0.0);
    assert_eq!(pixel_to_mm_hr(114), 0.25);
    assert_eq!(pixel_to_mm_hr(255), 50.0);
}
