use serde::Deserialize;

/// Represents a log entry from a transaction in a block.
///
/// This struct is used to deserialize log data, which contains information about specific
/// events emitted by contracts during the execution of a transaction.
#[derive(Debug, Deserialize, Clone)]
pub struct LogEntry {
    /// Topics associated with the log entry, which identify the event being emitted.
    #[serde(default)]
    pub topics: Vec<String>,
    /// Data associated with the log entry, typically encoded event data.
    #[serde(default)]
    pub data: String,
    /// The index of the transaction within the block where this log entry was generated.
    #[serde(default, rename = "transactionIndex")]
    pub transaction_index: u64,
    /// The index of the log entry within the block.
    #[serde(default, rename = "logIndex")]
    pub log_index: u64,
    #[serde(default)]
    pub address: String,
    #[serde(default, rename = "blockNumber")]
    pub block_number: u64,
    #[serde(default, rename = "blockHash")]
    pub block_hash: String,
    #[serde(default, rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(default)]
    pub removed: bool,
}
