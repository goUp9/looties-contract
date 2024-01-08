use anchor_lang::prelude::*;
use crate::{
    constants::*,
    error::GameError,
};

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

#[derive(InitSpace, Default, AnchorDeserialize, AnchorSerialize, Clone)]
pub struct NftInfo {
    pub collection_address: Pubkey,         // 32
    pub mint_info: Pubkey,                  // 32
    pub rewarded: bool,
}

impl NftInfo {
    pub fn new(collection_address: Pubkey, mint_info: Pubkey) -> Self {
        Self {
            collection_address,
            mint_info,
            rewarded: false,
        }
    }
}
