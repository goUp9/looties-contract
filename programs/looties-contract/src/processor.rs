use anchor_lang::prelude::*;
use solana_program::program::{invoke, invoke_signed};
use solana_program::entrypoint::ProgramResult;
use sha2::{Digest, Sha256};

use crate::account::Reward;

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

pub fn calc_reward(rewards: &Vec<Reward>, rand: u16) -> u8 {
    let mut cur_sum: u16 = 0;
    let mut idx: u8 = 0;

    for reward in rewards.iter() {
        cur_sum += reward.chance;
        if rand <= cur_sum {
            continue;
        }
        idx += 1;
    }

    return idx;
}

//  Generate pseudo random number

pub struct HashStruct {
    pub nonce: u64,
    pub initial_seed: u64,
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

pub fn get_rand(seed: u64, nonce: u64) -> u64 {
    let hashstruct = HashStruct {
        nonce,
        initial_seed: seed,
    };
    let vec_to_hash = unsafe { self::any_as_u8_slice(&hashstruct) };
    let hash = &(Sha256::new().chain_update(vec_to_hash).finalize()[..32]);

    // hash is a vector of 32 8bit numbers.  We can take slices of this to generate our 4 random u64s
    let mut hashed_randoms: [u64; 4] = [0; 4];
    for i in 0..4 {
        let hash_slice = &hash[i * 8..(i + 1) * 8];
        hashed_randoms[i] =
            u64::from_le_bytes(hash_slice.try_into().expect("slice with incorrect length"));
    }

    return hashed_randoms[3];
}
