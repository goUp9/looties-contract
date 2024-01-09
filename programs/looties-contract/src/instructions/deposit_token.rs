use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Mint, Token};
use crate::{
  constants::GLOBAL_AUTHORITY_SEED,
  error::GameError,
  state::{GlobalPool, BoxPool},
};

#[derive(Accounts)]
pub struct DepositToken<'info> {
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

  // Associated Token Account for admin which holds Token
  #[account(
    mut,
    constraint = token_admin.mint == token_mint.key(),
  )]
  pub token_admin: Account<'info, TokenAccount>,

  #[account(
    mut,
    constraint = token_vault.mint == token_mint.key(),
    constraint = token_vault.owner == global_pool.key(),
  )]
  pub token_vault: Account<'info, TokenAccount>,

  pub token_mint: Account<'info, Mint>,

  // SPL Token Program to transfer Token
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}

/**
 * Deposit Token to PDA
 *
 * @param - token amount to deposit
 */
pub fn deposit_token_handler(ctx: Context<DepositToken>, token_amount: u64) -> Result<()> {
  let global_pool = &mut ctx.accounts.global_pool;
  let box_pool = &mut ctx.accounts.box_pool;

  msg!("Depositer: {}", ctx.accounts.admin.to_account_info().key());
  msg!(
    "Asking to deposit: {} Token {}",
    token_amount,
    ctx.accounts.token_mint.key(),
  );

  if token_amount > 0 {
    let id = match global_pool.token_address.iter().position(|&token_mint| token_mint == ctx.accounts.token_mint.key()) {
      Some(id) => id,
      None => return err!(GameError::TokenAddressUnknown),
    };
    //  Transfer Token to PDA from admin
    let cpi_ctx = CpiContext::new(
      ctx.accounts.token_program.to_account_info(),
      Transfer {
        from: ctx.accounts.token_admin.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
        to: ctx.accounts.token_vault.to_account_info(),
      },
    );
    token::transfer(cpi_ctx, token_amount)?;
    box_pool.token_amount[id] += token_amount;
  }

  Ok(())
}
