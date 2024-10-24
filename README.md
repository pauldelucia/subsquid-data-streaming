# Subsquid Rust Data Streaming Library

This Rust library allows you to stream on-chain data from Ethereum-like blockchains using an API exposed by the Subsquid data lake. It enables users to filter logs and transactions based on specific criteria and provides customizable options for selecting which fields of the data to return.

The library is designed to be flexible, supporting parallel queries, block range streaming, and handling of large amounts of on-chain data in a performant manner.

## Features

- **Stream Blockchain Data**: Stream logs and transactions from a specific block range in real-time.
- **Customizable Filters**: Filter logs by address, topics, and transactions by sender/recipient.
- **Field Selection**: Choose which fields to include in the output for logs and transactions (topics, data, transaction hash, etc.).

## Installation

Add the following to your `Cargo.toml` to use the library in your project:

```toml
[dependencies]
subsquid-take-home = "0.1.0"
```

## Example Usage

Here’s how to use the library to stream data from a Subsquid node:

```rust
use subsquid_data_streaming::{DataStream, DataSource, LogFilter, TransactionFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_stream = DataStream::new()
        .set_data_source(DataSource::Subsquid("https://v2.archive.subsquid.io/network/ethereum-mainnet".to_string()))
        .from_block(6_000_000)
        .to_block(6_001_000)
        .add_log(LogFilter::new().with_address("0xabcd").with_topic("Burn(address,int24,int24,uint128,uint256)"))
        .add_tx(TransactionFilter::new().with_from("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"))
        .build()
        .await?;
        
    while let Some(data_result) = data_stream.next().await {
        match data_result {
            Ok(data) => {
                for item in data {
                    println!("Block: {}, Logs: {:?}", item.header.number, item.logs);
                }
            },
            Err(err) => eprintln!("Error streaming data: {:?}", err),
        }
    }
    
    Ok(())
}
```

### Explanation

- **Data Source**: We set up a data source using Subsquid's API, pointing to the Ethereum Mainnet.
- **Block Range**: The data stream is configured to pull data from block 6_000_000 to 6_001_000.
- **Filters**:
    - We filter logs with a specific address and topic.
    - We also filter transactions where the from address is specified.
- **Stream Data**: The stream fetches data in chunks and handles it in real-time.

## Components

### DataStream

The core of the library that sets up the streaming process. It allows you to define:

- The data source (e.g., Subsquid).
- Block range (start and end).
- Filters for logs and transactions.
- Field options to specify what data fields should be included in the output.

### Filters

- **LogFilter**: Filters logs by specific addresses and topics.
- **TransactionFilter**: Filters transactions by from or to addresses.

### Options

- **LogOptions**: Specify which fields (e.g., topics, data) to include in the logs.
- **TransactionOptions**: Specify which fields (e.g., hash, gas) to include in transactions.

## Error Handling
All errors are handled using the DataStreamError enum, which covers network errors, invalid responses, deserialization issues, and configuration errors.