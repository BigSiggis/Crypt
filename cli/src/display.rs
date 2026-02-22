use colored::Colorize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Display collection statistics from on-chain data.
pub async fn show_stats(rpc_url: &str) {
    println!("{}", "  CRYPT COLLECTION STATS".bright_green());
    println!("  ────────────────────────────────");

    let client = RpcClient::new(rpc_url.to_string());

    // Try to find collection PDA
    let program_id = match Pubkey::from_str("CRYPTxGraveyardSo1ana1111111111111111111111") {
        Ok(p) => p,
        Err(_) => {
            eprintln!("  {} Invalid program ID", "ERROR".red());
            return;
        }
    };

    let (collection_pda, _bump) = Pubkey::find_program_address(
        &[b"collection"],
        &program_id,
    );

    println!("  Program:    {}", program_id.to_string().bright_cyan());
    println!("  Collection: {}", collection_pda.to_string().bright_cyan());
    println!("  Network:    {}", rpc_url.bright_yellow());

    match client.get_account(&collection_pda) {
        Ok(account) => {
            println!("  Status:     {}", "ACTIVE".bright_green());
            println!("  Data size:  {} bytes", account.data.len());

            // Parse collection data
            if account.data.len() > 48 {
                let total_minted = u64::from_le_bytes(
                    account.data[40..48].try_into().unwrap_or([0; 8])
                );
                println!("  Minted:     {}", total_minted.to_string().bright_magenta().bold());
            }
        }
        Err(_) => {
            println!("  Status:     {} (not yet deployed)", "INACTIVE".yellow());
            println!("\n  The collection hasn't been initialized yet.");
            println!("  Run the initialize instruction to set up the collection.");
        }
    }

    println!("\n  Rarity Distribution:");
    println!("    {} Common    (score 0-39)", "■".white());
    println!("    {} Rare      (score 40-74)", "■".bright_cyan());
    println!("    {} Legendary (score 75+)", "■".bright_magenta());
}
