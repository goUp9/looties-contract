use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
#[derive(InitSpace, Default)]
pub struct BoxPool {
    pub admin: Pubkey,                              // 32

    pub sol_amount: u64,                            // 8
    pub token_amount: [u64; MAX_TOKEN_IN_GAME],     // 8 * 20

    #[max_len(MAX_NAME_LENGTH)]
    pub name: String,                               // 4 + 25
    #[max_len(MAX_DESCRIPTION_LENGTH)]
    pub description: String,                        // 4 + 100
    #[max_len(MAX_IMAGE_URL_LENGTH)]
    pub image_url: String,                          // 4 + 100
    pub price_in_sol: u64,                          // 8

    pub prizes: Pubkey,                             // 32

    #[max_len(MAX_REWARD_IN_BOX)]
    pub rewards: Vec<Reward>,                       // 4 + ___ * 20
}

#[derive(InitSpace, Default, AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Reward {
    #[max_len(MAX_NAME_LENGTH)]
    pub name: String,                       // 4 + 25
    #[max_len(MAX_DESCRIPTION_LENGTH)]      // 4 + 100
    pub description: String,
    #[max_len(MAX_IMAGE_URL_LENGTH)]
    pub image_url: String,                  // 4 + 100

    // 1: SOL, 2: SPL, 3: NFT.
    pub reward_type: u8,                    // 1
    // {%}*100, 100%=10_000
    pub chance: u16,                        // 2

    pub sol: u64,                           // 8

    pub token: u64,                         // 8
    pub token_address: Pubkey,              // 32

    pub collection_address: Pubkey,         // 32
}