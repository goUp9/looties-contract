use anchor_lang::prelude::*;
use crate::{
  constants::{GLOBAL_AUTHORITY_SEED, PRIZE_POOL_SEED, PLAYER_POOL_SEED, SOL_VAULT_SEED, ADMIN1, ADMIN2, ADMIN3},
  error::GameError,
  state::{GlobalPool, BoxPool, PrizePool, PlayerPool}, processor::{sol_transfer_user, calc_reward, get_rand},
};

#[derive(Accounts)]
pub struct OpenBox<'info> {
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
    init_if_needed,
    payer = player,
    space = 8 + PlayerPool::INIT_SPACE,
    seeds = [PLAYER_POOL_SEED.as_ref(), player.key().as_ref()],
    bump,
  )]
  pub player_pool: Account<'info, PlayerPool>,

  #[account(
    mut,
    seeds = [SOL_VAULT_SEED.as_ref()],
    bump,
  )]
  /// CHECK
  pub sol_vault: AccountInfo<'info>,

  /// CHECK
  #[account(mut)]
  pub admin1: AccountInfo<'info>,

  /// CHECK
  #[account(mut)]
  pub admin2: AccountInfo<'info>,

  /// CHECK
  #[account(mut)]
  pub admin3: AccountInfo<'info>,

  // system
  pub system_program: Program<'info, System>,
}


/**
 * Opens box with SOL
 * 
 * @param             - open times
 */
pub fn open_box_handler<'info>(
  ctx: Context<'_, '_, '_, 'info, OpenBox<'info>>,
  open_times: u16,
) -> Result<()> {
  let global_pool = &mut ctx.accounts.global_pool;
  let box_pool = &mut ctx.accounts.box_pool;
  let player_pool = &mut ctx.accounts.player_pool;
  let prize_pool = &mut ctx.accounts.prize_pool;
  let player = ctx.accounts.player.to_account_info();

  if player_pool.is_initialized {
    require_eq!(player.key(), player_pool.player, GameError::InvalidAuthority);
  } else {
    player_pool.is_initialized = true;
    player_pool.player = player.key();
  }

  //  Check box is legit
  require!(global_pool.boxes.contains(&box_pool.key()), GameError::BoxAddressUnknown);

  require!(ctx.accounts.admin1.key().to_string() == String::from(ADMIN1), GameError::InvalidAdminAddress);
  require!(ctx.accounts.admin2.key().to_string() == String::from(ADMIN2), GameError::InvalidAdminAddress);
  require!(ctx.accounts.admin3.key().to_string() == String::from(ADMIN3), GameError::InvalidAdminAddress);
  require!(1 <= open_times && open_times <= 3, GameError::OpenTimeExceed);

  msg!("Open box- player: {}, open times: {}", ctx.accounts.player.key(), open_times);

  // Transfer SOL to vault from player.
  let total_amount = box_pool.price_in_sol * open_times as u64;
  let platform_fee = total_amount / 100;
  sol_transfer_user(
    ctx.accounts.player.to_account_info(),
    ctx.accounts.admin1.to_account_info(),
    ctx.accounts.system_program.to_account_info(),
    platform_fee,
  )?;
  sol_transfer_user(
    ctx.accounts.player.to_account_info(),
    ctx.accounts.admin2.to_account_info(),
    ctx.accounts.system_program.to_account_info(),
    platform_fee,
  )?;
  sol_transfer_user(
    ctx.accounts.player.to_account_info(),
    ctx.accounts.admin3.to_account_info(),
    ctx.accounts.system_program.to_account_info(),
    platform_fee,
  )?;
  sol_transfer_user(
    ctx.accounts.player.to_account_info(),
    ctx.accounts.sol_vault.to_account_info(),
    ctx.accounts.system_program.to_account_info(),
    total_amount - 3 * platform_fee,
  )?;
  box_pool.sol_amount += total_amount - 3 * platform_fee;

  // Generate random number
  let clock = Clock::get()?;

  let mut seed = clock.unix_timestamp as u64;
  let mut random_numbers: Vec<u64> = Vec::new();
  let mut reward_idxs: Vec<u8> = Vec::new();

  for _ in 0..open_times {
    let rand = get_rand(seed, clock.slot) % 10000 + 1;
    random_numbers.push(rand);
    seed = rand;

    //  Find reward item
    reward_idxs.push(calc_reward(&box_pool.rewards, rand as u16));
  }

  player_pool.last_reward_idxs = reward_idxs.clone();
  player_pool.box_addr = box_pool.key();
  player_pool.open_times = open_times;

  for i in 0..open_times as usize {
    let mut select = false;
    let reward = &box_pool.rewards[reward_idxs[i] as usize];

    if reward.reward_type == 1 {
      player_pool.claimable_sol += reward.sol;
    }
    
    for j in 0..global_pool.token_count as usize {
      let token_address = global_pool.token_address[j];
      if token_address == reward.token_address && reward.reward_type == 2 {
        player_pool.claimable_token[j] += reward.token;
      }
    }

    for j in 0..prize_pool.nfts.len() {
      let prize = &mut prize_pool.nfts[j];
      if reward.collection_address == prize.collection_address && prize.rewarded == false && reward.reward_type == 3 && select == false {
        select = true;
        prize.rewarded = true;
        player_pool.claimable_nfts.push(prize.mint_info);
      }
    }

    require!(reward.reward_type != 3 || select == true, GameError::InsufficientFunds);
  }

  Ok(())
}