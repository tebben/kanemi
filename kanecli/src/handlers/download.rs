use crate::commands::download::DownloadOptions;
use kanemi::dataplatform::api::OpenDataAPI;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadMessage {
    pub success: bool,
    pub data: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorMessage {
    pub success: bool,
    pub error: String,
}

pub async fn handle_command(options: DownloadOptions) {
    let dataset_config = kanemi::dataplatform::models::config::DatasetConfig::new(
        options.dataset_name,
        options.dataset_version,
    );

    let oda = OpenDataAPI::new(options.api_key, dataset_config, None);
    let latets_download_url = oda
        .download_latest_file(options.output_dir, options.output_filename, Some(true))
        .await;
    if let Err(e) = latets_download_url {
        let message = ErrorMessage {
            success: false,
            error: e.to_string(),
        };

        print_message(message);
        return;
    }

    let message = DownloadMessage {
        success: true,
        data: latets_download_url.unwrap(),
    };

    print_message(message);
}

fn print_message<T: Serialize>(msg: T) {
    let json = serde_json::to_string(&msg).unwrap();
    println!("{}", json);
}
