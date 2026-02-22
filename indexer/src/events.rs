//! Crypt on-chain event definitions.
//! These mirror the events emitted by the Anchor program.

use serde::{Deserialize, Serialize};

/// Emitted when a new Crypt Card is minted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardMintedEvent {
    pub mint_id: u64,
    pub owner: String,
    pub tx_hash: String,
    pub rarity: u8,
    pub card_type: u8,
    pub title: String,
    pub soul_seed: [u8; 32],
    pub timestamp: i64,
}

/// Emitted when a card is transferred between wallets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardTransferredEvent {
    pub mint_id: u64,
    pub from: String,
    pub to: String,
    pub tx_hash: String,
    pub timestamp: i64,
}

/// Emitted when a card is permanently burned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardBurnedEvent {
    pub mint_id: u64,
    pub owner: String,
    pub tx_hash: String,
    pub rarity: u8,
    pub timestamp: i64,
}

/// Emitted when a card's rarity is upgraded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RarityUpgradedEvent {
    pub mint_id: u64,
    pub owner: String,
    pub old_rarity: u8,
    pub new_rarity: u8,
    pub timestamp: i64,
}

/// Emitted when a user interacts with a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardInteractionEvent {
    pub card_mint_id: u64,
    pub user: String,
    pub interaction_type: u8,
    pub timestamp: i64,
}

/// Parsed event from an on-chain transaction log.
#[derive(Debug, Clone)]
pub enum CryptEvent {
    CardMinted(CardMintedEvent),
    CardTransferred(CardTransferredEvent),
    CardBurned(CardBurnedEvent),
    RarityUpgraded(RarityUpgradedEvent),
    CardInteraction(CardInteractionEvent),
}

impl CryptEvent {
    /// Get the event name for logging.
    pub fn name(&self) -> &'static str {
        match self {
            Self::CardMinted(_) => "CARD_MINTED",
            Self::CardTransferred(_) => "CARD_TRANSFERRED",
            Self::CardBurned(_) => "CARD_BURNED",
            Self::RarityUpgraded(_) => "RARITY_UPGRADED",
            Self::CardInteraction(_) => "CARD_INTERACTION",
        }
    }

    /// Get the timestamp of the event.
    pub fn timestamp(&self) -> i64 {
        match self {
            Self::CardMinted(e) => e.timestamp,
            Self::CardTransferred(e) => e.timestamp,
            Self::CardBurned(e) => e.timestamp,
            Self::RarityUpgraded(e) => e.timestamp,
            Self::CardInteraction(e) => e.timestamp,
        }
    }
}
