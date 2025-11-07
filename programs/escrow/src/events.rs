use anchor_lang::prelude::*;

#[event]
pub struct EscrowInitialized{
    pub initializer: pubkey,
    pub receiver: pubkey,
    pub mint: pubkey,
    pub amount: u64,
    pub expiry: u64
}

#[event]
pub struct EscrowClaimed{
    pub intializer: pubkey,
    pub receiver: pubkey,
    pub mint: pubkey,
    pub amount: u64,
}

#[event]
pub struct EscrowCanceled{
    pub initializer: pubkey,
    pub receiver: pubkey,
    pub mint: pubkey,
    pub amount: u64
}