use account::*;
use anchor_lang::prelude::*;
use context::*;
use error::*;

mod account;
mod context;
mod error;

declare_id!("t1ynC7jhTJfZD8idR58Yz6EW8XiwajKzNXusf2tguBV");

#[program]
pub mod looties_contract {
    use anchor_lang::solana_program::system_instruction;
    use anchor_spl::token_interface::{self, spl_token_2022::instruction::AuthorityType};

    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"lotties_escrow_pda_seed";

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        name: String,
        description: String,
        price: u64,
        image_url: String,
        rewards: Vec<Reward>,
    ) -> Result<()> {
        msg!("this is price {}", price);
        let escrow_account = ctx.accounts.escrow_account.to_account_info();
        if !ctx.accounts.rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            return err!(EscrowError::NotRentExempt);
        }

        let mut chance_sum = 0;
        for reward in rewards.iter() {
            chance_sum += reward.chance;
        }

        if chance_sum != 100_000 {
            return err!(EscrowError::ChanceSumInvalid);
        }

        ctx.accounts.escrow_account.initializer_key = *ctx.accounts.initializer.key;
        ctx.accounts
            .escrow_account
            .pda_associate_token_account = *ctx
            .accounts
            .pda_associate_token_account
            .to_account_info()
            .key;
        ctx.accounts
            .escrow_account
            .initializer_receive_token_account = *ctx
            .accounts
            .initializer_receive_token_account
            .to_account_info()
            .key;
        ctx.accounts.escrow_account.name = name;
        ctx.accounts.escrow_account.description = description;
        ctx.accounts.escrow_account.price = price;
        ctx.accounts.escrow_account.image_url = image_url;
        ctx.accounts.escrow_account.rewards = rewards;

        let (pda, _bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        token_interface::set_authority(
            ctx.accounts.into(),
            AuthorityType::AccountOwner,
            Some(pda),
        )?;

        Ok(())
    }

    pub fn update_escrow(
        ctx: Context<UpdateEscrow>,
        name: String,
        description: String,
        price: u64,
        image_url: String,
        rewards: Vec<Reward>,
    ) -> Result<()> {
        if ctx.accounts.escrow_account.initializer_key != *ctx.accounts.initializer.key {
            return err!(EscrowError::AccessDenied);
        }

        let mut chance_sum = 0;
        for reward in rewards.iter() {
            chance_sum += reward.chance;
        }

        if chance_sum != 100_000 {
            return err!(EscrowError::ChanceSumInvalid);
        }

        ctx.accounts
            .escrow_account
            .initializer_receive_token_account = *ctx
            .accounts
            .initializer_receive_token_account
            .to_account_info()
            .key;
        ctx.accounts.escrow_account.name = name;
        ctx.accounts.escrow_account.description = description;
        ctx.accounts.escrow_account.price = price;
        ctx.accounts.escrow_account.image_url = image_url;
        ctx.accounts.escrow_account.rewards = rewards;

        Ok(())
    }

    pub fn deposit_sol(
        ctx: Context<DepositSOL>,
        deposit_amount: u64,
    ) -> Result<()> {
        let from_account = &ctx.accounts.from;
        let to_account = &ctx.accounts.to;

        // Create the transfer instruction
        let transfer_instruction = system_instruction::transfer(from_account.key, to_account.key, deposit_amount);

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                from_account.to_account_info(),
                to_account.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        Ok(())
    }
}
