use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod autodca {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCrankAuthority>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCrankAuthority<'info> {
    pub signer: Signer<'info>
}

#[derive(Accounts)]
pub struct TransferCrankAuthority<'info> {
    pub signer: Signer<'info>
}

#[derive(Accounts)]
pub struct AcceptCrankAuthority<'info> {
    pub signer: Signer<'info>
}

#[derive(Accounts)]
pub struct SetCrankTreasury<'info> {
    pub signer: Signer<'info>
}

#[derive(Accounts)]
pub struct InitializeDcaMetadata<'info> {
    pub signer: Signer<'info>
}

#[derive(Accounts)]
pub struct TriggerDcaPayment<'info> {
    pub signer: Signer<'info>
}

#[account]
pub struct CrankAuthority {
    pub current_authority: Pubkey,
    pub pending_authority: Pubkey,
    pub crank_treasury: Pubkey
}

#[account]
pub struct DcaMetadata {
    pub owner: Pubkey,
    pub from_token_mint: Pubkey,
    pub to_token_mint: Pubkey,
    pub owner_from_token_account: Pubkey,
    pub owner_to_token_account: Pubkey,
    pub amount_delegated: u64,
    pub interval: i64,
    pub crank_authority: Pubkey,
    pub invocations: u16,
    pub created_at: i64,
}