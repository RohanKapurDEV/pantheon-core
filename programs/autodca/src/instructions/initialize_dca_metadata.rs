use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeDcaMetadata<'info> {
    pub payer: Signer<'info>,
}

pub fn handler(ctx: Context<InitializeDcaMetadata>) -> Result<()> {
    Ok(())
}
