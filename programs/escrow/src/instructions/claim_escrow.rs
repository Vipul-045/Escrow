use anchor_lang::prealude::*;

use anchor_spl::token::{tranfer, Mint, TokenAccount, Transfer as TokenTransfer};

#[event_cpi]
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

    #[accounts(mut)]
    pub initializer: SystemAccount<'info>,

    #[accounts(mut)]
    pub receiver: Signer<'info>,

    #[account(
        seeds: [b"initializer_vault", escrow.key().as_ref()],
        bump
    )]
    pub initializer_vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token:: mint = initializer_mint,
        associated_token::authority = initializer_vault_authority
    )]
    pub initializer_vault: Account<'info, TokenAccount>,

    #[account(
        seeds: [b"receiver_vault", escrow.key().as_ref()],
        bump
    )]
    pub receiver_vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token:: mint = receiver_mint,
        associated_token:: authority = receiver_vault_authority;
    )]
    pub receiver_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token:: mint = initializer_mint,
        associated_token:: authority = receiver,
    )]
    pub receiver_initializer_token_account: Account<'info, TokenAccount>,

    #[account(
        mut, 
        associated_token:: mint = receiver_mint,
        associated_token:: authority = receiver,
    )]
    pub receiver_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token:: mint = receiver_mint,
        associated_token:: authority = initializer
    )]
    pub initializer_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token:: mint = initializer_mint,
        associated_token:: authority = escrow.fee_collector
    )]
    pub initializer_fee_collector: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token:: mint = reciver_mint,
        associated_token:: authority = escrow.fee_collector
    )]
    pub receiver_fee_collector: Account<'info, TokenAccount>,

    pub initializer_mint: Account<'info, Mint>,

    pub receiver_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>

}

impl<'info> ClaimEscrow<'info> {
    pub fn claim_escrow(cyx: Context<ClaimEscrow>) -> Result<()>{
        let escrow = &ctx.accounts.escrow;
        let clock = Clock::get()?;

        require!(clock.unix_timestamp <= escrow.expiry, Escrow.Error::EscrowExpired);

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

        
}