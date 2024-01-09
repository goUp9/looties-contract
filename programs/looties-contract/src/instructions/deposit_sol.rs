use anchor_lang::prelude::*;
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, SOL_VAULT_SEED},
  error::GameError,
  state::{GlobalPool, BoxPool}, processor::sol_transfer_user,
};

#[derive(Accounts)]
pub struct DepositSol<'info> {
  // Only admin can deposit to Vault
  #[account(
    mut,
    constraint = box_pool.admin == *admin.key || global_pool.super_admin == *admin.key @ GameError::InvalidAdmin
  )]
  pub admin: Signer<'info>,

  #[account(
    mut,
    seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
    bump,
  )]
  pub global_pool: Account<'info, GlobalPool>,

  #[account(mut)]
  pub box_pool: Account<'info, BoxPool>,

  #[account(
    mut,
    seeds = [SOL_VAULT_SEED.as_ref()],
    bump,
  )]
  /// CHECK
  pub sol_vault: AccountInfo<'info>,

  // SPL Token Program to transfer Token
  pub system_program: Program<'info, System>,
}

/**
 * Deposit SOL to PDA
 *
 * @param - sol amount to deposit
 */
pub fn deposit_sol_handler(ctx: Context<DepositSol>, sol_amount: u64) -> Result<()> {
  let box_pool = &mut ctx.accounts.box_pool;

  msg!("Depositer: {}", ctx.accounts.admin.to_account_info().key());
  msg!(
    "Asking to deposit: {} SOL",
    sol_amount,
  );
  if sol_amount > 0 {
    sol_transfer_user(
      ctx.accounts.admin.to_account_info(),
      ctx.accounts.sol_vault.to_account_info(),
      ctx.accounts.system_program.to_account_info(),
      sol_amount,
    )?;
    box_pool.sol_amount += sol_amount;
  }

  Ok(())
}
