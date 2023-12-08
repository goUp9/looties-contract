import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from '@solana/web3.js';

export interface GlobalPool {
    // 8 + 32
    superAdmin: PublicKey,
    tokenAddress: PublicKey[],
    tokenCount: anchor.BN,
    boxes: PublicKey[],
}

export interface BoxPool {
    admin: PublicKey,
    solAmount: anchor.BN,
    tokenAmount: anchor.BN,
    name: String,
    description: String,
    imageUrl: String,
    priceInSol: anchor.BN,
    prizes: PublicKey,
    rewards: Reward[],
}

export interface Reward {
    name: String,
    description: String,
    imageUrl: String,
    // 1: SOL, 2: SPL, 3: NFT.
    rewardType: number,
    // {%}*100, 100%=10_000
    chance: number,
    sol: anchor.BN,
    token: anchor.BN,
    tokenAddress: PublicKey,
    collectionAddress: PublicKey,
}

export interface PrizePool {
    nfts: NftInfo[],
}

export interface NftInfo {
    collectionAddress: PublicKey,
    mintInfo: PublicKey,
}

