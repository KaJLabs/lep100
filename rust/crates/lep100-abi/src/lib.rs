//! lep100-abi: placeholder ABI helpers.

#[derive(Debug, thiserror::Error)]
pub enum AbiError {
    #[error("invalid input")]
    InvalidInput,
}
