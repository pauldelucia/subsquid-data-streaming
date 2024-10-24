use crate::errors::DataStreamError;
use reqwest::Client;

/// `RouterClient` is responsible for interacting with the API gateway (router) to retrieve
/// information such as the dataset height and worker URLs.
///
/// The `RouterClient` sends HTTP requests to the base URL of the API and parses the responses,
/// which are necessary to fetch on-chain data through workers.
#[derive(Clone)]
pub struct RouterClient {
    base_url: String, // The base URL of the API router.
    client: Client,   // The HTTP client for making requests.
}

impl RouterClient {
    /// Creates a new `RouterClient` with the given `base_url`.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the API router.
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    /// Retrieves the height of the dataset from the router by sending a GET request to the `/height` endpoint.
    ///
    /// # Returns
    ///
    /// * `Result<u64, DataStreamError>` - The dataset height on success, or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns a `DataStreamError` if there is an issue with the request or response parsing.
    pub async fn get_dataset_height(&self) -> Result<u64, DataStreamError> {
        let url = format!("{}/height", self.base_url);
        let resp = self.client.get(&url).send().await?; // Send a GET request to fetch the dataset height.
        let text = resp.text().await?; // Get the response body as a string.

        // Parse the response text as an integer representing the dataset height.
        let height = text.parse::<u64>().map_err(|e| {
            DataStreamError::InvalidResponse(format!("Failed to parse height: {}", e))
        })?;

        Ok(height)
    }

    /// Retrieves the URL of a worker responsible for a specific block by sending a GET request to the `{block_number}/worker` endpoint.
    ///
    /// # Arguments
    ///
    /// * `block_number` - The block number for which a worker URL is requested.
    ///
    /// # Returns
    ///
    /// * `Result<String, DataStreamError>` - The URL of the worker responsible for the given block on success, or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns a `DataStreamError` if there is an issue with the request or response parsing.
    pub async fn get_worker_url(&self, block_number: u64) -> Result<String, DataStreamError> {
        let url = format!("{}/{}/worker", self.base_url, block_number);
        let resp = self.client.get(&url).send().await?; // Send a GET request to fetch the worker URL.
        let worker_url = resp.text().await?; // Get the response body as the worker URL string.
        Ok(worker_url)
    }
}
