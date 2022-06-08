use anchor_lang::prelude::*;

/// This instruction is effectively a withdrawal to the current_authority's token account so they can
/// run the funds through jupiter aggregator. Unfortunately, the jupiter swap cannot be called from an
/// onchain program and so funds must be temporally extracted from the contract and run through typescript
/// code that calls the jupiter sdk sensibly
#[derive(Accounts)]
pub struct TriggerDcaPayment<'info> {
    pub payer: Signer<'info>,
}

pub fn handler(ctx: Context<TriggerDcaPayment>) -> Result<()> {
    Ok(())
}
