use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token, self, Transfer};
use crate::{
  constants::GLOBAL_AUTHORITY_SEED,
  error::GameError,
  state::{GlobalPool, BoxPool}
};

#[derive(Accounts)]
pub struct WithdrawToken<'info> {
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
 * Withdraw Token from PDA
 *
 * @param - token amount to withdraw
 */
pub fn withdraw_token_handler(ctx: Context<WithdrawToken>, token_amount: u64) -> Result<()> {
  let global_pool = &mut ctx.accounts.global_pool;
  let box_pool = &mut ctx.accounts.box_pool;
  msg!("Withdrawer: {}", ctx.accounts.admin.to_account_info().key());
  msg!(
    "Asking to withdraw: {} Token {}",
    token_amount,
    ctx.accounts.token_mint.key(),
  );

  let id = match global_pool.token_address.iter().position(|&token_mint| token_mint == ctx.accounts.token_mint.key()) {
    Some(id) => id,
    None => return err!(GameError::TokenAddressUnknown),
  };
  require!(box_pool.token_amount[id] >= token_amount, GameError::InsufficientFunds);

  if token_amount > 0 {
    //  Transfer Token to admin from PDA
    let global_bump = ctx.bumps.global_pool;
    let seeds = &[GLOBAL_AUTHORITY_SEED.as_bytes(), &[global_bump]];
    let signer = [&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
      ctx.accounts.token_program.to_account_info(),
      Transfer {
        from: ctx.accounts.token_vault.to_account_info(),
        authority: ctx.accounts.global_pool.to_account_info(),
        to: ctx.accounts.token_admin.to_account_info(),
      },
      &signer,
    );
    token::transfer(cpi_ctx, token_amount)?;
    box_pool.token_amount[id] -= token_amount;
  }

  Ok(())
}
