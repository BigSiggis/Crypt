use anchor_lang::prelude::*;
use crate::state::CryptCard;
use crate::utils::compute_soul_seed;

#[derive(Accounts)]
pub struct VerifyCard<'info> {
    pub card: Account<'info, CryptCard>,
}

/// Verify that a card's soul signature seed matches its transaction hash.
/// Returns true if the on-chain soul_seed matches the deterministic computation.
/// Used by frontends to validate card authenticity and detect tampering.
pub fn process_verify(ctx: Context<VerifyCard>, tx_hash: String) -> Result<bool> {
    let card = &ctx.accounts.card;

    // Verify tx_hash matches
    if card.tx_hash != tx_hash {
        msg!("Verification FAILED: tx_hash mismatch");
        return Ok(false);
    }

    // Verify soul_seed is deterministically correct
    let expected_seed = compute_soul_seed(&tx_hash);
    if card.soul_seed != expected_seed {
        msg!("Verification FAILED: soul_seed mismatch");
        return Ok(false);
    }

    msg!(
        "CRYPT Card #{} verified — soul signature authentic",
        card.mint_id
    );

    emit!(CardVerified {
        mint_id: card.mint_id,
        tx_hash: card.tx_hash.clone(),
        verified: true,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(true)
}

#[event]
pub struct CardVerified {
    pub mint_id: u64,
    pub tx_hash: String,
    pub verified: bool,
    pub timestamp: i64,
}
