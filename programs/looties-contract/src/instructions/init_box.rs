use anchor_lang::prelude::*;
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, MAX_BOX_IN_GAME, MAX_REWARD_IN_BOX, CHANCE_SUM, BOX_AUTHORITY_SEED, PRIZE_POOL_SEED},
  error::GameError,
  state::{GlobalPool, Reward, PrizePool, BoxPool},
};

#[derive(Accounts)]
pub struct InitBox<'info> {
  // Only admin can create box
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
    init,
    seeds = [BOX_AUTHORITY_SEED.as_ref(), rand_key.key().as_ref()],
    bump,
    space = 8 + BoxPool::INIT_SPACE,
    payer = super_admin
  )]
  pub box_pool: Account<'info, BoxPool>,

  /// CHECK
  #[account(mut)]
  pub rand_key: AccountInfo<'info>,

  #[account(
    init,
    seeds = [PRIZE_POOL_SEED.as_ref(), box_pool.key().as_ref()],
    bump,
    space = 8 + PrizePool::INIT_SPACE,
    payer = super_admin
  )]
  pub prize_pool: Account<'info, PrizePool>,

  // system
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}

/**
 * Create new box
 *
 * @param - admin key
 *        - name of box
 *        - description of box
 *        - image url of box
 *        - price in sol to open the box
 *        - rewards
 */
pub fn init_box_handler<'info>(
  ctx: Context<'_, '_, '_, 'info, InitBox<'info>>,
  admin: Pubkey,
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
    sum += reward.chance;
    require!(reward.reward_type == 1 || reward.reward_type == 2 || reward.reward_type == 3, GameError::RewardTypeInvalid);
    let token_mint = reward.token_address;
    if reward.reward_type == 2 && global_pool.token_address.iter().all(|&x| x != token_mint) {
      return err!(GameError::TokenAddressUnknown);
    }
  }
  require!(sum == CHANCE_SUM, GameError::ChanceSumInvalid);

  box_pool.admin = admin;
  box_pool.name = name;
  box_pool.description = description;
  box_pool.image_url = image_url;
  box_pool.price_in_sol = price_in_sol;
  box_pool.prizes = ctx.accounts.prize_pool.key();
  box_pool.rewards = rewards;

  global_pool.add_box(box_pool.key())
}

