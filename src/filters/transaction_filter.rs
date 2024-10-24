use serde::Serialize;

/// Represents a filter for Ethereum transactions based on `from` and `to` addresses.
#[derive(Clone, Serialize)]
pub struct TransactionFilter {
    /// A list of Ethereum addresses that the transaction originated from.
    pub from: Option<Vec<String>>,
    /// A list of Ethereum addresses that the transaction is sent to.
    pub to: Option<Vec<String>>,
}

impl TransactionFilter {
    /// Creates a new `TransactionFilter` with no specified `from` or `to` addresses.
    ///
    /// # Example
    ///
    /// ```
    /// use subsquid_data_streaming::TransactionFilter;
    ///
    /// let filter = TransactionFilter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            from: None,
            to: None,
        }
    }

    /// Adds an address to the `from` field of the transaction filter.
    ///
    /// Converts the provided address to lowercase before adding it.
    /// If the `from` field is `None`, it initializes it as an empty `Vec`.
    ///
    /// # Parameters
    ///
    /// * `address` - A reference to the address to be added to the `from` field.
    ///
    /// # Example
    ///
    /// ```
    /// use subsquid_data_streaming::TransactionFilter;
    ///
    /// let filter = TransactionFilter::new().with_from("0xabcd");
    /// ```
    pub fn with_from(mut self, address: &str) -> Self {
        self.from
            .get_or_insert(Vec::new())
            .push(address.to_lowercase());
        self
    }

    /// Adds an address to the `to` field of the transaction filter.
    ///
    /// Converts the provided address to lowercase before adding it.
    /// If the `to` field is `None`, it initializes it as an empty `Vec`.
    ///
    /// # Parameters
    ///
    /// * `address` - A reference to the address to be added to the `to` field.
    ///
    /// # Example
    ///
    /// ```
    /// use subsquid_data_streaming::TransactionFilter;
    ///
    /// let filter = TransactionFilter::new().with_to("0xefgh");
    /// ```
    pub fn with_to(mut self, address: &str) -> Self {
        self.to
            .get_or_insert(Vec::new())
            .push(address.to_lowercase());
        self
    }
}

/// Represents a serialized filter for transactions used in requests to the data lake.
///
/// This struct is used to serialize filter options for transactions, with `from` and `to` addresses.
#[derive(Serialize)]
pub struct TransactionsFilter {
    /// An optional list of Ethereum addresses that the transaction originated from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<String>>,
    /// An optional list of Ethereum addresses that the transaction is sent to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
}

impl TransactionsFilter {
    /// Creates a `TransactionsFilter` from a `TransactionFilter`.
    ///
    /// This method converts the provided `TransactionFilter` into a `TransactionsFilter`.
    ///
    /// # Parameters
    ///
    /// * `filter` - A reference to a `TransactionFilter` instance to convert.
    ///
    /// # Example
    ///
    /// ```
    /// use subsquid_data_streaming::TransactionFilter;
    /// use subsquid_data_streaming::filters::transaction_filter::TransactionsFilter;
    ///
    /// let tx_filter = TransactionFilter::new().with_from("0xabcd");
    /// let transactions_filter = TransactionsFilter::from(&tx_filter);
    /// ```
    pub fn from(filter: &TransactionFilter) -> Self {
        Self {
            from: filter.from.clone(),
            to: filter.to.clone(),
        }
    }
}
