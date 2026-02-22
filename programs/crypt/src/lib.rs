use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod utils;
pub mod analytics;

use instructions::*;

declare_id!("CRYPTxGraveyardSo1ana1111111111111111111111");

#[program]
pub mod crypt {
    use super::*;

    /// Initialize the Crypt collection — sets authority, merkle tree config,
    /// and prepares the on-chain registry for compressed NFT minting.
    pub fn initialize_collection(
        ctx: Context<InitializeCollection>,
        args: InitCollectionArgs,
    ) -> Result<()> {
        instructions::collection::initialize(ctx, args)
    }

    /// Mint a Crypt Card as a compressed NFT.
    /// Each card is derived from a real Solana transaction — the tx hash,
    /// rarity score, card type, narration, and soul signature seed are
    /// stored on-chain as the card's permanent identity.
    pub fn mint_card(
        ctx: Context<MintCard>,
        args: MintCardArgs,
    ) -> Result<()> {
        instructions::mint::process_mint(ctx, args)
    }

    /// Batch mint multiple cards from a wallet scan.
    /// Processes up to 8 cards in a single transaction for efficiency.
    pub fn batch_mint(
        ctx: Context<BatchMint>,
        args: Vec<MintCardArgs>,
    ) -> Result<()> {
        instructions::mint::process_batch_mint(ctx, args)
    }

    /// Transfer a Crypt Card to another wallet.
    /// Validates ownership and updates the card's owner field.
    pub fn transfer_card(
        ctx: Context<TransferCard>,
        card_id: u64,
    ) -> Result<()> {
        instructions::transfer::process_transfer(ctx, card_id)
    }

    /// Burn a Crypt Card — permanent destruction.
    /// Emits a burn event and closes the account, returning rent to the owner.
    pub fn burn_card(
        ctx: Context<BurnCard>,
        card_id: u64,
    ) -> Result<()> {
        instructions::burn::process_burn(ctx, card_id)
    }

    /// Update collection metadata (authority only).
    pub fn update_collection(
        ctx: Context<UpdateCollection>,
        args: UpdateCollectionArgs,
    ) -> Result<()> {
        instructions::collection::update(ctx, args)
    }

    /// Verify a card's soul signature matches its on-chain data.
    /// Used by frontends to validate card authenticity.
    pub fn verify_card(
        ctx: Context<VerifyCard>,
        tx_hash: String,
    ) -> Result<bool> {
        instructions::verify::process_verify(ctx, tx_hash)
    }

    /// Record a social interaction (like, comment hash) on a card.
    /// Integrates with Tapestry Protocol for on-chain social graph.
    pub fn interact(
        ctx: Context<Interact>,
        args: InteractArgs,
    ) -> Result<()> {
        instructions::social::process_interact(ctx, args)
    }

    /// Claim a rarity upgrade when a card's underlying transaction
    /// crosses a scoring threshold (e.g., held token moons).
    pub fn upgrade_rarity(
        ctx: Context<UpgradeRarity>,
        card_id: u64,
        new_rarity: u8,
        proof: [u8; 32],
    ) -> Result<()> {
        instructions::upgrade::process_upgrade(ctx, card_id, new_rarity, proof)
    }
}
