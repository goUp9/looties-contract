use anchor_lang::prelude::*;
use crate::{
    constants::*,
    error::GameError,
};

#[account]
#[derive(InitSpace, Default)]
pub struct GlobalPool {
    pub super_admin: Pubkey,                    // 32

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