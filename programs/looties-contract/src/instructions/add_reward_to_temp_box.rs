use anchor_lang::prelude::*;
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, MAX_REWARD_IN_BOX, BOX_AUTHORITY_SEED, TEMP_BOX_SEED},
  error::GameError,
  state::{GlobalPool, Reward, BoxPool},
};

#[derive(Accounts)]
pub struct AddRewardToTempBox<'info> {
  // Only admin can create temp box
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
    seeds = [BOX_AUTHORITY_SEED.as_ref(), TEMP_BOX_SEED.as_ref()],
    bump,
  )]
  pub box_pool: Account<'info, BoxPool>,
}

/**
 * Add reward
 *
 * @param - reward
 */
pub fn add_reward_to_temp_box_handler<'info>(
  ctx: Context<'_, '_, '_, 'info, AddRewardToTempBox<'info>>,
  reward: Reward,
) -> Result<()> {
  let box_pool = &mut ctx.accounts.box_pool;

  require!(box_pool.rewards.len() < MAX_REWARD_IN_BOX, GameError::ExceedMaxReward);

  box_pool.rewards.push(reward);

  Ok(())
}
