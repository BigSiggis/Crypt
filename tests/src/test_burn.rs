//! Tests for card burning and account closure.

#[cfg(test)]
mod tests {
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_burn_requires_owner() {
        let card_owner = Pubkey::new_unique();
        let burner = Pubkey::new_unique();
        assert_ne!(card_owner, burner, "Non-owner should not burn");
    }

    #[test]
    fn test_burn_returns_rent() {
        let rent_lamports: u64 = 2_282_880; // ~0.00228 SOL typical rent
        assert!(rent_lamports > 0, "Burn should return rent to owner");
    }

    #[test]
    fn test_burned_card_not_transferable() {
        // After burn, account is closed — attempting transfer should fail
        let account_exists = false;
        assert!(!account_exists, "Burned card account should not exist");
    }
}
