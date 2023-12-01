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
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"lotties_escrow_pda_seed";

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        name: String,
        description: String,
        price: u32,
        image_url: String,
        rewards: Vec<Reward>,
    ) -> Result<()> {
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
}

