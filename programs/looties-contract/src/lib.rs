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
}
