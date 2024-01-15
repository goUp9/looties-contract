use anchor_lang::prelude::*;
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, MAX_BOX_IN_GAME, MAX_REWARD_IN_BOX, CHANCE_SUM, BOX_AUTHORITY_SEED, PRIZE_POOL_SEED, TEMP_BOX_SEED, MAX_NAME_LENGTH, MAX_DESCRIPTION_LENGTH, MAX_IMAGE_URL_LENGTH},
  error::GameError,
  state::{GlobalPool, PrizePool, BoxPool},
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
    mut,
    seeds = [BOX_AUTHORITY_SEED.as_ref(), TEMP_BOX_SEED.as_ref()],
    bump,
  )]
  pub temp_box_pool: Account<'info, BoxPool>,

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
 */
pub fn init_box_handler<'info>(ctx: Context<'_, '_, '_, 'info, InitBox<'info>>) -> Result<()> {
  let global_pool = &mut ctx.accounts.global_pool;
  let box_pool = &mut ctx.accounts.box_pool;
  let temp_box_pool = &mut ctx.accounts.temp_box_pool;

  require!(global_pool.boxes.len() < MAX_BOX_IN_GAME, GameError::ExceedMaxBox);
  require!(temp_box_pool.rewards.len() <= MAX_REWARD_IN_BOX, GameError::ExceedMaxReward);

  let mut sum: u16 = 0;
  for reward in temp_box_pool.rewards.iter() {
    sum += reward.chance;
    require!(reward.reward_type == 1 || reward.reward_type == 2 || reward.reward_type == 3, GameError::RewardTypeInvalid);
    require!(reward.name.len() <= MAX_NAME_LENGTH, GameError::ExceedMaxNameLength);
    require!(reward.description.len() <= MAX_DESCRIPTION_LENGTH, GameError::ExceedMaxDescriptionLength);
    require!(reward.image_url.len() <= MAX_IMAGE_URL_LENGTH, GameError::ExceedMaxImageUrlLength);
    let token_mint = reward.token_address;
    if reward.reward_type == 2 && global_pool.token_address.iter().all(|&x| x != token_mint) {
      return err!(GameError::TokenAddressUnknown);
    }
  }
  require!(sum == CHANCE_SUM, GameError::ChanceSumInvalid);
  require!(temp_box_pool.name.len() <= MAX_NAME_LENGTH, GameError::ExceedMaxNameLength);
  require!(temp_box_pool.description.len() <= MAX_DESCRIPTION_LENGTH, GameError::ExceedMaxDescriptionLength);
  require!(temp_box_pool.image_url.len() <= MAX_IMAGE_URL_LENGTH, GameError::ExceedMaxImageUrlLength);

  box_pool.admin = temp_box_pool.admin.clone();
  box_pool.name = temp_box_pool.name.clone();
  box_pool.description = temp_box_pool.description.clone();
  box_pool.image_url = temp_box_pool.image_url.clone();
  box_pool.price_in_sol = temp_box_pool.price_in_sol.clone();
  box_pool.prizes = ctx.accounts.prize_pool.key();
  box_pool.rewards = temp_box_pool.rewards.clone();
  box_pool.rand_key = ctx.accounts.rand_key.key();

  temp_box_pool.admin = Pubkey::default();
  temp_box_pool.name.clear();
  temp_box_pool.description.clear();
  temp_box_pool.image_url.clear();
  temp_box_pool.price_in_sol = 0;
  temp_box_pool.rewards.clear();

  global_pool.add_box(box_pool.key())
}
