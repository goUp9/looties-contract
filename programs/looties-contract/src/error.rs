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
    /// Token Address Already Exist
    #[msg("Token Address Already Exist")]
    TokenAddressAlreadyExist,
    /// Exceed Max Token Address
    #[msg("Exceed Max Token Address")]
    ExceedMaxTokenAddress,
    /// Invalid Super Admin
    #[msg("Invalid Super Admin")]
    InvalidSuperAdmin,
    /// Invalid Admin
    #[msg("Invalid Admin")]
    InvalidAdmin,
    /// Exceed Max Reward
    #[msg("Exceed Max Reward")]
    ExceedMaxReward,
    /// Invalid Reward Type
    #[msg("Invalid Reward Type")]
    RewardTypeInvalid,
    /// Unknown token address
    #[msg("Unknown token address")]
    TokenAddressUnknown,
    /// Chance Sum Invalid
    #[msg("Chance sum must be 10000")]
    ChanceSumInvalid,
    /// Remaining Account Count Dismatch
    #[msg("Remaining Account Count Dismatch")]
    RemainingAccountCountDismatch,
    /// Source ata dismatch
    #[msg("Source ata dismatch")]
    SrcAtaDismatch,
    /// Dest ata dismatch
    #[msg("Dest ata dismatch")]
    DestAtaDismatch,
    /// Argument Invalid
    #[msg("Argument Invalid")]
    ArgumentInvalid,
    /// Collection address doesn't exsit in box
    #[msg("Collection address doesn't exsit in box")]
    CollectionAddressNotExsit,
    /// NFT Already Exist
    #[msg("NFT Already Exist")]
    NFTAlreadyExist,
    /// Exceed Max NFT
    #[msg("Exceed Max NFT")]
    ExceedMaxNFT,
    /// Exceed Max Token
    #[msg("Exceed Max Token")]
    ExceedMaxToken,
    /// Unknown box address
    #[msg("Unknown box address")]
    BoxAddressUnknown,
    /// Invalid Admin Address
    #[msg("Invalid Admin Address")]
    InvalidAdminAddress,
    /// NFT collection is empty
    #[msg("NFT collection is empty")]
    NFTNotFound,
    /// Insufficient Funds
    #[msg("Insufficient Funds")]
    InsufficientFunds,
    /// Open_time cannot exceed 3.
    #[msg("Open_time cannot exceed 3.")]
    OpenTimeExceed,
    /// Sol balance exist in the box
    #[msg("Sol balance exist in the box")]
    SolBalanceExist,
    /// Token balance exist in the box
    #[msg("Token balance exist in the box")]
    TokenBalanceExist,
    /// NFT balance exist in the box
    #[msg("NFT balance exist in the box")]
    NFTBalanceExist,
    /// Invalid Authority
    #[msg("Invalid Authority")]
    InvalidAuthority,
}
