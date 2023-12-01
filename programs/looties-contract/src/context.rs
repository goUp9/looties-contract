use crate::account::*;
use anchor_lang::prelude::*;
use anchor_spl::{token::Token, token_interface::TokenAccount};

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub initializer_receive_token_account: InterfaceAccount<'info, TokenAccount>,
    // Escrow account
    #[account(init, payer = initializer, space = 8 + EscrowAccount::LEN)]
    pub escrow_account: Account<'info, EscrowAccount>,
    // system
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
