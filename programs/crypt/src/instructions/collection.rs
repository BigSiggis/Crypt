use anchor_lang::prelude::*;
use crate::state::Collection;
use crate::errors::CryptError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitCollectionArgs {
    pub uri: String,
    pub max_supply: u64,
    pub mint_fee: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateCollectionArgs {
    pub uri: Option<String>,
    pub max_supply: Option<u64>,
    pub mint_fee: Option<u64>,
    pub paused: Option<bool>,
    pub treasury: Option<Pubkey>,
}

#[derive(Accounts)]
pub struct InitializeCollection<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Collection::SIZE,
        seeds = [b"collection"],
        bump,
    )]
    pub collection: Account<'info, Collection>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCollection<'info> {
    #[account(
        mut,
        seeds = [b"collection"],
        bump = collection.bump,
        has_one = authority @ CryptError::Unauthorized,
    )]
    pub collection: Account<'info, Collection>,

    pub authority: Signer<'info>,
}

/// Initialize the Crypt collection with metadata and configuration.
pub fn initialize(
    ctx: Context<InitializeCollection>,
    args: InitCollectionArgs,
) -> Result<()> {
    require!(args.uri.len() <= 200, CryptError::UriTooLong);

    let collection = &mut ctx.accounts.collection;
    collection.authority = ctx.accounts.authority.key();
    collection.total_minted = 0;
    collection.max_supply = args.max_supply;
    collection.uri = args.uri;
    collection.mint_fee = args.mint_fee;
    collection.treasury = ctx.accounts.authority.key();
    collection.paused = false;
    collection.created_at = Clock::get()?.unix_timestamp;
    collection.bump = ctx.bumps.collection;

    msg!("CRYPT collection initialized — authority: {}", collection.authority);
    Ok(())
}

/// Update collection configuration (authority only).
pub fn update(
    ctx: Context<UpdateCollection>,
    args: UpdateCollectionArgs,
) -> Result<()> {
    let collection = &mut ctx.accounts.collection;

    if let Some(uri) = args.uri {
        require!(uri.len() <= 200, CryptError::UriTooLong);
        collection.uri = uri;
    }
    if let Some(max_supply) = args.max_supply {
        collection.max_supply = max_supply;
    }
    if let Some(mint_fee) = args.mint_fee {
        collection.mint_fee = mint_fee;
    }
    if let Some(paused) = args.paused {
        collection.paused = paused;
    }
    if let Some(treasury) = args.treasury {
        collection.treasury = treasury;
    }

    msg!("CRYPT collection updated");
    Ok(())
}
