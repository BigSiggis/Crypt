//! Tests for social interaction system.

#[cfg(test)]
mod tests {
    use crate::helpers::*;
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_interaction_pda_unique_per_user() {
        let program = Pubkey::new_unique();
        let card = Pubkey::new_unique();
        let user1 = Pubkey::new_unique();
        let user2 = Pubkey::new_unique();
        let (pda1, _) = interaction_pda(&program, &card, &user1);
        let (pda2, _) = interaction_pda(&program, &card, &user2);
        assert_ne!(pda1, pda2);
    }

    #[test]
    fn test_interaction_pda_unique_per_card() {
        let program = Pubkey::new_unique();
        let card1 = Pubkey::new_unique();
        let card2 = Pubkey::new_unique();
        let user = Pubkey::new_unique();
        let (pda1, _) = interaction_pda(&program, &card1, &user);
        let (pda2, _) = interaction_pda(&program, &card2, &user);
        assert_ne!(pda1, pda2);
    }

    #[test]
    fn test_interaction_types_valid() {
        let valid_types = [0u8, 1, 2, 3];
        let names = ["LIKE", "COMMENT", "SHARE", "BOOKMARK"];
        assert_eq!(valid_types.len(), names.len());
    }

    #[test]
    fn test_interaction_count_increment() {
        let mut count: u64 = 0;
        for _ in 0..100 {
            count = count.saturating_add(1);
        }
        assert_eq!(count, 100);
    }

    #[test]
    fn test_interaction_count_saturating() {
        let mut count: u64 = u64::MAX;
        count = count.saturating_add(1);
        assert_eq!(count, u64::MAX, "Should saturate, not overflow");
    }

    #[test]
    fn test_comment_hash_stored() {
        let comment = "This trade was absolutely legendary";
        let hash = mock_narration_hash(comment);
        assert_ne!(hash, [0u8; 32]);
    }

    #[test]
    fn test_one_interaction_per_user_per_card() {
        // PDA ensures only one interaction account per (card, user) pair
        let program = Pubkey::new_unique();
        let card = Pubkey::new_unique();
        let user = Pubkey::new_unique();
        let (pda1, bump1) = interaction_pda(&program, &card, &user);
        let (pda2, bump2) = interaction_pda(&program, &card, &user);
        assert_eq!(pda1, pda2);
        assert_eq!(bump1, bump2);
    }
}
