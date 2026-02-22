//! Crypt Program — Integration Test Suite
//! 
//! Tests the full lifecycle of Crypt Cards on Solana:
//! - Collection initialization and configuration
//! - Single and batch card minting
//! - Card transfers between wallets
//! - Card burning and account closure
//! - Rarity scoring and upgrades
//! - Soul signature verification
//! - Social interactions (likes, comments)
//! - Edge cases and error handling

mod test_collection;
mod test_mint;
mod test_transfer;
mod test_burn;
mod test_scoring;
mod test_soul;
mod test_social;
mod test_verify;
mod test_upgrade;
mod helpers;
