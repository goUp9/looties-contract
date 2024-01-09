use anchor_lang::prelude::*;
use anchor_spl::token::{Token, self, Transfer};
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, PRIZE_POOL_SEED},
  error::GameError,
  state::{GlobalPool, BoxPool, PrizePool},
};

#[derive(Accounts)]
pub struct WithdrawNfts<'info> {
  // Only admin can withdraw nfts
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
    seeds = [PRIZE_POOL_SEED.as_ref(), box_pool.key().as_ref()],
    bump,
  )]
  pub prize_pool: Account<'info, PrizePool>,

  // system
  pub token_program: Program<'info, Token>,
}

/**
 * Withdraw NFTs
 *
 * @param            - NFT list to withdraw
 * 
 * @remainingAccount - NFT's ATA(global_pool's ATA, admin's ATA) list to withdraw
 */
pub fn withdraw_nfts_handler<'info>(
  ctx: Context<'_, '_, '_, 'info, WithdrawNfts<'info>>,
  nfts: Vec<Pubkey>
) -> Result<()> {
  let global_pool = &mut ctx.accounts.global_pool;
  let prize_pool = &mut ctx.accounts.prize_pool;

  //  Transfer NFTs to admin wallet
  let remaining_accounts: Vec<AccountInfo> = ctx.remaining_accounts.to_vec();
  prize_pool.nfts.retain(|nft| nft.rewarded == false);

  require!(remaining_accounts.len() == nfts.len() * 2, GameError::RemainingAccountCountDismatch);

  let mut idx = 0;

  for nft in nfts {
    let src_ata = spl_associated_token_account::get_associated_token_address(
      &global_pool.key(),
      &nft,
    );
    require!(
      remaining_accounts[idx].key().eq(&src_ata),
      GameError::SrcAtaDismatch
    );

    let dest_ata = spl_associated_token_account::get_associated_token_address(
      &ctx.accounts.admin.key(),
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

    idx += 2;
  }

  Ok(())
}
