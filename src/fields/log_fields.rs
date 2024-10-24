use serde::Serialize;

/// Represents options for selecting log fields.
///
/// This struct contains a map that defines which log fields should be included in the response.
/// The map's keys are the field names, and the values are booleans indicating whether the field
/// should be included or excluded.
#[derive(Clone, Debug, Default, Serialize)]
pub struct LogFields {
    pub topic0: bool,
    pub data: bool,
    pub transaction_index: bool,
    pub log_index: bool,
    pub address: bool,
    pub block_number: bool,
    pub block_hash: bool,
    pub transaction_hash: bool,
    pub removed: bool,
}
