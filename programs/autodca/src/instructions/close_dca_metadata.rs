use crate::error::*;
use crate::state::DcaMetadata;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseDcaMetadata<'info> {
    #[account(
        mut,
        constraint = payer.key() == dca_metadata.owner @ AutoDcaError::IncorrectOwner
    )]
    pub payer: Signer<'info>,

    #[account(
        mut,
        close = payer
    )]
    pub dca_metadata: Account<'info, DcaMetadata>,
}

pub fn handler(_ctx: Context<CloseDcaMetadata>) -> Result<()> {
    Ok(())
}
