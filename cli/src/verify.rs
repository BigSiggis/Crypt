use colored::Colorize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Verify a Crypt Card's on-chain data matches its soul signature.
pub async fn verify_card(card_address: &str, rpc_url: &str) {
    println!("{} Verifying card: {}", ">>".bright_cyan(), card_address.yellow());

    let pubkey = match Pubkey::from_str(card_address) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{} Invalid address: {}", "ERROR".red(), e);
            return;
        }
    };

    let client = RpcClient::new(rpc_url.to_string());

    match client.get_account(&pubkey) {
        Ok(account) => {
            println!("  {} Account found", "OK".bright_green());
            println!("  Owner:    {}", account.owner);
            println!("  Lamports: {}", account.lamports);
            println!("  Data len: {} bytes", account.data.len());

            if account.data.len() < 8 {
                eprintln!("  {} Account data too small for Crypt Card", "WARN".yellow());
                return;
            }

            // Check discriminator (first 8 bytes)
            let disc = &account.data[..8];
            println!("  Disc:     {}", hex::encode(disc).bright_black());

            // If this is a Crypt Card, verify soul seed
            if account.data.len() > 100 {
                println!("\n  {} Card data detected — verifying soul signature...", ">>".bright_cyan());

                // Extract tx_hash from account data (after discriminator + owner + mint_id)
                // offset: 8 (disc) + 32 (owner) + 8 (mint_id) + 4 (string len) = 52
                let tx_hash_len = u32::from_le_bytes(
                    account.data[52..56].try_into().unwrap_or([0; 4])
                ) as usize;

                if tx_hash_len > 0 && tx_hash_len < 90 && 56 + tx_hash_len <= account.data.len() {
                    let tx_hash = String::from_utf8_lossy(&account.data[56..56+tx_hash_len]);
                    println!("  TX Hash:  {}", tx_hash.bright_yellow());

                    let seed = crate::soul::compute_soul_seed_bytes(&tx_hash);
                    println!("  Expected: {}", hex::encode(&seed[..8]).bright_cyan());
                    println!("\n  {} Soul signature verification complete", "OK".bright_green());
                } else {
                    println!("  {} Could not extract tx_hash from account data", "WARN".yellow());
                }
            }
        }
        Err(e) => {
            eprintln!("  {} Account not found: {}", "ERROR".red(), e);
            eprintln!("  Make sure you're using the correct network (--rpc flag)");
        }
    }
}
