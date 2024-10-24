/// Where data should be fetched from
///
/// The Subsquid data lake currently has an offset of about 1000-2000 blocks from the Ethereum chain tip.
/// The EVM RPC endpoint can be used to get the "hot blocks" not yet present in the data lake (unimplemented).
pub enum DataSource {
    Subsquid(String),
    EvmRpc(String),
}
