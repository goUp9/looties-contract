use crate::account::*;
use anchor_lang::prelude::*;
use anchor_spl::{token::Token, token_interface::TokenAccount};

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub initializer_receive_token_account: InterfaceAccount<'info, TokenAccount>,
    // Escrow account
    #[account(init, payer = initializer, space = 8 + EscrowAccount::INIT_SPACE)]
    pub escrow_account: Account<'info, EscrowAccount>,
    // system
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UpdateEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub initializer_receive_token_account: InterfaceAccount<'info, TokenAccount>,
    // Escrow account
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    // system
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(deposit_amount: u64)]
pub struct DepositSOL<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        mut,
        constraint = initializer_deposit_token_account.amount >= deposit_amount
    )]
    pub initializer_deposit_token_account: InterfaceAccount<'info, TokenAccount>,
    // Escrow account
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    // system
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
