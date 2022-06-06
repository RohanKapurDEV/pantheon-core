use crate::error::*;
use crate::state::{CrankAuthority, DcaMetadata};
use crate::utils::get_timestamp;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

/// Word of caution:
///
/// There is an assumption in this code that isn't enforced or checked (because I don't think there exists a way
/// to do it): the assumption is that the from_mint and to_mint token mints are both currently available via the
/// Jupiter aggregator. If a mint is passed in that isn't supported in Jupiter, there is no way for the contract
/// to know that and it won't throw any errors. At this point, every attempt to DCA using that mint will basically
/// just fail. It is what it is, fellas.
#[derive(Accounts)]
pub struct InitializeDcaMetadata<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub crank_authority: Account<'info, CrankAuthority>,

    #[account(
        init,
        payer = payer,
        space = 8 + DcaMetadata::SIZE
    )]
    pub dca_metadata: Account<'info, DcaMetadata>,

    pub from_mint: Account<'info, Mint>,

    pub to_mint: Account<'info, Mint>,

    pub from_mint_user_token_account: Account<'info, TokenAccount>, // safe to assume this exists, no need to init

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = to_mint,
        associated_token::authority = payer,
        constraint = to_mint_user_token_account.mint == to_mint.key() @ AutoDcaError::IncorrectMint
    )]
    pub to_mint_user_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        token::mint = from_mint,
        token::authority = dca_metadata,
        seeds = [b"vault", from_mint.key().as_ref(), payer.key().as_ref()],
        bump,
        constraint = from_mint_vault_token_account.mint == from_mint.key() @ AutoDcaError::IncorrectMint
    )]
    pub from_mint_vault_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        token::mint = to_mint,
        token::authority = dca_metadata,
        seeds = [b"vault", to_mint.key().as_ref(), payer.key().as_ref()],
        bump,
        constraint = to_mint_vault_token_account.mint == to_mint.key() @ AutoDcaError::IncorrectMint
    )]
    pub to_mint_vault_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'a> InitializeDcaMetadata<'a> {
    fn accounts(ctx: &Context<InitializeDcaMetadata>) -> Result<()> {
        require!(
            ctx.accounts.from_mint.key() != ctx.accounts.to_mint.key(),
            AutoDcaError::TokenMintsCannotBeTheSame
        );
        Ok(())
    }
}

#[access_control(InitializeDcaMetadata::accounts(&ctx))]
pub fn handler(
    ctx: Context<InitializeDcaMetadata>,
    amount_per_interval: u64,
    interval_length: u64,
    interval_counter: u16,
    max_intervals: u16,
) -> Result<()> {
    let timestamp = get_timestamp();

    let dca_metadata = &mut ctx.accounts.dca_metadata;

    dca_metadata.owner = ctx.accounts.payer.key();
    dca_metadata.from_token_mint = ctx.accounts.from_mint.key();
    dca_metadata.to_token_mint = ctx.accounts.to_mint.key();
    dca_metadata.owner_from_token_account = ctx.accounts.from_mint_user_token_account.key();
    dca_metadata.owner_to_token_account = ctx.accounts.to_mint_user_token_account.key();
    dca_metadata.vault_from_token_account = ctx.accounts.from_mint_vault_token_account.key();
    dca_metadata.vault_to_token_account = ctx.accounts.to_mint_vault_token_account.key();
    dca_metadata.amount_per_interval = amount_per_interval;
    dca_metadata.interval_length = interval_length;
    dca_metadata.interval_counter = interval_counter;
    dca_metadata.max_intervals = max_intervals;
    dca_metadata.crank_authority = ctx.accounts.crank_authority.key();
    dca_metadata.created_at = timestamp;

    Ok(())
}
