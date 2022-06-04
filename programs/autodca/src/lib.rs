use anchor_lang::prelude::*;

mod error;
mod instructions;
mod state;
mod utils;

use error::*;
use instructions::*;
use state::*;
use utils::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod autodca {
    use super::*;

    pub fn initialize_crank_authority(
        ctx: Context<InitializeCrankAuthority>,
        fee_bps: u16,
    ) -> Result<()> {
        instructions::initialize_crank_authority::handler(ctx, fee_bps)
    }

    pub fn transfer_crank_authority(ctx: Context<TransferCrankAuthority>) -> Result<()> {
        instructions::transfer_crank_authority::handler(ctx)
    }

    pub fn accept_crank_authority(ctx: Context<AcceptCrankAuthority>) -> Result<()> {
        instructions::accept_crank_authority::handler(ctx)
    }

    pub fn set_crank_treasury(ctx: Context<SetCrankTreasury>) -> Result<()> {
        instructions::set_crank_treasury::handler(ctx)
    }

    pub fn set_crank_fee_bps(ctx: Context<SetCrankFeeBps>, fee_bps: u16) -> Result<()> {
        instructions::set_crank_fee_bps::handler(ctx, fee_bps)
    }

    pub fn initialize_dca_metadata(ctx: Context<InitializeDcaMetadata>) -> Result<()> {
        todo!()
    }

    pub fn trigger_dca_payment(ctx: Context<TriggerDcaPayment>) -> Result<()> {
        todo!()
    }

    pub fn close_dca_metadata(ctx: Context<CloseDcaMetadata>) -> Result<()> {
        todo!()
    }
}

/// This instruction is effectively a withdrawal to the current_authority's token account so they can
/// run the funds through jupiter aggregator. Unfortunately, the jupiter swap cannot be called from an
/// onchain program and so funds must be temporally extracted from the contract and run through typescript
/// code that calls the jupiter sdk sensibly
#[derive(Accounts)]
pub struct TriggerDcaPayment<'info> {
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseDcaMetadata<'info> {
    pub payer: Signer<'info>,
}
