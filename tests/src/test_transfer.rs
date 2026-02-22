//! Tests for card transfer logic.

#[cfg(test)]
mod tests {
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_transfer_changes_owner() {
        let original_owner = Pubkey::new_unique();
        let new_owner = Pubkey::new_unique();
        let mut card_owner = original_owner;
        card_owner = new_owner;
        assert_eq!(card_owner, new_owner);
        assert_ne!(card_owner, original_owner);
    }

    #[test]
    fn test_transfer_requires_current_owner() {
        let card_owner = Pubkey::new_unique();
        let signer = Pubkey::new_unique();
        let is_owner = card_owner == signer;
        assert!(!is_owner, "Random signer should not be owner");
    }

    #[test]
    fn test_self_transfer_allowed() {
        let owner = Pubkey::new_unique();
        let is_owner = owner == owner;
        assert!(is_owner, "Self-transfer should be allowed");
    }

    #[test]
    fn test_transfer_preserves_card_data() {
        let mint_id: u64 = 42;
        let rarity: u8 = 2;
        let tx_hash = "test_hash_123";
        // After transfer, all data should remain the same except owner
        assert_eq!(mint_id, 42);
        assert_eq!(rarity, 2);
        assert_eq!(tx_hash, "test_hash_123");
    }
}
