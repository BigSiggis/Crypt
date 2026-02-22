//! Crypt Indexer — monitors the Solana blockchain for Crypt program events.
//!
//! Watches for:
//! - CardMinted events → indexes new cards
//! - CardTransferred events → updates ownership
//! - CardBurned events → marks cards as destroyed
//! - RarityUpgraded events → tracks rarity changes
//! - CardInteraction events → aggregates social stats
//!
//! In production, this would write to a database (Postgres, DynamoDB)
//! and serve a REST/GraphQL API for the frontend.

mod events;
mod processor;
mod store;
mod watcher;

use colored::Colorize;

#[tokio::main]
async fn main() {
    println!("{}", "\n ╔═══════════════════════════════════════╗".bright_cyan());
    println!("{}", " ║  CRYPT INDEXER                        ║".bright_cyan());
    println!("{}", " ║  Monitoring Solana for Crypt events    ║".bright_cyan());
    println!("{}", " ╚═══════════════════════════════════════╝\n".bright_cyan());

    let rpc_url = std::env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    let program_id = std::env::var("CRYPT_PROGRAM_ID")
        .unwrap_or_else(|_| "CRYPTxGraveyardSo1ana1111111111111111111111".to_string());

    println!("  {} {}", "RPC:".bright_green(), rpc_url);
    println!("  {} {}", "Program:".bright_green(), program_id);
    println!("  {} Watching for events...\n", ">>".bright_cyan());

    let store = store::InMemoryStore::new();
    let mut watcher = watcher::EventWatcher::new(&rpc_url, &program_id, store);

    match watcher.start().await {
        Ok(()) => println!("{}", "Indexer stopped gracefully".bright_green()),
        Err(e) => eprintln!("{} {}", "Indexer error:".red(), e),
    }
}
