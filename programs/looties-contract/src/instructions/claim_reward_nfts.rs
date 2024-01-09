use anchor_lang::prelude::*;
use anchor_spl::token::{Token, self, Transfer};
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, PLAYER_POOL_SEED},
  error::GameError,
  state::{GlobalPool, PlayerPool},
};

#[derive(Accounts)]
pub struct ClaimRewardNfts<'info> {
  #[account(mut)]
  pub player: Signer<'info>,

  #[account(
    mut,
    seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
    bump,
  )]
  pub global_pool: Box<Account<'info, GlobalPool>>,

  #[account(
    mut,
    seeds = [PLAYER_POOL_SEED.as_ref(), player.key().as_ref()],
    bump,
  )]
  pub player_pool: Account<'info, PlayerPool>,

  // system
  pub token_program: Program<'info, Token>,
}

/**
 * Claim reward
 * 
 * @remainingAccounts - list of claimable NFT's ATA(global_pool's ATA, player's ATA)
 */
pub fn claim_reward_nfts_handler<'info>(
  ctx: Context<'_, '_, '_, 'info, ClaimRewardNfts<'info>>,
  nfts: Vec<Pubkey>,
) -> Result<()> {
  let global_pool = &mut ctx.accounts.global_pool;
  let player_pool = &mut ctx.accounts.player_pool;
  let remaining_accounts: Vec<AccountInfo> = ctx.remaining_accounts.to_vec();

  require!(remaining_accounts.len()== nfts.len() * 2, GameError::RemainingAccountCountDismatch);

  for i in 0..nfts.len() {
    let nft = nfts[i];
    let idx = i * 2;

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

    player_pool.claimable_nfts.retain(|&x| x != nft);
  }

  Ok(())
}
