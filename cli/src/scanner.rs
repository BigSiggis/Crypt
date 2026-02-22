use colored::Colorize;
use serde::{Deserialize, Serialize};
use crate::scoring;
use crate::soul;

const HELIUS_BASE: &str = "https://api.helius.xyz/v0";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusTransaction {
    pub signature: Option<String>,
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    pub source: Option<String>,
    pub timestamp: Option<i64>,
    pub native_transfers: Option<Vec<NativeTransfer>>,
    pub token_transfers: Option<Vec<TokenTransfer>>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeTransfer {
    pub from_user_account: Option<String>,
    pub to_user_account: Option<String>,
    pub amount: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenTransfer {
    pub from_user_account: Option<String>,
    pub to_user_account: Option<String>,
    pub mint: Option<String>,
    pub token_amount: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct CryptCard {
    pub id: usize,
    pub tx_hash: String,
    pub tx_type: String,
    pub rarity: String,
    pub score: u32,
    pub sol_amount: f64,
    pub title: String,
    pub platform: String,
    pub soul_seed: String,
    pub timestamp: i64,
}

/// Known memecoin mints
const MEMECOINS: &[&str] = &[
    "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263", // BONK
    "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm", // WIF
    "7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr", // POPCAT
    "MEW1gQWJ3nEXg2qgERiKu7FAFj79PHvQVREQUzScPP5",  // MEW
    "A3eME5CetyZPBoWbRUwY3tSe25S6tb18ba9ZPbWk9eFJ",  // PENG
    "WENWENvqqNya429ubCdR81ZmD69brwQaaBYY6p3LCpk",   // WEN
];

/// Known DeFi sources
const DEFI_SOURCES: &[&str] = &[
    "JUPITER", "RAYDIUM", "ORCA", "MARINADE", "DRIFT",
    "MANGO", "TENSOR", "MAGIC_EDEN",
];

/// Scan a Solana wallet via Helius API and generate Crypt Cards.
pub async fn scan_wallet(
    address: &str,
    api_key: Option<&str>,
    limit: usize,
    format: &str,
    min_rarity: Option<&str>,
) {
    // Validate address
    if address.len() < 32 || address.len() > 44 {
        eprintln!("{}", "Error: Invalid Solana address".red());
        return;
    }

    let key = match api_key {
        Some(k) => k.to_string(),
        None => {
            eprintln!("{}", "Error: Helius API key required. Set HELIUS_API_KEY env var or use --api-key".red());
            eprintln!("Get a free key at: https://helius.dev");
            return;
        }
    };

    println!("{} {}", "Scanning wallet:".bright_green(), address.yellow());
    println!("{} Fetching up to {} transactions...\n", ">>".bright_cyan(), limit);

    // Fetch transactions from Helius
    let url = format!(
        "{}/addresses/{}/transactions?api-key={}&limit={}",
        HELIUS_BASE, address, key, limit
    );

    let client = reqwest::Client::new();
    let response = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{} {}", "Helius API error:".red(), e);
            return;
        }
    };

    if !response.status().is_success() {
        eprintln!("{} HTTP {}", "Helius API error:".red(), response.status());
        return;
    }

    let txs: Vec<HeliusTransaction> = match response.json().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{} {}", "Parse error:".red(), e);
            return;
        }
    };

    println!("{} {} transactions fetched\n", ">>".bright_cyan(), txs.len());

    // Score and classify each transaction
    let mut cards: Vec<CryptCard> = Vec::new();

    for (i, tx) in txs.iter().enumerate() {
        let tx_type = tx.tx_type.clone().unwrap_or_default();
        let source = tx.source.clone().unwrap_or_default();
        let sig = tx.signature.clone().unwrap_or_default();
        let timestamp = tx.timestamp.unwrap_or(0);

        // Calculate SOL amount
        let sol_amount = get_sol_amount(tx);
        let net_sol = get_net_sol(tx, address);

        // Check for memecoin involvement
        let is_memecoin = tx.token_transfers.as_ref()
            .map(|transfers| transfers.iter().any(|t| {
                t.mint.as_ref().map(|m| MEMECOINS.contains(&m.as_str())).unwrap_or(false)
            }))
            .unwrap_or(false);

        let is_defi = DEFI_SOURCES.contains(&source.as_str());

        // Compute score and rarity
        let score = scoring::compute_rarity_score(&tx_type, sol_amount, is_memecoin, is_defi, net_sol);
        let rarity = match scoring::score_to_rarity(score) {
            2 => "LEGENDARY",
            1 => "RARE",
            _ => "COMMON",
        };

        // Apply rarity filter
        if let Some(min) = min_rarity {
            match min.to_uppercase().as_str() {
                "LEGENDARY" => if rarity != "LEGENDARY" { continue; },
                "RARE" => if rarity == "COMMON" { continue; },
                _ => {}
            }
        }

        if score == 0 { continue; }

        // Build title
        let title = build_title(&tx_type, sol_amount, &source);

        // Generate soul seed
        let seed = soul::compute_soul_seed_bytes(&sig);
        let seed_hex = hex::encode(&seed[..8]);

        cards.push(CryptCard {
            id: i + 1,
            tx_hash: if sig.len() > 12 { format!("{}...{}", &sig[..6], &sig[sig.len()-4..]) } else { sig },
            tx_type: tx_type.clone(),
            rarity: rarity.to_string(),
            score,
            sol_amount,
            title,
            platform: source,
            soul_seed: seed_hex,
            timestamp,
        });
    }

    // Sort by score descending
    cards.sort_by(|a, b| b.score.cmp(&a.score));
    cards.truncate(8);

    // Output
    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&cards).unwrap_or_default());
        }
        "table" => {
            print_table(&cards);
        }
        _ => {
            print_cards(&cards);
        }
    }

    println!(
        "\n{} {} cards generated ({} legendary, {} rare, {} common)\n",
        ">>".bright_cyan(),
        cards.len(),
        cards.iter().filter(|c| c.rarity == "LEGENDARY").count(),
        cards.iter().filter(|c| c.rarity == "RARE").count(),
        cards.iter().filter(|c| c.rarity == "COMMON").count(),
    );
}

