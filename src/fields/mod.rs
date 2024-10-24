//! Options for specifying what data to return from filtered Ethereum logs and transactions

/// Options for selecting both log and transaction fields.
pub mod fields;
/// Options for selecting log fields.
pub mod log_fields;
/// Options for selecting transaction fields.
pub mod transaction_fields;

pub use log_fields::LogFields;
pub use transaction_fields::TransactionFields;
