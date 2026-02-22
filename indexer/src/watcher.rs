//! Blockchain event watcher — polls for new transactions containing Crypt events.

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use std::str::FromStr;
use std::time::Duration;
use crate::processor;
use crate::store::InMemoryStore;
use colored::Colorize;

/// Watches the Solana blockchain for Crypt program events.
pub struct EventWatcher {
    rpc: RpcClient,
    program_id: Pubkey,
    store: InMemoryStore,
    last_signature: Option<String>,
    poll_interval: Duration,
}

impl EventWatcher {
    pub fn new(rpc_url: &str, program_id: &str, store: InMemoryStore) -> Self {
        let pid = Pubkey::from_str(program_id)
            .expect("Invalid program ID");
        Self {
            rpc: RpcClient::new_with_commitment(
                rpc_url.to_string(),
                CommitmentConfig::confirmed(),
            ),
            program_id: pid,
            store,
            last_signature: None,
            poll_interval: Duration::from_secs(2),
        }
    }

    /// Start watching for events. Runs until interrupted.
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("  {} Starting event watcher (poll interval: {:?})", 
            ">>".bright_cyan(), self.poll_interval);

        let mut iteration = 0u64;

        loop {
            iteration += 1;

            // Fetch recent signatures for the program
            match self.rpc.get_signatures_for_address(&self.program_id) {
                Ok(signatures) => {
                    let mut new_count = 0;

                    for sig_info in signatures.iter().rev() {
                        // Skip if we've already processed this signature
                        if let Some(ref last) = self.last_signature {
                            if sig_info.signature == *last { continue; }
                        }

                        // Fetch full transaction
                        let sig = solana_sdk::signature::Signature::from_str(&sig_info.signature)
                            .unwrap_or_default();

                        match self.rpc.get_transaction(
                            &sig,
                            solana_transaction_status::UiTransactionEncoding::Json,
                        ) {
                            Ok(tx) => {
                                // Extract logs
                                if let Some(meta) = tx.transaction.meta {
                                    if let Some(logs) = meta.log_messages {
                                        let opt_logs: Vec<String> = match logs {
                                            solana_transaction_status::option_serializer::OptionSerializer::Some(l) => l,
                                            _ => vec![],
                                        };
                                        let events = processor::parse_program_logs(&opt_logs);
                                        for event in &events {
                                            self.store.process_event(event);
                                            new_count += 1;
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                if iteration % 30 == 0 {
                                    eprintln!("  {} Failed to fetch tx: {}", "WARN".yellow(), e);
                                }
                            }
                        }
                    }

                    // Update last processed signature
                    if let Some(latest) = signatures.first() {
                        self.last_signature = Some(latest.signature.clone());
                    }

                    if new_count > 0 {
                        println!("  {} Processed {} new events", ">>".bright_green(), new_count);
                        self.store.print_stats();
                    }
                }
                Err(e) => {
                    if iteration % 15 == 0 {
                        eprintln!("  {} RPC error: {}", "WARN".yellow(), e);
                    }
                }
            }

            // Print heartbeat every 30 iterations
            if iteration % 30 == 0 {
                println!("  {} Heartbeat — iteration {}, watching...", 
                    "..".bright_black(), iteration);
            }

            tokio::time::sleep(self.poll_interval).await;
        }
    }
}
