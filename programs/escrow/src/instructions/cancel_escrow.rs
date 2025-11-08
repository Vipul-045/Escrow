use anchor_lang::prelude::*;

use crate::events::EscrowCanceled;
use crate::states::Escrow;
use crate::errors::*;

use anchor_spl::token::{
    TokenAccount,
    Token,
    Mint,
    Transfer as TokenTransfer,
    transfer,
};

#[derive(Accounts)]
pub struct CancelEscrow <'info> {
    #[account(
        mut, 
        has_one = initializer,
        has_one = initializer_mint,
        close = initializer,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        seeds = [b"initializer_vault", escrow.key().as_ref()],
        bump
    )]
    pub initializer_vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = initializer_mint,
        associated_token::authority = initializer_vault_authority,
    )]
    pub initializer_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = initializer_mint,
        associated_token::authority = initializer,
    )]
    pub initializer_token_account: Account<'info, TokenAccount>,

    pub initializer_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>
}

    pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;

        let clock = Clock::get()?;
        require!(clock.unix_timestamp > escrow.expiry , EscrowError::EscrowNotExpired);

        let escrow_key = escrow.key();
        let initializer_vault_bump = ctx.bumps.initializer_vault_authority;
        let seeds = &[b"initializer_vault", escrow_key.as_ref(), &[initializer_vault_bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = TokenTransfer {
            from: ctx.accounts.initializer_vault.to_account_info(),
            to: ctx.accounts.initializer_token_account.to_account_info(),
            authority: ctx.accounts.initializer_vault_authority.to_account_info(),
        };

        let cpi_ctx = CpiContext:: new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );

        transfer(cpi_ctx, escrow.initializer_amount)?;

        emit!(EscrowCanceled{
            initializer: escrow.initializer,
            receiver: escrow.receiver,
            mint: escrow.initializer_mint,
            amount: escrow.initializer_amount,
        });
        Ok(())
    }