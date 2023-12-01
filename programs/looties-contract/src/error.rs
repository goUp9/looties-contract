// program specific errors
use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    /// Not Rent Exempt
    #[msg("Not Rent Exempt")]
    NotRentExempt,
    /// Not Rent Exempt
    #[msg("Chance sum must be 100000")]
    ChanceSumInvalid,
}
