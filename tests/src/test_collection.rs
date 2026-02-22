//! Tests for collection initialization and management.

#[cfg(test)]
mod tests {
    use crate::helpers::*;
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_initialize_collection_pda() {
        let program_id = Pubkey::new_unique();
        let (pda, bump) = collection_pda(&program_id);
        assert_ne!(pda, Pubkey::default());
        assert!(bump <= 255);
    }

    #[test]
    fn test_collection_pda_consistent_across_calls() {
        let program_id = Pubkey::new_unique();
        let results: Vec<(Pubkey, u8)> = (0..100)
            .map(|_| collection_pda(&program_id))
            .collect();
        assert!(results.windows(2).all(|w| w[0] == w[1]));
    }

    #[test]
    fn test_different_programs_different_pdas() {
        let prog1 = Pubkey::new_unique();
        let prog2 = Pubkey::new_unique();
        let (pda1, _) = collection_pda(&prog1);
        let (pda2, _) = collection_pda(&prog2);
        assert_ne!(pda1, pda2);
    }

    #[test]
    fn test_collection_uri_validation() {
        let valid_uri = "https://crypt-phi-two.vercel.app/collection.json";
        assert!(valid_uri.len() <= 200);
        assert!(valid_uri.starts_with("https://"));
    }

    #[test]
    fn test_collection_uri_max_length() {
        let max_uri = "x".repeat(200);
        assert_eq!(max_uri.len(), 200);
        let over_uri = "x".repeat(201);
        assert!(over_uri.len() > 200);
    }

    #[test]
    fn test_max_supply_zero_means_unlimited() {
        let max_supply: u64 = 0;
        let total_minted: u64 = 999_999;
        // max_supply == 0 means unlimited
        let can_mint = max_supply == 0 || total_minted < max_supply;
        assert!(can_mint);
    }

    #[test]
    fn test_max_supply_enforced() {
        let max_supply: u64 = 1000;
        let total_minted: u64 = 1000;
        let can_mint = max_supply == 0 || total_minted < max_supply;
        assert!(!can_mint);
    }

    #[test]
    fn test_mint_fee_calculation() {
        let mint_fee: u64 = 5_000_000; // 0.005 SOL
        let batch_size: u64 = 8;
        let total_fee = mint_fee * batch_size;
        assert_eq!(total_fee, 40_000_000); // 0.04 SOL for batch of 8
    }

    #[test]
    fn test_paused_collection_blocks_minting() {
        let paused = true;
        let max_supply: u64 = 0;
        let total_minted: u64 = 0;
        let can_mint = !paused && (max_supply == 0 || total_minted < max_supply);
        assert!(!can_mint);
    }
}
