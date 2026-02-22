use anchor_lang::prelude::*;
use crate::state::CryptCard;
use crate::errors::CryptError;

#[derive(Accounts)]
#[instruction(card_id: u64)]
pub struct BurnCard<'info> {
    #[account(
        mut,
        close = owner,
        constraint = card.owner == owner.key() @ CryptError::NotCardOwner,
    )]
    pub card: Account<'info, CryptCard>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

/// Permanently burn a Crypt Card.
/// The account is closed and rent is returned to the owner.
pub fn process_burn(ctx: Context<BurnCard>, _card_id: u64) -> Result<()> {
    let card = &ctx.accounts.card;

    emit!(CardBurned {
        mint_id: card.mint_id,
        owner: card.owner,
        tx_hash: card.tx_hash.clone(),
        rarity: card.rarity,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!(
        "CRYPT Card #{} burned — {} returned to the void",
        card.mint_id,
        card.tx_hash
    );

    Ok(())
}

#[event]
pub struct CardBurned {
    pub mint_id: u64,
    pub owner: Pubkey,
    pub tx_hash: String,
    pub rarity: u8,
    pub timestamp: i64,
}
