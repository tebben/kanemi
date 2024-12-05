mod knmi;

use dotenv::dotenv;
use knmi::dataplatform::api::OpenDataAPI;
use knmi::nowcast::dataset::read_hdf5;
use knmi::utils::projection;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    //read_hdf5("./example_data/test.hdf5".to_string());
    //return;

    let api_key = env::var("KNMI_API_KEY").expect("API_KEY must be set");
    let oda = OpenDataAPI::new(
        String::from("https://api.dataplatform.knmi.nl/open-data/v1"),
        String::from("radar_forecast"),
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
