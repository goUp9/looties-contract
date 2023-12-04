use crate::account::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    token_interface::{SetAuthority, TokenAccount},
};

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(mut)]
    pub pda_associate_token_account: InterfaceAccount<'info, TokenAccount>,
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
pub struct DepositSOL<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    /// CHECK:
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info> From<&mut InitializeEscrow<'info>>
    for CpiContext<'_, '_, '_, 'info, SetAuthority<'info>>
{
    fn from(accounts: &mut InitializeEscrow<'info>) -> Self {
        let cpi_accounts = SetAuthority {
            account_or_mint: accounts
                .pda_associate_token_account
                .to_account_info()
                .clone(),
            current_authority: accounts.initializer.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}