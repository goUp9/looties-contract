use crate::account::*;
use crate::constants::*;
use crate::error::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Token;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        space = 8 + GlobalPool::INIT_SPACE,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
        payer = admin
    )]
    pub global_pool: Account<'info, GlobalPool>,

    // system
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UpdateGlobal<'info> {
    // Current admin
    #[account(
        mut,
        constraint = global_pool.admin == *admin.key @ GameError::InvalidAdmin
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_pool: Account<'info, GlobalPool>,
}

#[derive(Accounts)]
#[instruction(_rand: Pubkey)]
pub struct InitBox<'info> {
    // Only admin can create box
    #[account(
        mut,
        constraint = global_pool.admin == *admin.key @ GameError::InvalidAdmin
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_pool: Account<'info, GlobalPool>,

    #[account(
        init,
        seeds = [_rand.as_ref()],
        bump,
        space = 8 + BoxPool::INIT_SPACE,
        payer = admin
    )]
    pub box_pool: Account<'info, BoxPool>,

    #[account(
        init,
        seeds = [box_pool.key().as_ref()],
        bump,
        space = 8 + PrizePool::INIT_SPACE,
        payer = admin
    )]
    pub prize_pool: Account<'info, PrizePool>,

    // system
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UpdateBox<'info> {
    // Only admin can update box
    #[account(
        mut,
        constraint = global_pool.admin == *admin.key @GameError::InvalidAdmin
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_pool: Account<'info, GlobalPool>,

    #[account(mut)]
    pub box_pool: Account<'info, BoxPool>,

    // system
    pub token_program: Program<'info, Token>,
}
