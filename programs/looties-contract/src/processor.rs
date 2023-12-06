use anchor_lang::prelude::*;
use solana_program::program::{invoke, invoke_signed};
use solana_program::entrypoint::ProgramResult;

// transfer sol
pub fn sol_transfer_with_signer<'info>(
    source: AccountInfo<'info>,
    destination: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    signers: &[&[&[u8]]; 1],
    amount: u64,
) -> ProgramResult {
    let ix = solana_program::system_instruction::transfer(source.key, destination.key, amount);
    invoke_signed(&ix, &[source, destination, system_program], signers)
}

pub fn sol_transfer_user<'info>(
    source: AccountInfo<'info>,
    destination: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    amount: u64,
) -> ProgramResult {
    let ix = solana_program::system_instruction::transfer(source.key, destination.key, amount);
    invoke(&ix, &[source, destination, system_program])
}
