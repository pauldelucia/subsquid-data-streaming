use std::collections::HashMap;

use crate::fields::fields::Fields;
use crate::filters::log_filter::LogsFilter;
use crate::filters::transaction_filter::TransactionsFilter;
use crate::{LogFields, LogFilter, TransactionFields, TransactionFilter};
use serde::Serialize;

/// Represents a query to be sent to the worker node, specifying the block range and filtering criteria.
///
/// The `WorkerQuery` defines the range of blocks to fetch and optional filters for logs and transactions.
/// The fields define which data to retrieve for each log and transaction (topics, data, etc.).
#[derive(Serialize)]
pub(crate) struct WorkerQuery {
    #[serde(rename = "fromBlock")]
    pub(crate) from_block: u64, // Starting block for the query.
    #[serde(rename = "toBlock", skip_serializing_if = "Option::is_none")]
    pub to_block: Option<u64>, // Optional end block for the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<LogsFilter>>, // Filters for logs based on address and topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<TransactionsFilter>>, // Filters for transactions based on sender, receiver, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Fields>, // Specifies which fields (topics, data, etc.) to retrieve.
}

impl WorkerQuery {
    /// Creates a new `WorkerQuery` from log and transaction filters, as well as options for which fields to retrieve.
    ///
    /// # Arguments
    ///
    /// * `from_block` - The starting block number for the query.
    /// * `to_block` - The optional ending block number.
    /// * `log_filters` - A list of log filters to be applied.
    /// * `tx_filters` - A list of transaction filters to be applied.
    /// * `log_options` - Options for selecting log fields (topics, data, etc.).
    /// * `tx_options` - Options for selecting transaction fields (hash, gas, etc.).
    ///
    /// # Returns
    ///
    /// A `WorkerQuery` instance ready to be sent to the worker.
    pub fn from_filters(
        from_block: u64,
        to_block: Option<u64>,
        log_filters: &[LogFilter],
        tx_filters: &[TransactionFilter],
        log_options: &Option<LogFields>,
        tx_options: &Option<TransactionFields>,
    ) -> Self {
        let fields = if log_options.is_some() || tx_options.is_some() {
            Some(Fields {
                log: log_options.as_ref().map(|opts| {
                    let mut log_map = HashMap::new();
                    log_map.insert("topics".to_string(), opts.topic0);
                    log_map.insert("data".to_string(), opts.data);
                    log_map
                }),
                transaction: tx_options.as_ref().map(|opts| {
                    let mut tx_map = HashMap::new();
                    tx_map.insert("hash".to_string(), opts.hash);
                    tx_map.insert("to".to_string(), opts.to);
                    tx_map.insert("from".to_string(), opts.from);
                    // Add more options as needed
                    tx_map
                }),
            })
        } else {
            None
        };

        Self {
            from_block,
            to_block,
            logs: if !log_filters.is_empty() {
                Some(
                    log_filters
                        .iter()
                        .map(|filter| LogsFilter::from(filter))
                        .collect(),
                )
            } else {
                Some(vec![])
            },
            transactions: if !tx_filters.is_empty() {
                Some(
                    tx_filters
                        .iter()
                        .map(|filter| TransactionsFilter::from(filter))
                        .collect(),
                )
            } else {
                None
            },
            fields,
        }
    }
}
