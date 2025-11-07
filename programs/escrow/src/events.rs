use anchor_lang::prelude::*;

#[event]
pub struct EscrowInitialized{
    pub initializer: Pubkey,
    pub receiver: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub expiry: i64
}

#[event]
pub struct EscrowClaimed{
    pub intializer: Pubkey,
    pub receiver: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

#[event]
pub struct EscrowCanceled{
    pub initializer: Pubkey,
    pub receiver: Pubkey,
    pub mint: Pubkey,
    pub amount: u64
}