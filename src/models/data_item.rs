use super::{LogEntry, TransactionEntry};
use serde::Deserialize;

/// Represents a single data item containing a block header, logs, and transactions.
///
/// This struct is used to deserialize a data item that contains information about a block
/// and optionally includes logs and transactions.
#[derive(Debug, Deserialize)]
pub struct DataItem {
    /// The block header containing metadata about the block.
    pub header: BlockHeader,
    /// Optional list of log entries related to the block.
    pub logs: Option<Vec<LogEntry>>,
    /// Optional list of transaction entries related to the block.
    pub transactions: Option<Vec<TransactionEntry>>,
}

/// Represents the header of a block in the blockchain.
///
/// The block header includes the block number and other potential metadata.
#[derive(Debug, Deserialize)]
pub struct BlockHeader {
    /// The block number of this block.
    pub number: u64,
}

/// Returns the block number of the last item in the provided list of `DataItem`s.
///
/// This function extracts the block number from the `header` of the last `DataItem` in the list.
///
/// # Parameters
///
/// * `data_items` - A slice of `DataItem` structs.
///
/// # Returns
///
/// * `Option<u64>` - The block number of the last item if present, otherwise `None`.
///
/// # Example
///
/// ```
/// use subsquid_data_streaming::models::data_item::last_block_number;
///
/// let data_items = vec![];  // Populate with `DataItem` instances
/// let last_block = last_block_number(&data_items);
/// ```
pub fn last_block_number(data_items: &[DataItem]) -> Option<u64> {
    data_items.last().map(|item| item.header.number)
}
