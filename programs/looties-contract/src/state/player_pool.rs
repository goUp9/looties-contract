use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
#[derive(InitSpace, Default)]
pub struct PlayerPool {
    pub player: Pubkey, // 32

    pub is_initialized: bool,

    // opened game data
    #[max_len(MAX_OPEN_IN_GAME)]
    pub rewards: Vec<GameData>,

    pub claimable_sol: u64,
    pub claimable_token: [u64; MAX_TOKEN_IN_GAME],     // 8 * 20
    #[max_len(MAX_CLAIMABLE_REWAED_IN_GAME)]
    pub claimable_nfts: Vec<Pubkey>,
}

#[derive(InitSpace, Default, AnchorDeserialize, AnchorSerialize, Clone)]
pub struct GameData {
    pub box_addr: Pubkey,
    pub open_times: u16,
    #[max_len(MAX_OPEN_TIME_IN_GAME)]
    pub reward_idxs: Vec<u8>,
}