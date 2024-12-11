use super::errors::ApiError;
use super::models;
use models::config::DatasetConfig;
use models::response::*;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct OpenDataAPI {
    base_url: String,
    dataset_config: DatasetConfig,
    api_key: String,
}

impl OpenDataAPI {
    pub fn new(base_url: String, api_key: String, dataset_config: DatasetConfig) -> Self {
        OpenDataAPI {
            base_url,
            api_key,
            dataset_config,
        }
    }

    fn get_latest_file_url_and_params(&self, max_files: i8) -> (String, [(&str, String); 3]) {
        let url = format!(
            "{}/datasets/{}/versions/{}/files",
            &self.base_url, &self.dataset_config.dataset_name, &self.dataset_config.version
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
            &self.base_url,
            &self.dataset_config.dataset_name,
            &self.dataset_config.version,
            filename
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
    pub async fn get_latest_files(&self, max_files: i8) -> Result<FilesResponse, ApiError> {
        let (url, query_params) = self.get_latest_file_url_and_params(max_files);
        let response = self
            .create_get_request(url, Some(&query_params))
            .send()
            .await
            .map_err(|e| ApiError::FetchError(e.to_string()))?;

        let data: FilesResponse = response
            .json()
            .await
            .map_err(|e| ApiError::FilesResponseParseError(e.to_string()))?;

        Ok(data)
    }

    // This function returns the download URL for a given file
    pub async fn get_download_url(&self, filename: String) -> Result<UrlResponse, ApiError> {
        let url = self.get_file_download_url(filename);
        let response = self
            .create_get_request(url, None)
            .send()
            .await
            .map_err(|e| ApiError::FetchError(e.to_string()))?;

        let data: UrlResponse = response
            .json()
            .await
            .map_err(|e| ApiError::UrlResponseParseError(e.to_string()))?;

        Ok(data)
    }

    // This function downloads a file from a given URL and saves it to the output path
    pub async fn download_file(&self, url: String, output_path: String) -> Result<(), ApiError> {
        let client = Client::new();
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| ApiError::FetchError(e.to_string()))?;

        if response.status().is_success() {
            let mut file = File::create(output_path)
                .await
                .map_err(|e| ApiError::SaveFileError(e.to_string()))?;

            let content = response
                .bytes()
                .await
                .map_err(|e| ApiError::FetchError(e.to_string()))?;

            let write = file.write_all(&content).await;
            if write.is_err() {
                return Err(ApiError::SaveFileError(write.err().unwrap().to_string()));
            }

            Ok(())
        } else {
            Err(ApiError::FetchError(format!(
                "Failed to download file: {}",
                response.status()
            )))
        }
    }
}
