use env_logger::Env;
use futures::StreamExt;
use subsquid_data_streaming::{DataSource, DataStream, LogFilter, LogOptions};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let topic = "0x1c411e9a96e071241c2f21f7726b17ae89e3cab4c78be50e062b03a9fffbbad1";
    let start_block = 20_000_000;
    let end_block = 20_000_001;

    log::info!(
        "Looking for logs with topic {} in blocks {} through {}...",
        topic,
        start_block,
        end_block
    );

    let data_stream = DataStream::new()
        .set_data_source(DataSource::Subsquid(
            "https://v2.archive.subsquid.io/network/ethereum-mainnet".to_string(),
        ))
        .add_log_filter(LogFilter::new().with_topic(topic))
        .add_log_options(LogOptions {
            topic0: true,
            data: true,
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
                    if let Some(logs) = item.logs {
                        for log in logs {
                            log::info!(
                                "Received Log: Topics = {:?}, Tx Index = {:?}",
                                log.topics,
                                log.transaction_index
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