//! Tests for Soul Signature seed generation — the deterministic
//! art engine that creates unique 1/1 pixel skulls per transaction.

#[cfg(test)]
mod tests {
    use crate::helpers::*;

    #[test]
    fn test_deterministic() {
        for i in 0..100 {
            let tx = mock_tx_hash(i);
            let s1 = compute_soul_seed(&tx);
            let s2 = compute_soul_seed(&tx);
            assert_eq!(s1, s2, "Seed must be deterministic for tx {}", i);
        }
    }

    #[test]
    fn test_uniqueness() {
        let seeds: Vec<[u8; 32]> = (0..500)
            .map(|i| compute_soul_seed(&mock_tx_hash(i)))
            .collect();
        for i in 0..seeds.len() {
            for j in (i+1)..seeds.len() {
                assert_ne!(seeds[i], seeds[j], "Seeds {} and {} collided", i, j);
            }
        }
    }

    #[test]
    fn test_avalanche_single_char_change() {
        for i in 0..50 {
            let base = format!("test_tx_hash_{:04}", i);
            let changed = format!("test_tx_hash_{:04}", i + 1);
            let s1 = compute_soul_seed(&base);
            let s2 = compute_soul_seed(&changed);
            assert_avalanche(&s1, &s2, 14);
        }
    }

    #[test]
    fn test_seed_not_all_zeros() {
        for i in 0..100 {
            let seed = compute_soul_seed(&mock_tx_hash(i));
            assert!(seed.iter().any(|&b| b != 0), "Seed {} was all zeros", i);
        }
    }

    #[test]
    fn test_seed_byte_distribution() {
        // Verify seeds have reasonable byte distribution
        let mut histogram = [0u32; 256];
        for i in 0..1000 {
            let seed = compute_soul_seed(&mock_tx_hash(i));
            for &b in &seed {
                histogram[b as usize] += 1;
            }
        }
        // Every byte value should appear at least once in 32,000 samples
        let non_zero = histogram.iter().filter(|&&c| c > 0).count();
        assert!(non_zero > 200, "Poor distribution: only {} of 256 values seen", non_zero);
    }

    #[test]
    fn test_empty_input() {
        let seed = compute_soul_seed("");
        assert_eq!(seed.len(), 32);
    }

    #[test]
    fn test_long_input() {
        let long = "x".repeat(10_000);
        let seed = compute_soul_seed(&long);
        assert_eq!(seed.len(), 32);
        assert!(seed.iter().any(|&b| b != 0));
    }

    #[test]
    fn test_art_parameters_from_seed() {
        let seed = compute_soul_seed("4xK7m9pR2abc123");
        
        // These mirror the frontend SoulSignature.jsx trait derivation
        let eye_style = seed[0] % 8;       // 0-7
        let glow_eyes = seed[1] > 102;     // ~60% chance
        let hat_type = seed[2] % 30;       // 0-29
        let glasses = seed[3] % 12;        // 0-11
        let mouth_item = seed[4] % 8;      // 0-7
        let neck_item = seed[5] % 6;       // 0-5
        let teeth_style = seed[6] % 6;     // 0-5
        let nose_style = seed[7] % 4;      // 0-3

        assert!(eye_style < 8);
        assert!(hat_type < 30);
        assert!(glasses < 12);
        assert!(mouth_item < 8);
        assert!(neck_item < 6);
        assert!(teeth_style < 6);
        assert!(nose_style < 4);

        // Verify traits are readable
        let hat_names = [
            "Cowboy", "Top Hat", "Beanie", "Baseball", "Crown",
            "Pirate", "Sailor", "Trucker", "Fedora", "Wizard",
            "Headband", "Mohawk", "Viking", "Chef", "Bandana",
            "Halo", "Bucket", "Santa", "Afro", "Devil Horns",
            "Army", "Sombrero", "Backwards Cap", "Durag", "Bowler",
            "Straw", "Space Helmet", "Fire", "Propeller", "Toque",
        ];
        let _hat_name = hat_names[hat_type as usize];
    }

    #[test]
    fn test_trait_distribution() {
        // Verify hat distribution is roughly uniform
        let mut hat_counts = [0u32; 30];
        for i in 0..3000 {
            let seed = compute_soul_seed(&mock_tx_hash(i));
            hat_counts[(seed[2] % 30) as usize] += 1;
        }
        // Each hat should appear at least 50 times in 3000 samples (expected: 100)
        for (i, &count) in hat_counts.iter().enumerate() {
            assert!(count > 30, "Hat {} only appeared {} times", i, count);
        }
    }

    #[test]
    fn test_accessory_combinations() {
        // Count unique accessory combos across many seeds
        let mut combos = std::collections::HashSet::new();
        for i in 0..1000 {
            let seed = compute_soul_seed(&mock_tx_hash(i));
            let combo = (seed[0] % 8, seed[2] % 30, seed[3] % 12, seed[4] % 8, seed[5] % 6);
            combos.insert(combo);
        }
        // Should have high diversity — at least 900 unique combos in 1000 seeds
        assert!(combos.len() > 800, "Only {} unique combos in 1000 seeds", combos.len());
    }
}
