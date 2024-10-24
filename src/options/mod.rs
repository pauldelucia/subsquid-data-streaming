//! Options for specifying what data to return from filtered Ethereum logs and transactions

/// Options for selecting both log and transaction fields.
pub mod field_options;
/// Options for selecting log fields.
pub mod log_options;
/// Options for selecting transaction fields.
pub mod transaction_options;

pub use log_options::LogOptions;
pub use transaction_options::TransactionOptions;
