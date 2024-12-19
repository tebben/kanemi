use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub filename: String,
    pub size: u64,
    pub created: String,
    pub last_modified: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilesResponse {
    pub is_truncated: bool,
    pub result_count: i32,
    pub files: Vec<File>,
    pub max_results: i32,
    pub start_after_filename: String,
    pub next_page_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlResponse {
    pub content_type: String,
    pub last_modified: String,
    pub size: String,
    pub temporary_download_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationData {
    pub dataset_name: String,
    pub dataset_version: String,
    pub filename: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationReponse {
    pub specversion: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub source: String,
    pub id: String,
    pub time: String,
    pub datacontenttype: String,
    pub data: NotificationData,
}
