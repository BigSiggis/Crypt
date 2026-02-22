use colored::Colorize;

/// Compute rarity score — mirrors the on-chain scoring logic.
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
        "TOKEN_MINT" => { score += 50; }
        "BURN" | "BURN_NFT" => { score += 20; }
        _ => { score -= 20; }
    }

    if sol_amount < 0.01
        && !matches!(tx_type, "NFT_MINT" | "COMPRESSED_NFT_MINT" | "TOKEN_MINT" | "BURN" | "BURN_NFT")
    { score -= 25; }

    score.max(0) as u32
}

pub fn score_to_rarity(score: u32) -> u8 {
    if score >= 75 { 2 } else if score >= 40 { 1 } else { 0 }
}

/// Display a detailed score breakdown for a transaction type.
pub fn show_score(tx_type: &str, sol: f64, memecoin: bool, defi: bool) {
    let score = compute_rarity_score(tx_type, sol, memecoin, defi, 0.0);
    let rarity = match score_to_rarity(score) {
        2 => "LEGENDARY".bright_magenta().bold(),
        1 => "RARE".bright_cyan(),
        _ => "COMMON".white(),
    };

    println!("{}", "  SCORE BREAKDOWN".bright_green());
    println!("  ────────────────────────────");
    println!("  Type:       {}", tx_type.bright_yellow());
    println!("  SOL:        {:.4}", sol);
    println!("  Memecoin:   {}", if memecoin { "YES".bright_magenta() } else { "no".white() });
    println!("  DeFi:       {}", if defi { "YES".bright_cyan() } else { "no".white() });
    println!("  ────────────────────────────");
    println!("  Score:      {}", score.to_string().bright_green().bold());
    println!("  Rarity:     {}", rarity);

    // Show threshold info
    println!("\n  Thresholds: 0-39 Common | 40-74 Rare | 75+ Legendary");
}
