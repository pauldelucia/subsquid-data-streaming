use crate::errors::DataStreamError;
use crate::models::data_item::DataItem;
use crate::worker_query::WorkerQuery;
use reqwest::Client;

/// `WorkerClient` is responsible for sending the `WorkerQuery` to the worker node and fetching the corresponding data.
///
/// The worker node processes the query and returns a batch of data items (logs, transactions, etc.).
pub struct WorkerClient {
    base_url: String, // The base URL of the worker node.
    client: Client,   // The HTTP client for making requests.
}

impl WorkerClient {
    /// Creates a new `WorkerClient` with the given base URL.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the worker node.
    ///
    /// # Returns
    ///
    /// A new `WorkerClient` instance.
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    /// Sends a `WorkerQuery` to the worker node and fetches the data matching the query.
    ///
    /// # Arguments
    ///
    /// * `query` - The query specifying the block range, filters, and field options.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<DataItem>, DataStreamError>` - A list of data items on success, or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns a `DataStreamError` if there is an issue with the request or response deserialization.
    pub(crate) async fn fetch_data(
        &self,
        query: &WorkerQuery,
    ) -> Result<Vec<DataItem>, DataStreamError> {
        let resp = self.client.post(&self.base_url).json(query).send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();

        if status.is_success() {
            // Deserialize the response into a vector of `DataItem`s.
            let data_items: Vec<DataItem> =
                serde_json::from_str(&text).map_err(DataStreamError::DeserializationError)?;
            Ok(data_items)
        } else {
            // Handle error response and deserialize the error as JSON if possible.
            let error_response: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
            Err(DataStreamError::InvalidResponse(format!(
                "Worker returned status {}: {}",
                status, error_response
            )))
        }
    }
}
