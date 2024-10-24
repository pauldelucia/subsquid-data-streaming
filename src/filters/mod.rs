//! Filters for fetched Ethereum logs and transactions

/// Filters for fetched Ethereum logs
pub mod log_filter;
/// Filters for fetched Ethereum transactions
pub mod transaction_filter;

pub use log_filter::LogFilter;
pub use transaction_filter::TransactionFilter;
