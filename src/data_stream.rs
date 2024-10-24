use crate::data_source::DataSource;
use crate::errors::DataStreamError;
use crate::filters::{LogFilter, TransactionFilter};
use crate::models::data_item::{last_block_number, DataItem};
use crate::options::{LogOptions, TransactionOptions};
use crate::router_client::RouterClient;
use crate::utils::parse_block_range;
use crate::worker_client::WorkerClient;
use crate::worker_query::WorkerQuery;
use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::mpsc::{channel, Receiver};
use tokio::sync::Semaphore;

/// `DataStream` represents the main structure for fetching on-chain data from the EVM API.
/// It streams continuous data batches that match user-defined filters for logs and transactions.
///
/// # Usage Example
/// ```
/// use subsquid_data_streaming::{DataStream, DataSource, LogFilter, LogOptions, TransactionOptions};
///
/// let data_stream = DataStream::new()
///     .set_data_source(DataSource::Subsquid("https://v2.archive.subsquid.io/network/ethereum-mainnet".to_string()))
///     .from_block(6_000_000)
///     .add_log(LogFilter::new().with_address("0xabcd").with_topic("Burn(address,int24,int24,uint128,uint256)"))
///     .select_log_options(LogOptions::default())
///     .select_tx_options(TransactionOptions::default());
///
/// // Stream and process the data
/// ```
pub struct DataStream {
    data_source: Option<DataSource>, // Specifies the data source (e.g., Subsquid API)
    log_filters: Vec<LogFilter>,     // Filters for logs to be streamed
    tx_filters: Vec<TransactionFilter>, // Filters for transactions to be streamed
    log_options: Option<LogOptions>, // Options for log data (e.g., fields to select)
    tx_options: Option<TransactionOptions>, // Options for transaction data (e.g., fields to select)
    router_client: Option<RouterClient>, // Router client for interacting with the data source API
    receiver: Option<Receiver<Result<Vec<DataItem>, DataStreamError>>>, // Receiver for streaming data batches
    current_block: u64,    // Current block number being processed
    dataset_height: u64,   // Maximum block height available in the dataset
    from_block: u64,       // Starting block for the data stream
    to_block: Option<u64>, // Optional end block for the data stream
}

impl DataStream {
    /// Creates a new `DataStream` with no data source or filters initially configured.
    pub fn new() -> Self {
        Self {
            data_source: None,
            log_filters: Vec::new(),
            tx_filters: Vec::new(),
            log_options: None,
            tx_options: None,
            router_client: None,
            receiver: None,
            current_block: 0,
            dataset_height: 0,
            from_block: 0,
            to_block: None,
        }
    }

    /// Builds the data stream and initializes the router client. This fetches the dataset height and
    /// starts streaming data from the desired block range.
    ///
    /// # Errors
    /// Returns a `DataStreamError` if there are issues with setting up the stream, such as the data source not being set.
    pub async fn build(mut self) -> Result<Self, DataStreamError> {
        match &self.data_source {
            Some(DataSource::Subsquid(url)) => {
                self.router_client = Some(RouterClient::new(url.clone()));
                self.dataset_height = self
                    .router_client
                    .as_ref()
                    .unwrap()
                    .get_dataset_height()
                    .await?;
                if self.current_block == 0 {
                    self.current_block = self.initial_block();
                }
                self.start_streaming().await?;
                Ok(self)
            }
            Some(DataSource::EvmRpc(_)) => Err(DataStreamError::ConfigurationError(
                "EvmRpc data source not yet implemented".into(),
            )),
            None => Err(DataStreamError::ConfigurationError(
                "Data source not set".into(),
            )),
        }
    }

    /// Sets the initial block number to start fetching from.
    fn initial_block(&self) -> u64 {
        self.from_block
    }

