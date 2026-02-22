//! Rarity scoring engine — determines card tier from transaction data.

use crate::types::Rarity;

/// Rarity scorer with configurable thresholds.
pub struct RarityScorer {
    pub rare_threshold: u32,
    pub legendary_threshold: u32,
}

impl Default for RarityScorer {
    fn default() -> Self {
        Self {
            rare_threshold: 40,
            legendary_threshold: 75,
        }
    }
}

impl RarityScorer {
    pub fn new(rare: u32, legendary: u32) -> Self {
        Self { rare_threshold: rare, legendary_threshold: legendary }
    }

    /// Compute a rarity score from transaction characteristics.
    pub fn score(&self, params: &ScoreParams) -> ScoreResult {
        let mut score: i32 = 0;
        let mut factors: Vec<String> = Vec::new();

        match params.tx_type.as_str() {
            "SWAP" => {
                score += 25;
                factors.push("Base: SWAP (+25)".into());

                if params.sol_amount > 100.0 {
                    score += 80; factors.push(format!("Whale trade: {:.1} SOL (+80)", params.sol_amount));
                } else if params.sol_amount > 50.0 {
                    score += 60; factors.push(format!("Large trade: {:.1} SOL (+60)", params.sol_amount));
                } else if params.sol_amount > 10.0 {
                    score += 35; factors.push(format!("Notable trade: {:.1} SOL (+35)", params.sol_amount));
                } else if params.sol_amount > 2.0 {
                    score += 15; factors.push(format!("Solid trade: {:.1} SOL (+15)", params.sol_amount));
                } else if params.sol_amount > 0.5 {
                    score += 5; factors.push("Small trade (+5)".into());
                } else {
                    score -= 5; factors.push("Dust trade (-5)".into());
                }

                if params.is_defi_source {
                    score += 5; factors.push("DeFi source (+5)".into());
                }
                if params.is_memecoin {
                    score += 25; factors.push("Memecoin (+25)".into());
                }
            }
            "NFT_MINT" | "COMPRESSED_NFT_MINT" => {
                score += 35; factors.push("Base: NFT_MINT (+35)".into());
                if params.sol_amount > 10.0 {
                    score += 40; factors.push(format!("Premium mint: {:.1} SOL (+40)", params.sol_amount));
                } else if params.sol_amount > 2.0 {
                    score += 20; factors.push(format!("Paid mint: {:.1} SOL (+20)", params.sol_amount));
                }
            }
            "NFT_SALE" => {
                score += 30; factors.push("Base: NFT_SALE (+30)".into());
                if params.sol_amount > 50.0 {
                    score += 70; factors.push(format!("Whale sale: {:.1} SOL (+70)", params.sol_amount));
                } else if params.sol_amount > 10.0 {
                    score += 40; factors.push(format!("Big sale: {:.1} SOL (+40)", params.sol_amount));
                } else if params.sol_amount > 2.0 {
                    score += 15; factors.push(format!("Sale: {:.1} SOL (+15)", params.sol_amount));
                } else {
                    score -= 5; factors.push("Small sale (-5)".into());
                }
                if params.net_sol > 0.0 {
                    score += 20; factors.push("Profit (+20)".into());
                }
            }
            "TRANSFER" | "SOL_TRANSFER" => {
                if params.sol_amount > 500.0 {
                    score += 80; factors.push(format!("Massive: {:.0} SOL (+80)", params.sol_amount));
                } else if params.sol_amount > 100.0 {
                    score += 55; factors.push(format!("Whale move: {:.0} SOL (+55)", params.sol_amount));
                } else if params.sol_amount > 20.0 {
                    score += 25; factors.push(format!("Big move: {:.0} SOL (+25)", params.sol_amount));
                } else if params.sol_amount > 5.0 {
                    score += 10; factors.push("Transfer (+10)".into());
                } else {
                    score -= 15; factors.push("Small transfer (-15)".into());
                }
            }
            "STAKE_SOL" | "UNSTAKE_SOL" => {
                if params.sol_amount > 100.0 {
                    score += 50; factors.push(format!("Whale stake: {:.0} SOL (+50)", params.sol_amount));
                } else if params.sol_amount > 20.0 {
                    score += 25; factors.push(format!("Stake: {:.0} SOL (+25)", params.sol_amount));
                } else {
                    score += 5; factors.push("Small stake (+5)".into());
                }
            }
            "TOKEN_MINT" => {
                score += 50; factors.push("Token creation (+50)".into());
            }
            "BURN" | "BURN_NFT" => {
                score += 20; factors.push("Burn (+20)".into());
            }
            _ => {
                score -= 20; factors.push(format!("Unknown type: {} (-20)", params.tx_type));
            }
        }

        if params.sol_amount < 0.01
            && !matches!(params.tx_type.as_str(), "NFT_MINT" | "COMPRESSED_NFT_MINT" | "TOKEN_MINT" | "BURN" | "BURN_NFT")
        {
            score -= 25; factors.push("Dust penalty (-25)".into());
        }

        let final_score = score.max(0) as u32;
        let rarity = if final_score >= self.legendary_threshold { Rarity::Legendary }
            else if final_score >= self.rare_threshold { Rarity::Rare }
            else { Rarity::Common };

        ScoreResult { score: final_score, rarity, factors }
    }
}

/// Input parameters for scoring.
#[derive(Debug, Clone)]
pub struct ScoreParams {
    pub tx_type: String,
    pub sol_amount: f64,
    pub is_memecoin: bool,
    pub is_defi_source: bool,
    pub net_sol: f64,
}

/// Scoring result with breakdown.
#[derive(Debug, Clone)]
pub struct ScoreResult {
    pub score: u32,
    pub rarity: Rarity,
    pub factors: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scorer() -> RarityScorer { RarityScorer::default() }

    fn params(tx_type: &str, sol: f64) -> ScoreParams {
        ScoreParams {
            tx_type: tx_type.to_string(), sol_amount: sol,
            is_memecoin: false, is_defi_source: false, net_sol: 0.0,
        }
    }

    #[test] fn test_whale_swap() { assert_eq!(scorer().score(&params("SWAP", 150.0)).rarity, Rarity::Legendary); }
    #[test] fn test_dust_swap() { assert_eq!(scorer().score(&params("SWAP", 0.001)).rarity, Rarity::Common); }
    #[test] fn test_nft_premium() { assert_eq!(scorer().score(&params("NFT_MINT", 20.0)).rarity, Rarity::Legendary); }
    #[test] fn test_token_creation() { assert_eq!(scorer().score(&params("TOKEN_MINT", 0.0)).rarity, Rarity::Rare); }
    #[test] fn test_factors_populated() { assert!(!scorer().score(&params("SWAP", 5.0)).factors.is_empty()); }

    #[test]
    fn test_custom_thresholds() {
        let strict = RarityScorer::new(60, 90);
        let result = strict.score(&params("SWAP", 15.0));
        assert_eq!(result.rarity, Rarity::Common); // 60 score, threshold is 60
    }
}
