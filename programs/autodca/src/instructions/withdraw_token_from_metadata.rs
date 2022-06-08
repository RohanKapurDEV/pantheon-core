use crate::error::*;
use crate::state::{CrankAuthority, DcaMetadata};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct WithdrawTokenFromMetadata<'info> {
    #[account(
        constraint = payer.key() == dca_metadata.owner @ AutoDcaError::IncorrectOwner
    )]
    pub payer: Signer<'info>,

    pub dca_metadata: Account<'info, DcaMetadata>,

    #[account(
        constraint = crank_authority.key() == dca_metadata.crank_authority @ AutoDcaError::InvalidCrankAuthority
    )]
    pub crank_authority: Account<'info, CrankAuthority>,

    #[account(
        constraint = from_mint_user_token_account.key() == dca_metadata.owner_from_token_account @ AutoDcaError::IncorrectFromMintTokenAccount,
        constraint = from_mint_user_token_account.mint == from_mint.key() @ AutoDcaError::IncorrectMint
    )]
    pub from_mint_user_token_account: Account<'info, TokenAccount>,

    #[account(
        constraint = to_mint_user_token_account.key() == dca_metadata.owner_to_token_account @ AutoDcaError::IncorrectToMintTokenAccount,
        constraint = to_mint_user_token_account.mint == to_mint.key() @ AutoDcaError::IncorrectMint
    )]
    pub to_mint_user_token_account: Account<'info, TokenAccount>,

    #[account(
        constraint = from_mint_vault_token_account.key() == dca_metadata.vault_from_token_account @ AutoDcaError::IncorrectFromMintTokenAccount,
        constraint = from_mint_vault_token_account.mint == from_mint.key() @ AutoDcaError::IncorrectMint
    )]
    pub from_mint_vault_token_account: Account<'info, TokenAccount>,

    #[account(
        constraint = to_mint_vault_token_account.key() == dca_metadata.vault_to_token_account @ AutoDcaError::IncorrectToMintTokenAccount,
        constraint = to_mint_vault_token_account.mint == to_mint.key() @ AutoDcaError::IncorrectMint
    )]
    pub to_mint_vault_token_account: Account<'info, TokenAccount>,

    #[account(
        constraint = from_mint.key() == dca_metadata.from_token_mint @ AutoDcaError::IncorrectMint
    )]
    pub from_mint: Account<'info, Mint>,

    #[account(
        constraint = to_mint.key() == dca_metadata.to_token_mint @ AutoDcaError::IncorrectMint
    )]
    pub to_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawTokenFromMetadata<'info> {
    fn accounts(
        ctx: &Context<WithdrawTokenFromMetadata>,
        from_token: bool,
        amount: u64,
    ) -> Result<()> {
        if from_token {
            require!(
                ctx.accounts.from_mint_vault_token_account.amount >= amount,
                AutoDcaError::InsufficientBalanceInSelectedTokenAccount
            )
        } else {
            require!(
                ctx.accounts.to_mint_vault_token_account.amount >= amount,
                AutoDcaError::InsufficientBalanceInSelectedTokenAccount
            )
        }

        Ok(())
    }
}

#[access_control(WithdrawTokenFromMetadata::accounts(&ctx, from_token, amount))]
pub fn handler(
    ctx: Context<WithdrawTokenFromMetadata>,
    from_token: bool,
    amount: u64,
) -> Result<()> {
    if from_token {
        let transfer_accounts = Transfer {
            from: ctx.accounts.from_mint_vault_token_account.to_account_info(),
            to: ctx.accounts.from_mint_user_token_account.to_account_info(),
            authority: ctx.accounts.dca_metadata.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
        );

        transfer(cpi_ctx, amount)?;
    } else {
        let transfer_accounts = Transfer {
            from: ctx.accounts.to_mint_vault_token_account.to_account_info(),
            to: ctx.accounts.to_mint_user_token_account.to_account_info(),
            authority: ctx.accounts.dca_metadata.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
        );

        transfer(cpi_ctx, amount)?;
    }

    Ok(())
}
