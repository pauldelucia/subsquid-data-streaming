use serde::Serialize;

/// Represents options for selecting transaction fields.
///
/// This struct contains a map that defines which transaction fields should be included in the response.
/// The map's keys are the field names, and the values are booleans indicating whether the field should
/// be included or excluded.
#[derive(Clone, Debug, Default, Serialize)]
pub struct TransactionFields {
    pub hash: bool,
    pub nonce: bool,
    pub transaction_index: bool,
    pub to: bool,
    pub from: bool,
    pub value: bool,
    pub gas: bool,
    pub gas_price: bool,
    pub input: bool,
}
