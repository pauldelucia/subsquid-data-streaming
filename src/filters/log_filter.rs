use serde::Serialize;

/// Represents a filter for Ethereum logs based on address and topics.
#[derive(Clone, Debug)]
pub struct LogFilter {
    /// A list of Ethereum addresses to filter logs by.
    pub address: Vec<String>,
    /// A list of topics to filter logs by.
    pub topic0: Vec<String>,
}

impl LogFilter {
    /// Creates a new `LogFilter` with empty address and topics.
    ///
    /// # Example
    ///
    /// ```
    /// use subsquid_data_streaming::LogFilter;
    ///
    /// let filter = LogFilter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            address: Vec::new(),
            topic0: Vec::new(),
        }
    }

    /// Adds an address to the filter.
    ///
    /// Converts the provided address to lowercase before adding it.
    ///
    /// # Parameters
    ///
    /// * `address` - A reference to the address to be added to the filter.
    ///
    /// # Example
    ///
    /// ```
    /// use subsquid_data_streaming::LogFilter;
    ///
    /// let filter = LogFilter::new().with_address("0xabcd");
    /// ```
    pub fn with_address(mut self, address: &str) -> Self {
        self.address.push(address.to_lowercase());
        self
    }

    /// Adds a topic to the filter's `topic` field.
    ///
    /// Converts the provided topic to lowercase before adding it.
    ///
    /// # Parameters
    ///
    /// * `topic` - A reference to the topic to be added to the filter.
    ///
    /// # Example
    ///
    /// ```
    /// use subsquid_data_streaming::LogFilter;
    ///
    /// let filter = LogFilter::new().with_topic("Transfer(address,address,uint256)");
    /// ```
    pub fn with_topic(mut self, topic: &str) -> Self {
        self.topic0.push(topic.to_lowercase());
        self
    }
}

/// Represents a serialized filter for log data used in a request to the data lake.
///
/// This struct is used to serialize filter options for logs and topics.
#[derive(Clone, Debug, Serialize)]
pub struct LogsFilter {
    /// An optional list of Ethereum addresses to filter logs by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    /// An optional list of topic values to filter logs by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic0: Option<Vec<String>>,
    /// Specifies whether the transaction data should be included in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<bool>,
}

impl LogsFilter {
    /// Creates a `LogsFilter` from a `LogFilter`.
    ///
    /// This method converts the provided `LogFilter` into a `LogsFilter` and automatically
    /// sets the `transaction` field to `Some(true)`.
    ///
    /// # Parameters
    ///
    /// * `log_filter` - A reference to a `LogFilter` instance to convert.
    ///
    /// # Example
    ///
    /// ```
    /// use subsquid_data_streaming::LogFilter;
    /// use subsquid_data_streaming::filters::log_filter::LogsFilter;
    ///
    /// let log_filter = LogFilter::new().with_address("0xabcd").with_topic("Transfer(address,address,uint256)");
    /// let logs_filter = LogsFilter::from(&log_filter);
    /// ```
    pub fn from(log_filter: &LogFilter) -> Self {
        Self {
            address: if !log_filter.address.is_empty() {
                Some(log_filter.address.clone())
            } else {
                None
            },
            topic0: if !log_filter.topic0.is_empty() {
                Some(log_filter.topic0.clone())
            } else {
                None
            },
            // Transaction inclusion is hardcoded to true for this example.
            transaction: Some(true),
        }
    }
}
