//! Wallet profiling — classifies wallets based on transaction history.
//! Used to determine which transactions are most "card-worthy" for each wallet.

use std::collections::HashMap;

/// Wallet archetype based on transaction patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletArchetype {
    /// High-frequency memecoin trader, many small trades
    Degen,
    /// Large position sizes, low frequency
    Whale,
    /// Primarily NFT activity (mints, sales, listings)
    Collector,
    /// Mostly staking and DeFi yield farming
    Farmer,
    /// Token creator, launched projects
    Builder,
    /// Mixed activity, no dominant pattern
    Explorer,
    /// New wallet, limited history
    Newcomer,
    /// Inactive, long gaps between transactions
    Ghost,
}

impl WalletArchetype {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Degen => "DEGEN",
            Self::Whale => "WHALE",
            Self::Collector => "COLLECTOR",
            Self::Farmer => "FARMER",
            Self::Builder => "BUILDER",
            Self::Explorer => "EXPLORER",
            Self::Newcomer => "NEWCOMER",
            Self::Ghost => "GHOST",
        }
    }

    /// Get the archetype's flavor text for card narrations.
    pub fn flavor(&self) -> &'static str {
        match self {
            Self::Degen => "A creature of pure instinct. Charts are suggestions. Sleep is optional.",
            Self::Whale => "The water parts when this wallet moves. Everyone watches. Nobody leads.",
            Self::Collector => "Every mint is a bet on culture. Every sale is a statement.",
            Self::Farmer => "Patient capital. Compounding faith. The long game is the only game.",
            Self::Builder => "From zero to contract address. Creating something from nothing.",
            Self::Explorer => "No pattern. No thesis. Just motion. Sometimes that's enough.",
            Self::Newcomer => "Fresh address. Clean slate. Everything ahead is unwritten.",
            Self::Ghost => "Long silence between transactions. But when it moves, it means something.",
        }
    }
}

/// Statistics for wallet classification.
#[derive(Debug, Clone, Default)]
pub struct WalletStats {
    pub total_txs: u64,
    pub swap_count: u64,
    pub nft_count: u64,
    pub transfer_count: u64,
    pub stake_count: u64,
    pub token_creates: u64,
    pub burns: u64,
    pub memecoin_trades: u64,
    pub max_sol_moved: f64,
    pub total_sol_volume: f64,
    pub avg_tx_value_sol: f64,
    pub unique_tokens_traded: u64,
    pub unique_nft_collections: u64,
    pub first_tx_timestamp: i64,
    pub last_tx_timestamp: i64,
    pub active_days: u64,
    pub longest_gap_days: u64,
}

impl WalletStats {
    /// Classify the wallet into an archetype.
    pub fn classify(&self) -> WalletArchetype {
        if self.total_txs < 5 {
            return WalletArchetype::Newcomer;
        }

        if self.longest_gap_days > 90 && self.total_txs < 20 {
            return WalletArchetype::Ghost;
        }

        // Calculate ratios
        let swap_ratio = self.swap_count as f64 / self.total_txs as f64;
        let nft_ratio = self.nft_count as f64 / self.total_txs as f64;
        let stake_ratio = self.stake_count as f64 / self.total_txs as f64;
        let memecoin_ratio = if self.swap_count > 0 {
            self.memecoin_trades as f64 / self.swap_count as f64
        } else { 0.0 };

        // Token creators
        if self.token_creates >= 2 {
            return WalletArchetype::Builder;
        }

        // Whale detection: large average trade size
        if self.avg_tx_value_sol > 50.0 || self.max_sol_moved > 500.0 {
            return WalletArchetype::Whale;
        }

        // Degen: high swap frequency with memecoin exposure
        if swap_ratio > 0.6 && (memecoin_ratio > 0.3 || self.unique_tokens_traded > 20) {
            return WalletArchetype::Degen;
        }

        // Collector: primarily NFT activity
        if nft_ratio > 0.4 || self.unique_nft_collections > 10 {
            return WalletArchetype::Collector;
        }

        // Farmer: staking/DeFi focused
        if stake_ratio > 0.3 {
            return WalletArchetype::Farmer;
        }

        WalletArchetype::Explorer
    }

