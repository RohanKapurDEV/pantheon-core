use crate::error::*;
use crate::state::CrankAuthority;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AcceptCrankAuthority<'info> {
    #[account(constraint = payer.key() == crank_authority.pending_authority @ AutoDcaError::InvalidPendingAuthority)]
    pub payer: Signer<'info>,

    pub crank_authority: Account<'info, CrankAuthority>,
}

pub fn handler(ctx: Context<AcceptCrankAuthority>) -> Result<()> {
    let crank_authority = &mut ctx.accounts.crank_authority;

    crank_authority.current_authority = ctx.accounts.payer.key();
    crank_authority.pending_authority = Pubkey::default();

    Ok(())
}
