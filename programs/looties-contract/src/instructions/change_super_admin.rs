use anchor_lang::prelude::*;
use crate::{
  constants::GLOBAL_AUTHORITY_SEED,
  error::GameError,
  state::GlobalPool,
};

#[derive(Accounts)]
pub struct ChangeSuperAdmin<'info> {
  // Current super admin
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
}

/**
 * Change super admin for game
 *
 * @param - new_admin
 */
pub fn change_super_admin_handler(ctx: Context<ChangeSuperAdmin>, new_admin: Pubkey) -> Result<()> {
  let global_pool = &mut ctx.accounts.global_pool;

  global_pool.super_admin = new_admin;

  Ok(())
}
