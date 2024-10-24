//! Structs to represent fetched items

/// Contains a block header along with associated log and transaction entries
pub mod data_item;
/// Data from fetched logs
pub mod log_entry;
/// Data from fetched transactions
pub mod transaction_entry;

pub use log_entry::LogEntry;
pub use transaction_entry::TransactionEntry;
