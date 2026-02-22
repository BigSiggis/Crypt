use anchor_lang::prelude::*;
use crate::state::CryptCard;
use crate::errors::CryptError;

#[derive(Accounts)]
#[instruction(card_id: u64)]
pub struct TransferCard<'info> {
    #[account(
        mut,
        constraint = card.owner == current_owner.key() @ CryptError::NotCardOwner,
    )]
    pub card: Account<'info, CryptCard>,

    pub current_owner: Signer<'info>,

    /// CHECK: Any valid Solana address can receive a card
    pub new_owner: AccountInfo<'info>,
}

/// Transfer a Crypt Card to a new owner.
pub fn process_transfer(ctx: Context<TransferCard>, _card_id: u64) -> Result<()> {
    let card = &mut ctx.accounts.card;
    let new_owner = ctx.accounts.new_owner.key();
    let old_owner = card.owner;

    card.owner = new_owner;

    emit!(CardTransferred {
        mint_id: card.mint_id,
        from: old_owner,
        to: new_owner,
        tx_hash: card.tx_hash.clone(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!(
        "CRYPT Card #{} transferred: {} → {}",
        card.mint_id, old_owner, new_owner
    );

    Ok(())
}

#[event]
pub struct CardTransferred {
    pub mint_id: u64,
    pub from: Pubkey,
    pub to: Pubkey,
    pub tx_hash: String,
    pub timestamp: i64,
}
