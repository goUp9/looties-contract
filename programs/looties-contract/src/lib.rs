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

        global_pool.admin = ctx.accounts.admin.key();
        global_pool.token_count = 0;

        Ok(())
    }

    /**
     * Change admin
     *
     * @param - new_admin
     */
    pub fn change_admin(ctx: Context<UpdateGlobal>, new_admin: Pubkey) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;

        // Don't need check admin since it signed the transaction
        global_pool.admin = new_admin;

        Ok(())
    }

    /**
     * Create new box
     *
     * @param - random address to use as seed
     *          name of box
     *          description of box
     *          image url of box
     *          price in sol to open the box
     *          rewards
     */
    pub fn init_box<'info>(
        ctx: Context<'_, '_, '_, 'info, InitBox<'info>>,
        _rand: Pubkey,
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
            let token_mint = reward.token_address.unwrap();
            if reward.reward_type == 2 && global_pool.token_address.iter().all(|&x| x != token_mint) {
                return err!(GameError::TokenAddressUnknown);
            }
        }
        require!(sum == CHANCE_SUM, GameError::ChanceSumInvalid);

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
     *
     * @remainingAccount - NFT's ATA(global_pool's ATA, admin's ATA) list included in box
     */
    pub fn remove_box<'info>(ctx: Context<'_, '_, '_, 'info, RemoveBox<'info>>) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;
        let box_pool = &mut ctx.accounts.box_pool;
        let prize_pool = &mut ctx.accounts.prize_pool;

        global_pool.remove_box(box_pool.key());

        //  Transfer NFTs to admin wallet
        let remaining_accounts: Vec<AccountInfo> = ctx.remaining_accounts.to_vec();

        let mut idx = 0;
        let nfts = &prize_pool.nfts;

        require!(remaining_accounts.len() == nfts.len() * 2, GameError::RemainingAccountCountDismatch);

        for nft in nfts {
            let src_ata = spl_associated_token_account::get_associated_token_address(
                &global_pool.key(),
                &nft.mint_info,
            );
            require!(
                remaining_accounts[idx].key().eq(&src_ata),
                GameError::SrcAtaDismatch
            );

            let dest_ata = spl_associated_token_account::get_associated_token_address(
                &ctx.accounts.admin.key(),
                &nft.mint_info,
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

            idx += 2;
        }

        prize_pool.nfts.clear();

        Ok(())
    }

    /**
     * Deposit NFTs
     *
     * @param            - list of nft collection address
     *                   - list of nft mint address
     * 
     * @remainingAccount - NFT's ATA(admin's ATA, global_pool's ATA) list included in box
     * 
     */
    pub fn deposit_nfts<'info>(
        ctx: Context<'_, '_, '_, 'info, DepositNfts<'info>>,
        collection_addr: Vec<Pubkey>,
        mint_addr: Vec<Pubkey>,
    ) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;
        let box_pool = &mut ctx.accounts.box_pool;
        let prize_pool = &mut ctx.accounts.prize_pool;
        let remaining_accounts: Vec<AccountInfo> = ctx.remaining_accounts.to_vec();

        let len = collection_addr.len();
        require!(remaining_accounts.len() == len * 2, GameError::RemainingAccountCountDismatch);
        require!(mint_addr.len() == len, GameError::ArgumentInvalid);
        
        //  Transfer NFTs to program ATA
        let mut idx = 0;
        for i in 0..len {
            let collection = collection_addr[i];

            require!(box_pool.rewards.iter().any(|reward| {
                if let Some(collection_addr) = reward.collection_address {
                    reward.reward_type == 3 && collection_addr == collection
                } else {
                    false
                }
            }), GameError::CollectionAddressNotExsit);

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

            prize_pool.add_nft(collection, nft)?;

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
        ctx: Context<'_, '_, '_, 'info, RemoveBox<'info>>,
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

        if !global_pool.token_address.iter().all(|&token_mint| token_mint != ctx.accounts.token_mint.key()) {
            require!(global_pool.token_count < MAX_TOKEN_IN_GAME as u64, GameError::ExceedMaxToken);
            global_pool.token_address.push(ctx.accounts.token_mint.key());
        }

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
        }

        if token_amount > 0 {
            //  Transfer Token to PDA from admin
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_admin.to_account_info(),
                    authority: ctx.accounts.token_vault.to_account_info(),
                    to: ctx.accounts.token_vault.to_account_info(),
                },
            );
            token::transfer(cpi_ctx, token_amount)?;
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
        msg!("Withdrawer: {}", ctx.accounts.admin.to_account_info().key());
        msg!(
            "Asking to withdraw: {} SOL, {} Token {}",
            sol_amount,
            token_amount,
            ctx.accounts.token_mint.key(),
        );

        let sol_vault_bump = ctx.bumps.sol_vault;
        let token_vault_bump = ctx.bumps.token_vault;

        if sol_amount > 0 {
            sol_transfer_with_signer(
                ctx.accounts.sol_vault.to_account_info(),
                ctx.accounts.admin.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                &[&[SOL_VAULT_SEED.as_ref(), &[sol_vault_bump]]],
                sol_amount,
            )?;
        }

        if token_amount > 0 {
            //  Transfer Token to admin from PDA
            let token_address = ctx.accounts.token_mint.key();
            let seeds = &[token_address.as_ref(), &[token_vault_bump]];
            let signer = [&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_vault.to_account_info(),
                    authority: ctx.accounts.token_vault.to_account_info(),
                    to: ctx.accounts.token_admin.to_account_info(),
                },
                &signer,
            );
            token::transfer(cpi_ctx, token_amount)?;
        }

        Ok(())
    }

    /**
     * Opens box with SOL
     * 
     * @param             - open times
     * 
     * @remainingAccounts - list of SPL token's ATA(global_pool's ATA, player's ATA) included in game
     *                    - list of NFT's ATA(global_pool's ATA, player's ATA) included in box
     */
    pub fn open_box<'info>(
        ctx: Context<'_, '_, '_, 'info, OpenBox<'info>>,
        open_times: u16,
    ) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;
        let box_pool = &mut ctx.accounts.box_pool;
        let prize_pool = &mut ctx.accounts.prize_pool;
        let remaining_accounts: Vec<AccountInfo> = ctx.remaining_accounts.to_vec();

        //  Check box is legit
        require!(global_pool.boxes.contains(&box_pool.key()), GameError::BoxAddressUnknown);

        require!(remaining_accounts.len() == (global_pool.token_count as usize + prize_pool.nfts.len()) * 2, GameError::RemainingAccountCountDismatch);

        msg!("Open box- player: {}, open times: {}", ctx.accounts.player.key(), open_times);

        // Transfer SOL to vault from player.
        sol_transfer_user(
            ctx.accounts.player.to_account_info(),
            ctx.accounts.sol_vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            box_pool.price_in_sol * open_times as u64,
        )?;

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

        msg!("Rand: {:?}", random_numbers);
        msg!("Reward item index: {:?}", reward_idxs);

        let mut reward_sol = 0;
        let mut reward_tokens = vec![0; global_pool.token_count as usize];
        let mut reward_nfts = vec![];
        let mut reward_nft_idxs = vec![];

        for reward_id in reward_idxs {
            let reward = &box_pool.rewards[reward_id as usize];

            match reward.reward_type {
                1 => {
                    reward_sol += reward.sol;
                },
                2 => {
                    let token_mint = reward.token_address.unwrap();
                    let index = global_pool.token_address.iter().position(|&x| x == token_mint).unwrap();
                    reward_tokens[index] += reward.token;
                },
                3 => {
                    let collection = reward.collection_address.unwrap();
                    let id = prize_pool.find_nft(collection, &reward_nfts)?;
                    reward_nft_idxs.push(id);
                    reward_nfts.push(prize_pool.nfts[id].mint_info);
                },
                _ => {}
            }
        }

        let sol_vault_bump = ctx.bumps.sol_vault;

        if reward_sol > 0 {
            sol_transfer_with_signer(
                ctx.accounts.sol_vault.to_account_info(),
                ctx.accounts.player.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                &[&[SOL_VAULT_SEED.as_ref(), &[sol_vault_bump]]],
                reward_sol,
            )?;
        }

        for (idx, &reward_token) in reward_tokens.iter().enumerate() {
            if reward_token > 0 {
                //  Transfer Token to user from PDA
                let token_address = global_pool.token_address[idx];
                let (src_ata, bump) = Pubkey::find_program_address(&[token_address.as_ref()], ctx.program_id);
                let seeds = &[token_address.as_ref(), &[bump]];
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
                        authority: remaining_accounts[account_idx].to_account_info(),
                        to: remaining_accounts[account_idx + 1].to_account_info(),
                    },
                    &signer,
                );
                token::transfer(cpi_ctx, reward_token)?;
            }
        }


        for nft_idx in reward_nft_idxs {
            let nft = &prize_pool.nfts[nft_idx];
            let idx = 2 * global_pool.token_count as usize + nft_idx * 2;

            let src_ata = spl_associated_token_account::get_associated_token_address(
                &global_pool.key(),
                &nft.mint_info,
            );
            require!(
                remaining_accounts[idx].key().eq(&src_ata),
                GameError::SrcAtaDismatch
            );

            let dest_ata = spl_associated_token_account::get_associated_token_address(
                &ctx.accounts.player.key(),
                &nft.mint_info,
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
        }

        for nft in reward_nfts {
            prize_pool.remove_nft(nft);
        }

        Ok(())
    }
}
