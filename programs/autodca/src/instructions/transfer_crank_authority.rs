use crate::error::*;
use crate::state::CrankAuthority;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferCrankAuthority<'info> {
    #[account(constraint = payer.key() == crank_authority.current_authority @ AutoDcaError::InvalidCrankAuthority)]
    pub payer: Signer<'info>,

    pub crank_authority: Account<'info, CrankAuthority>,

    /// CHECK: Only used as account field
    pub pending_authority: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<TransferCrankAuthority>) -> Result<()> {
    let crank_authority = &mut ctx.accounts.crank_authority;

    crank_authority.pending_authority = ctx.accounts.pending_authority.key();

    Ok(())
}
