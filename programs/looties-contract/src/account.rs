use anchor_lang::prelude::*;
use crate::{
    constants::*,
    error::GameError,
};

#[account]
#[derive(InitSpace, Default)]
pub struct GlobalPool {
    pub admin: Pubkey,                          // 32

    #[max_len(MAX_TOKEN_IN_GAME)]
    pub token_address: Vec<Pubkey>,             // 32 * 20
    pub token_count: u64,                       // 8

    // Box list
    #[max_len(MAX_BOX_IN_GAME)]
    pub boxes: Vec<Pubkey>,                     // 4 + 32 * 200
}

impl GlobalPool {
    pub fn add_box(&mut self, box_addr: Pubkey) -> Result<()> {
        require!(self.boxes.iter().all(|&x| x != box_addr), GameError::BoxAlreadyExist);
        require!(self.boxes.len() < MAX_BOX_IN_GAME, GameError::ExceedMaxBox);
        self.boxes.push(box_addr);
        Ok(())
    }

    pub fn remove_box(&mut self, box_addr: Pubkey) {
        self.boxes.retain(|&x| x != box_addr);
    }
}

#[account]
#[derive(InitSpace, Default)]
pub struct BoxPool {
    #[max_len(MAX_NAME_LENGTH)]
    pub name: String,                       // 4 + 25
    #[max_len(MAX_DESCRIPTION_LENGTH)]
    pub description: String,                // 4 + 100
    #[max_len(MAX_IMAGE_URL_LENGTH)]
    pub image_url: String,                  // 4 + 100
    pub price_in_sol: u64,                  // 8

    pub prizes: Pubkey,                     // 32

    #[max_len(MAX_REWARD_IN_BOX)]
    pub rewards: Vec<Reward>,               // 4 + ___ * 20
}

#[account]
#[derive(InitSpace, Default)]
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
    pub token_address: Option<Pubkey>,      // 1 + 32

    pub collection_address: Option<Pubkey>, // 1 + 32
}

#[account]
#[derive(InitSpace, Default)]
pub struct PrizePool {
    #[max_len(MAX_NFT_IN_BOX)]
    pub nfts: Vec<NftInfo>,                 // 4 + 64 * 150
}

impl PrizePool {
    pub fn add_nft(&mut self, collection_address: Pubkey, mint_info: Pubkey) -> Result<()> {
        require!(self.nfts.iter().all(|x| x.mint_info != mint_info), GameError::NFTAlreadyExist);
        require!(self.nfts.len() < MAX_NFT_IN_BOX, GameError::ExceedMaxNFT);
        self.nfts.push(NftInfo::new(collection_address, mint_info));
        Ok(())
    }

    pub fn remove_nft(&mut self, mint_info: Pubkey) {
        self.nfts.retain(|x| x.mint_info != mint_info);
    }

    pub fn find_nft(&mut self, collection_address: Pubkey, exclude_nfts: &Vec<Pubkey>) -> Result<usize> {
        if let Some(index) = self.nfts.iter().position(|x| x.collection_address == collection_address && !exclude_nfts.contains(&x.mint_info)) {
            Ok(index)
        } else {
            err!(GameError::NFTNotFound)
        }
    }
}

#[account]
#[derive(InitSpace, Default)]
pub struct NftInfo {
    pub collection_address: Pubkey,         // 32
    pub mint_info: Pubkey,                  // 32
}

impl NftInfo {
    pub fn new(collection_address: Pubkey, mint_info: Pubkey) -> Self {
        Self {
            collection_address,
            mint_info,
        }
    }
}