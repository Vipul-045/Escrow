use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError{
    #[msg("The escrow has already expired")]EscrowExpired,

    #[msg("The escrow is still active and canoot be cannot canceled yet")]EscrowStillActive,

    #[msg("Escrow is not expired yet")]EscrowNotExpired,
}