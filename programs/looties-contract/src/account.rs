use anchor_lang::prelude::*;

macro_rules! size {
    ($name: ident, $size:expr) => {
        impl $name {
            pub const LEN: usize = $size;
        }
    };
}

#[account]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub name: String,
    pub description: String,
    pub price: u32,
    pub image_url: String,
    pub rewards: Vec<Reward>,
}
size!(EscrowAccount, 9000);

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct Reward {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub reward_type: u8, // 0: NFT, 1: SOL, 2: SPL.
    pub key: Pubkey,
    pub chance: u32, // ${%} * 100_000
    pub price: u32,
    pub prizes: Vec<Pubkey>,
}