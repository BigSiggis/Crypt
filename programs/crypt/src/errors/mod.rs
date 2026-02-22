use anchor_lang::prelude::*;

#[error_code]
pub enum CryptError {
    #[msg("Unauthorized: caller is not the collection authority")]
    Unauthorized,

    #[msg("Invalid rarity value — must be 0 (common), 1 (rare), or 2 (legendary)")]
    InvalidRarity,

    #[msg("Invalid card type — must be 0-4 (swap, rug, mint, diamond_hands, big_move)")]
    InvalidCardType,

    #[msg("Card not owned by signer")]
    NotCardOwner,

    #[msg("Transaction hash already minted by this wallet")]
    AlreadyMinted,

    #[msg("Batch size exceeds maximum of 8 cards")]
    BatchTooLarge,

    #[msg("Soul signature verification failed")]
    VerificationFailed,

    #[msg("Invalid rarity upgrade proof")]
    InvalidUpgradeProof,

    #[msg("Card rarity cannot be downgraded")]
    CannotDowngrade,

    #[msg("Interaction type not recognized")]
    InvalidInteractionType,

    #[msg("Title exceeds maximum length of 100 characters")]
    TitleTooLong,

    #[msg("Transaction hash exceeds maximum length of 88 characters")]
    TxHashTooLong,

    #[msg("Platform string exceeds maximum length of 32 characters")]
    PlatformTooLong,

    #[msg("Collection URI exceeds maximum length of 200 characters")]
    UriTooLong,

    #[msg("Collection has reached maximum supply")]
    MaxSupplyReached,

    #[msg("Insufficient funds for minting fee")]
    InsufficientFunds,
}
