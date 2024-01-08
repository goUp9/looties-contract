use anchor_lang::prelude::*;
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, MAX_REWARD_IN_BOX, CHANCE_SUM},
  error::GameError,
  state::{GlobalPool, BoxPool, Reward},
};


#[derive(Accounts)]
pub struct UpdateBox<'info> {
  // Only admin can update box
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

  #[account(mut)]
  pub box_pool: Account<'info, BoxPool>,
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
pub fn update_box_handler<'info>(
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
