use anchor_lang::prelude::*;
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, PRIZE_POOL_SEED},
  error::GameError,
  state::{GlobalPool, PrizePool, BoxPool},
};


#[derive(Accounts)]
pub struct RemoveBox<'info> {
  // Only admin can remove box
  #[account(
    mut,
    constraint = global_pool.super_admin == *super_admin.key @ GameError::InvalidSuperAdmin
  )]
  pub super_admin: Signer<'info>,

  #[account(
    mut,
    seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
    bump,
  )]
  pub global_pool: Account<'info, GlobalPool>,

  #[account(
    mut,
    close = super_admin,
  )]
  pub box_pool: Account<'info, BoxPool>,

  #[account(
    mut,
    seeds = [PRIZE_POOL_SEED.as_ref(), box_pool.key().as_ref()],
    bump,
    close = super_admin,
  )]
  pub prize_pool: Account<'info, PrizePool>,
}

/**
 * Remove box
 */
pub fn remove_box_handler<'info>(ctx: Context<'_, '_, '_, 'info, RemoveBox<'info>>) -> Result<()> {
  let global_pool = &mut ctx.accounts.global_pool;
  let box_pool = &mut ctx.accounts.box_pool;
  let prize_pool = &mut ctx.accounts.prize_pool;

  require!(box_pool.sol_amount == 0, GameError::SolBalanceExist);
  for i in 0..global_pool.token_count as usize {
    require!(box_pool.token_amount[i] == 0, GameError::TokenBalanceExist);
  }

  require!(prize_pool.nfts.len() == 0, GameError::NFTBalanceExist);

  global_pool.remove_box(box_pool.key());

  Ok(())
}
