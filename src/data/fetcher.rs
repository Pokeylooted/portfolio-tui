use anyhow::Result;
use reqwest::Client;
use std::fs;
use std::path::Path;

/// Fetches data from a source (GitHub or local file)
pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    /// Create a new fetcher
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Fetch data from a source
    pub async fn fetch(&self, source: &str) -> Result<String> {
        if source.starts_with("http") {
            self.fetch_from_github(source).await
        } else {
            self.fetch_from_local(source)
        }
    }

    /// Fetch data from GitHub
    async fn fetch_from_github(&self, url: &str) -> Result<String> {
        // Convert GitHub URL to raw content URL if needed
        let raw_url = if url.contains("github.com") && url.contains("/blob/") {
            url.replace("github.com", "raw.githubusercontent.com")
                .replace("/blob/", "/")
        } else {
            url.to_string()
        };

        // Fetch the content
        let response = self.client.get(&raw_url).send().await?;
        let content = response.text().await?;
        Ok(content)
    }

    /// Fetch data from a local file
    fn fetch_from_local(&self, path: &str) -> Result<String> {
        let content = fs::read_to_string(Path::new(path))?;
        Ok(content)
    }
}