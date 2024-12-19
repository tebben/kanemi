use super::errors::ApiError;
use super::models::{self};
use models::config::DatasetConfig;
use models::response::{FilesResponse, UrlResponse};
use reqwest::Client;
use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

pub struct OpenDataAPI {
    base_url: String,
    dataset_config: DatasetConfig,
    api_key: String,
}

impl OpenDataAPI {
    pub fn new(
        api_key: String,
        dataset_config: DatasetConfig,
        custom_base_url: Option<String>,
    ) -> Self {
        OpenDataAPI {
            base_url: custom_base_url
                .unwrap_or_else(|| "https://api.dataplatform.knmi.nl/open-data/v1".to_string()),
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

    fn get_file_download_url(&self, filename: &str) -> String {
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

    /// Return the latest file from the dataset
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

    /// This function returns the download URL for the given file
    pub async fn get_download_url(&self, filename: &str) -> Result<UrlResponse, ApiError> {
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

    pub async fn get_latest_download_url(
        &self,
    ) -> Result<(models::response::File, UrlResponse), ApiError> {
        let latest_files = self.get_latest_files(1).await?;
        if latest_files.files.len() != 1 {
            return Err(ApiError::FetchError("No files found".to_string()));
        }

        let file = latest_files.files[0].clone();
        let response = self.get_download_url(&file.filename).await?;

        Ok((file, response))
    }

    pub async fn download_latest_file(
        &self,
        output_path: &str,
        filename: Option<String>,
        overwrite: Option<bool>,
    ) -> Result<(models::response::File, String), ApiError> {
        let (file, response) = self.get_latest_download_url().await?;
        let filename: String = filename.unwrap_or(file.filename.clone());
        let output_filepath = Path::new(output_path)
            .join(filename.clone())
            .to_str()
            .unwrap()
            .to_string();

        self.download_file(
            &response.temporary_download_url,
            &output_filepath,
            overwrite,
        )
        .await?;

        Ok((file, output_filepath))
    }

    /// This function downloads a file from the given URL and saves it to the given output_filepath
    pub async fn download_file(
        &self,
        url: &str,
        output_filepath: &str,
        overwrite: Option<bool>,
    ) -> Result<(), ApiError> {
        // Ensure the parent directory exists
        let output_filepath = Path::new(&output_filepath);
        if let Some(parent) = output_filepath.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| ApiError::SaveFileError(e.to_string()))?;
        }

        // Skip download if file already exists and overwrite is false
        if output_filepath.exists() && overwrite.unwrap_or(false) {
            fs::remove_file(output_filepath)
                .await
                .map_err(|e| ApiError::SaveFileError(e.to_string()))?;
        } else if output_filepath.exists() {
            return Ok(());
        }

        let client = Client::new();
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| ApiError::FetchError(e.to_string()))?;

        if response.status().is_success() {
            let mut file = File::create(output_filepath)
                .await
                .map_err(|e| ApiError::SaveFileError(e.to_string()))?;

            let content = response
                .bytes()
                .await
                .map_err(|e| ApiError::FetchError(e.to_string()))?;

            file.write_all(&content)
                .await
                .map_err(|e| ApiError::SaveFileError(e.to_string()))?;

            // Explicitly flush the buffer to ensure all data is written
            // not doing this will result in errors while calling download and
            // dataset load functions multiple times in a row
            file.flush()
                .await
                .map_err(|e| ApiError::SaveFileError(e.to_string()))?;

            Ok(())
        } else {
            Err(ApiError::FetchError(format!(
                "Failed to download file: {}",
                response.status()
            )))
        }
    }
}
