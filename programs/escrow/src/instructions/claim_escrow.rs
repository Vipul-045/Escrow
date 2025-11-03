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
    pub initializer_vault: Account<'info, TokenAccount>

    


}