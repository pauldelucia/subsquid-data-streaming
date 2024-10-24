use env_logger::Env;
use futures::StreamExt;
use subsquid_data_streaming::{DataSource, DataStream, TransactionFields, TransactionFilter};
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
        .select_tx_fields(TransactionFields {
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
