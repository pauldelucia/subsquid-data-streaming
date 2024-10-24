//! Subsquid Rust on-chain data streaming library.
//!
//! This library is designed to provide a data stream for blockchain events like logs and transactions,
//! utilizing filters and options to narrow down the data. It can interact with workers that fetch data
//! from the blockchain, and users can specify block ranges, filters, and data field selections.
//!
//! The core components are:
//! - **DataStream**: The main streaming structure to fetch blockchain data in real-time.
//! - **Filters**: Used to define what logs and transactions to capture.
//! - **Options**: Used to define what data fields to include in the result (topics, data, transaction hash, etc.).

/// Defines the supported data sources (e.g., Subsquid, EVM RPC).
pub mod data_source;

/// Core functionality for building and managing the data stream.
pub mod data_stream;

/// Error handling definitions for the library.
pub mod errors;

/// Filtering mechanisms for logs and transactions.
pub mod filters;

/// Models representing logs, transactions, and block data.
pub mod models;

/// Options to define which fields (topics, data, etc.) should be returned.
pub mod options;

/// Client responsible for interacting with the router to get worker URLs.
pub mod router_client;

/// Utility functions used in parsing or handling block ranges.
mod utils;

/// Client responsible for interacting with the worker to fetch data.
pub mod worker_client;

/// Structure defining the worker query.
pub mod worker_query;

pub use data_source::DataSource; // Represents the supported data sources (e.g., Subsquid).
pub use data_stream::DataStream; // The main structure for building and managing the data stream.
pub use errors::DataStreamError; // Errors that can be encountered during streaming.
pub use filters::{LogFilter, TransactionFilter}; // Log and transaction filters.
pub use models::{LogEntry, TransactionEntry}; // Structures representing logs and transactions.
pub use options::{LogOptions, TransactionOptions}; // Options for selecting fields in logs and transactions.
