use serde::{Deserialize, Deserializer};

/// Represents a transaction entry from a block.
///
/// This struct is used to deserialize transaction data, which contains information about
/// a transaction such as its hash, nonce, and other relevant fields.
#[derive(Debug, Deserialize, Clone)]
pub struct TransactionEntry {
    #[serde(default)]
    pub hash: Option<String>,
    #[serde(default)]
    pub nonce: Option<u64>,
    #[serde(default, rename = "transactionIndex")]
    pub transaction_index: Option<u64>,
    #[serde(default)]
    pub to: Option<String>,
    // #[serde(default, rename = "blockHash")]
    // pub block_hash: Option<String>,
    // #[serde(default, rename = "blockNumber")]
    // pub block_number: Option<u64>,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default, deserialize_with = "deserialize_hex_to_u64", rename = "value")]
    pub value: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_hex_to_u64", rename = "gas")]
    pub gas: Option<u64>,
    #[serde(
        default,
        rename = "gasPrice",
        deserialize_with = "deserialize_hex_to_u64"
    )]
    pub gas_price: Option<u64>,
    #[serde(default)]
    pub input: Option<String>,
}

/// Deserializes a hexadecimal string into a `u64` value.
///
/// This function is used to handle fields that represent numeric values in hex format in JSON.
///
/// # Parameters
///
/// * `deserializer` - A deserializer instance used to convert JSON data into a `u64` value.
///
/// # Returns
///
/// * `Result<Option<u64>, D::Error>` - The deserialized `u64` value.
fn deserialize_hex_to_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.starts_with("0x") {
        u64::from_str_radix(s.trim_start_matches("0x"), 16)
            .map(Some)
            .map_err(serde::de::Error::custom)
    } else {
        Ok(None)
    }
}
