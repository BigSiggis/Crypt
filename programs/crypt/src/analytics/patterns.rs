//! Transaction pattern detection for interesting card moments.
//! Identifies notable patterns like diamond hands, rug pulls,
//! and early mints that make for the best cards.

/// Detected transaction patterns that make good cards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    /// Bought and held through >50% drawdown
    DiamondHands { token: String, max_drawdown_pct: u32 },
    /// Token went to zero or near-zero after purchase
    RugPull { token: String, loss_pct: u32 },
    /// Was in the first N minters of a collection
    EarlyMint { collection: String, mint_number: u32, total_supply: u32 },
    /// Made a profitable flip (buy → sell within 24h)
    QuickFlip { token: String, profit_pct: u32, hold_hours: u32 },
    /// Massive single transaction (>100 SOL)
    WhaleMove { sol_amount: f64 },
    /// Moved to cold storage (large transfer to new/inactive address)
    ColdStorage { sol_amount: f64 },
    /// First transaction ever on the wallet
    GenesisTransaction,
    /// Survived a major market crash while holding
    CrashSurvivor { drawdown_pct: u32 },
    /// Participated in a token airdrop
    AirdropReceiver { token: String, value_sol: f64 },
    /// Created a token that reached significant volume
    TokenLauncher { token: String },
}

impl Pattern {
    /// Card type for this pattern.
    pub fn card_type(&self) -> &'static str {
        match self {
            Self::DiamondHands { .. } => "DIAMOND_HANDS",
            Self::RugPull { .. } => "RUG",
            Self::EarlyMint { .. } => "MINT",
            Self::QuickFlip { .. } => "SWAP",
            Self::WhaleMove { .. } => "BIG_MOVE",
            Self::ColdStorage { .. } => "BIG_MOVE",
            Self::GenesisTransaction => "MINT",
            Self::CrashSurvivor { .. } => "DIAMOND_HANDS",
            Self::AirdropReceiver { .. } => "SWAP",
            Self::TokenLauncher { .. } => "MINT",
        }
    }

    /// Base rarity score bonus for this pattern.
    pub fn rarity_bonus(&self) -> u32 {
        match self {
            Self::DiamondHands { max_drawdown_pct, .. } => {
                if *max_drawdown_pct > 80 { 40 }
                else if *max_drawdown_pct > 50 { 25 }
                else { 15 }
            }
            Self::RugPull { loss_pct, .. } => {
                if *loss_pct > 99 { 30 } // complete rug
                else if *loss_pct > 90 { 20 }
                else { 10 }
            }
            Self::EarlyMint { mint_number, .. } => {
                if *mint_number <= 10 { 50 }
                else if *mint_number <= 100 { 30 }
                else { 15 }
            }
            Self::QuickFlip { profit_pct, .. } => {
                if *profit_pct > 1000 { 40 }
                else if *profit_pct > 100 { 25 }
                else { 10 }
            }
            Self::WhaleMove { sol_amount } => {
                if *sol_amount > 1000.0 { 50 }
                else if *sol_amount > 100.0 { 30 }
                else { 15 }
            }
            Self::ColdStorage { .. } => 20,
            Self::GenesisTransaction => 35,
            Self::CrashSurvivor { drawdown_pct } => {
                if *drawdown_pct > 70 { 45 }
                else { 25 }
            }
            Self::AirdropReceiver { value_sol, .. } => {
                if *value_sol > 10.0 { 30 }
                else { 10 }
            }
            Self::TokenLauncher { .. } => 40,
        }
    }

    /// Generate a narration title for this pattern.
    pub fn title(&self) -> String {
        match self {
            Self::DiamondHands { token, max_drawdown_pct } =>
                format!("HELD {} THROUGH {}% DRAWDOWN", token, max_drawdown_pct),
            Self::RugPull { token, loss_pct } =>
                format!("{} — RUGGED ({}% LOSS)", token, loss_pct),
            Self::EarlyMint { collection, mint_number, .. } =>
                format!("EARLY MINT #{} — {}", mint_number, collection),
            Self::QuickFlip { token, profit_pct, hold_hours } =>
                format!("{} FLIPPED +{}% IN {}H", token, profit_pct, hold_hours),
            Self::WhaleMove { sol_amount } =>
                format!("{:.0} SOL WHALE MOVE", sol_amount),
            Self::ColdStorage { sol_amount } =>
                format!("{:.0} SOL → COLD STORAGE", sol_amount),
            Self::GenesisTransaction =>
                "GENESIS — FIRST TRANSACTION".to_string(),
            Self::CrashSurvivor { drawdown_pct } =>
                format!("SURVIVED {}% CRASH", drawdown_pct),
            Self::AirdropReceiver { token, .. } =>
                format!("{} AIRDROP CLAIMED", token),
            Self::TokenLauncher { token } =>
                format!("LAUNCHED {}", token),
        }
    }
}

