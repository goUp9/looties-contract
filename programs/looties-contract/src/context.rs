use crate::account::*;
use crate::constants::*;
use crate::error::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

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
        constraint = global_pool.admin == *admin.key @ GameError::InvalidAdmin
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

#[derive(Accounts)]
pub struct RemoveBox<'info> {
    // Only admin can remove box
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

    #[account(mut)]
    pub box_pool: Account<'info, BoxPool>,

    #[account(
        mut,
        seeds = [box_pool.key().as_ref()],
        bump,
    )]
    pub prize_pool: Account<'info, PrizePool>,

    // system
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DepositNfts<'info> {
    // Only admin can deposit nfts
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

    #[account(mut)]
    pub box_pool: Account<'info, BoxPool>,

    #[account(
        mut,
        seeds = [box_pool.key().as_ref()],
        bump,
    )]
    pub prize_pool: Account<'info, PrizePool>,

    // system
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawNfts<'info> {
    // Only admin can withdraw nfts
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

    #[account(mut)]
    pub box_pool: Account<'info, BoxPool>,

    #[account(
        mut,
        seeds = [box_pool.key().as_ref()],
        bump,
    )]
    pub prize_pool: Account<'info, PrizePool>,

    // system
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    // Only admin can deposit to Vault
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
        mut,
        seeds = [SOL_VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK
    pub sol_vault: AccountInfo<'info>,

    // Associated Token Account for admin which holds Token
    #[account(mut)]
    pub token_admin: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[token_mint.key().as_ref()],
        bump,
    )]
    pub token_vault: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    // SPL Token Program to transfer Token
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    // Only admin can withdraw from Vault
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
    pub global_pool: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        seeds = [SOL_VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK
    pub sol_vault: AccountInfo<'info>,

    // Associated Token Account for admin which holds Token
    #[account(mut)]
    pub token_admin: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[token_mint.key().as_ref()],
        bump,
    )]
    pub token_vault: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    // SPL Token Program to transfer Token
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
