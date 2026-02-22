use anchor_lang::prelude::*;

/// Rarity tiers for Crypt Cards.
/// Scoring is based on transaction value, type, and historical significance.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Rarity {
    Common = 0,     // Score 0-39:  dust trades, small transfers
    Rare = 1,       // Score 40-74: memecoin apes, notable sales, big moves
    Legendary = 2,  // Score 75+:   whale trades, rug survivals, creator moments
}

impl Rarity {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Rarity::Common),
            1 => Some(Rarity::Rare),
            2 => Some(Rarity::Legendary),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Rarity::Common => "COMMON",
            Rarity::Rare => "RARE",
            Rarity::Legendary => "LEGENDARY",
        }
    }

    pub fn can_upgrade_to(&self, target: &Rarity) -> bool {
        (*target as u8) > (*self as u8)
    }
}

/// Card types derived from Solana transaction classification.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum CardType {
    Swap = 0,           // DEX trades (Jupiter, Raydium, Orca)
    Rug = 1,            // Rug pulls, failed tokens, -99% trades
    Mint = 2,           // NFT mints, token creation events
    DiamondHands = 3,   // Long holds through volatility
    BigMove = 4,        // Whale transfers, cold storage moves
}

impl CardType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(CardType::Swap),
            1 => Some(CardType::Rug),
            2 => Some(CardType::Mint),
            3 => Some(CardType::DiamondHands),
            4 => Some(CardType::BigMove),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CardType::Swap => "SWAP",
            CardType::Rug => "RUG",
            CardType::Mint => "MINT",
            CardType::DiamondHands => "DIAMOND_HANDS",
            CardType::BigMove => "BIG_MOVE",
        }
    }
}

/// A Crypt Card — an on-chain NFT representing a moment from wallet history.
/// PDA: seeds = [b"card", tx_hash.as_bytes(), minter.key().as_ref()]
#[account]
pub struct CryptCard {
    /// Current owner of the card
    pub owner: Pubkey,
    /// Sequential mint ID within the collection
    pub mint_id: u64,
    /// The Solana transaction signature this card represents
    pub tx_hash: String,
    /// Rarity tier (0=common, 1=rare, 2=legendary)
    pub rarity: u8,
    /// Transaction type classification
    pub card_type: u8,
    /// Card title (e.g., "420 SOL → BONK")
    pub title: String,
    /// SHA-256 hash of the AI-generated narration text
    pub narration_hash: [u8; 32],
    /// Deterministic seed for Soul Signature art generation
    /// Derived from tx_hash — ensures 1:1 unique art per card
    pub soul_seed: [u8; 32],
    /// DEX or platform where the transaction occurred
    pub platform: String,
    /// Profit/loss string (e.g., "+4,200%", "-99.8%")
    pub pnl: String,
    /// Unix timestamp of original transaction
    pub tx_timestamp: i64,
    /// Unix timestamp of when the card was minted
    pub minted_at: i64,
    /// Number of on-chain likes/interactions
    pub interaction_count: u64,
    /// Audius track ID for the card's soundtrack
    pub soundtrack_id: String,
    /// PDA bump seed
    pub bump: u8,
}

impl CryptCard {
    pub const SIZE: usize = 32  // owner
        + 8                     // mint_id
        + (4 + 88)            // tx_hash
        + 1                     // rarity
        + 1                     // card_type
        + (4 + 100)           // title
        + 32                    // narration_hash
        + 32                    // soul_seed
        + (4 + 32)            // platform
        + (4 + 32)            // pnl
        + 8                     // tx_timestamp
        + 8                     // minted_at
        + 8                     // interaction_count
        + (4 + 32)            // soundtrack_id
        + 1;                    // bump

    pub fn rarity_enum(&self) -> Rarity {
        Rarity::from_u8(self.rarity).unwrap_or(Rarity::Common)
    }

    pub fn card_type_enum(&self) -> CardType {
        CardType::from_u8(self.card_type).unwrap_or(CardType::Swap)
    }
}
