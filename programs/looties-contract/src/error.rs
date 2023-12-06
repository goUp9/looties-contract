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
    /// Invalid Admin
    #[msg("Invalid Admin")]
    InvalidAdmin,
    /// Exceed Max Reward
    #[msg("Exceed Max Reward")]
    ExceedMaxReward,
    /// Chance Sum Invalid
    #[msg("Chance sum must be 10000")]
    ChanceSumInvalid,
}
