use anchor_lang::prelude::*;
use crate::{
  constants::GLOBAL_AUTHORITY_SEED,
  error::GameError,
  state::{GlobalPool, BoxPool},
};
#[derive(Accounts)]
pub struct ChangeAdmin<'info> {
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
 * Change admin for box
 *
 * @param - new_admin
 */
pub fn change_admin_handler(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
  let box_pool = &mut ctx.accounts.box_pool;

  // Don't need check admin since it signed the transaction
  box_pool.admin = new_admin;

  Ok(())
}
