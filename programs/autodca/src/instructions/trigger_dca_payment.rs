use crate::error::*;
use crate::state::{CrankAuthority, DcaMetadata};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

/// This instruction is effectively a withdrawal to the current_authority's token account so they can
/// run the funds through jupiter aggregator. Unfortunately, the jupiter swap cannot be called from an
/// onchain program and so funds must be temporally extracted from the contract and run through typescript
/// code that calls the jupiter sdk sensibly
#[derive(Accounts)]
pub struct TriggerDcaPayment<'info> {
    #[account(
        constraint = payer.key() == crank_authority.current_authority @ AutoDcaError::CurrentCrankAuthorityNotSigner
    )]
    pub payer: Signer<'info>,

    #[account(
        constraint = crank_authority.key() == dca_metadata.crank_authority @ AutoDcaError::InvalidCrankAuthority
    )]
    pub crank_authority: Account<'info, CrankAuthority>,

    pub dca_metadata: Account<'info, DcaMetadata>,

    #[account(
        constraint = from_mint_crank_authority_token_account.owner == payer.key() @ AutoDcaError::CurrentCrankDoesNotOwnTokenAccount,
        constraint = from_mint_crank_authority_token_account.mint == from_mint.key() @ AutoDcaError::IncorrectMint
    )]
    pub from_mint_crank_authority_token_account: Account<'info, TokenAccount>,

    #[account(
        constraint = from_mint_vault_token_account.mint == from_mint.key() @ AutoDcaError::IncorrectMint
    )]
    pub from_mint_vault_token_account: Account<'info, TokenAccount>,

    #[account(
        constraint = from_mint.key() == dca_metadata.from_token_mint @ AutoDcaError::IncorrectMint
    )]
    pub from_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<TriggerDcaPayment>) -> Result<()> {
    // Read DcaMetadata and ensure that payment_schedule is not being violated
    let dca_metadata = &mut ctx.accounts.dca_metadata;

    let clock = Clock::get()?;

    let amount_per_interval: u64 = dca_metadata.amount_per_interval;
    let interval_length: u64 = dca_metadata.interval_length;
    let interval_counter: u16 = dca_metadata.interval_counter;
    let max_intervals: u16 = dca_metadata.max_intervals;
    let created_at: u64 = dca_metadata.created_at;

    let new_current_interval = interval_counter + 1;
    let current_timestamp = clock.unix_timestamp as u64;

    // Do not let the interval surpass what was initially set into the account
    require!(
        new_current_interval <= max_intervals,
        AutoDcaError::CurrentIntervalOutOfBounds
    );

    let min_interval_timestamp = created_at + (new_current_interval as u64 * interval_length);

    // Make sure the payment schedule is not being preemptively triggered by the crank authority
    require!(
        current_timestamp >= min_interval_timestamp,
        AutoDcaError::DcaScheduleInViolation
    );

    // Handle transfer
    let transfer_accounts = Transfer {
        from: ctx.accounts.from_mint_vault_token_account.to_account_info(),
        to: ctx
            .accounts
            .from_mint_crank_authority_token_account
            .to_account_info(),
        authority: dca_metadata.clone().to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
    );

    transfer(cpi_ctx, amount_per_interval)?;

    dca_metadata.interval_counter = new_current_interval;

    Ok(())
}
