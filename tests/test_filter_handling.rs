use futures::StreamExt;
use subsquid_data_streaming::{
    DataSource, DataStream, LogFilter, LogOptions, TransactionFilter, TransactionOptions,
};

#[tokio::test]
async fn test_filter_handling() {
    // Build the DataStream with log and transaction filters
    let data_stream = DataStream::new()
        .set_data_source(DataSource::Subsquid(
            "https://v2.archive.subsquid.io/network/ethereum-mainnet".to_string(),
        ))
        .add_log_options(LogOptions {
            topic0: true,
            data: true,
            ..Default::default()
        })
        .add_tx_options(TransactionOptions {
            hash: true,
            ..Default::default()
        })
        .add_log_filter(
            LogFilter::new()
                .with_address("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")
                .with_topic("Transfer(address,address,uint256)"),
        )
        .add_tx_filter(
            TransactionFilter::new().with_from("0x742d35cc6634c0532925a3b844bc454e4438f44e"),
        )
        .from_block(20_000_000)
        .to_block(20_000_100)
        .build()
        .await
        .expect("Failed to build DataStream");

    tokio::pin!(data_stream);

    let mut received_data = false;

    // Verify data retrieval and matching filters
    while let Some(result) = data_stream.next().await {
        match result {
            Ok(data_batch) => {
                assert!(!data_batch.is_empty(), "Data batch should not be empty");
                for item in data_batch {
                    if let Some(transactions) = &item.transactions {
                        for tx in transactions {
                            assert_eq!(
                                tx.hash.clone().expect("Expected a hash in the response").to_lowercase(),
                                "0xbb4b3fc2b746877dce70862850602f1d19bd890ab4db47e6b7ee1da1fe578a0d",
                                "Transaction sender should match the filter"
                            );
                        }
                    }
                }
                received_data = true;
            }
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    assert!(
        received_data,
        "Should have received data with the applied filters"
    );
}
