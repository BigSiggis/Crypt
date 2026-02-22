/// Transaction scoring engine — mirrors the frontend scoring logic.
/// Determines rarity tier based on transaction characteristics.
///
/// Score thresholds:
///   0-39:  Common  — dust trades, small transfers, unknown types
///   40-74: Rare    — memecoin apes, notable NFT sales, big moves
///   75+:   Legendary — whale trades, rug survivals, token creation
///
/// Scoring factors:
///   - Transaction value in SOL
///   - Transaction type (swap, NFT mint/sale, transfer, stake)
///   - Platform source (Jupiter, Raydium, etc.)
///   - Token classification (memecoin, blue chip, stablecoin)
///   - Net PnL direction

pub fn compute_rarity_score(
    tx_type: &str,
    sol_amount: f64,
    is_memecoin: bool,
    is_defi_source: bool,
    net_sol: f64,
) -> u32 {
    let mut score: i32 = 0;

    match tx_type {
        "SWAP" => {
            score += 25;
            if sol_amount > 100.0 { score += 80; }
            else if sol_amount > 50.0 { score += 60; }
            else if sol_amount > 10.0 { score += 35; }
            else if sol_amount > 2.0 { score += 15; }
            else if sol_amount > 0.5 { score += 5; }
            else { score -= 5; }

            if is_defi_source { score += 5; }
            if is_memecoin { score += 25; }
        }
        "NFT_MINT" | "COMPRESSED_NFT_MINT" => {
            score += 35;
            if sol_amount > 10.0 { score += 40; }
            else if sol_amount > 2.0 { score += 20; }
        }
        "NFT_SALE" => {
            score += 30;
            if sol_amount > 50.0 { score += 70; }
            else if sol_amount > 10.0 { score += 40; }
            else if sol_amount > 2.0 { score += 15; }
            else { score -= 5; }
            if net_sol > 0.0 { score += 20; }
        }
        "TRANSFER" | "SOL_TRANSFER" => {
            if sol_amount > 500.0 { score += 80; }
            else if sol_amount > 100.0 { score += 55; }
            else if sol_amount > 20.0 { score += 25; }
            else if sol_amount > 5.0 { score += 10; }
            else { score -= 15; }
        }
        "STAKE_SOL" | "UNSTAKE_SOL" => {
            if sol_amount > 100.0 { score += 50; }
            else if sol_amount > 20.0 { score += 25; }
            else { score += 5; }
        }
        "TOKEN_MINT" => {
            score += 50;
        }
        "BURN" | "BURN_NFT" => {
            score += 20;
        }
        _ => {
            score -= 20;
        }
    }

    // Deductions for dust transactions
    if sol_amount < 0.01
        && !matches!(tx_type, "NFT_MINT" | "COMPRESSED_NFT_MINT" | "TOKEN_MINT" | "BURN" | "BURN_NFT")
    {
        score -= 25;
    }

    score.max(0) as u32
}

/// Convert a numeric score to a rarity tier.
pub fn score_to_rarity(score: u32) -> u8 {
    if score >= 75 { 2 }      // Legendary
    else if score >= 40 { 1 }  // Rare
    else { 0 }                 // Common
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whale_swap_legendary() {
        let score = compute_rarity_score("SWAP", 150.0, false, true, 0.0);
        assert!(score >= 75, "Whale swap should be legendary, got {}", score);
        assert_eq!(score_to_rarity(score), 2);
    }

    #[test]
    fn test_memecoin_ape_rare() {
        let score = compute_rarity_score("SWAP", 5.0, true, true, 0.0);
        assert!(score >= 40, "Memecoin ape should be rare, got {}", score);
        assert_eq!(score_to_rarity(score), 1);
    }

    #[test]
    fn test_dust_trade_common() {
        let score = compute_rarity_score("SWAP", 0.001, false, false, 0.0);
        assert!(score < 40, "Dust trade should be common, got {}", score);
        assert_eq!(score_to_rarity(score), 0);
    }

    #[test]
    fn test_token_creation_rare() {
        let score = compute_rarity_score("TOKEN_MINT", 0.0, false, false, 0.0);
        assert!(score >= 40, "Token creation should be rare, got {}", score);
    }

    #[test]
    fn test_massive_transfer_legendary() {
        let score = compute_rarity_score("TRANSFER", 600.0, false, false, -600.0);
        assert!(score >= 75, "600 SOL transfer should be legendary, got {}", score);
    }

    #[test]
    fn test_nft_whale_sale() {
        let score = compute_rarity_score("NFT_SALE", 80.0, false, false, 80.0);
        assert!(score >= 75, "80 SOL NFT sale should be legendary, got {}", score);
    }

    #[test]
    fn test_unknown_type_penalized() {
        let score = compute_rarity_score("UNKNOWN", 1.0, false, false, 0.0);
        assert!(score < 40, "Unknown type should be penalized, got {}", score);
    }
}
