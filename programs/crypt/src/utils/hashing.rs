use anchor_lang::prelude::*;

/// Compute a deterministic 32-byte soul seed from a transaction hash.
/// This seed drives the generative art engine — same tx_hash always
/// produces the same Soul Signature art.
///
/// Uses a simple but effective hash cascade:
///   1. XOR-fold the tx_hash bytes into 32 positions
///   2. Apply bit mixing (multiply, shift, XOR) for avalanche
///   3. Result is a uniformly distributed 32-byte seed
///
/// This is NOT cryptographic — it's a fast, deterministic seed generator
/// optimized for generating visual parameters (colors, shapes, positions).
pub fn compute_soul_seed(tx_hash: &str) -> [u8; 32] {
    let bytes = tx_hash.as_bytes();
    let mut seed = [0u8; 32];

    // Phase 1: XOR-fold input bytes across 32 positions
    for (i, &b) in bytes.iter().enumerate() {
        seed[i % 32] ^= b;
    }

    // Phase 2: Bit mixing — ensure small changes cascade
    for i in 0..32 {
        let mut h: u64 = seed[i] as u64;
        h = h.wrapping_mul(0x517cc1b727220a95);
        h ^= h >> 17;
        h = h.wrapping_mul(0x6c62272e07bb0142);
        h ^= h >> 11;
        seed[i] = (h & 0xFF) as u8;
    }

    // Phase 3: Chain adjacent bytes for additional diffusion
    for i in 1..32 {
        seed[i] ^= seed[i - 1].wrapping_add(37);
    }
    // Reverse pass
    for i in (0..31).rev() {
        seed[i] ^= seed[i + 1].wrapping_add(53);
    }

    seed
}

/// Verify an upgrade proof against card data.
/// In production, this would validate a Merkle proof from an oracle.
/// For the hackathon, we use a simplified HMAC-like verification.
pub fn verify_upgrade_proof(
    tx_hash: &str,
    soul_seed: &[u8; 32],
    new_rarity: u8,
    proof: &[u8; 32],
) -> bool {
    let mut expected = compute_soul_seed(tx_hash);

    // Mix in soul seed and target rarity
    for i in 0..32 {
        expected[i] ^= soul_seed[i];
    }
    expected[0] ^= new_rarity;

    // Apply mixing
    for i in 0..32 {
        let mut h: u64 = expected[i] as u64;
        h = h.wrapping_mul(0x9e3779b97f4a7c15);
        h ^= h >> 13;
        expected[i] = (h & 0xFF) as u8;
    }

    expected == *proof
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soul_seed_deterministic() {
        let tx = "4xK7m9pR2abc123def456";
        let seed1 = compute_soul_seed(tx);
        let seed2 = compute_soul_seed(tx);
        assert_eq!(seed1, seed2, "Same tx should produce same seed");
    }

    #[test]
    fn test_soul_seed_different_inputs() {
        let seed1 = compute_soul_seed("tx_hash_A");
        let seed2 = compute_soul_seed("tx_hash_B");
        assert_ne!(seed1, seed2, "Different txs should produce different seeds");
    }

    #[test]
    fn test_soul_seed_avalanche() {
        // Changing one character should change many bytes
        let seed1 = compute_soul_seed("abcdef123456");
        let seed2 = compute_soul_seed("abcdef123457");
        let diff_count = seed1.iter()
            .zip(seed2.iter())
            .filter(|(a, b)| a != b)
            .count();
        assert!(diff_count > 16, "One char change should affect >50% of bytes, got {}/32", diff_count);
    }

    #[test]
    fn test_upgrade_proof_valid() {
        let tx = "test_tx_hash";
        let seed = compute_soul_seed(tx);
        // Generate a valid proof
        let mut expected = compute_soul_seed(tx);
        for i in 0..32 { expected[i] ^= seed[i]; }
        expected[0] ^= 2; // legendary
        for i in 0..32 {
            let mut h: u64 = expected[i] as u64;
            h = h.wrapping_mul(0x9e3779b97f4a7c15);
            h ^= h >> 13;
            expected[i] = (h & 0xFF) as u8;
        }
        assert!(verify_upgrade_proof(tx, &seed, 2, &expected));
    }

    #[test]
    fn test_upgrade_proof_invalid() {
        let tx = "test_tx_hash";
        let seed = compute_soul_seed(tx);
        let fake_proof = [0u8; 32];
        assert!(!verify_upgrade_proof(tx, &seed, 2, &fake_proof));
    }
}
