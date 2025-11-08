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
        receiver: Pubkey,
        expiry: i64,
        fee_basis_points: u16,
        fee_collector: Pubkey,
        ) -> Result<()> {
            instructions::initialize_escrow::initialize_escrow(ctx, initializer_amount, receiver_amount, receiver, expiry, fee_basis_points, fee_collector)
    }

    pub fn claim_escrow(ctx: Context<ClaimEscrow>) -> Result<()> {
            instructions::claim_escrow::claim_escrow(ctx)
    }

    pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()>{
            instructions::cancel_escrow::cancel_escrow(ctx)
    }
}
