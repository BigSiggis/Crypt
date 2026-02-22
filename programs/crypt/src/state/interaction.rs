use anchor_lang::prelude::*;

/// Types of social interactions on a Crypt Card.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum InteractionType {
    Like = 0,
    Comment = 1,
    Share = 2,
    Bookmark = 3,
}

impl InteractionType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(InteractionType::Like),
            1 => Some(InteractionType::Comment),
            2 => Some(InteractionType::Share),
            3 => Some(InteractionType::Bookmark),
            _ => None,
        }
    }
}

/// On-chain record of a social interaction with a Crypt Card.
/// PDA: seeds = [b"interaction", card.key().as_ref(), user.key().as_ref()]
#[account]
pub struct Interaction {
    /// The card being interacted with
    pub card: Pubkey,
    /// The user performing the interaction
    pub user: Pubkey,
    /// Type of interaction
    pub interaction_type: u8,
    /// Optional comment hash (SHA-256 of comment text)
    pub comment_hash: [u8; 32],
    /// Timestamp of interaction
    pub created_at: i64,
    /// PDA bump
    pub bump: u8,
}

impl Interaction {
    pub const SIZE: usize = 32 + 32 + 1 + 32 + 8 + 1;
}
