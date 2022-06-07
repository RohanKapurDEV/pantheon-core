use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::state::{CrankAuthority, DcaMetadata};

#[derive(Accounts)]
pub struct WithdrawTokenFromMetadata<'info> {
    pub payer: Signer<'info>,

    pub dca_metadata: Account<'info, DcaMetadata>,

    pub crank_authority: Account<'info, CrankAuthority>,

    pub from_mint_user_token_account: Account<'info, TokenAccount>,

    pub to_mint_user_token_account: Account<'info, TokenAccount>,

    pub from_mint_vault_token_account: Account<'info, TokenAccount>,

    pub to_mint_vault_token_account: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<WithdrawTokenFromMetadata>, from_token: bool) -> Result<()> {
    Ok(())
}
