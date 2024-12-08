use clap::Parser;
use dotenv::dotenv;
use knmi_nowcast::{
    dataplatform::api::OpenDataAPI,
    nowcast::{dataset, transformation::pixel_to_mm_hr},
};

#[derive(Parser, Debug)]
#[command(name = "knmi")]
#[command(version = "1.0")]
#[command(about = "A CLI tool to get KNMI precipitation forecasts", long_about = None)]
struct Cli {
    /// API Key for accessing the service
    #[arg(short, long, env = "KNMI_API_KEY")]
    api_key: String,

    /// Location as a comma-separated string "longitude,latitude"
    #[arg(short, long, env = "KNMI_LOCATION", value_parser = parse_location)]
    location: (f64, f64),

    /// Output directory for storing downloaded files
    #[arg(
        short,
        long,
        env = "KNMI_OUTPUT_DIR",
        required = false,
        default_value = "./output"
    )]
    output_dir: Option<String>,

    /// Input file to load, new file will be downloaded if not provided
    #[arg(short, long, env = "KNMI_INPUT_FILE", required = false)]
    input_file: Option<String>,
}

fn parse_location(value: &str) -> Result<(f64, f64), String> {
    let parts: Vec<&str> = value.split(',').map(str::trim).collect();

    if parts.len() != 2 {
        return Err("Location must be in the format 'longitude,latitude'".to_string());
    }

    let longitude = parts[0]
        .parse::<f64>()
        .map_err(|_| "Invalid longitude value")?;
    let latitude = parts[1]
        .parse::<f64>()
        .map_err(|_| "Invalid latitude value")?;

    if (-180.0..=180.0).contains(&longitude) && (-90.0..=90.0).contains(&latitude) {
        Ok((longitude, latitude))
    } else {
        Err(
            "Longitude must be between -180.0 and 180.0, and latitude between -90.0 and 90.0."
                .to_string(),
        )
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Cli::parse();
    let (longitude, latitude) = args.location;
    let api_key = args.api_key;
    let output_dir = args.output_dir;
    let input_file = args.input_file;
    let mut filepath = String::from("");

    if let Some(output_dir) = output_dir {
        filepath = output_dir.clone();
        create_dir_if_not_exists(&output_dir);
    }

    // if input file is provided, directly load and print data
    if let Some(input_file) = input_file {
        load_and_print_data(input_file, longitude, latitude);
        return;
    }

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
    let download_url = oda.get_download_url(filename.clone()).await;
    if let Err(e) = download_url {
        eprintln!("Error: {}", e);
        return;
    }

    let download_url = download_url.unwrap();

    // create full path full path is filename if output_dir is not provided
    if !filepath.is_empty() {
        filepath.push_str("/");
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
        date_time_first.format("%d-%m-%Y").to_string(),
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
