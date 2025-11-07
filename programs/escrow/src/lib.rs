use anchor_lang::prelude::*;

declare_id!("2VQ3PfDrZp2fgCFCTTRu8VV3X5x7CSV5QhYyPrGHzFyn");

pub mod instructions;
pub mod states;
pub mod events;
pub mod errors;

pub use instructions::*;

#[program]
pub mod escrow {

    use super::*;

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        initializer_amount: u64,
        receiver_amount: u64,
        expiry: i64,
        receiver: Pubkey,
        fee_basis_points: u16,
        fee_collector: Pubkey,
        )-> Result<()> {
            initialize_escrow(ctx, initializer_amount, receiver_amount, expiry, receiver, fee_basis_points, fee_collector)
        
    }
}

#[derive(Accounts)]
pub struct Initialize {}
