use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer as TokenTransfer},
};

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
    seeds: [b"initializer_vault", escrow.key().as_ref()],
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

    pub rent: Sysvar<'info, Rent>,
}

//Changes

impl<'info> InitializeEscrow<'info> {
    pub fn InitializeEscrow(
        ctx: Context<InitializeEscrow>,
        initializer_amount: u64,
        receiver_amount: u64,
        receiver: pubkey,
        expiry: i16,
        fee_basis_result: u16,
        fee_collector: pubkey,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        let initializer = &ctx.accounts.initializer;
        escrow.initializer = initializer.key;
        escrow.receiver = receiver;
        escrow.initializer_mint = ctx.accounts.initializer_mint.key();
        escrow.initializer_amount = initializer_amount;
        escrow.receiver_mint = ctx.accounts.receiver_mint.key();
        escrow.receiver_amount = receiver_amount;
        escrow.fee_basis_result = fee_basis_result;
        escrow.fee_collector = fee_collector;
        escrow.expiry = expiry;

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.initializer_token_account.to_account_info(),
                to: ctx.accounts.initializer_vault.to_account_info(),
                authority: initializer.to_account_info(),
            },
        );

        transfer(cpi_ctx, initializer_amount)?;

        emit_cpi!(EscrowInitialized {
            initializer: initializer.key();
            receiver: receiver,
            mint: ctx.accounts.initializer_mint.key(),
            amount: initializer_amount,
            expiry,
        });
        OK(())
    }
}
