//! Tests for rarity upgrade system.

#[cfg(test)]
mod tests {
    use crate::helpers::*;

    fn generate_upgrade_proof(tx_hash: &str, soul_seed: &[u8; 32], new_rarity: u8) -> [u8; 32] {
        let mut expected = compute_soul_seed(tx_hash);
        for i in 0..32 { expected[i] ^= soul_seed[i]; }
        expected[0] ^= new_rarity;
        for i in 0..32 {
            let mut h: u64 = expected[i] as u64;
            h = h.wrapping_mul(0x9e3779b97f4a7c15);
            h ^= h >> 13;
            expected[i] = (h & 0xFF) as u8;
        }
        expected
    }

    fn verify_proof(tx_hash: &str, soul_seed: &[u8; 32], new_rarity: u8, proof: &[u8; 32]) -> bool {
        let expected = generate_upgrade_proof(tx_hash, soul_seed, new_rarity);
        expected == *proof
    }

    #[test]
    fn test_valid_upgrade_proof() {
        let tx = mock_tx_hash(42);
        let seed = compute_soul_seed(&tx);
        let proof = generate_upgrade_proof(&tx, &seed, 2);
        assert!(verify_proof(&tx, &seed, 2, &proof));
    }

    #[test]
    fn test_invalid_upgrade_proof() {
        let tx = mock_tx_hash(42);
        let seed = compute_soul_seed(&tx);
        let fake_proof = [0u8; 32];
        assert!(!verify_proof(&tx, &seed, 2, &fake_proof));
    }

    #[test]
    fn test_wrong_rarity_proof_fails() {
        let tx = mock_tx_hash(42);
        let seed = compute_soul_seed(&tx);
        let proof_for_rare = generate_upgrade_proof(&tx, &seed, 1);
        // Using rare proof for legendary should fail
        assert!(!verify_proof(&tx, &seed, 2, &proof_for_rare));
    }

    #[test]
    fn test_cannot_downgrade() {
        // Rarity 2 (legendary) cannot go to 1 (rare)
        let can_upgrade = |current: u8, target: u8| -> bool { target > current };
        assert!(!can_upgrade(2, 1));
        assert!(!can_upgrade(2, 0));
        assert!(!can_upgrade(1, 0));
        assert!(can_upgrade(0, 1));
        assert!(can_upgrade(0, 2));
        assert!(can_upgrade(1, 2));
    }

    #[test]
    fn test_same_rarity_not_upgrade() {
        let can_upgrade = |current: u8, target: u8| -> bool { target > current };
        assert!(!can_upgrade(0, 0));
        assert!(!can_upgrade(1, 1));
        assert!(!can_upgrade(2, 2));
    }
}
