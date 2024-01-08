use anchor_lang::prelude::*;
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, MAX_TOKEN_IN_GAME},
  error::GameError,
  state::GlobalPool,
};

#[derive(Accounts)]
pub struct AddTokenAddress<'info> {
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
 * Add SPL token address for game.
 *
 * @param - token mint address
 */
pub fn add_token_address_handler(ctx: Context<AddTokenAddress>, token_address: Pubkey) -> Result<()> {
  let global_pool = &mut ctx.accounts.global_pool;

  require!(global_pool.token_address.len() < MAX_TOKEN_IN_GAME, GameError::ExceedMaxTokenAddress);
  require!(global_pool.token_address.iter().all(|&x| x != token_address), GameError::TokenAddressAlreadyExist);

  global_pool.token_address.push(token_address);
  global_pool.token_count += 1;

  Ok(())
}
