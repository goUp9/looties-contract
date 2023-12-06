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
    use anchor_spl::token::{self, Transfer};

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

    /**
     * Update box
     *
     * @remainingAccount
     */
    pub fn remove_box<'info>(ctx: Context<'_, '_, '_, 'info, RemoveBox<'info>>) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;
        let box_pool = &mut ctx.accounts.box_pool;
        let prize_pool = &mut ctx.accounts.prize_pool;

        global_pool.remove_box(box_pool.key());

        //  Transfer NFTs to admin wallet
        let remaining_accounts: Vec<AccountInfo> = ctx.remaining_accounts.to_vec();
        let mut idx = 0;
        let nfts = &prize_pool.nfts;
        for nft in nfts {
            let src_ata = spl_associated_token_account::get_associated_token_address(
                &global_pool.key(),
                &nft.mint_info,
            );
            require!(
                remaining_accounts[idx].key().eq(&src_ata),
                GameError::SrcAtaDismatch
            );

            let dest_ata = spl_associated_token_account::get_associated_token_address(
                &ctx.accounts.admin.key(),
                &nft.mint_info,
            );
            require!(
                remaining_accounts[idx + 1].key().eq(&dest_ata),
                GameError::DestAtaDismatch
            );

            //  Transfer NFT
            let global_bump = ctx.bumps.global_pool;
            let seeds = &[GLOBAL_AUTHORITY_SEED.as_bytes(), &[global_bump]];
            let signer = &[&seeds[..]];

            let cpi_accounts = Transfer {
                from: remaining_accounts[idx].clone(),
                to: remaining_accounts[idx + 1].clone(),
                authority: global_pool.to_account_info(),
            };

            let token_program = &mut &ctx.accounts.token_program;
            token::transfer(
                CpiContext::new_with_signer(
                    token_program.clone().to_account_info(),
                    cpi_accounts,
                    signer,
                ),
                1,
            )?;

            idx += 2;
        }

        Ok(())
    }
}
