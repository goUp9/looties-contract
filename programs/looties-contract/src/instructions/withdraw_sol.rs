use anchor_lang::prelude::*;
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, SOL_VAULT_SEED},
  error::GameError,
  state::{GlobalPool, BoxPool}, processor::sol_transfer_with_signer,
};

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
  // Only admin can withdraw from Vault
  #[account(
    mut,
    constraint = box_pool.admin == *admin.key @ GameError::InvalidAdmin
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
 * Withdraw SOL from PDA
 *
 * @param - sol amount to withdraw
 */
pub fn withdraw_sol_handler(ctx: Context<WithdrawSol>, sol_amount: u64) -> Result<()> {
  let box_pool = &mut ctx.accounts.box_pool;
  msg!("Withdrawer: {}", ctx.accounts.admin.to_account_info().key());
  msg!(
    "Asking to withdraw: {} SOL",
    sol_amount,
  );

  let sol_vault_bump = ctx.bumps.sol_vault;

  require!(box_pool.sol_amount >= sol_amount, GameError::InsufficientFunds);
  if sol_amount > 0 {
    sol_transfer_with_signer(
      ctx.accounts.sol_vault.to_account_info(),
      ctx.accounts.admin.to_account_info(),
      ctx.accounts.system_program.to_account_info(),
      &[&[SOL_VAULT_SEED.as_ref(), &[sol_vault_bump]]],
      sol_amount,
    )?;
    box_pool.sol_amount -= sol_amount;
  }

  Ok(())
}
