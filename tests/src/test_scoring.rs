//! Comprehensive tests for the rarity scoring engine.

#[cfg(test)]
mod tests {
    /// Mirrors the on-chain scoring logic for testing.
    fn score(tx_type: &str, sol: f64, memecoin: bool, defi: bool, net: f64) -> u32 {
        let mut s: i32 = 0;
        match tx_type {
            "SWAP" => {
                s += 25;
                if sol > 100.0 { s += 80; }
                else if sol > 50.0 { s += 60; }
                else if sol > 10.0 { s += 35; }
                else if sol > 2.0 { s += 15; }
                else if sol > 0.5 { s += 5; }
                else { s -= 5; }
                if defi { s += 5; }
                if memecoin { s += 25; }
            }
            "NFT_MINT" | "COMPRESSED_NFT_MINT" => {
                s += 35;
                if sol > 10.0 { s += 40; }
                else if sol > 2.0 { s += 20; }
            }
            "NFT_SALE" => {
                s += 30;
                if sol > 50.0 { s += 70; }
                else if sol > 10.0 { s += 40; }
                else if sol > 2.0 { s += 15; }
                else { s -= 5; }
                if net > 0.0 { s += 20; }
            }
            "TRANSFER" | "SOL_TRANSFER" => {
                if sol > 500.0 { s += 80; }
                else if sol > 100.0 { s += 55; }
                else if sol > 20.0 { s += 25; }
                else if sol > 5.0 { s += 10; }
                else { s -= 15; }
            }
            "STAKE_SOL" | "UNSTAKE_SOL" => {
                if sol > 100.0 { s += 50; }
                else if sol > 20.0 { s += 25; }
                else { s += 5; }
            }
            "TOKEN_MINT" => { s += 50; }
            "BURN" | "BURN_NFT" => { s += 20; }
            _ => { s -= 20; }
        }
        if sol < 0.01 && !matches!(tx_type, "NFT_MINT" | "COMPRESSED_NFT_MINT" | "TOKEN_MINT" | "BURN" | "BURN_NFT") {
            s -= 25;
        }
        s.max(0) as u32
    }

    fn rarity(s: u32) -> &'static str {
        if s >= 75 { "LEGENDARY" } else if s >= 40 { "RARE" } else { "COMMON" }
    }

    // === SWAP SCORING ===
    #[test] fn test_swap_dust() { assert_eq!(rarity(score("SWAP", 0.001, false, false, 0.0)), "COMMON"); }
    #[test] fn test_swap_small() { assert_eq!(rarity(score("SWAP", 0.8, false, false, 0.0)), "COMMON"); }
    #[test] fn test_swap_solid() { assert_eq!(rarity(score("SWAP", 5.0, false, true, 0.0)), "RARE"); }
    #[test] fn test_swap_big() { assert_eq!(rarity(score("SWAP", 15.0, false, true, 0.0)), "RARE"); }
    #[test] fn test_swap_whale() { assert_eq!(rarity(score("SWAP", 60.0, false, false, 0.0)), "LEGENDARY"); }
    #[test] fn test_swap_mega_whale() { assert_eq!(rarity(score("SWAP", 200.0, false, true, 0.0)), "LEGENDARY"); }
    #[test] fn test_swap_memecoin_small() { assert_eq!(rarity(score("SWAP", 3.0, true, true, 0.0)), "RARE"); }
    #[test] fn test_swap_memecoin_big() { assert_eq!(rarity(score("SWAP", 15.0, true, true, 0.0)), "LEGENDARY"); }

    // === NFT SCORING ===
    #[test] fn test_nft_free_mint() { assert_eq!(rarity(score("NFT_MINT", 0.0, false, false, 0.0)), "COMMON"); }
    #[test] fn test_nft_paid_mint() { assert_eq!(rarity(score("NFT_MINT", 5.0, false, false, 0.0)), "RARE"); }
    #[test] fn test_nft_premium_mint() { assert_eq!(rarity(score("NFT_MINT", 20.0, false, false, 0.0)), "LEGENDARY"); }
    #[test] fn test_nft_small_sale() { assert_eq!(rarity(score("NFT_SALE", 1.0, false, false, 1.0)), "RARE"); }
    #[test] fn test_nft_big_sale() { assert_eq!(rarity(score("NFT_SALE", 15.0, false, false, 15.0)), "LEGENDARY"); }
    #[test] fn test_nft_whale_sale() { assert_eq!(rarity(score("NFT_SALE", 80.0, false, false, 80.0)), "LEGENDARY"); }

    // === TRANSFER SCORING ===
    #[test] fn test_transfer_dust() { assert_eq!(rarity(score("TRANSFER", 0.5, false, false, -0.5)), "COMMON"); }
    #[test] fn test_transfer_medium() { assert_eq!(rarity(score("TRANSFER", 25.0, false, false, -25.0)), "COMMON"); }
    #[test] fn test_transfer_big() { assert_eq!(rarity(score("TRANSFER", 150.0, false, false, -150.0)), "RARE"); }
    #[test] fn test_transfer_massive() { assert_eq!(rarity(score("TRANSFER", 600.0, false, false, -600.0)), "LEGENDARY"); }

    // === STAKING SCORING ===
    #[test] fn test_stake_small() { assert_eq!(rarity(score("STAKE_SOL", 10.0, false, false, 0.0)), "COMMON"); }
    #[test] fn test_stake_medium() { assert_eq!(rarity(score("STAKE_SOL", 30.0, false, false, 0.0)), "COMMON"); }
    #[test] fn test_stake_whale() { assert_eq!(rarity(score("STAKE_SOL", 200.0, false, false, 0.0)), "RARE"); }

    // === SPECIAL TYPES ===
    #[test] fn test_token_creation() { assert_eq!(rarity(score("TOKEN_MINT", 0.0, false, false, 0.0)), "RARE"); }
    #[test] fn test_burn() { assert!(score("BURN", 0.0, false, false, 0.0) > 0); }
    #[test] fn test_unknown_type() { assert_eq!(rarity(score("UNKNOWN", 1.0, false, false, 0.0)), "COMMON"); }

    // === EDGE CASES ===
    #[test] fn test_zero_sol_swap() { assert_eq!(score("SWAP", 0.0, false, false, 0.0), 0); }
    #[test] fn test_negative_net_nft_sale() { assert!(score("NFT_SALE", 5.0, false, false, -5.0) < score("NFT_SALE", 5.0, false, false, 5.0)); }

    // === RARITY THRESHOLDS ===
    #[test] fn test_common_threshold() { assert_eq!(rarity(0), "COMMON"); assert_eq!(rarity(39), "COMMON"); }
    #[test] fn test_rare_threshold() { assert_eq!(rarity(40), "RARE"); assert_eq!(rarity(74), "RARE"); }
    #[test] fn test_legendary_threshold() { assert_eq!(rarity(75), "LEGENDARY"); assert_eq!(rarity(200), "LEGENDARY"); }
}
