use anchor_lang::prelude::*;
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, BOX_AUTHORITY_SEED, TEMP_BOX_SEED},
  error::GameError,
  state::{GlobalPool, BoxPool},
};

#[derive(Accounts)]
pub struct InitTempBox<'info> {
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
    init_if_needed,
    seeds = [BOX_AUTHORITY_SEED.as_ref(), TEMP_BOX_SEED.as_ref()],
    bump,
    space = 8 + BoxPool::INIT_SPACE,
    payer = super_admin
  )]
  pub box_pool: Account<'info, BoxPool>,

  // system
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}

/**
 * Create or Init temp box
 *
 * @param - admin key
 *        - name of box
 *        - description of box
 *        - image url of box
 *        - price in sol to open the box
 */
pub fn init_temp_box_handler<'info>(
  ctx: Context<'_, '_, '_, 'info, InitTempBox<'info>>,
  admin: Pubkey,
  name: String,
  description: String,
  image_url: String,
  price_in_sol: u64,
) -> Result<()> {
  let box_pool = &mut ctx.accounts.box_pool;

  box_pool.admin = admin;
  box_pool.name = name;
  box_pool.description = description;
  box_pool.image_url = image_url;
  box_pool.price_in_sol = price_in_sol;
  box_pool.rewards.clear();

  Ok(())
}

