use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer as TokenTransfer}
}

#[Derive(Accounts)]
pub struct InitializeEscrow<'info> {
#[account(
        init,
        payer: initializer,
        space: 8+32,
)]
pub escrow: Account<'info, Escrow>,

#[account(mut)]
pub intializer: signer<'info>,

#[account(mut)]
pub initializer_token_account: Account<'info, TokenAccount>,

#[account(
    seeds: [b"initializer_vault", escrow.key().as.ref()],
    bump
)]
pub initializer_vault_authority: UncheckedAccount<'info>,

#[account(
    init,
    payer: initializer,
    associated_token::mint: initializer_mint,
    assosiated_toke::authority: initializer_vault_authority
)]
pub initializer_vault: Account<'info, TokenAccount>,

#[account(
    seeds: [b"receiver_vault", escrow.key().as.ref()],
    bump
)]
pub receiver_vault_authority: UncheckedAccount<'info>,

#[account(
    init,
    payer: receiver,
    associated_token::mint: receiver_mint,
    associated_token::athority: receiver_vault_authority
)]
pub receiver_vault: Account<'info, TokenAccount>,

pub initializer_mint: Account<'info, Mint>,

pub receiver_mint: Account<'info, Mint>,

pub token_program: Program<'info, token>,

pub associated_token_program: Program<'info, AssociatedToken>,

pub system_program: Program<'info, System>,

pub rent: Sysvar<'info, Rent>
}