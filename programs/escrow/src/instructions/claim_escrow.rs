use crate::events::EscrowClaimed;
use crate::states::Escrow;
use crate::errors::*;

use anchor_lang::prelude::*;

use anchor_spl::token::{transfer, Token, Mint, TokenAccount, Transfer as TokenTransfer};

#[derive(Accounts)]
pub struct ClaimEscrow <'info>{
    #[account(
        mut,
        has_one = initializer,
        has_one = initializer_mint,
        has_one = receiver,
        has_one = receiver_mint,
        close = initializer
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub initializer: SystemAccount<'info>,

    #[account(mut)]
    pub receiver: Signer<'info>,

    #[account(
        seeds = [b"initializer_vault", escrow.key().as_ref()],
        bump
    )]
    pub initializer_vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = initializer_mint,
        associated_token::authority = initializer_vault_authority
    )]
    pub initializer_vault: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"receiver_vault", escrow.key().as_ref()],
        bump
    )]
    pub receiver_vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = receiver_mint,
        associated_token::authority = receiver_vault_authority
    )]
    pub receiver_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = initializer_mint,
        associated_token::authority = receiver,
    )]
    pub receiver_initializer_token_account: Account<'info, TokenAccount>,

    #[account(
        mut, 
        associated_token::mint = receiver_mint,
        associated_token::authority = receiver,
    )]
    pub receiver_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = receiver_mint,
        associated_token::authority = initializer
    )]
    pub initializer_receiver_token_account: Account<'info, TokenAccount>,

    pub fee_collector: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = initializer_mint,
        associated_token::authority = escrow.fee_collector
    )]
    pub initializer_fee_collector: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = receiver_mint,
        associated_token::authority = escrow.fee_collector
    )]
    pub receiver_fee_collector: Account<'info, TokenAccount>,

    pub initializer_mint: Account<'info, Mint>,

    pub receiver_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>

}


    pub fn claim_escrow(ctx: Context<ClaimEscrow>) -> Result<()>{
        let escrow = & ctx.accounts.escrow;
        let clock = Clock::get()?;

        require!(clock.unix_timestamp <= escrow.expiry, EscrowError::EscrowExpired);

        let escrow_key = escrow.key();

        let initializer_fee = (escrow.initializer_amount as u128)
            .checked_mul(escrow.fee_basis_points as u128)
            .unwrap()
            .checked_div(10000)
            .unwrap() as u64;

        let receiver_fee = (escrow.receiver_amount as u128)
            .checked_mul(escrow.fee_basis_points as u128)
            .unwrap()
            .checked_div(10000)
            .unwrap() as u64;

        let initializer_amount_after_fee = escrow.initializer_amount.checked_sub(initializer_fee).unwrap();
        let receiver_amount_after_fee = escrow.receiver_amount.checked_sub(receiver_fee).unwrap();

        let receiver_deposit_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer{
                from: ctx.accounts.receiver_token_account.to_account_info(),
                to: ctx.accounts.receiver_token_account.to_account_info(),
                authority: ctx.accounts.receiver.to_account_info(),
            },
        );

        transfer(receiver_deposit_ctx, escrow.receiver_amount)?;

        let initializer_vault_bump = ctx.bumps.initializer_vault_authority;
        let initializer_seeds = &[b"initializer_vault", escrow_key.as_ref(), &[initializer_vault_bump]];
        let initializer_signer = &[&initializer_seeds[..]];

        let initializer_to_receiver_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer{
                from: ctx.accounts.initializer_vault.to_account_info(),
                to: ctx.accounts.receiver_initializer_token_account.to_account_info(),
                authority: ctx.accounts.initializer_vault_authority.to_account_info(),
            },
            initializer_signer,
        );
        transfer(initializer_to_receiver_ctx, initializer_amount_after_fee)?;

        if initializer_fee > 0 {
            let initializer_fee_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TokenTransfer{
                    from: ctx.accounts.initializer_vault.to_account_info(),
                    to: ctx.accounts.initializer_fee_collector.to_account_info(),
                    authority: ctx.accounts.initializer_vault_authority.to_account_info(),
                },
                initializer_signer,
            );
            transfer(initializer_fee_ctx, initializer_fee)?;
        }

        let receiver_vault_bump = ctx.bumps.receiver_vault_authority;
        let receiver_seeds = &[b"receiver_vault", escrow_key.as_ref(), &[receiver_vault_bump]];
        let receiver_signer = &[&receiver_seeds[..]];

        let receiver_to_initializer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer{
                from: ctx.accounts.receiver_vault.to_account_info(),
                to: ctx.accounts.initializer_receiver_token_account.to_account_info(),
                authority: ctx.accounts.receiver_vault_authority.to_account_info(),
            },
            receiver_signer,
        );
        transfer(receiver_to_initializer_ctx, receiver_amount_after_fee)?;

        if receiver_fee > 0 {
            let receiver_fee_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TokenTransfer{
                    from: ctx.accounts.receiver_vault.to_account_info(),
                    to: ctx.accounts.receiver_fee_collector.to_account_info(),
                    authority: ctx.accounts.receiver_vault_authority.to_account_info(),
                },
                receiver_signer,
            );
            transfer(receiver_fee_ctx, receiver_fee)?;
        }

        emit!(EscrowClaimed { 
            intializer: escrow.initializer,
            receiver: escrow.receiver,
            mint: escrow.initializer_mint,
            amount: escrow.initializer_amount,
        });
        
        Ok(())
    }