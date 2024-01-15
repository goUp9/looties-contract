use anchor_lang::prelude::*;

use instructions::*;
use crate::state::Reward;

mod constants;
mod error;
mod state;
mod processor;
mod instructions;

declare_id!("t1ynC7jhTJfZD8idR58Yz6EW8XiwajKzNXusf2tguBV");

#[program]
pub mod looties_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize_handler(ctx)
    }
    
    pub fn change_super_admin(ctx: Context<ChangeSuperAdmin>, new_admin: Pubkey) -> Result<()> {
        change_super_admin_handler(ctx, new_admin)
    }

    pub fn add_token_address(ctx: Context<AddTokenAddress>, token_address: Pubkey) -> Result<()> {
        add_token_address_handler(ctx, token_address)
    }

    pub fn change_admin(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
        change_admin_handler(ctx, new_admin)
    }
    
    pub fn init_temp_box<'info>(
        ctx: Context<'_, '_, '_, 'info, InitTempBox<'info>>,
        admin: Pubkey,
        name: String,
        description: String,
        image_url: String,
        price_in_sol: u64,
    ) -> Result<()> {
        init_temp_box_handler(ctx, admin, name, description, image_url, price_in_sol)
    }

    pub fn add_reward_to_temp_box<'info>(ctx: Context<'_, '_, '_, 'info, AddRewardToTempBox<'info>>, reward: Reward) -> Result<()> {
        add_reward_to_temp_box_handler(ctx, reward)
    }

    pub fn init_box<'info>(ctx: Context<'_, '_, '_, 'info, InitBox<'info>>) -> Result<()> {
        init_box_handler(ctx)
    }

    pub fn update_box<'info>(ctx: Context<'_, '_, '_, 'info, UpdateBox<'info>>) -> Result<()> {
        update_box_handler(ctx)
    }

    pub fn remove_box<'info>(ctx: Context<'_, '_, '_, 'info, RemoveBox<'info>>) -> Result<()> {
        remove_box_handler(ctx)
    }

    pub fn deposit_nfts<'info>(ctx: Context<'_, '_, '_, 'info, DepositNfts<'info>>, collection_addr: Pubkey, mint_addr: Vec<Pubkey>) -> Result<()> {
        deposit_nfts_handler(ctx, collection_addr, mint_addr)
    }

    pub fn withdraw_nfts<'info>(ctx: Context<'_, '_, '_, 'info, WithdrawNfts<'info>>, nfts: Vec<Pubkey>) -> Result<()> {
        withdraw_nfts_handler(ctx, nfts)
    }

    pub fn deposit_sol(ctx: Context<DepositSol>, sol_amount: u64) -> Result<()> {
        deposit_sol_handler(ctx, sol_amount)
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, sol_amount: u64) -> Result<()> {
        withdraw_sol_handler(ctx, sol_amount)
    }

    pub fn deposit_token(ctx: Context<DepositToken>, token_amount: u64) -> Result<()> {
        deposit_token_handler(ctx, token_amount)
    }

    pub fn withdraw_token(ctx: Context<WithdrawToken>, token_amount: u64) -> Result<()> {
        withdraw_token_handler(ctx, token_amount)
    }

    pub fn open_box<'info>(ctx: Context<'_, '_, '_, 'info, OpenBox<'info>>, open_times: u16) -> Result<()> {
        open_box_handler(ctx, open_times)
    }

    pub fn claim_reward_token<'info>(ctx: Context<'_, '_, '_, 'info, ClaimRewardToken<'info>>) -> Result<()> {
        claim_reward_token_handler(ctx)
    }

    pub fn claim_reward_nfts<'info>(ctx: Context<'_, '_, '_, 'info, ClaimRewardNfts<'info>>, nfts: Vec<Pubkey>) -> Result<()> {
        claim_reward_nfts_handler(ctx, nfts)
    }
      
}

