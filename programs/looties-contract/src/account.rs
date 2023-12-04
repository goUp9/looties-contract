use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EscrowAccount {
    // 32
    pub initializer_key: Pubkey,
    // 32
    pub initializer_receive_token_account: Pubkey,
    // 4 + 50
    #[max_len(50)]
    pub name: String,
    // 4 + 200
    #[max_len(200)]
    pub description: String,
    // 4
    pub price: u32,
    // 4 + 200
    #[max_len(200)]
    pub image_url: String,
    // 4 + 20 * 
    #[max_len(10)]
    pub rewards: Vec<Reward>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct Reward {
    // 4 + 50
    #[max_len(50)]
    pub name: String,
    // 4 + 200
    #[max_len(200)]
    pub description: String,
    // 4 + 200
    #[max_len(200)]
    pub image_url: String,
    // 1
    pub reward_type: u8, // 0: NFT, 1: SOL, 2: SPL.
    // 32
    pub key: Pubkey,
    // 4
    pub chance: u32, // ${%} * 100_000
    // 4
    pub price: u32,
    // 4 + 20 * 32
    #[max_len(10)]
    pub prizes: Vec<Pubkey>,
}