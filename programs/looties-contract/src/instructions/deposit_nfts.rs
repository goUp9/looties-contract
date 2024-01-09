use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token};
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, PRIZE_POOL_SEED},
  error::GameError,
  state::{GlobalPool, BoxPool, PrizePool},
};

#[derive(Accounts)]
pub struct DepositNfts<'info> {
  // Only admin can deposit nfts
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
    seeds = [PRIZE_POOL_SEED.as_ref(), box_pool.key().as_ref()],
    bump,
  )]
  pub prize_pool: Account<'info, PrizePool>,

  // system
  pub token_program: Program<'info, Token>,
}

/**
 * Deposit NFTs
 *
 * @param            - list of nft collection address
 *                   - list of nft mint address
 * 
 * @remainingAccount - NFT's ATA(box admin's ATA, global_pool's ATA) list included in box
 * 
 */
pub fn deposit_nfts_handler<'info>(
  ctx: Context<'_, '_, '_, 'info, DepositNfts<'info>>,
  collection_addr: Pubkey,
  mint_addr: Vec<Pubkey>,
) -> Result<()> {
  let global_pool = &mut ctx.accounts.global_pool;
  let box_pool = &mut ctx.accounts.box_pool;
  let prize_pool = &mut ctx.accounts.prize_pool;
  let remaining_accounts: Vec<AccountInfo> = ctx.remaining_accounts.to_vec();

  let len = mint_addr.len();
  require!(remaining_accounts.len() == len * 2, GameError::RemainingAccountCountDismatch);
  require!(box_pool.rewards.iter().any(|reward| reward.reward_type == 3 && reward.collection_address == collection_addr), GameError::CollectionAddressNotExsit);
  prize_pool.nfts.retain(|nft| nft.rewarded == false);

  //  Transfer NFTs to program ATA
  let mut idx = 0;
  for i in 0..len {
    let nft = mint_addr[i];

    let src_ata = spl_associated_token_account::get_associated_token_address(
      &ctx.accounts.admin.key(),
      &nft,
    );
    require!(
      remaining_accounts[idx].key().eq(&src_ata),
      GameError::SrcAtaDismatch
    );

    let dest_ata = spl_associated_token_account::get_associated_token_address(
      &global_pool.key(),
      &nft,
    );
    require!(
      remaining_accounts[idx + 1].key().eq(&dest_ata),
      GameError::DestAtaDismatch
    );

    //  Transfer NFT
    let token_program = &mut &ctx.accounts.token_program;
    let cpi_accounts = Transfer {
      from: remaining_accounts[idx].clone(),
      to: remaining_accounts[idx + 1].clone(),
      authority: ctx.accounts.admin.to_account_info().clone(),
    };
    token::transfer(
      CpiContext::new(token_program.clone().to_account_info(), cpi_accounts),
      1,
    )?;

    prize_pool.add_nft(collection_addr, nft)?;

    idx += 2;
  }

  Ok(())
}

