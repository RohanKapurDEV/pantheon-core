use crate::error::*;
use crate::state::CrankAuthority;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetCrankFeeBps<'info> {
    #[account(constraint = payer.key() == crank_authority.current_authority @ AutoDcaError::InvalidCrankAuthority)]
    pub payer: Signer<'info>,

    pub crank_authority: Account<'info, CrankAuthority>,

    /// CHECK: Used as field for CrankAuthority account
    pub crank_treasury: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<SetCrankFeeBps>, fee_bps: u16) -> Result<()> {
    require!(fee_bps <= 10000, AutoDcaError::InvalidFeeBpsParameter);

    let crank_authority = &mut ctx.accounts.crank_authority;

    crank_authority.crank_fee_bps = fee_bps;

    Ok(())
}
