# Subsquid Rust Data Streaming Library

This Rust library allows you to stream on-chain data from Ethereum-like blockchains using an API exposed by the Subsquid data lake. It enables users to filter logs and transactions based on specific criteria and provides customizable options for selecting which fields of the data to return.

The library is designed to be flexible, supporting parallel queries, block range streaming, and handling of large amounts of on-chain data in a performant manner.

## Features

- **Stream Blockchain Data**: Stream logs and transactions from a specific block range in real-time.
- **Customizable Filters**: Filter logs by address, topics, and transactions by sender/recipient.
- **Field Selection**: Choose which fields to include in the output for logs and transactions (topics, data, transaction hash, etc.).

## Example Usage

Here’s how to use the library to stream data from a Subsquid node:

```rust
use env_logger::Env;
use futures::StreamExt;
use subsquid_data_streaming::{DataSource, DataStream, TransactionFilter, TransactionFields};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let sender = "0x6e869cadc1cb3d4c6291e6e939b5b55d51c69084";
    let start_block = 20_000_000;
    let end_block = 20_010_000;

    log::info!(
        "Looking for transactions with sender {} in blocks {} through {}...",
        sender,
        start_block,
        end_block
    );

    let data_stream = DataStream::new()
        .set_data_source(DataSource::Subsquid(
            "https://v2.archive.subsquid.io/network/ethereum-mainnet".to_string(),
        ))
        .add_tx_filter(TransactionFilter::new().with_from(sender))
        .add_tx_fields(TransactionFields {
            hash: true,
            to: true,
            ..Default::default()
        })
        .from_block(start_block)
        .to_block(end_block)
        .build()
        .await
        .expect("Failed to build DataStream");

    tokio::pin!(data_stream);

    while let Some(result) = data_stream.next().await {
        match result {
            Ok(data_batch) => {
                for item in data_batch {
                    if let Some(transactions) = item.transactions {
                        for tx in transactions {
                            log::info!(
                                "Received Transaction: Hash = {:?}, To Address = {:?}",
                                tx.hash.unwrap_or_default(),
                                tx.to.unwrap_or_default()
                            );
                        }
                    }
                }
            }
            Err(e) => log::error!("Error: {:?}", e),
        }
    }

    // Sleep for a short duration to allow logger to output everything before exiting
    sleep(Duration::from_secs(2)).await;
}
```

### Explanation

- **Data Source**: We set up a data source using Subsquid's API, pointing to the Ethereum Mainnet.
- **Block Range**: The data stream is configured to pull data from block 20_000_000 to 20_010_000.
- **Filters**:
    - We filter transactions with a specific sender.
- **Stream Data**: The stream fetches data in chunks and handles it in real-time.
- **Fields**:
    - From the fetched transactions, we log the transactions hashes and recipients

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

### Fields

- **LogFields**: Specify which fields (e.g., topics, data) to include in the logs.
- **TransactionFields**: Specify which fields (e.g., hash, gas) to include in transactions.

## Error Handling
All errors are handled using the DataStreamError enum, which covers network errors, invalid responses, deserialization issues, and configuration errors.
