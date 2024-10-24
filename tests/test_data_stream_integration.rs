use futures::StreamExt;
use subsquid_data_streaming::{filters::LogFilter, options::LogOptions, DataSource, DataStream};

#[tokio::test]
async fn test_data_stream_integration() {
    // Build the DataStream
    let data_stream = DataStream::new()
        .set_data_source(DataSource::Subsquid(
            "https://v2.archive.subsquid.io/network/ethereum-mainnet".to_string(),
        ))
        .add_log_options(LogOptions {
            topic0: true,
            data: true,
            ..Default::default()
        })
        .add_log_filter(
            LogFilter::new()
                .with_address("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")
                .with_topic("Transfer(address,address,uint256)"),
        )
        .from_block(20_000_000)
        .to_block(20_000_100)
        .build()
        .await
        .expect("Failed to build DataStream");

    tokio::pin!(data_stream);

    // Verify that data is retrieved successfully
    while let Some(result) = data_stream.next().await {
        match result {
            Ok(data_batch) => {
                assert!(!data_batch.is_empty(), "Data batch should not be empty");
            }
            Err(e) => panic!("Error: {:?}", e),
        }
    }
}
