//! Crypt SDK — Rust client library for the Crypt Solana program.
//!
//! Provides high-level functions for:
//! - Scanning wallets and building Crypt Cards
//! - Minting cards as on-chain accounts
//! - Transferring and burning cards
//! - Verifying soul signature authenticity
//! - Computing rarity scores
//!
//! # Example
//! ```rust,ignore
//! use crypt_sdk::{CryptClient, SoulSignature};
//!
//! let client = CryptClient::new("https://api.devnet.solana.com");
//! let seed = SoulSignature::compute("4xK7m...");
//! println!("Soul seed: {:?}", seed);
//! ```

pub mod client;
pub mod types;
pub mod soul;
pub mod scoring;
pub mod error;

pub use client::CryptClient;
pub use types::*;
pub use soul::SoulSignature;
pub use scoring::RarityScorer;
pub use error::CryptSdkError;