fn get_sol_amount(tx: &HeliusTransaction) -> f64 {
    tx.native_transfers.as_ref()
        .map(|transfers| {
            transfers.iter()
                .map(|t| (t.amount.unwrap_or(0) as f64 / 1e9).abs())
                .fold(0.0_f64, f64::max)
        })
        .unwrap_or(0.0)
}

fn get_net_sol(tx: &HeliusTransaction, wallet: &str) -> f64 {
    tx.native_transfers.as_ref()
        .map(|transfers| {
            transfers.iter().fold(0.0, |acc, t| {
                let amount = t.amount.unwrap_or(0) as f64 / 1e9;
                if t.to_user_account.as_deref() == Some(wallet) { acc + amount }
                else if t.from_user_account.as_deref() == Some(wallet) { acc - amount }
                else { acc }
            })
        })
        .unwrap_or(0.0)
}

fn build_title(tx_type: &str, sol: f64, source: &str) -> String {
    match tx_type {
        "SWAP" => format!("{:.2} SOL SWAP on {}", sol, source),
        "NFT_MINT" | "COMPRESSED_NFT_MINT" => {
            if sol > 0.0 { format!("MINTED NFT for {:.2} SOL", sol) }
            else { "FREE MINT".to_string() }
        }
        "NFT_SALE" => format!("NFT SOLD for {:.2} SOL", sol),
        "TRANSFER" | "SOL_TRANSFER" => format!("{:.2} SOL TRANSFER", sol),
        "STAKE_SOL" => format!("STAKED {:.2} SOL", sol),
        "UNSTAKE_SOL" => format!("UNSTAKED {:.2} SOL", sol),
        "TOKEN_MINT" => "LAUNCHED A TOKEN".to_string(),
        "BURN" | "BURN_NFT" => "BURNED".to_string(),
        _ => format!("{} via {}", tx_type, source),
    }
}

fn print_cards(cards: &[CryptCard]) {
    for card in cards {
        let rarity_colored = match card.rarity.as_str() {
            "LEGENDARY" => card.rarity.bright_magenta().bold(),
            "RARE" => card.rarity.bright_cyan(),
            _ => card.rarity.white(),
        };

        let border = match card.rarity.as_str() {
            "LEGENDARY" => "═".bright_magenta(),
            "RARE" => "─".bright_cyan(),
            _ => "─".white(),
        };

        println!("  {}{}{}",
            border, border.to_string().repeat(38), border);
        println!("  {} #{:<3} {} {:<32} {}",
            "|".bright_cyan(),
            card.id, rarity_colored,
            card.title.chars().take(32).collect::<String>(),
            "|".bright_cyan());
        println!("  {} {:<6} Score:{:<3} Soul:{}  {} {}",
            "|".bright_cyan(),
            card.tx_type, card.score, card.soul_seed,
            card.tx_hash.bright_black(),
            "|".bright_cyan());
        println!("  {}{}{}",
            border, border.to_string().repeat(38), border);
    }
}

fn print_table(cards: &[CryptCard]) {
    println!("{:<4} {:<10} {:<10} {:<5} {:<35} {:<15}",
        "#", "RARITY", "TYPE", "SCORE", "TITLE", "TX");
    println!("{}", "─".repeat(80));
    for card in cards {
        println!("{:<4} {:<10} {:<10} {:<5} {:<35} {:<15}",
            card.id, card.rarity, card.tx_type, card.score,
            card.title.chars().take(35).collect::<String>(), card.tx_hash);
    }
}
