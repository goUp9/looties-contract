use anchor_lang::prelude::*;
use anchor_spl::token::{Token, self, Transfer};
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, PRIZE_POOL_SEED, PLAYER_POOL_SEED, SOL_VAULT_SEED},
  error::GameError,
  state::{GlobalPool, BoxPool, PrizePool, PlayerPool}, processor::sol_transfer_with_signer,
};

#[derive(Accounts)]
pub struct ClaimReward<'info> {
  #[account(mut)]
  pub player: Signer<'info>,

  #[account(
    mut,
    seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
    bump,
  )]
  pub global_pool: Box<Account<'info, GlobalPool>>,

  #[account(mut)]
  pub box_pool: Account<'info, BoxPool>,

  #[account(
    mut,
    seeds = [PRIZE_POOL_SEED.as_ref(), box_pool.key().as_ref()],
    bump,
  )]
  pub prize_pool: Account<'info, PrizePool>,

  #[account(
    mut,
    seeds = [PLAYER_POOL_SEED.as_ref(), player.key().as_ref()],
    bump,
    constraint = player_pool.player == player.key(),
    constraint = player_pool.box_addr == box_pool.key(),
  )]
  pub player_pool: Account<'info, PlayerPool>,

  #[account(
    mut,
    seeds = [SOL_VAULT_SEED.as_ref()],
    bump,
  )]
  /// CHECK
  pub sol_vault: AccountInfo<'info>,

  // system
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
}

/**
 * Claim reward
 * 
 * @remainingAccounts - list of SPL token's ATA(global_pool's ATA, player's ATA) included in game
 *                    - list of claimable NFT's ATA(global_pool's ATA, player's ATA)
 */
pub fn claim_reward_handler<'info>(
  ctx: Context<'_, '_, '_, 'info, ClaimReward<'info>>,
) -> Result<()> {
  let global_pool = &mut ctx.accounts.global_pool;
  let box_pool = &mut ctx.accounts.box_pool;
  let prize_pool = &mut ctx.accounts.prize_pool;
  let player_pool = &mut ctx.accounts.player_pool;
  let remaining_accounts: Vec<AccountInfo> = ctx.remaining_accounts.to_vec();
  
  require!(remaining_accounts.len() == (global_pool.token_count as usize + player_pool.claimable_nfts.len()) * 2, GameError::RemainingAccountCountDismatch);

  let sol_vault_bump = ctx.bumps.sol_vault;
  
  require!(box_pool.sol_amount >= player_pool.claimable_sol, GameError::InsufficientFunds);
  sol_transfer_with_signer(
    ctx.accounts.sol_vault.to_account_info(),
    ctx.accounts.player.to_account_info(),
    ctx.accounts.system_program.to_account_info(),
    &[&[SOL_VAULT_SEED.as_ref(), &[sol_vault_bump]]],
    player_pool.claimable_sol,
  )?;
  box_pool.sol_amount -= player_pool.claimable_sol;
  player_pool.claimable_sol = 0;

  for idx in 0..global_pool.token_count as usize {
    if player_pool.claimable_token[idx] == 0 {
      continue;
    }

    require!(box_pool.token_amount[idx] >= player_pool.claimable_token[idx], GameError::InsufficientFunds);
    let token_address = global_pool.token_address[idx];

    let src_ata = spl_associated_token_account::get_associated_token_address(
      &global_pool.key(),
      &token_address,
    );

    let global_bump = ctx.bumps.global_pool;
    let seeds = &[GLOBAL_AUTHORITY_SEED.as_bytes(), &[global_bump]];
    let signer = [&seeds[..]];

    let account_idx = idx * 2;
    require!(
      remaining_accounts[account_idx].key().eq(&src_ata),
      GameError::SrcAtaDismatch
    );

    let dest_ata = spl_associated_token_account::get_associated_token_address(
      &ctx.accounts.player.key(),
      &token_address,
    );
    require!(
      remaining_accounts[account_idx + 1].key().eq(&dest_ata),
      GameError::DestAtaDismatch
    );

    let cpi_ctx = CpiContext::new_with_signer(
      ctx.accounts.token_program.to_account_info(),
      Transfer {
        from: remaining_accounts[account_idx].to_account_info(),
        authority: global_pool.to_account_info(),
        to: remaining_accounts[account_idx + 1].to_account_info(),
      },
      &signer,
    );
    token::transfer(cpi_ctx, player_pool.claimable_token[idx])?;

    box_pool.token_amount[idx] -= player_pool.claimable_token[idx];
    player_pool.claimable_token[idx] = 0;
  }

  for i in 0..player_pool.claimable_nfts.len() {
    let nft = player_pool.claimable_nfts[i];
    let idx = 2 * global_pool.token_count as usize + i * 2;

    let src_ata = spl_associated_token_account::get_associated_token_address(
      &global_pool.key(),
      &nft,
    );
    require!(
      remaining_accounts[idx].key().eq(&src_ata),
      GameError::SrcAtaDismatch
    );

    let dest_ata = spl_associated_token_account::get_associated_token_address(
      &ctx.accounts.player.key(),
      &nft,
    );
    require!(
      remaining_accounts[idx + 1].key().eq(&dest_ata),
      GameError::DestAtaDismatch
    );

    //  Transfer NFT
    let global_bump = ctx.bumps.global_pool;
    let seeds = &[GLOBAL_AUTHORITY_SEED.as_bytes(), &[global_bump]];
    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
      from: remaining_accounts[idx].clone(),
      to: remaining_accounts[idx + 1].clone(),
      authority: global_pool.to_account_info(),
    };

    let token_program = &mut &ctx.accounts.token_program;
    token::transfer(
      CpiContext::new_with_signer(
        token_program.clone().to_account_info(),
        cpi_accounts,
        signer,
      ),
      1,
    )?;

    prize_pool.remove_nft(nft);
  }

  Ok(())
}
