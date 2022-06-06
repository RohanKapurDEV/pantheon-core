use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WithdrawTokenFromMetadata<'info> {
    pub payer: Signer<'info>,
}

pub fn handler(ctx: Context<WithdrawTokenFromMetadata>) -> Result<()> {
    Ok(())
}
