//! Collection-level metrics and analytics.

/// Collection health metrics.
#[derive(Debug, Clone, Default)]
pub struct CollectionMetrics {
    pub total_cards: u64,
    pub active_cards: u64,
    pub burned_cards: u64,
    pub unique_owners: u64,
    pub total_interactions: u64,
    pub avg_interactions_per_card: f64,
    pub rarity_distribution: RarityDistribution,
    pub type_distribution: TypeDistribution,
    pub mints_last_24h: u64,
    pub mints_last_7d: u64,
    pub transfers_last_24h: u64,
}

#[derive(Debug, Clone, Default)]
pub struct RarityDistribution {
    pub common: u64,
    pub rare: u64,
    pub legendary: u64,
}

impl RarityDistribution {
    pub fn total(&self) -> u64 {
        self.common + self.rare + self.legendary
    }

    pub fn common_pct(&self) -> f64 {
        if self.total() == 0 { 0.0 } else { self.common as f64 / self.total() as f64 * 100.0 }
    }

    pub fn rare_pct(&self) -> f64 {
        if self.total() == 0 { 0.0 } else { self.rare as f64 / self.total() as f64 * 100.0 }
    }

    pub fn legendary_pct(&self) -> f64 {
        if self.total() == 0 { 0.0 } else { self.legendary as f64 / self.total() as f64 * 100.0 }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TypeDistribution {
    pub swaps: u64,
    pub rugs: u64,
    pub mints: u64,
    pub diamond_hands: u64,
    pub big_moves: u64,
}

impl TypeDistribution {
    pub fn total(&self) -> u64 {
        self.swaps + self.rugs + self.mints + self.diamond_hands + self.big_moves
    }

    pub fn most_common(&self) -> &'static str {
        let counts = [
            (self.swaps, "SWAP"),
            (self.rugs, "RUG"),
            (self.mints, "MINT"),
            (self.diamond_hands, "DIAMOND_HANDS"),
            (self.big_moves, "BIG_MOVE"),
        ];
        counts.iter().max_by_key(|(c, _)| c).map(|(_, n)| *n).unwrap_or("NONE")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rarity_distribution() {
        let dist = RarityDistribution { common: 70, rare: 25, legendary: 5 };
        assert_eq!(dist.total(), 100);
        assert!((dist.common_pct() - 70.0).abs() < 0.01);
        assert!((dist.rare_pct() - 25.0).abs() < 0.01);
        assert!((dist.legendary_pct() - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_empty_distribution() {
        let dist = RarityDistribution::default();
        assert_eq!(dist.common_pct(), 0.0);
    }

    #[test]
    fn test_most_common_type() {
        let dist = TypeDistribution { swaps: 50, rugs: 10, mints: 20, diamond_hands: 5, big_moves: 15 };
        assert_eq!(dist.most_common(), "SWAP");
    }
}
