use dotenv::dotenv;
use knmi_nowcast::{
    dataplatform::api::OpenDataAPI,
    nowcast::{dataset, transformation::pixel_to_mm_hr},
};
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

    // get latest filename
    let latest_files = oda.get_latest_files(1).await;
    if let Err(e) = latest_files {
        eprintln!("Error: {}", e);
        return;
    }

    let latest_files = latest_files.unwrap();
    if latest_files.files.len() != 1 {
        eprintln!("Error: No files found");
        return;
    }

    let filename = &latest_files.files[0].filename;

    // get download url
    let download_url = oda.get_download_url(filename).await;
    if let Err(e) = download_url {
        eprintln!("Error: {}", e);
        return;
    }

    let download_url = download_url.unwrap();

    // download file
    let download = oda
        .download_file(&download_url.temporary_download_url, filename)
        .await;
    if let Err(e) = download {
        eprintln!("Error: {}", e);
        return;
    }

    println!("Downloaded file: {}", filename);

    let dataset = dataset::Dataset::new(filename.clone());
    if let Err(e) = dataset {
        eprintln!("Error: {}", e);
        return;
    }

    let dataset = dataset.unwrap();
    let longitude = 4.913034;
    let latitude = 52.342332;

    // loop over images 1 to 25 and store datetime and mm/h values to print later
    let mut values = Vec::new();
    for i in 1..26 {
        let image = dataset.read_image(i);
        if let Err(e) = image {
            eprintln!("Error: {}", e);
            return;
        }

        let image = image.unwrap();
        let value = image.get_value_at_lon_lat(longitude, latitude).unwrap();
        let mm_per_hour = pixel_to_mm_hr(value.unwrap());
        values.push((image.datetime, mm_per_hour));
    }

    println!("Values for lon: {}, lat: {}", longitude, latitude);

    for (datetime, value) in values {
        println!("{} - {} mm/h", datetime, value);
    }
}
