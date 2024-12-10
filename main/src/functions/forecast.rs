use knmi_nowcast::{
    dataplatform::{api::OpenDataAPI, models::config::DatasetConfig},
    nowcast::{dataset, transformation::pixel_to_mm_hr},
};

pub async fn get_forecast(
    api_key: String,
    dataset_config: DatasetConfig,
    input_file: Option<String>,
    output_dir: Option<String>,
    longitude: f64,
    latitude: f64,
) {
    // if input file is provided, directly load and print data
    if let Some(input_file) = input_file {
        load_and_print_data(input_file, longitude, latitude);
        return;
    }

    let mut filepath = String::from("");
    if let Some(output_dir) = output_dir {
        filepath = output_dir.clone();
        create_dir_if_not_exists(&output_dir);
    }

    print_from_download(api_key, dataset_config, filepath, longitude, latitude).await;
}

async fn print_from_download(
    api_key: String,
    dataset_config: DatasetConfig,
    mut filepath: String,
    longitude: f64,
    latitude: f64,
) {
    let oda = OpenDataAPI::new(
        "https://api.dataplatform.knmi.nl/open-data/v1".to_string(),
        api_key,
        dataset_config,
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
    let download_url = oda.get_download_url(filename.clone()).await;
    if let Err(e) = download_url {
        eprintln!("Error: {}", e);
        return;
    }

    let download_url = download_url.unwrap();

    // create full path full path is filename if output_dir is not provided
    if !filepath.is_empty() {
        filepath.push('/');
    }
    filepath.push_str(filename);

    // download file
    let download = oda
        .download_file(download_url.temporary_download_url, filepath.clone())
        .await;
    if let Err(e) = download {
        eprintln!("Error: {}", e);
        return;
    }

    println!("Downloaded file: {}", filepath);
    load_and_print_data(filepath.clone(), longitude, latitude);
}

// create dir if not exists
fn create_dir_if_not_exists(dir: &str) {
    if !std::path::Path::new(&dir).exists() {
        std::fs::create_dir_all(dir).unwrap();
    }
}

fn load_and_print_data(filename: String, longitude: f64, latitude: f64) {
    let dataset = dataset::Dataset::new(filename);
    if let Err(e) = dataset {
        eprintln!("Error: {}", e);
        return;
    }

    let dataset = dataset.unwrap();

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

    let date_time_first = values.first().unwrap().0;
    println!(
        "\x1b[45m\x1b[37m\x1b[1m{} - Precipitation @ {}, {}\x1b[0m",
        date_time_first.format("%d-%m-%Y"),
        longitude,
        latitude
    );

    println!("\x1b[92m-----------------------------------\x1b[0m");
    print!("\x1b[33m{:<10}\x1b[0m", "Time");
    print!("\x1b[33m{:<10}\x1b[0m", "Precipitation (mm/h)");
    println!();
    println!("\x1b[92m-----------------------------------\x1b[0m");

    for (datetime, value) in values {
        print!("\x1b[33m{:<10}\x1b[0m", datetime.format("%H:%M"));
        println!("\x1b[33m{:<10}\x1b[0m", value);
    }
}
