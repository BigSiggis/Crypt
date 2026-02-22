//! Test helpers and utilities for Crypt integration tests.

use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
};

/// Generate a deterministic test keypair from a seed.
pub fn test_keypair(seed: u8) -> Keypair {
    let mut bytes = [0u8; 64];
    bytes[0] = seed;
    // Fill with deterministic pattern
    for i in 1..32 {
        bytes[i] = bytes[i-1].wrapping_mul(7).wrapping_add(seed);
    }
    // Copy public key portion
    for i in 32..64 {
        bytes[i] = bytes[i-32].wrapping_add(1);
    }
    Keypair::from_bytes(&bytes).unwrap_or_else(|_| Keypair::new())
}

/// Derive the collection PDA.
pub fn collection_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"collection"], program_id)
}

/// Derive a card PDA from tx_hash and minter.
pub fn card_pda(program_id: &Pubkey, tx_hash: &str, minter: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"card", tx_hash.as_bytes(), minter.as_ref()],
        program_id,
    )
}

/// Derive an interaction PDA.
pub fn interaction_pda(program_id: &Pubkey, card: &Pubkey, user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"interaction", card.as_ref(), user.as_ref()],
        program_id,
    )
}

/// Generate a mock transaction hash for testing.
pub fn mock_tx_hash(id: u32) -> String {
    format!("{}MockTxHash{:06}abcdef1234567890abcdef1234567890abcdef1234567890{}", 
        id, id, id % 10)
}

/// Generate a mock narration hash.
pub fn mock_narration_hash(text: &str) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

/// Compute soul seed (mirrors on-chain logic).
pub fn compute_soul_seed(tx_hash: &str) -> [u8; 32] {
    let bytes = tx_hash.as_bytes();
    let mut seed = [0u8; 32];

    for (i, &b) in bytes.iter().enumerate() {
        seed[i % 32] ^= b;
    }

    for i in 0..32 {
        let mut h: u64 = seed[i] as u64;
        h = h.wrapping_mul(0x517cc1b727220a95);
        h ^= h >> 17;
        h = h.wrapping_mul(0x6c62272e07bb0142);
        h ^= h >> 11;
        seed[i] = (h & 0xFF) as u8;
    }

    for i in 1..32 {
        seed[i] ^= seed[i - 1].wrapping_add(37);
    }
    for i in (0..31).rev() {
        seed[i] ^= seed[i + 1].wrapping_add(53);
    }

    seed
}

/// Format SOL amount for display.
pub fn format_sol(lamports: u64) -> String {
    let sol = lamports as f64 / 1e9;
    if sol >= 1000.0 { format!("{:.0}K SOL", sol / 1000.0) }
    else if sol >= 1.0 { format!("{:.2} SOL", sol) }
    else { format!("{:.4} SOL", sol) }
}

/// Assert that two byte arrays differ in at least `min_diff` positions.
pub fn assert_avalanche(a: &[u8; 32], b: &[u8; 32], min_diff: usize) {
    let diff = a.iter().zip(b.iter()).filter(|(x, y)| x != y).count();
    assert!(
        diff >= min_diff,
        "Expected at least {} byte differences, got {}",
        min_diff, diff
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_pda_deterministic() {
        let program = Pubkey::new_unique();
        let (pda1, bump1) = collection_pda(&program);
        let (pda2, bump2) = collection_pda(&program);
        assert_eq!(pda1, pda2);
        assert_eq!(bump1, bump2);
    }

    #[test]
    fn test_card_pda_unique_per_tx() {
        let program = Pubkey::new_unique();
        let minter = Pubkey::new_unique();
        let (pda1, _) = card_pda(&program, "tx_hash_1", &minter);
        let (pda2, _) = card_pda(&program, "tx_hash_2", &minter);
        assert_ne!(pda1, pda2);
    }

    #[test]
    fn test_card_pda_unique_per_minter() {
        let program = Pubkey::new_unique();
        let minter1 = Pubkey::new_unique();
        let minter2 = Pubkey::new_unique();
        let (pda1, _) = card_pda(&program, "same_tx", &minter1);
        let (pda2, _) = card_pda(&program, "same_tx", &minter2);
        assert_ne!(pda1, pda2);
    }

    #[test]
    fn test_mock_tx_hash_unique() {
        let h1 = mock_tx_hash(1);
        let h2 = mock_tx_hash(2);
        assert_ne!(h1, h2);
        assert!(h1.len() > 32);
    }

    #[test]
    fn test_soul_seed_deterministic() {
        let s1 = compute_soul_seed("test_hash");
        let s2 = compute_soul_seed("test_hash");
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_soul_seed_avalanche() {
        let s1 = compute_soul_seed("hash_A");
        let s2 = compute_soul_seed("hash_B");
        assert_avalanche(&s1, &s2, 16);
    }

    #[test]
    fn test_format_sol() {
        assert_eq!(format_sol(1_000_000_000), "1.00 SOL");
        assert_eq!(format_sol(100_000_000), "0.1000 SOL");
        assert_eq!(format_sol(1_500_000_000_000), "1500K SOL");
    }
}
