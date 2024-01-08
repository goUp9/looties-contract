use anchor_lang::prelude::*;
use crate::{
  constants::GLOBAL_AUTHORITY_SEED,
  state::GlobalPool,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(mut)]
  pub super_admin: Signer<'info>,

  #[account(
    init,
    space = 8 + GlobalPool::INIT_SPACE,
    seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
    bump,
    payer = super_admin
  )]
  pub global_pool: Account<'info, GlobalPool>,

  // system
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
  let global_pool = &mut ctx.accounts.global_pool;

  global_pool.super_admin = ctx.accounts.super_admin.key();
  global_pool.token_count = 0;

  Ok(())
}
