// program specific errors
use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    /// Box Already Exist
    #[msg("Box Already Exist")]
    BoxAlreadyExist,
    /// Exceed Max Box
    #[msg("Exceed Max Box")]
    ExceedMaxBox,
    /// Chance Sum Invalid
    #[msg("Chance sum must be 10000")]
    ChanceSumInvalid,
    /// Access Denied
    #[msg("Access Denied")]
    AccessDenied,
    /// Insufficient Funds
    #[msg("Insufficient Funds")]
    InsufficientFunds,
    /// Invalid Reward Type
    #[msg("Invalid Reward Type")]
    InvalidRewardType,
    /// Invalid Reward Data
    #[msg("Invalid Reward Data")]
    InvalidRewardData,
}
