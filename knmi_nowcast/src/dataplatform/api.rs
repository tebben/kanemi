use models::response::*;
use reqwest::Client;
use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use super::models;

pub struct OpenDataAPI {
    base_url: String,
    dataset_name: String,
    version: String,
    api_key: String,
}

impl OpenDataAPI {
    pub fn new(base_url: String, dataset_name: String, version: String, api_key: String) -> Self {
        OpenDataAPI {
            base_url,
            dataset_name,
            version,
            api_key,
        }
    }

    fn get_latest_file_url_and_params(&self, max_files: i8) -> (String, [(&str, String); 3]) {
        let url = format!(
            "{}/datasets/{}/versions/{}/files",
            &self.base_url, &self.dataset_name, &self.version
        );

        let query_params = [
            ("maxKeys", max_files.to_string()),
            ("orderBy", "created".to_string()),
            ("sorting", "desc".to_string()),
        ];

        (url, query_params)
    }

    fn get_file_download_url(&self, filename: String) -> String {
        format!(
            "{}/datasets/{}/versions/{}/files/{}/url",
            &self.base_url, &self.dataset_name, &self.version, filename
        )
    }

    fn create_get_request(
        &self,
        url: String,
        query_params: Option<&[(&str, String); 3]>,
    ) -> reqwest::RequestBuilder {
        let client = Client::new();
        let request = client.get(url).header("Authorization", &self.api_key);

        match query_params {
            Some(params) => request.query(params),
            None => request,
        }
    }

    // This function returns the latest file from the dataset
    pub async fn get_latest_files(&self, max_files: i8) -> Result<FilesResponse, reqwest::Error> {
        let (url, query_params) = self.get_latest_file_url_and_params(max_files);
        let response = self
            .create_get_request(url, Some(&query_params))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let json: FilesResponse = response.json().await?;
        Ok(json)
    }

    // This function returns the download URL for a given file
    pub async fn get_download_url(&self, filename: String) -> Result<UrlResponse, reqwest::Error> {
        let url = self.get_file_download_url(filename);
        let response = self.create_get_request(url, None).send().await?;

        response.error_for_status_ref()?;

        let data: UrlResponse = response.json().await?;
        Ok(data)
    }

    // This function downloads a file from a given URL and saves it to the output path
    pub async fn download_file(
        &self,
        url: String,
        output_path: String,
    ) -> Result<(), Box<dyn Error>> {
        let client = Client::new();
        let response = client.get(url).send().await?;

        if response.status().is_success() {
            let mut file = File::create(output_path).await?;
            let content = response.bytes().await?;
            file.write_all(&content).await?;
            Ok(())
        } else {
            Err("Failed to download file".into())
        }
    }
}
