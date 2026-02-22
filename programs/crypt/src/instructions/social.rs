use anchor_lang::prelude::*;
use crate::state::{CryptCard, Interaction, InteractionType};
use crate::errors::CryptError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InteractArgs {
    pub interaction_type: u8,
    pub comment_hash: Option<[u8; 32]>,
}

#[derive(Accounts)]
pub struct Interact<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Interaction::SIZE,
        seeds = [
            b"interaction",
            card.key().as_ref(),
            user.key().as_ref(),
        ],
        bump,
    )]
    pub interaction: Account<'info, Interaction>,

    #[account(mut)]
    pub card: Account<'info, CryptCard>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Record an on-chain social interaction with a Crypt Card.
/// Integrates with Tapestry Protocol for social graph data.
pub fn process_interact(ctx: Context<Interact>, args: InteractArgs) -> Result<()> {
    let interaction_type = InteractionType::from_u8(args.interaction_type)
        .ok_or(CryptError::InvalidInteractionType)?;

    let interaction = &mut ctx.accounts.interaction;
    interaction.card = ctx.accounts.card.key();
    interaction.user = ctx.accounts.user.key();
    interaction.interaction_type = args.interaction_type;
    interaction.comment_hash = args.comment_hash.unwrap_or([0u8; 32]);
    interaction.created_at = Clock::get()?.unix_timestamp;
    interaction.bump = ctx.bumps.interaction;

    // Increment card's interaction counter
    let card = &mut ctx.accounts.card;
    card.interaction_count = card.interaction_count.saturating_add(1);

    emit!(CardInteraction {
        card_mint_id: card.mint_id,
        user: interaction.user,
        interaction_type: interaction.interaction_type,
        timestamp: interaction.created_at,
    });

    msg!(
        "Interaction on Card #{}: {:?} by {}",
        card.mint_id,
        match interaction_type {
            InteractionType::Like => "LIKE",
            InteractionType::Comment => "COMMENT",
            InteractionType::Share => "SHARE",
            InteractionType::Bookmark => "BOOKMARK",
        },
        interaction.user
    );

    Ok(())
}

#[event]
pub struct CardInteraction {
    pub card_mint_id: u64,
    pub user: Pubkey,
    pub interaction_type: u8,
    pub timestamp: i64,
}
