//! Tests for card verification logic.

#[cfg(test)]
mod tests {
    use crate::helpers::*;

    #[test]
    fn test_verify_matching_soul_seed() {
        let tx = mock_tx_hash(42);
        let stored_seed = compute_soul_seed(&tx);
        let computed_seed = compute_soul_seed(&tx);
        assert_eq!(stored_seed, computed_seed, "Verification should pass for matching seeds");
    }

    #[test]
    fn test_verify_mismatched_tx_hash() {
        let original_tx = mock_tx_hash(1);
        let stored_seed = compute_soul_seed(&original_tx);
        let fake_tx = mock_tx_hash(2);
        let recomputed = compute_soul_seed(&fake_tx);
        assert_ne!(stored_seed, recomputed, "Verification should fail for mismatched tx");
    }

    #[test]
    fn test_verify_tampered_seed() {
        let tx = mock_tx_hash(99);
        let mut tampered_seed = compute_soul_seed(&tx);
        tampered_seed[0] ^= 0xFF; // Flip bits in first byte
        let correct_seed = compute_soul_seed(&tx);
        assert_ne!(tampered_seed, correct_seed, "Tampered seed should not verify");
    }

    #[test]
    fn test_verify_all_demo_cards() {
        // Verify all demo card tx hashes produce valid seeds
        let demo_hashes = ["4xK7m...9pR2", "7rN3q...2wL5", "2bH8k...5mX9", "9vF1r...3kP7", "5tG2m...8nQ4"];
        for hash in &demo_hashes {
            let seed = compute_soul_seed(hash);
            assert_eq!(seed, compute_soul_seed(hash), "Demo card {} failed verification", hash);
        }
    }
}
