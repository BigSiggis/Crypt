use anchor_lang::prelude::*;
use crate::state::{Rarity, CardType};
use crate::errors::CryptError;
use crate::instructions::MintCardArgs;

/// Validate all fields of a MintCardArgs struct before processing.
pub fn validate_card_args(args: &MintCardArgs) -> Result<()> {
    // Validate tx_hash length
    require!(
        args.tx_hash.len() > 0 && args.tx_hash.len() <= 88,
        CryptError::TxHashTooLong
    );

    // Validate rarity
    require!(
        Rarity::from_u8(args.rarity).is_some(),
        CryptError::InvalidRarity
    );

    // Validate card type
    require!(
        CardType::from_u8(args.card_type).is_some(),
        CryptError::InvalidCardType
    );

    // Validate title length
    require!(
        args.title.len() <= 100,
        CryptError::TitleTooLong
    );

    // Validate platform length
    require!(
        args.platform.len() <= 32,
        CryptError::PlatformTooLong
    );

    Ok(())
}

/// Validate a Solana transaction signature format.
/// Base58-encoded, typically 87-88 characters.
pub fn is_valid_tx_signature(sig: &str) -> bool {
    if sig.len() < 32 || sig.len() > 88 {
        return false;
    }
    sig.chars().all(|c| {
        c.is_ascii_alphanumeric() && c != '0' && c != 'O' && c != 'I' && c != 'l'
    })
}

/// Validate a Solana wallet address format.
/// Base58-encoded, 32-44 characters.
pub fn is_valid_wallet_address(addr: &str) -> bool {
    addr.len() >= 32 && addr.len() <= 44 && addr.chars().all(|c| c.is_ascii_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tx_signature() {
        assert!(is_valid_tx_signature("4xK7m9pR2abc123def4567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12345"));
    }

    #[test]
    fn test_invalid_tx_signature_too_short() {
        assert!(!is_valid_tx_signature("abc"));
    }

    #[test]
    fn test_valid_wallet() {
        assert!(is_valid_wallet_address("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"));
    }

    #[test]
    fn test_invalid_wallet_too_short() {
        assert!(!is_valid_wallet_address("abc"));
    }
}
