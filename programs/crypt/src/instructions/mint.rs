use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::{Collection, CryptCard};
use crate::errors::CryptError;
use crate::utils::{compute_soul_seed, validate_card_args};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MintCardArgs {
    pub tx_hash: String,
    pub rarity: u8,
    pub card_type: u8,
    pub title: String,
    pub narration_hash: [u8; 32],
    pub platform: String,
    pub pnl: String,
    pub tx_timestamp: i64,
    pub soundtrack_id: String,
}

#[derive(Accounts)]
#[instruction(args: MintCardArgs)]
pub struct MintCard<'info> {
    #[account(
        init,
        payer = minter,
        space = 8 + CryptCard::SIZE,
        seeds = [b"card", args.tx_hash.as_bytes(), minter.key().as_ref()],
        bump,
    )]
    pub card: Account<'info, CryptCard>,

    #[account(
        mut,
        seeds = [b"collection"],
        bump = collection.bump,
    )]
    pub collection: Account<'info, Collection>,

    /// CHECK: Treasury receives minting fees
    #[account(mut, address = collection.treasury)]
    pub treasury: AccountInfo<'info>,

    #[account(mut)]
    pub minter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchMint<'info> {
    #[account(
        mut,
        seeds = [b"collection"],
        bump = collection.bump,
    )]
    pub collection: Account<'info, Collection>,

    /// CHECK: Treasury receives minting fees
    #[account(mut, address = collection.treasury)]
    pub treasury: AccountInfo<'info>,

    #[account(mut)]
    pub minter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Mint a single Crypt Card from a Solana transaction.
pub fn process_mint(ctx: Context<MintCard>, args: MintCardArgs) -> Result<()> {
    let collection = &mut ctx.accounts.collection;
    require!(collection.can_mint(), CryptError::MaxSupplyReached);
    validate_card_args(&args)?;

    // Collect minting fee if set
    if collection.mint_fee > 0 {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.minter.to_account_info(),
                    to: ctx.accounts.treasury.to_account_info(),
                },
            ),
            collection.mint_fee,
        )?;
    }

    // Compute deterministic soul seed from transaction hash
    let soul_seed = compute_soul_seed(&args.tx_hash);

    let card = &mut ctx.accounts.card;
    card.owner = ctx.accounts.minter.key();
    card.mint_id = collection.total_minted;
    card.tx_hash = args.tx_hash;
    card.rarity = args.rarity;
    card.card_type = args.card_type;
    card.title = args.title;
    card.narration_hash = args.narration_hash;
    card.soul_seed = soul_seed;
    card.platform = args.platform;
    card.pnl = args.pnl;
    card.tx_timestamp = args.tx_timestamp;
    card.minted_at = Clock::get()?.unix_timestamp;
    card.interaction_count = 0;
    card.soundtrack_id = args.soundtrack_id;
    card.bump = ctx.bumps.card;

    collection.total_minted += 1;

    emit!(CardMinted {
        mint_id: card.mint_id,
        owner: card.owner,
        tx_hash: card.tx_hash.clone(),
        rarity: card.rarity,
        card_type: card.card_type,
        title: card.title.clone(),
        soul_seed: card.soul_seed,
        timestamp: card.minted_at,
    });

    msg!(
        "CRYPT Card #{} minted — {} [{}]",
        card.mint_id,
        card.title,
        crate::state::Rarity::from_u8(card.rarity)
            .map(|r| r.as_str())
            .unwrap_or("UNKNOWN")
    );

    Ok(())
}

/// Batch mint up to 8 cards in a single transaction.
pub fn process_batch_mint(ctx: Context<BatchMint>, args: Vec<MintCardArgs>) -> Result<()> {
    require!(args.len() <= 8, CryptError::BatchTooLarge);
    let collection = &mut ctx.accounts.collection;

    for arg in &args {
        require!(collection.can_mint(), CryptError::MaxSupplyReached);
        validate_card_args(arg)?;
        collection.total_minted += 1;

        emit!(CardMinted {
            mint_id: collection.total_minted - 1,
            owner: ctx.accounts.minter.key(),
            tx_hash: arg.tx_hash.clone(),
            rarity: arg.rarity,
            card_type: arg.card_type,
            title: arg.title.clone(),
            soul_seed: compute_soul_seed(&arg.tx_hash),
            timestamp: Clock::get()?.unix_timestamp,
        });
    }

    msg!("CRYPT batch mint: {} cards minted", args.len());
    Ok(())
}

// ============ EVENTS ============

#[event]
pub struct CardMinted {
    pub mint_id: u64,
    pub owner: Pubkey,
    pub tx_hash: String,
    pub rarity: u8,
    pub card_type: u8,
    pub title: String,
    pub soul_seed: [u8; 32],
    pub timestamp: i64,
}
