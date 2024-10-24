use serde::Serialize;
use std::collections::HashMap;

/// Represents the fields that can be selected for logs and transactions.
///
/// This struct allows for specifying which fields of logs and transactions should be included
/// in the response. The fields are represented as a map where the keys are field names and the
/// values are booleans indicating whether the field should be included.
#[derive(Clone, Debug, Serialize)]
pub struct FieldsOptions {
    /// Optional map specifying which fields of logs to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<HashMap<String, bool>>,
    /// Optional map specifying which fields of transactions to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<HashMap<String, bool>>,
}
