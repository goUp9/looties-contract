use anchor_lang::prelude::*;

use account::*;
use constants::*;
use context::*;
use error::*;
use processor::*;

mod account;
mod constants;
mod context;
mod error;
mod processor;

declare_id!("t1ynC7jhTJfZD8idR58Yz6EW8XiwajKzNXusf2tguBV");

#[program]
pub mod looties_contract {
    use anchor_spl::token::{self, Transfer};

    use super::*;

    //  Initialize Global Pool
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;

        global_pool.super_admin = ctx.accounts.super_admin.key();
        global_pool.token_count = 0;

        Ok(())
    }

    /**
     * Change super admin for game
     *
     * @param - new_admin
     */
    pub fn change_super_admin(ctx: Context<UpdateGlobal>, new_admin: Pubkey) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;

        global_pool.super_admin = new_admin;

        Ok(())
    }

    /**
     * Add SPL token address for game.
     *
     * @param - token mint address
     */
    pub fn add_token_address(ctx: Context<UpdateGlobal>, token_address: Pubkey) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;

        require!(global_pool.token_address.len() < MAX_TOKEN_IN_GAME, GameError::ExceedMaxTokenAddress);
        require!(global_pool.token_address.iter().all(|&x| x != token_address), GameError::TokenAddressAlreadyExist);

        global_pool.token_address.push(token_address);
        global_pool.token_count += 1;

        Ok(())
    }

    /**
     * Change admin for box
     *
     * @param - new_admin
     */
    pub fn change_admin(ctx: Context<UpdateBox>, new_admin: Pubkey) -> Result<()> {
        let box_pool = &mut ctx.accounts.box_pool;

        // Don't need check admin since it signed the transaction
        box_pool.admin = new_admin;

        Ok(())
    }

    /**
     * Create new box
     *
     * @param - admin key
     *        - name of box
     *        - description of box
     *        - image url of box
     *        - price in sol to open the box
     *        - rewards
     */
    pub fn init_box<'info>(
        ctx: Context<'_, '_, '_, 'info, InitBox<'info>>,
        admin: Pubkey,
        name: String,
        description: String,
        image_url: String,
        price_in_sol: u64,
        rewards: Vec<Reward>,
    ) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;
        let box_pool = &mut ctx.accounts.box_pool;

        require!(global_pool.boxes.len() < MAX_BOX_IN_GAME, GameError::ExceedMaxBox);
        require!(rewards.len() <= MAX_REWARD_IN_BOX, GameError::ExceedMaxReward);

        let mut sum: u16 = 0;
        for reward in rewards.iter() {
            sum += reward.chance;
            require!(reward.reward_type == 1 || reward.reward_type == 2 || reward.reward_type == 3, GameError::RewardTypeInvalid);
            let token_mint = reward.token_address;
            if reward.reward_type == 2 && global_pool.token_address.iter().all(|&x| x != token_mint) {
                return err!(GameError::TokenAddressUnknown);
            }
        }
        require!(sum == CHANCE_SUM, GameError::ChanceSumInvalid);

        box_pool.admin = admin;
        box_pool.name = name;
        box_pool.description = description;
        box_pool.image_url = image_url;
        box_pool.price_in_sol = price_in_sol;
        box_pool.prizes = ctx.accounts.prize_pool.key();
        box_pool.rewards = rewards;

        global_pool.add_box(box_pool.key())
    }

    /**
     * Update box
     *
     * @param - name of box
     *          description of box
     *          image url of box
     *          price in sol to open the box
     *          rewards
     */
    pub fn update_box<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateBox<'info>>,
        name: String,
        description: String,
        image_url: String,
        price_in_sol: u64,
        rewards: Vec<Reward>,
    ) -> Result<()> {
        let box_pool = &mut ctx.accounts.box_pool;

        require!(rewards.len() <= MAX_REWARD_IN_BOX, GameError::ExceedMaxReward);

        let mut sum: u16 = 0;
        for reward in rewards.iter() {
            sum += reward.chance
        }
        require!(sum == CHANCE_SUM, GameError::ChanceSumInvalid);

        box_pool.name = name;
        box_pool.description = description;
        box_pool.image_url = image_url;
        box_pool.price_in_sol = price_in_sol;
        box_pool.rewards = rewards;

        Ok(())
    }

    /**
     * Remove box
     */
    pub fn remove_box<'info>(ctx: Context<'_, '_, '_, 'info, RemoveBox<'info>>) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;
        let box_pool = &mut ctx.accounts.box_pool;
        let prize_pool = &mut ctx.accounts.prize_pool;

        require!(box_pool.sol_amount == 0, GameError::SolBalanceExist);
        for i in 0..global_pool.token_count as usize {
            require!(box_pool.token_amount[i] == 0, GameError::TokenBalanceExist);
        }

        require!(prize_pool.nfts.len() == 0, GameError::NFTBalanceExist);

        global_pool.remove_box(box_pool.key());

        Ok(())
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
    pub fn deposit_nfts<'info>(
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

    /**
     * Withdraw NFTs
     *
     * @param            - NFT list to withdraw
     * 
     * @remainingAccount - NFT's ATA(global_pool's ATA, admin's ATA) list to withdraw
     */
    pub fn withdraw_nfts<'info>(
        ctx: Context<'_, '_, '_, 'info, WithdrawNfts<'info>>,
        nfts: Vec<Pubkey>
    ) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;
        let prize_pool = &mut ctx.accounts.prize_pool;

        //  Transfer NFTs to admin wallet
        let remaining_accounts: Vec<AccountInfo> = ctx.remaining_accounts.to_vec();

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

    /**
     * Deposit SOL/Token to PDA
     *
     * @param - sol amount to deposit
     *        - token amount to deposit
     */
    pub fn deposit(ctx: Context<Deposit>, sol_amount: u64, token_amount: u64) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;
        let box_pool = &mut ctx.accounts.box_pool;

        msg!("Depositer: {}", ctx.accounts.admin.to_account_info().key());
        msg!(
            "Asking to deposit: {} SOL, {} Token {}",
            sol_amount,
            token_amount,
            ctx.accounts.token_mint.key(),
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

        if token_amount > 0 {
            let id = match global_pool.token_address.iter().position(|&token_mint| token_mint == ctx.accounts.token_mint.key()) {
                Some(id) => id,
                None => return err!(GameError::TokenAddressUnknown),
            };
            //  Transfer Token to PDA from admin
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
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

    /**
     * Withdraw SOL/Token from PDA
     *
     * @param - sol amount to withdraw
     *        - token amount to withdraw
     */
    pub fn withdraw(ctx: Context<Withdraw>, sol_amount: u64, token_amount: u64) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;
        let box_pool = &mut ctx.accounts.box_pool;
        msg!("Withdrawer: {}", ctx.accounts.admin.to_account_info().key());
        msg!(
            "Asking to withdraw: {} SOL, {} Token {}",
            sol_amount,
            token_amount,
            ctx.accounts.token_mint.key(),
        );

        let sol_vault_bump = ctx.bumps.sol_vault;

        let id = match global_pool.token_address.iter().position(|&token_mint| token_mint == ctx.accounts.token_mint.key()) {
            Some(id) => id,
            None => return err!(GameError::TokenAddressUnknown),
        };
        require!(box_pool.sol_amount >= sol_amount, GameError::InsufficientFunds);
        require!(box_pool.token_amount[id] >= token_amount, GameError::InsufficientFunds);

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

        if token_amount > 0 {
            //  Transfer Token to admin from PDA
            let global_bump = ctx.bumps.global_pool;
            let seeds = &[GLOBAL_AUTHORITY_SEED.as_bytes(), &[global_bump]];
            let signer = [&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
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

    /**
     * Initialize player pool
     */
    pub fn init_player(ctx: Context<InitPlayer>) -> Result<()> {
        let player_pool = &mut ctx.accounts.player_pool;

        player_pool.player = ctx.accounts.player.key();

        Ok(())
    }

    /**
     * Opens box with SOL
     * 
     * @param             - open times
     */
    pub fn open_box<'info>(
        ctx: Context<'_, '_, '_, 'info, OpenBox<'info>>,
        open_times: u16,
    ) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;
        let box_pool = &mut ctx.accounts.box_pool;
        let player_pool = &mut ctx.accounts.player_pool;
        let prize_pool = &mut ctx.accounts.prize_pool;

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
            let reward = &box_pool.rewards[i];

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

            require!(select == true, GameError::InsufficientFunds);
        }

        Ok(())
    }
    
    /**
     * Claim reward
     * 
     * @remainingAccounts - list of SPL token's ATA(global_pool's ATA, player's ATA) included in game
     *                    - list of claimable NFT's ATA(global_pool's ATA, player's ATA)
     */
    pub fn claim_reward<'info>(
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
                token::Transfer {
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
}
