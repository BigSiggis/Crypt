use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// Rarity tiers for Crypt Cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Rare,
    Legendary,
}

impl Rarity {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v { 0 => Some(Self::Common), 1 => Some(Self::Rare), 2 => Some(Self::Legendary), _ => None }
    }

    pub fn as_u8(&self) -> u8 {
        match self { Self::Common => 0, Self::Rare => 1, Self::Legendary => 2 }
    }

    pub fn as_str(&self) -> &'static str {
        match self { Self::Common => "COMMON", Self::Rare => "RARE", Self::Legendary => "LEGENDARY" }
    }

    pub fn color_code(&self) -> &'static str {
        match self { Self::Common => "#888888", Self::Rare => "#00d4b0", Self::Legendary => "#9660ff" }
    }
}

/// Card types derived from transaction classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    Swap,
    Rug,
    Mint,
    DiamondHands,
    BigMove,
}

impl CardType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v { 0 => Some(Self::Swap), 1 => Some(Self::Rug), 2 => Some(Self::Mint), 3 => Some(Self::DiamondHands), 4 => Some(Self::BigMove), _ => None }
    }

    pub fn as_u8(&self) -> u8 {
        match self { Self::Swap => 0, Self::Rug => 1, Self::Mint => 2, Self::DiamondHands => 3, Self::BigMove => 4 }
    }

    pub fn as_str(&self) -> &'static str {
        match self { Self::Swap => "SWAP", Self::Rug => "RUG", Self::Mint => "MINT", Self::DiamondHands => "DIAMOND_HANDS", Self::BigMove => "BIG_MOVE" }
    }
}

/// A Crypt Card with all metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptCard {
    pub owner: Pubkey,
    pub mint_id: u64,
    pub tx_hash: String,
    pub rarity: Rarity,
    pub card_type: CardType,
    pub title: String,
    pub narration_hash: [u8; 32],
    pub soul_seed: [u8; 32],
    pub platform: String,
    pub pnl: String,
    pub tx_timestamp: i64,
    pub minted_at: i64,
    pub interaction_count: u64,
    pub soundtrack_id: String,
}

/// Parameters for minting a new card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintParams {
    pub tx_hash: String,
    pub rarity: Rarity,
    pub card_type: CardType,
    pub title: String,
    pub narration: String,
    pub platform: String,
    pub pnl: String,
    pub tx_timestamp: i64,
    pub soundtrack_id: String,
}

/// Collection statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionStats {
    pub authority: Pubkey,
    pub total_minted: u64,
    pub max_supply: u64,
    pub mint_fee: u64,
    pub paused: bool,
    pub created_at: i64,
}

/// Result of a wallet scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub address: String,
    pub total_transactions: usize,
    pub cards: Vec<CryptCard>,
    pub legendary_count: usize,
    pub rare_count: usize,
    pub common_count: usize,
}
