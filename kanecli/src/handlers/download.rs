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
    let dataset_config =
        kanemi::dataplatform::models::config::DatasetConfig::new(options.name, options.version);

    let oda = OpenDataAPI::new(options.api_key, dataset_config, None);
    let download_result = oda
        .download_latest_file(&options.dir, options.filename, Some(true))
        .await;
    if let Err(e) = download_result {
        let message = ErrorMessage {
            success: false,
            error: e.to_string(),
        };

        print_message(message);
        return;
    }

    let (_, latest_download_url) = download_result.unwrap();
    let message = DownloadMessage {
        success: true,
        data: latest_download_url,
    };

    print_message(message);
}

fn print_message<T: Serialize>(msg: T) {
    let json = serde_json::to_string(&msg).unwrap();
    println!("{}", json);
}
