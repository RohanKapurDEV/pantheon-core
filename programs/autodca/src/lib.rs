use anchor_lang::prelude::*;

mod error;
mod instructions;
mod state;
mod utils;

use instructions::*;

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

    pub fn initialize_dca_metadata(
        ctx: Context<InitializeDcaMetadata>,
        amount_per_interval: u64,
        interval_length: u64,
        interval_counter: u16,
        max_intervals: u16,
    ) -> Result<()> {
        instructions::initialize_dca_metadata::handler(
            ctx,
            amount_per_interval,
            interval_length,
            interval_counter,
            max_intervals,
        )
    }

    pub fn trigger_dca_payment(ctx: Context<TriggerDcaPayment>) -> Result<()> {
        instructions::trigger_dca_payment::handler(ctx)
    }

    pub fn withdraw_token_from_metadata(ctx: Context<WithdrawTokenFromMetadata>) -> Result<()> {
        todo!()
    }

    pub fn close_dca_metadata(ctx: Context<CloseDcaMetadata>) -> Result<()> {
        todo!()
    }
}

#[derive(Accounts)]
pub struct CloseDcaMetadata<'info> {
    pub payer: Signer<'info>,
}
