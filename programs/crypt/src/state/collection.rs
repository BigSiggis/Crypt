use anchor_lang::prelude::*;

/// Global collection state — tracks all minted Crypt Cards.
/// PDA: seeds = [b"collection"]
#[account]
pub struct Collection {
    /// Authority who can update collection metadata
    pub authority: Pubkey,
    /// Total number of cards minted across all wallets
    pub total_minted: u64,
    /// Maximum supply (0 = unlimited)
    pub max_supply: u64,
    /// Collection metadata URI (off-chain JSON)
    pub uri: String,
    /// Minting fee in lamports (0 = free)
    pub mint_fee: u64,
    /// Treasury wallet for collecting fees
    pub treasury: Pubkey,
    /// Whether minting is currently paused
    pub paused: bool,
    /// Timestamp of collection creation
    pub created_at: i64,
    /// PDA bump seed
    pub bump: u8,
}

impl Collection {
    pub const SIZE: usize = 32  // authority
        + 8                     // total_minted
        + 8                     // max_supply
        + (4 + 200)            // uri (String)
        + 8                     // mint_fee
        + 32                    // treasury
        + 1                     // paused
        + 8                     // created_at
        + 1;                    // bump

    pub fn can_mint(&self) -> bool {
        !self.paused && (self.max_supply == 0 || self.total_minted < self.max_supply)
    }
}
