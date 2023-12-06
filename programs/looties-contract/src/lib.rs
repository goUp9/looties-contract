use anchor_lang::prelude::*;

use account::*;
use constants::*;
use context::*;
use error::*;
use processor::*;

mod account;
mod constants;
mod context;
mod error;
mod processor;

declare_id!("t1ynC7jhTJfZD8idR58Yz6EW8XiwajKzNXusf2tguBV");

#[program]
pub mod looties_contract {
    use super::*;

    //  Initialize Global Pool
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;

        global_pool.admin = ctx.accounts.admin.key();

        Ok(())
    }

    /**
     * Change admin
     *
     * @param - new_admin
     */
    pub fn change_admin(ctx: Context<UpdateGlobal>, new_admin: Pubkey) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;

        // Don't need check admin since it signed the transaction
        global_pool.admin = new_admin;

        Ok(())
    }

    /**
     * Create new box
     *
     * @param - random address to use as seed
     *          name of box
     *          description of box
     *          image url of box
     *          price in sol to open the box
     *          rewards
     */
    pub fn init_box<'info>(
        ctx: Context<'_, '_, '_, 'info, InitBox<'info>>,
        _rand: Pubkey,
        name: String,
        description: String,
        image_url: String,
        price_in_sol: u64,
        rewards: Vec<Reward>,
    ) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;
        let box_pool = &mut ctx.accounts.box_pool;

        require!(global_pool.boxes.len() < MAX_BOX_IN_GAME, GameError::ExceedMaxBox);
        require!(rewards.len() <= MAX_REWARD_IN_BOX, GameError::ExceedMaxReward);

        let mut sum: u16 = 0;
        for reward in rewards.iter() {
            sum += reward.chance
        }
        require!(sum == CHANCE_SUM, GameError::ChanceSumInvalid);

        box_pool.name = name;
        box_pool.description = description;
        box_pool.image_url = image_url;
        box_pool.price_in_sol = price_in_sol;
        box_pool.prizes = ctx.accounts.prize_pool.key();
        box_pool.rewards = rewards;

        global_pool.add_box(box_pool.key())
    }

    /**
     * Update box
     *
     * @param - name of box
     *          description of box
     *          image url of box
     *          price in sol to open the box
     *          rewards
     */
    pub fn update_box<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateBox<'info>>,
        name: String,
        description: String,
        image_url: String,
        price_in_sol: u64,
        rewards: Vec<Reward>,
    ) -> Result<()> {
        let box_pool = &mut ctx.accounts.box_pool;

        require!(rewards.len() <= MAX_REWARD_IN_BOX, GameError::ExceedMaxReward);

        let mut sum: u16 = 0;
        for reward in rewards.iter() {
            sum += reward.chance
        }
        require!(sum == CHANCE_SUM, GameError::ChanceSumInvalid);

        box_pool.name = name;
        box_pool.description = description;
        box_pool.image_url = image_url;
        box_pool.price_in_sol = price_in_sol;
        box_pool.rewards = rewards;

        Ok(())
    }
}
