use super::models::free::FreeResponse;
use super::models::reverse::ReverseResponse;
use super::options::free::FreeOptions;
use super::options::reverse::ReverseOptions;
use reqwest::Error;

#[derive(Debug)]
pub struct LocationServer {
    pub url: String,
}

impl Default for LocationServer {
    fn default() -> Self {
        LocationServer {
            url: "https://api.pdok.nl/bzk/locatieserver/search/v3_1".to_string(),
        }
    }
}

impl LocationServer {
    pub fn new(url: String) -> LocationServer {
        LocationServer { url }
    }

    pub async fn get_free(self, options: FreeOptions) -> Result<FreeResponse, Error> {
        let query = options.construct_query();
        let request = format!("{}/free?{}&wt=json", self.url, query);
        let response = reqwest::get(request).await?;
        let data: FreeResponse = response.json().await?;

        Ok(data)
    }

    pub async fn get_reverse(self, options: ReverseOptions) -> Result<ReverseResponse, Error> {
        let query = options.construct_query();
        let request = format!("{}/reverse?{}&wt=json", self.url, query);
        let response = reqwest::get(request).await?;
        let data: ReverseResponse = response.json().await?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_free() {}
}
