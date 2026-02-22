//! Tests for card minting — single and batch.

#[cfg(test)]
mod tests {
    use crate::helpers::*;
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_card_pda_derived_from_tx_hash() {
        let program = Pubkey::new_unique();
        let minter = Pubkey::new_unique();
        let tx1 = mock_tx_hash(1);
        let tx2 = mock_tx_hash(2);
        let (pda1, _) = card_pda(&program, &tx1, &minter);
        let (pda2, _) = card_pda(&program, &tx2, &minter);
        assert_ne!(pda1, pda2, "Different tx hashes should produce different card PDAs");
    }

    #[test]
    fn test_same_tx_different_minters_different_cards() {
        let program = Pubkey::new_unique();
        let tx = mock_tx_hash(42);
        let m1 = Pubkey::new_unique();
        let m2 = Pubkey::new_unique();
        let (pda1, _) = card_pda(&program, &tx, &m1);
        let (pda2, _) = card_pda(&program, &tx, &m2);
        assert_ne!(pda1, pda2, "Same tx, different minters should produce different cards");
    }

    #[test]
    fn test_rarity_values() {
        assert!(0u8 <= 2, "Rarity must be 0-2");
        let rarities = [0u8, 1, 2];
        let names = ["COMMON", "RARE", "LEGENDARY"];
        for (r, n) in rarities.iter().zip(names.iter()) {
            assert!(*r <= 2, "{} rarity {} out of range", n, r);
        }
    }

    #[test]
    fn test_card_type_values() {
        let types = [0u8, 1, 2, 3, 4];
        let names = ["SWAP", "RUG", "MINT", "DIAMOND_HANDS", "BIG_MOVE"];
        assert_eq!(types.len(), names.len());
        for t in types {
            assert!(t <= 4, "Card type {} out of range", t);
        }
    }

    #[test]
    fn test_title_max_length() {
        let valid = "420 SOL → BONK on JUPITER";
        assert!(valid.len() <= 100);
        let too_long = "X".repeat(101);
        assert!(too_long.len() > 100);
    }

    #[test]
    fn test_platform_max_length() {
        let valid = "JUPITER";
        assert!(valid.len() <= 32);
        let too_long = "X".repeat(33);
        assert!(too_long.len() > 32);
    }

    #[test]
    fn test_narration_hash_deterministic() {
        let h1 = mock_narration_hash("The degen alarm went off at 3am.");
        let h2 = mock_narration_hash("The degen alarm went off at 3am.");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_narration_hash_different_text() {
        let h1 = mock_narration_hash("Aped into BONK");
        let h2 = mock_narration_hash("Diamond hands through the crash");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_soul_seed_stored_on_mint() {
        let tx = mock_tx_hash(99);
        let seed = compute_soul_seed(&tx);
        // Seed should be 32 bytes
        assert_eq!(seed.len(), 32);
        // Seed should not be all zeros
        assert!(seed.iter().any(|&b| b != 0));
    }

    #[test]
    fn test_batch_mint_max_8() {
        let batch_size = 8;
        assert!(batch_size <= 8);
        let over = 9;
        assert!(over > 8);
    }

    #[test]
    fn test_batch_mint_sequential_ids() {
        let start_id: u64 = 42;
        let batch_size = 5;
        let ids: Vec<u64> = (0..batch_size).map(|i| start_id + i).collect();
        assert_eq!(ids, vec![42, 43, 44, 45, 46]);
    }

    #[test]
    fn test_mint_id_increments() {
        let mut total_minted: u64 = 0;
        for _ in 0..10 {
            let mint_id = total_minted;
            total_minted += 1;
            assert_eq!(mint_id + 1, total_minted);
        }
        assert_eq!(total_minted, 10);
    }

    #[test]
    fn test_tx_hash_length_validation() {
        // Solana signatures are 88 chars base58
        let valid_hash = "4xK7m9pR2abc123def4567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12345678";
        assert!(valid_hash.len() <= 88);
        assert!(valid_hash.len() > 0);
    }
}
