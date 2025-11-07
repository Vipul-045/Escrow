use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub initializer: Pubkey,
    pub receiver: Pubkey,

    pub initializer_mint: Pubkey,
    pub initializer_amount: u64,

    pub receiver_mint: Pubkey,
    pub receiver_amount: u64,

    pub fee_basis_points: u16,
    pub fee_collector: Pubkey,

    pub expiry: i64,
}