    /// Starts the streaming process by submitting block ranges to the worker nodes. It spawns tasks
    /// for each block range and handles the concurrent streaming of data using a semaphore to limit concurrency.
    ///
    /// # Errors
    /// Returns a `DataStreamError` if there are issues with worker queries or sending data to the stream.
    async fn start_streaming(&mut self) -> Result<(), DataStreamError> {
        let (sender, receiver) = channel(10);
        self.receiver = Some(receiver);

        let (from_block, to_block) = self.compute_block_range();
        let max_block = self.dataset_height;

        let chunk_size = 10_000; // Defines the block range size per query
        let block_ranges = parse_block_range(from_block, to_block, chunk_size, max_block);

        let max_concurrent_tasks = 20; // Limits the number of concurrent block range queries
        let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));

        for (start, end) in block_ranges {
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            let router_client = self.router_client.clone().unwrap();
            let sender = sender.clone();
            let log_filters = self.log_filters.clone();
            let tx_filters = self.tx_filters.clone();
            let log_options = self.log_options.clone();
            let tx_options = self.tx_options.clone();

            tokio::spawn(async move {
                let _permit = permit;
                let mut current_block = start;
                let dataset_height = end;

                while current_block <= dataset_height {
                    match router_client.get_worker_url(current_block).await {
                        Ok(worker_url) => {
                            let worker_client = WorkerClient::new(worker_url);
                            let query = WorkerQuery::from_filters(
                                current_block,
                                Some(dataset_height),
                                &log_filters,
                                &tx_filters,
                                &log_options,
                                &tx_options,
                            );

                            match worker_client.fetch_data(&query).await {
                                Ok(data_batch) => {
                                    let last_block_opt = last_block_number(&data_batch);

                                    if sender.send(Ok(data_batch)).await.is_err() {
                                        break;
                                    }

                                    // Move to the next block after the last one processed
                                    if let Some(last_block) = last_block_opt {
                                        current_block = last_block + 1;
                                    } else {
                                        current_block += 1;
                                    }
                                }
                                Err(e) => {
                                    if sender.send(Err(e)).await.is_err() {
                                        break;
                                    }
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            let _ = sender.send(Err(e)).await;
                            break;
                        }
                    }
                }
            });
        }

        Ok(())
    }

    /// Sets the data source for the stream (e.g., Subsquid).
    pub fn set_data_source(mut self, data_source: DataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Specifies the starting block for the data stream.
    pub fn from_block(mut self, block_number: u64) -> Self {
        self.from_block = block_number;
        self
    }

    /// Specifies the ending block for the data stream.
    pub fn to_block(mut self, block_number: u64) -> Self {
        self.to_block = Some(block_number);
        self
    }

    /// Adds a filter for logs to be fetched in the data stream.
    pub fn add_log_filter(mut self, filter: LogFilter) -> Self {
        self.log_filters.push(filter);
        self
    }

    /// Adds a filter for transactions to be fetched in the data stream.
    pub fn add_tx_filter(mut self, filter: TransactionFilter) -> Self {
        self.tx_filters.push(filter);
        self
    }

    /// Sets the options for log data (e.g., fields to include in the results).
    pub fn add_log_options(mut self, options: LogOptions) -> Self {
        self.log_options = Some(options);
        self
    }

    /// Sets the options for transaction data (e.g., fields to include in the results).
    pub fn add_tx_options(mut self, options: TransactionOptions) -> Self {
        self.tx_options = Some(options);
        self
    }

    /// Computes the block range for streaming.
    fn compute_block_range(&self) -> (u64, Option<u64>) {
        (self.from_block, self.to_block)
    }
}

impl Stream for DataStream {
    type Item = Result<Vec<DataItem>, DataStreamError>;

    /// Polls the next available data batch in the stream.
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if let Some(receiver) = &mut this.receiver {
            match Pin::new(receiver).poll_recv(cx) {
                Poll::Ready(Some(item)) => Poll::Ready(Some(item)),
                Poll::Ready(None) => Poll::Ready(None),
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Ready(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::LogFilter;

    /// Test the creation of a `DataStream` with a data source.
    #[tokio::test]
    async fn test_data_stream_creation() {
        let data_stream = DataStream::new().set_data_source(DataSource::Subsquid(
            "https://v2.archive.subsquid.io/network/ethereum-mainnet".to_string(),
        ));
        assert!(data_stream.data_source.is_some());
    }

    /// Test the addition of log filters to the data stream.
    #[tokio::test]
    async fn test_data_stream_add_log() {
        let data_stream = DataStream::new()
            .set_data_source(DataSource::Subsquid(
                "https://v2.archive.subsquid.io/network/ethereum-mainnet".to_string(),
            ))
            .from_block(6_082_465)
            .add_log_filter(LogFilter {
                address: vec!["0xabcd".to_string()],
                topic0: vec![
                    "Burn(address,int24,int24,uint128,uint256)".to_string(),
                    "Initialize(uint160,int24)".to_string(),
                ],
            });
        assert!(data_stream.data_source.is_some());
        assert_eq!(data_stream.log_filters.first().unwrap().topic0.len(), 2);
    }
}
