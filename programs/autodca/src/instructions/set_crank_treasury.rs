use crate::error::*;
use crate::state::CrankAuthority;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetCrankTreasury<'info> {
    #[account(constraint = payer.key() == crank_authority.current_authority @ AutoDcaError::InvalidCrankAuthority)]
    pub payer: Signer<'info>,

    pub crank_authority: Account<'info, CrankAuthority>,

    /// CHECK: Used as field for CrankAuthority account
    pub crank_treasury: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<SetCrankTreasury>) -> Result<()> {
    let crank_authority = &mut ctx.accounts.crank_authority;

    crank_authority.crank_treasury = ctx.accounts.crank_treasury.key();

    Ok(())
}
