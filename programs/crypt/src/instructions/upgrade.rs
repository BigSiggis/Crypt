use anchor_lang::prelude::*;
use crate::state::{CryptCard, Rarity};
use crate::errors::CryptError;
use crate::utils::verify_upgrade_proof;

#[derive(Accounts)]
#[instruction(card_id: u64)]
pub struct UpgradeRarity<'info> {
    #[account(
        mut,
        constraint = card.owner == owner.key() @ CryptError::NotCardOwner,
    )]
    pub card: Account<'info, CryptCard>,

    pub owner: Signer<'info>,
}

/// Upgrade a card's rarity tier when its underlying transaction's
/// significance has increased (e.g., a held token mooned).
/// 
/// Requires a proof hash that validates the upgrade eligibility.
/// In production, this would verify against an oracle or
/// Helius webhook data.
pub fn process_upgrade(
    ctx: Context<UpgradeRarity>,
    _card_id: u64,
    new_rarity: u8,
    proof: [u8; 32],
) -> Result<()> {
    let card = &mut ctx.accounts.card;

    let current = Rarity::from_u8(card.rarity)
        .ok_or(CryptError::InvalidRarity)?;
    let target = Rarity::from_u8(new_rarity)
        .ok_or(CryptError::InvalidRarity)?;

    // Cannot downgrade
    require!(current.can_upgrade_to(&target), CryptError::CannotDowngrade);

    // Verify the upgrade proof
    require!(
        verify_upgrade_proof(&card.tx_hash, &card.soul_seed, new_rarity, &proof),
        CryptError::InvalidUpgradeProof
    );

    let old_rarity = card.rarity;
    card.rarity = new_rarity;

    emit!(RarityUpgraded {
        mint_id: card.mint_id,
        owner: card.owner,
        old_rarity,
        new_rarity,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!(
        "CRYPT Card #{} upgraded: {} → {}",
        card.mint_id,
        current.as_str(),
        target.as_str()
    );

    Ok(())
}

#[event]
pub struct RarityUpgraded {
    pub mint_id: u64,
    pub owner: Pubkey,
    pub old_rarity: u8,
    pub new_rarity: u8,
    pub timestamp: i64,
}