    /// Get the top transaction type for this wallet.
    pub fn dominant_activity(&self) -> &'static str {
        let activities = [
            (self.swap_count, "TRADING"),
            (self.nft_count, "NFT"),
            (self.transfer_count, "TRANSFERS"),
            (self.stake_count, "STAKING"),
        ];
        activities.iter()
            .max_by_key(|(count, _)| count)
            .map(|(_, name)| *name)
            .unwrap_or("MIXED")
    }

    /// Calculate wallet "age" in days.
    pub fn age_days(&self) -> u64 {
        if self.last_tx_timestamp > self.first_tx_timestamp {
            ((self.last_tx_timestamp - self.first_tx_timestamp) / 86400) as u64
        } else { 0 }
    }

    /// Calculate average transactions per day.
    pub fn tx_frequency(&self) -> f64 {
        let days = self.age_days().max(1) as f64;
        self.total_txs as f64 / days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_stats() -> WalletStats {
        WalletStats {
            total_txs: 50,
            first_tx_timestamp: 1700000000,
            last_tx_timestamp: 1705000000,
            active_days: 30,
            ..Default::default()
        }
    }

    #[test]
    fn test_newcomer() {
        let stats = WalletStats { total_txs: 3, ..Default::default() };
        assert_eq!(stats.classify(), WalletArchetype::Newcomer);
    }

    #[test]
    fn test_ghost() {
        let stats = WalletStats {
            total_txs: 10,
            longest_gap_days: 120,
            ..Default::default()
        };
        assert_eq!(stats.classify(), WalletArchetype::Ghost);
    }

    #[test]
    fn test_whale() {
        let mut stats = base_stats();
        stats.avg_tx_value_sol = 80.0;
        stats.swap_count = 20;
        assert_eq!(stats.classify(), WalletArchetype::Whale);
    }

    #[test]
    fn test_degen() {
        let mut stats = base_stats();
        stats.swap_count = 40;
        stats.memecoin_trades = 25;
        stats.unique_tokens_traded = 30;
        assert_eq!(stats.classify(), WalletArchetype::Degen);
    }

    #[test]
    fn test_collector() {
        let mut stats = base_stats();
        stats.nft_count = 30;
        stats.unique_nft_collections = 15;
        assert_eq!(stats.classify(), WalletArchetype::Collector);
    }

    #[test]
    fn test_farmer() {
        let mut stats = base_stats();
        stats.stake_count = 20;
        assert_eq!(stats.classify(), WalletArchetype::Farmer);
    }

    #[test]
    fn test_builder() {
        let mut stats = base_stats();
        stats.token_creates = 3;
        assert_eq!(stats.classify(), WalletArchetype::Builder);
    }

    #[test]
    fn test_explorer() {
        let mut stats = base_stats();
        stats.swap_count = 15;
        stats.nft_count = 10;
        stats.transfer_count = 15;
        stats.stake_count = 10;
        assert_eq!(stats.classify(), WalletArchetype::Explorer);
    }

    #[test]
    fn test_age_days() {
        let stats = WalletStats {
            first_tx_timestamp: 1700000000,
            last_tx_timestamp: 1700864000, // +10 days
            ..Default::default()
        };
        assert_eq!(stats.age_days(), 10);
    }

    #[test]
    fn test_tx_frequency() {
        let stats = WalletStats {
            total_txs: 100,
            first_tx_timestamp: 1700000000,
            last_tx_timestamp: 1700864000,
            ..Default::default()
        };
        assert!((stats.tx_frequency() - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_dominant_activity() {
        let stats = WalletStats {
            swap_count: 50, nft_count: 10, transfer_count: 5, stake_count: 3,
            ..Default::default()
        };
        assert_eq!(stats.dominant_activity(), "TRADING");
    }

    #[test]
    fn test_archetype_flavor_text() {
        for archetype in [
            WalletArchetype::Degen, WalletArchetype::Whale, WalletArchetype::Collector,
            WalletArchetype::Farmer, WalletArchetype::Builder, WalletArchetype::Explorer,
            WalletArchetype::Newcomer, WalletArchetype::Ghost,
        ] {
            assert!(!archetype.flavor().is_empty());
            assert!(!archetype.as_str().is_empty());
        }
    }
}
