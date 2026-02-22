//! Soul Signature — deterministic art seed generation.
//!
//! Each Solana transaction hash produces a unique 32-byte seed
//! that drives the generative pixel art engine. Same hash always
//! produces the same skull with the same accessories.

use sha2::{Sha256, Digest};

/// Soul Signature generator.
pub struct SoulSignature;

impl SoulSignature {
    /// Compute a deterministic 32-byte seed from a transaction hash.
    /// This is the core algorithm that ensures 1:1 unique art per card.
    pub fn compute(tx_hash: &str) -> [u8; 32] {
        let bytes = tx_hash.as_bytes();
        let mut seed = [0u8; 32];

        // Phase 1: XOR-fold
        for (i, &b) in bytes.iter().enumerate() {
            seed[i % 32] ^= b;
        }

        // Phase 2: Bit mixing (fast avalanche)
        for i in 0..32 {
            let mut h: u64 = seed[i] as u64;
            h = h.wrapping_mul(0x517cc1b727220a95);
            h ^= h >> 17;
            h = h.wrapping_mul(0x6c62272e07bb0142);
            h ^= h >> 11;
            seed[i] = (h & 0xFF) as u8;
        }

        // Phase 3: Bidirectional diffusion
        for i in 1..32 {
            seed[i] ^= seed[i - 1].wrapping_add(37);
        }
        for i in (0..31).rev() {
            seed[i] ^= seed[i + 1].wrapping_add(53);
        }

        seed
    }

    /// Compute a SHA-256 narration hash.
    pub fn hash_narration(text: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }

    /// Extract visual trait parameters from a soul seed.
    pub fn extract_traits(seed: &[u8; 32]) -> SoulTraits {
        SoulTraits {
            eye_style: seed[0] % 8,
            glow_eyes: seed[1] > 102,
            hat_type: seed[2] % 30,
            glasses_type: seed[3] % 12,
            mouth_item: seed[4] % 8,
            neck_item: seed[5] % 6,
            teeth_style: seed[6] % 6,
            nose_style: seed[7] % 4,
            has_scar: seed[8] > 153,
            has_crack: seed[9] > 153,
            has_eyepatch: seed[10] > 225,
        }
    }

    /// Verify that a stored seed matches a transaction hash.
    pub fn verify(tx_hash: &str, stored_seed: &[u8; 32]) -> bool {
        let computed = Self::compute(tx_hash);
        computed == *stored_seed
    }

    /// Generate an upgrade proof for rarity changes.
    pub fn upgrade_proof(tx_hash: &str, soul_seed: &[u8; 32], new_rarity: u8) -> [u8; 32] {
        let mut proof = Self::compute(tx_hash);
        for i in 0..32 { proof[i] ^= soul_seed[i]; }
        proof[0] ^= new_rarity;
        for i in 0..32 {
            let mut h: u64 = proof[i] as u64;
            h = h.wrapping_mul(0x9e3779b97f4a7c15);
            h ^= h >> 13;
            proof[i] = (h & 0xFF) as u8;
        }
        proof
    }
}

/// Visual traits extracted from a soul seed.
#[derive(Debug, Clone)]
pub struct SoulTraits {
    pub eye_style: u8,
    pub glow_eyes: bool,
    pub hat_type: u8,
    pub glasses_type: u8,
    pub mouth_item: u8,
    pub neck_item: u8,
    pub teeth_style: u8,
    pub nose_style: u8,
    pub has_scar: bool,
    pub has_crack: bool,
    pub has_eyepatch: bool,
}

impl SoulTraits {
    /// Get human-readable hat name.
    pub fn hat_name(&self) -> &'static str {
        const HATS: [&str; 30] = [
            "Cowboy", "Top Hat", "Beanie", "Baseball Cap", "Crown",
            "Pirate Hat", "Sailor Hat", "Trucker Cap", "Fedora", "Wizard Hat",
            "Headband", "Mohawk", "Viking Helmet", "Chef Hat", "Bandana",
            "Halo", "Bucket Hat", "Santa Hat", "Afro", "Devil Horns",
            "Army Helmet", "Sombrero", "Backwards Cap", "Durag", "Bowler Hat",
            "Straw Hat", "Space Helmet", "Fire Crown", "Propeller Hat", "Toque",
        ];
        HATS.get(self.hat_type as usize).unwrap_or(&"None")
    }

    /// Get human-readable glasses name.
    pub fn glasses_name(&self) -> &'static str {
        const GLASSES: [&str; 12] = [
            "Pit Vipers", "Aviators", "3D Glasses", "Heart Glasses",
            "Nerd Glasses", "Monocle", "Cyclops Visor", "Thug Life",
            "Star Glasses", "VR Headset", "Laser Eyes", "Lennon Rounds",
        ];
        GLASSES.get(self.glasses_type as usize).unwrap_or(&"None")
    }

    /// Get total accessory count for rarity weighting.
    pub fn accessory_count(&self) -> u8 {
        let mut count = 0u8;
        if self.hat_type > 0 { count += 1; }
        if self.glasses_type > 0 { count += 1; }
        if self.mouth_item > 0 { count += 1; }
        if self.neck_item > 0 { count += 1; }
        if self.has_scar { count += 1; }
        if self.has_crack { count += 1; }
        if self.has_eyepatch { count += 1; }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_deterministic() {
        let s1 = SoulSignature::compute("test");
        let s2 = SoulSignature::compute("test");
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_verify_valid() {
        let tx = "4xK7m9pR2abc";
        let seed = SoulSignature::compute(tx);
        assert!(SoulSignature::verify(tx, &seed));
    }

    #[test]
    fn test_verify_invalid() {
        let seed = SoulSignature::compute("real_tx");
        assert!(!SoulSignature::verify("fake_tx", &seed));
    }

    #[test]
    fn test_extract_traits() {
        let seed = SoulSignature::compute("test_hash");
        let traits = SoulSignature::extract_traits(&seed);
        assert!(traits.eye_style < 8);
        assert!(traits.hat_type < 30);
        assert!(!traits.hat_name().is_empty());
    }

    #[test]
    fn test_narration_hash() {
        let h1 = SoulSignature::hash_narration("test");
        let h2 = SoulSignature::hash_narration("test");
        assert_eq!(h1, h2);
        let h3 = SoulSignature::hash_narration("different");
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_accessory_count() {
        let seed = SoulSignature::compute("accessory_test");
        let traits = SoulSignature::extract_traits(&seed);
        assert!(traits.accessory_count() <= 7);
    }
}
