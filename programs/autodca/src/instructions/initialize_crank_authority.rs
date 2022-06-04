use crate::error::*;
use crate::state::CrankAuthority;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeCrankAuthority<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + CrankAuthority::SIZE
    )]
    pub crank_authority: Account<'info, CrankAuthority>,

    /// CHECK: Used as field for CrankAuthority account
    pub current_authority: UncheckedAccount<'info>,

    /// CHECK: Used as field for CrankAuthority account
    pub crank_treasury: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeCrankAuthority>, fee_bps: u16) -> Result<()> {
    require!(fee_bps <= 10000, AutoDcaError::InvalidFeeBpsParameter);

    let crank_authority = &mut ctx.accounts.crank_authority;

    crank_authority.current_authority = ctx.accounts.current_authority.key();
    crank_authority.pending_authority = Pubkey::default();
    crank_authority.crank_treasury = ctx.accounts.crank_treasury.key();
    crank_authority.crank_fee_bps = fee_bps;

    Ok(())
}