/// Detect if a sequence of transactions matches a known pattern.
pub fn detect_diamond_hands(
    buy_price_sol: f64,
    lowest_price_sol: f64,
    current_price_sol: f64,
    token: &str,
) -> Option<Pattern> {
    if buy_price_sol <= 0.0 { return None; }
    let drawdown = ((buy_price_sol - lowest_price_sol) / buy_price_sol * 100.0) as u32;
    if drawdown >= 30 && current_price_sol >= buy_price_sol * 0.5 {
        Some(Pattern::DiamondHands {
            token: token.to_string(),
            max_drawdown_pct: drawdown,
        })
    } else { None }
}

pub fn detect_rug_pull(buy_price_sol: f64, current_price_sol: f64, token: &str) -> Option<Pattern> {
    if buy_price_sol <= 0.0 { return None; }
    let loss = ((buy_price_sol - current_price_sol) / buy_price_sol * 100.0) as u32;
    if loss >= 90 {
        Some(Pattern::RugPull { token: token.to_string(), loss_pct: loss })
    } else { None }
}

pub fn detect_quick_flip(
    buy_price_sol: f64,
    sell_price_sol: f64,
    hold_seconds: u64,
    token: &str,
) -> Option<Pattern> {
    if buy_price_sol <= 0.0 || hold_seconds > 86400 { return None; }
    let profit = ((sell_price_sol - buy_price_sol) / buy_price_sol * 100.0) as u32;
    if profit >= 20 {
        Some(Pattern::QuickFlip {
            token: token.to_string(),
            profit_pct: profit,
            hold_hours: (hold_seconds / 3600) as u32,
        })
    } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diamond_hands_detected() {
        let p = detect_diamond_hands(10.0, 2.0, 8.0, "SOL");
        assert!(p.is_some());
        if let Some(Pattern::DiamondHands { max_drawdown_pct, .. }) = p {
            assert_eq!(max_drawdown_pct, 80);
        }
    }

    #[test]
    fn test_diamond_hands_not_enough_drawdown() {
        let p = detect_diamond_hands(10.0, 8.0, 10.0, "SOL");
        assert!(p.is_none());
    }

    #[test]
    fn test_rug_pull_detected() {
        let p = detect_rug_pull(5.0, 0.01, "SQUID");
        assert!(p.is_some());
        if let Some(Pattern::RugPull { loss_pct, .. }) = p {
            assert!(loss_pct > 99);
        }
    }

    #[test]
    fn test_rug_pull_not_enough_loss() {
        let p = detect_rug_pull(5.0, 3.0, "TOKEN");
        assert!(p.is_none());
    }

    #[test]
    fn test_quick_flip() {
        let p = detect_quick_flip(1.0, 3.0, 3600, "BONK");
        assert!(p.is_some());
        if let Some(Pattern::QuickFlip { profit_pct, hold_hours, .. }) = p {
            assert_eq!(profit_pct, 200);
            assert_eq!(hold_hours, 1);
        }
    }

    #[test]
    fn test_quick_flip_too_long() {
        let p = detect_quick_flip(1.0, 3.0, 100000, "BONK");
        assert!(p.is_none());
    }

    #[test]
    fn test_pattern_titles() {
        let patterns = vec![
            Pattern::DiamondHands { token: "SOL".into(), max_drawdown_pct: 75 },
            Pattern::RugPull { token: "SQUID".into(), loss_pct: 99 },
            Pattern::EarlyMint { collection: "DeGods".into(), mint_number: 5, total_supply: 10000 },
            Pattern::WhaleMove { sol_amount: 500.0 },
            Pattern::GenesisTransaction,
        ];
        for p in &patterns {
            assert!(!p.title().is_empty());
            assert!(p.rarity_bonus() > 0);
            assert!(!p.card_type().is_empty());
        }
    }

    #[test]
    fn test_early_mint_rarity_bonus() {
        let early = Pattern::EarlyMint { collection: "X".into(), mint_number: 5, total_supply: 10000 };
        let late = Pattern::EarlyMint { collection: "X".into(), mint_number: 500, total_supply: 10000 };
        assert!(early.rarity_bonus() > late.rarity_bonus());
    }
}
