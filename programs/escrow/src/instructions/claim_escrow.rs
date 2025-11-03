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
}