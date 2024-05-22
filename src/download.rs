use crate::EvmSelectors;
use anyhow::Result;
use reqwest::Client;
use std::{fs, path::Path, time::Duration};

impl EvmSelectors {
    /// Downloads the latest known selectors from the [OpenChain API].
    /// The result is returned as string and not persisted.
    /// Note that the download speed can differ significantly, from seconds to an hour or more.
    /// A timeout can be specified to limit the time the request is allowed to take.
    ///
    /// # Errors
    ///
    /// This function will return an error if the HTTP request fails.
    ///
    /// [OpenChain API]: https://docs.openchain.xyz/
    pub async fn download(timeout: Option<Duration>) -> Result<String> {
        let url = "https://api.openchain.xyz/signature-database/v1/export";
        let client = Client::new();
        let mut request = client.get(url);
        if let Some(timeout) = timeout {
            request = request.timeout(timeout);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to download from {}: Request returned bad status code {}",
                url,
                response.status()
            ));
        }

        Ok(response.text().await?)
    }

    /// Downloads the latest known selectors from the [OpenChain API] and persists them to the file at `path`.
    /// If the file exists, it will be overwritten. The directory structure will be created if it does not exist.
    /// Note that the download speed can differ significantly, from seconds to an hour or more.
    /// A timeout can be specified to limit the time the request is allowed to take.
    ///
    /// # Errors
    ///
    /// This function will return an error if the HTTP request fails, or if writing to the file fails.
    ///
    /// [OpenChain API]: https://docs.openchain.xyz/
    pub async fn download_to_file(path: &Path, timeout: Option<Duration>) -> Result<()> {
        let raw = Self::download(timeout).await?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, raw)?;
        Ok(())
    }
}
