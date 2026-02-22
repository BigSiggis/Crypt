use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptSdkError {
    #[error("RPC error: {0}")]
    Rpc(String),

    #[error("Transaction failed: {0}")]
    Transaction(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Card not found: {0}")]
    CardNotFound(String),

    #[error("Collection not initialized")]
    CollectionNotInitialized,

    #[error("Insufficient balance: need {needed} lamports, have {have}")]
    InsufficientBalance { needed: u64, have: u64 },

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Verification failed: {reason}")]
    VerificationFailed { reason: String },

    #[error("API error: {0}")]
    ApiError(String),
}

impl From<solana_client::client_error::ClientError> for CryptSdkError {
    fn from(e: solana_client::client_error::ClientError) -> Self {
        CryptSdkError::Rpc(e.to_string())
    }
}
