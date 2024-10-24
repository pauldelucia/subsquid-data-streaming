use subsquid_data_streaming::{DataSource, DataStream, DataStreamError};

#[tokio::test]
async fn test_error_handling() {
    // Try to build the data stream with an invalid URL and ensure it fails
    let data_stream_result = DataStream::new()
        .set_data_source(DataSource::Subsquid("https://invalid.url".to_string()))
        .build()
        .await;

    assert!(
        data_stream_result.is_err(),
        "Building DataStream with invalid URL should fail"
    );

    // Ensure the error is a network error
    if let Err(e) = data_stream_result {
        match e {
            DataStreamError::NetworkError(_) => {
                // Expected error type
            }
            _ => panic!("Unexpected error type: {:?}", e),
        }
    }
}
