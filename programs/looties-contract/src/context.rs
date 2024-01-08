use crate::account::*;
use crate::constants::*;
use crate::error::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;





#[derive(Accounts)]
pub struct InitPlayer<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        init,
        space = 8 + PlayerPool::INIT_SPACE,
        seeds = [PLAYER_POOL_SEED.as_ref(), player.key().as_ref()],
        bump,
        payer = player
    )]
    pub player_pool: Account<'info, PlayerPool>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
