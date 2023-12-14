import { Program, web3 } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Keypair, ParsedAccountData, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction } from "@solana/web3.js";
import fs from 'fs';
import { GlobalPool, BoxPool, PrizePool } from './types';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
const idl = require('../target/idl/looties_contract.json');

const PROGRAM_ID = "t1ynC7jhTJfZD8idR58Yz6EW8XiwajKzNXusf2tguBV";
const GLOBAL_AUTHORITY_SEED = "global-authority";
const BOX_AUTHORITY_SEED = "box-authority";
const SOL_VAULT_SEED = "sol-vault";

anchor.setProvider(anchor.AnchorProvider.local(web3.clusterApiUrl('devnet')));
const solConnection = anchor.getProvider().connection;
const wallet = anchor.AnchorProvider.local().wallet as anchor.Wallet;
const payer = wallet.payer;
const authority = payer.publicKey;
const provider = anchor.getProvider();

const tokenAAddress = new anchor.web3.PublicKey("2o2CadSEJ9D3RLoqBgL5L6cSuxQWd2ik175igkAACtPB");
const tokenBAddress = new anchor.web3.PublicKey("HXSWKQVRybmkRZvq8F8bkG2FT2JSqjGeAtnJxvDWrTeM");
const NFTcollection1 = new anchor.web3.PublicKey("7kZwsnGkJKRkit73yQC96XCzrEDBBzo7iD2s8FME5HtL");
const NFTcollection2 = new anchor.web3.PublicKey("7pXX8wRpGJzKaq2VPX1vjAXurtzekbfApMnjZuZsEicX");

console.log("payer : ", payer.publicKey.toBase58());

let program: Program = null;

// Address of the deployed program.
const programId = new anchor.web3.PublicKey(PROGRAM_ID);

// Generate the program client from IDL.
program = new anchor.Program(idl, programId);
console.log('ProgramId: ', program.programId.toBase58());

const main = async () => {
    const [globalAuthority, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );
    console.log('GlobalAuthority: ', globalAuthority.toBase58());

    // let boxPool = new anchor.web3.PublicKey("7RxiU83ApEACnFAd4Wp7gEd9R83SVmBfPvWC8cWdv3hY");
    // let boxPool = new anchor.web3.PublicKey("FCC4zEUSdAwdHiyvjeYinQKWDRdEfZNR3YzPZ95D1NRf");
    // let boxPool = new anchor.web3.PublicKey("7GRHddMmWGMSuut9NdvZfcCZG3r4HcPEdNuZYJEeyKn4");
    let boxPool = new anchor.web3.PublicKey("97i7UZ78FV97ixBkeWscAhvbsKVCujgjkyWcoYXpdNcJ");
    
    // await initProject(payer.publicKey);
    // super admin: CPvqXDUJBwGDH9e2SadrQzFYqCaKiF2UXmxgqkcQdYTZ
    // admin : 5RoELXPzGfPFJ8DqHXX6QmgLguYERWfptPC3SUkwCBGz
    // await changeSuperAdmin(payer.publicKey, new anchor.web3.PublicKey("CPvqXDUJBwGDH9e2SadrQzFYqCaKiF2UXmxgqkcQdYTZ"));
    // await addTokenAddress(payer.publicKey, tokenAAddress);
    // await initBoxTest();
    // await updateBoxTest(boxPool);

    let prizePool = new anchor.web3.PublicKey("EDyyhHVFr1QpG7FahJFDhSTVFmvCtm2rApUHEpxPgJsV");
    // await createTokenWalletForTest(payer.publicKey, boxPool, new anchor.BN(0), new anchor.BN(20 * 10 ** 9), tokenAAddress);
    // await createTokenWalletForTest(payer.publicKey, boxPool, new anchor.BN(0), new anchor.BN(20 * 10 ** 9), tokenBAddress);
    // await deposit(payer.publicKey, boxPool, new anchor.BN(20 * 10 ** 9), new anchor.BN(0 * 10 ** 9), tokenAAddress);
    // await deposit(payer.publicKey, boxPool, new anchor.BN(0), new anchor.BN(20 * 10 ** 9), tokenBAddress);
    // await withdraw(payer.publicKey, boxPool, new anchor.BN(10 ** 9), new anchor.BN(0), tokenAAddress);
    // await withdraw(payer.publicKey, boxPool, new anchor.BN(0), new anchor.BN(10 * 10 ** 9), tokenBAddress);

    // let collection1 = [
    //     new anchor.web3.PublicKey("3owqfGtGcnoGapVikreKCFqARGDn5dQqEiStYLSrHeDi"),
    //     new anchor.web3.PublicKey("9ARBwxAsTtNiJv3UYjrvNvJH81HUvqCSd7TeThNiTmKi"),
    //     new anchor.web3.PublicKey("4TV9nj18LTdnaRrPweGR4ZAuo9FzyxYrSwoHtnA7p6ny"),
    // ];
    // let collection2 = [
    //     new anchor.web3.PublicKey("6bhuV5vizoeheBZ5cPV3rP3bqwFQoPpZZFQoAphvps93"),
    //     new anchor.web3.PublicKey("H9oLT7uK1njJESsDFr1M7oA8ASnZYqvN6Qvc9rfFjQ1f"),
    // ];
    // await createNFTsAccountForTest(payer.publicKey, collection1);
    // await createNFTsAccountForTest(payer.publicKey, collection2);

    // await depositNfts(payer.publicKey, boxPool, [NFTcollection1, NFTcollection1, NFTcollection1], collection1);
    // await depositNfts(payer.publicKey, boxPool, [NFTcollection2, NFTcollection2], collection2);
    // await withdrawNfts(payer.publicKey, boxPool, [new anchor.web3.PublicKey("3owqfGtGcnoGapVikreKCFqARGDn5dQqEiStYLSrHeDi")]);

    // await openBox(payer.publicKey, boxPool, 5);

    // console.log(await getGlobalPool());
    console.log(await getBoxPool(boxPool));
    
    console.log("Game Vault");
    
    // const [solVault, svBump] = await PublicKey.findProgramAddress(
    //     [Buffer.from(SOL_VAULT_SEED)],
    //     program.programId
    // );
    
    // let balance = await provider.connection.getBalance(solVault);
    // console.log("SOL : ", balance.toString());
    // let gameTokenAccount = await getAssociatedTokenAccount(globalAuthority, tokenAAddress);
    // let accountInfo = await provider.connection.getTokenAccountBalance(gameTokenAccount);
    // console.log("TOken A : ", accountInfo.value.uiAmount);
    // gameTokenAccount = await getAssociatedTokenAccount(globalAuthority, tokenBAddress);
    // accountInfo = await provider.connection.getTokenAccountBalance(gameTokenAccount);
    // console.log("TOken B : ", accountInfo.value.uiAmount);
    // console.log("NFTs : ", (await getPrizePool(prizePool)).nfts.map((nft) => nft.mintInfo));
}

const initBoxTest = async () => {
    let defaultKey = PublicKey.default;

    let rewards = [
        {
            name: "1 SOL",
            description: "reward 1 sol",
            imageUrl: "https://no_url",
            rewardType: 1,
            chance: 10 * 10 ** 2,
            sol: new anchor.BN(1 * 10 ** 6),
            token: new anchor.BN(0),
            tokenAddress: defaultKey,
            collectionAddress: defaultKey,
        },
        {
            name: "2 SOL",
            description: "reward 2 sol",
            imageUrl: "https://no_url",
            rewardType: 1,
            chance: 10 * 10 ** 2,
            sol: new anchor.BN(2 * 10 ** 6),
            token: new anchor.BN(0),
            tokenAddress: defaultKey,
            collectionAddress: defaultKey,
        },
        {
            name: "1 TokenA",
            description: "reward 1 TokenA",
            imageUrl: "https://no_url",
            rewardType: 2,
            chance: 10 * 10 ** 2,
            sol: new anchor.BN(0),
            token: new anchor.BN(1 * 10 ** 6),
            tokenAddress: tokenAAddress,
            collectionAddress: defaultKey,
        },
        {
            name: "1 TokenB",
            description: "reward 1 TokenB",
            imageUrl: "https://no_url",
            rewardType: 2,
            chance: 10 * 10 ** 2,
            sol: new anchor.BN(0),
            token: new anchor.BN(1 * 10 ** 6),
            tokenAddress: tokenBAddress,
            collectionAddress: defaultKey,
        },
        {
            name: "NFT1",
            description: "NFT1",
            imageUrl: "https://no_url",
            rewardType: 3,
            chance: 30 * 10 ** 2,
            sol: new anchor.BN(0),
            token: new anchor.BN(0),
            tokenAddress: defaultKey,
            collectionAddress: NFTcollection1,
        },
        {
            name: "NFT2",
            description: "NFT2",
            imageUrl: "",
            rewardType: 3,
            chance: 30 * 10 ** 2,
            sol: new anchor.BN(0),
            token: new anchor.BN(0),
            tokenAddress: defaultKey,
            collectionAddress: NFTcollection2,
        },
    ]

    let rewards_2 = [
        {
            name: "Hell Case",
            description: "Hell Case",
            imageUrl: "https://looties-next-app.vercel.app/assets/cases/case-1.svg",
            rewardType: 1,
            chance: 15 * 10 ** 2,
            sol: new anchor.BN(0.5 * 10 ** 9),
            token: new anchor.BN(0),
            tokenAddress: defaultKey,
            collectionAddress: defaultKey,
        },
        {
            name: "Sussy Case",
            description: "Sussy Case",
            imageUrl: "https://looties-next-app.vercel.app/assets/cases/case-2.svg",
            rewardType: 1,
            chance: 10 * 10 ** 2,
            sol: new anchor.BN(0.69 * 10 ** 9),
            token: new anchor.BN(0),
            tokenAddress: defaultKey,
            collectionAddress: defaultKey,
        },
        {
            name: "Mask Case",
            description: "Mask Case",
            imageUrl: "https://looties-next-app.vercel.app/assets/cases/case-3.svg",
            rewardType: 1,
            chance: 30 * 10 ** 2,
            sol: new anchor.BN(0.05 * 10 ** 9),
            token: new anchor.BN(0),
            tokenAddress: defaultKey,
            collectionAddress: defaultKey,
        },
        {
            name: "Rio Case 2022",
            description: "Rio Case 2022",
            imageUrl: "https://looties-next-app.vercel.app/assets/cases/case-4.svg",
            rewardType: 1,
            chance: 45 * 10 ** 2,
            sol: new anchor.BN(0.05 * 10 ** 9),
            token: new anchor.BN(0),
            tokenAddress: defaultKey,
            collectionAddress: defaultKey,
        },
    ]
    await initBox(
        payer.publicKey,
        new anchor.web3.PublicKey("5RoELXPzGfPFJ8DqHXX6QmgLguYERWfptPC3SUkwCBGz"),
        "Radioactive Case",
        "Radioactive Case",
        "https://looties-next-app.vercel.app/sample-case.svg", new anchor.BN(0.11 * 10 ** 9),
        rewards_2,
        Keypair.generate().publicKey
    );
}

const updateBoxTest = async (boxAddress: PublicKey) => {
    let defaultKey = PublicKey.default;

    let rewards = [
        {
            name: "1 SOL",
            description: "reward 1 sol",
            imageUrl: "https://no_url",
            rewardType: 1,
            chance: 10 * 10 ** 2,
            sol: new anchor.BN(1 * 10 ** 6),
            token: new anchor.BN(0),
            tokenAddress: defaultKey,
            collectionAddress: defaultKey,
        },
        {
            name: "2 SOL",
            description: "reward 2 sol",
            imageUrl: "https://no_url",
            rewardType: 1,
            chance: 10 * 10 ** 2,
            sol: new anchor.BN(2 * 10 ** 6),
            token: new anchor.BN(0),
            tokenAddress: defaultKey,
            collectionAddress: defaultKey,
        },
        {
            name: "1 TokenA",
            description: "reward 1 TokenA",
            imageUrl: "https://no_url",
            rewardType: 2,
            chance: 10 * 10 ** 2,
            sol: new anchor.BN(0),
            token: new anchor.BN(1 * 10 ** 6),
            tokenAddress: tokenAAddress,
            collectionAddress: defaultKey,
        },
        {
            name: "2 TokenA",
            description: "reward 2 TokenA",
            imageUrl: "https://no_url",
            rewardType: 2,
            chance: 10 * 10 ** 2,
            sol: new anchor.BN(0),
            token: new anchor.BN(2 * 10 ** 6),
            tokenAddress: tokenAAddress,
            collectionAddress: defaultKey,
        },
        {
            name: "1 TokenB",
            description: "reward 1 TokenB",
            imageUrl: "https://no_url",
            rewardType: 2,
            chance: 30 * 10 ** 2,
            sol: new anchor.BN(0),
            token: new anchor.BN(1 * 10 ** 6),
            tokenAddress: tokenBAddress,
            collectionAddress: defaultKey,
        },
        {
            name: "2 TokenB",
            description: "reward 2 TokenB",
            imageUrl: "https://no_url",
            rewardType: 2,
            chance: 30 * 10 ** 2,
            sol: new anchor.BN(0),
            token: new anchor.BN(2 * 10 ** 6),
            tokenAddress: tokenBAddress,
            collectionAddress: defaultKey,
        },
    ]

    await updateBox(
        payer.publicKey,
        boxAddress,
        "First Box",
        "This is First Box in Game to test",
        "https://no_url", new anchor.BN(10 ** 6),
        rewards,
    );
}

const createTokenWalletForTest = async (
    admin: PublicKey,
    boxAddress: PublicKey,
    solAmount: anchor.BN,
    tokenAmount: anchor.BN,
    tokenAddress: PublicKey,
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [solVault, svBump] = await PublicKey.findProgramAddress(
        [Buffer.from(SOL_VAULT_SEED)],
        program.programId
    );

    console.log("|______________________________");
    let adminTokenAccount = await getAssociatedTokenAccount(admin, tokenAddress);
    console.log("|______________________________");
    let gameTokenAccount = await getAssociatedTokenAccount(globalAuthority, tokenAddress);
    console.log("|______________________________");

    {
        let tx = new Transaction();
        let { instructions, destinationAccounts } = await getATokenAccountsNeedCreate(
            solConnection,
            admin,
            admin,
            [tokenAddress]
        );
        if (instructions.length > 0) {
            instructions.map((ix) => tx.add(ix));
            await provider.sendAndConfirm(tx);
        }
    }
    console.log("|______________________________");

    {
        let tx = new Transaction();
        let { instructions, destinationAccounts } = await getATokenAccountsNeedCreate(
            solConnection,
            admin,
            globalAuthority,
            [tokenAddress]
        );
    
        if (instructions.length > 0) {
            instructions.map((ix) => tx.add(ix));
            await provider.sendAndConfirm(tx);
        }
    }

    let depositAmount = 2 * tokenAmount.toNumber();

    console.log("|______________________________");

    let accountInfo = await provider.connection.getTokenAccountBalance(adminTokenAccount);
    console.log(accountInfo);

    let tx = new Transaction();
    tx.add(Token.createMintToInstruction(
        TOKEN_PROGRAM_ID,
        tokenAddress,
        adminTokenAccount,
        payer.publicKey,
        [],
        depositAmount,
    ));
    await provider.sendAndConfirm(tx);
    accountInfo = await provider.connection.getTokenAccountBalance(adminTokenAccount);
    console.log(accountInfo);
}

const createNFTsAccountForTest = async (
    admin: PublicKey,
    nfts: PublicKey[],
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    {
        let tx = new Transaction();
        let { instructions, destinationAccounts } = await getATokenAccountsNeedCreate(
            solConnection,
            admin,
            admin,
            nfts
        );
        if (instructions.length > 0) {
            instructions.map((ix) => tx.add(ix));
            await provider.sendAndConfirm(tx);
        }
    }

    {
        let tx = new Transaction();
        let { instructions, destinationAccounts } = await getATokenAccountsNeedCreate(
            solConnection,
            admin,
            globalAuthority,
            nfts
        );
        if (instructions.length > 0) {
            instructions.map((ix) => tx.add(ix));
            await provider.sendAndConfirm(tx);
        }
    }
}

export const initProject = async (
    superAdmin: PublicKey,
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    console.log('==>Initializing program');

    let txId = await program.methods
        .initialize()
        .accounts({
            superAdmin,
            globalPool: globalAuthority,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .rpc();

    console.log("txHash =", txId);    
}

export const changeSuperAdmin = async (
    superAdmin: PublicKey,
    newSuperAdmin: PublicKey,
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    console.log('==>changeSuperAdmin to : ', newSuperAdmin.toString());

    let txId = await program.methods
        .changeSuperAdmin(newSuperAdmin)
        .accounts({
            superAdmin,
            globalPool: globalAuthority,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .rpc();

    console.log("txHash =", txId);    
}

export const addTokenAddress = async (
    superAdmin: PublicKey,
    tokenAddress: PublicKey,
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    console.log('==>addTokenAddress : ', tokenAddress.toString());

    let txId = await program.methods
        .addTokenAddress(tokenAddress)
        .accounts({
            superAdmin,
            globalPool: globalAuthority,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .rpc();

    console.log("txHash =", txId);    
}

export const initBox = async (
    superAdmin: PublicKey,
    admin: PublicKey,
    name: String,
    description: String,
    imageUrl: String,
    priceInSol: anchor.BN,
    rewards: any,
    _rand: PublicKey,
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );
    
    const [boxAuthority, bBump] = await PublicKey.findProgramAddress(
        [Buffer.from(BOX_AUTHORITY_SEED), _rand.toBuffer()],
        program.programId
    );
    console.log(boxAuthority.toString())

    const [prizeAuthority, pBump] = await PublicKey.findProgramAddress(
        [boxAuthority.toBuffer()],
        program.programId
    );
    console.log(prizeAuthority.toString())

    console.log('==>initBox : ');

    let txId = await program.methods
        .initBox(admin, name, description, imageUrl, priceInSol, rewards)
        .accounts({
            superAdmin,
            globalPool: globalAuthority,
            boxPool: boxAuthority,
            prizePool: prizeAuthority,
            randKey: _rand,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .rpc();
    
    console.log("txHash =", txId);    
}

export const updateBox = async (
    superAdmin: PublicKey,
    boxAddress: PublicKey,
    name: String,
    description: String,
    imageUrl: String,
    priceInSol: anchor.BN,
    rewards: any,
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );
    
    const [prizeAuthority, pBump] = await PublicKey.findProgramAddress(
        [boxAddress.toBuffer()],
        program.programId
    );
    console.log(prizeAuthority.toString())

    console.log('==>updateBox : ');

    let txId = await program.methods
        .updateBox(name, description, imageUrl, priceInSol, rewards)
        .accounts({
            superAdmin,
            globalPool: globalAuthority,
            boxPool: boxAddress,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .rpc();
    
    console.log("txHash =", txId);    
}

export const deposit = async (
    admin: PublicKey,
    boxAddress: PublicKey,
    solAmount: anchor.BN,
    tokenAmount: anchor.BN,
    tokenAddress: PublicKey,
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [solVault, svBump] = await PublicKey.findProgramAddress(
        [Buffer.from(SOL_VAULT_SEED)],
        program.programId
    );

    let adminTokenAccount = await getAssociatedTokenAccount(admin, tokenAddress);
    let gameTokenAccount = await getAssociatedTokenAccount(globalAuthority, tokenAddress);

    console.log('==>deposit : ', solAmount, " SOL : ", tokenAmount, " Token(", tokenAddress.toString(), ")");

    let accountInfo = await provider.connection.getTokenAccountBalance(adminTokenAccount);
    console.log("admin token before", accountInfo.value.uiAmount);
    accountInfo = await provider.connection.getTokenAccountBalance(gameTokenAccount);
    console.log("game token before", accountInfo.value.uiAmount);

    let txId = await program.methods
        .deposit(solAmount, tokenAmount)
        .accounts({
            admin,
            globalPool: globalAuthority,
            boxPool: boxAddress,
            solVault,
            tokenAdmin: adminTokenAccount,
            tokenVault: gameTokenAccount,
            tokenMint: tokenAddress,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();

    console.log("txHash =", txId);    
    accountInfo = await provider.connection.getTokenAccountBalance(adminTokenAccount);
    console.log("admin token after ", accountInfo.value.uiAmount);
    accountInfo = await provider.connection.getTokenAccountBalance(gameTokenAccount);
    console.log("game token after", accountInfo.value.uiAmount);
}

export const withdraw = async (
    admin: PublicKey,
    boxAddress: PublicKey,
    solAmount: anchor.BN,
    tokenAmount: anchor.BN,
    tokenAddress: PublicKey,
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [solVault, svBump] = await PublicKey.findProgramAddress(
        [Buffer.from(SOL_VAULT_SEED)],
        program.programId
    );

    let adminTokenAccount = await getAssociatedTokenAccount(admin, tokenAddress);
    let gameTokenAccount = await getAssociatedTokenAccount(globalAuthority, tokenAddress);

    console.log('==>withdraw : ', solAmount, " SOL : ", tokenAmount, " Token(", tokenAddress.toString(), ")");

    let accountInfo = await provider.connection.getTokenAccountBalance(adminTokenAccount);
    let balance = await provider.connection.getBalance(payer.publicKey);
    console.log("admin token before : SOL : ", balance, "Token : ", accountInfo.value.uiAmount);
    accountInfo = await provider.connection.getTokenAccountBalance(gameTokenAccount);
    balance = await provider.connection.getBalance(solVault);
    console.log("game token before : SOL : ", balance, "Token : ", accountInfo.value.uiAmount);

    let txId = await program.methods
        .withdraw(solAmount, tokenAmount)
        .accounts({
            admin,
            globalPool: globalAuthority,
            boxPool: boxAddress,
            solVault,
            tokenAdmin: adminTokenAccount,
            tokenVault: gameTokenAccount,
            tokenMint: tokenAddress,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();

    console.log("txHash =", txId);    

    accountInfo = await provider.connection.getTokenAccountBalance(adminTokenAccount);
    balance = await provider.connection.getBalance(payer.publicKey);
    console.log("admin token before : SOL : ", balance, "Token : ", accountInfo.value.uiAmount);
    accountInfo = await provider.connection.getTokenAccountBalance(gameTokenAccount);
    balance = await provider.connection.getBalance(solVault);
    console.log("game token before : SOL : ", balance, "Token : ", accountInfo.value.uiAmount);
}

export const depositNfts = async (
    admin: PublicKey,
    boxAddress: PublicKey,
    collections: PublicKey[],
    nfts: PublicKey[],
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [solVault, svBump] = await PublicKey.findProgramAddress(
        [Buffer.from(SOL_VAULT_SEED)],
        program.programId
    );
    let prizeAddress = (await getBoxPool(boxAddress)).prizes;

    let remainingAccounts = [];

    for (var nft of nfts) {
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(admin, nft), isSigner: false, isWritable: true })
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(globalAuthority, nft), isSigner: false, isWritable: true })
    }

    let txId = await program.methods
        .depositNfts(collections, nfts)
        .accounts({
            admin,
            globalPool: globalAuthority,
            boxPool: boxAddress,
            prizePool: prizeAddress,
            tokenProgram: TOKEN_PROGRAM_ID,
        })
        .remainingAccounts(remainingAccounts)
        .rpc();

    console.log("txHash =", txId);    
}

export const withdrawNfts = async (
    admin: PublicKey,
    boxAddress: PublicKey,
    nfts: PublicKey[],
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [solVault, svBump] = await PublicKey.findProgramAddress(
        [Buffer.from(SOL_VAULT_SEED)],
        program.programId
    );
    let prizeAddress = (await getBoxPool(boxAddress)).prizes;

    let remainingAccounts = [];

    for (var nft of nfts) {
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(globalAuthority, nft), isSigner: false, isWritable: true })
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(admin, nft), isSigner: false, isWritable: true })
    }

    let txId = await program.methods
        .withdrawNfts(nfts)
        .accounts({
            admin,
            globalPool: globalAuthority,
            boxPool: boxAddress,
            prizePool: prizeAddress,
            tokenProgram: TOKEN_PROGRAM_ID,
        })
        .remainingAccounts(remainingAccounts)
        .rpc();

    console.log("txHash =", txId);    
}

export const openBox = async (
    player: PublicKey,
    boxAddress: PublicKey,
    openTimes: number,
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [solVault, svBump] = await PublicKey.findProgramAddress(
        [Buffer.from(SOL_VAULT_SEED)],
        program.programId
    );
    let prizeAddress = (await getBoxPool(boxAddress)).prizes;
    let nfts = (await getPrizePool(prizeAddress)).nfts;

    let globalPool = await getGlobalPool();

    let remainingAccounts = [];

    {
        let tx = new Transaction();
        let { instructions, destinationAccounts } = await getATokenAccountsNeedCreate(
            solConnection,
            player,
            player,
            globalPool.tokenAddress
        );
        if (instructions.length > 0) {
            instructions.map((ix) => tx.add(ix));
            await provider.sendAndConfirm(tx);
        }
    }

    for (var tokenAddress of globalPool.tokenAddress) {
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(globalAuthority, tokenAddress), isSigner: false, isWritable: true })
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(player, tokenAddress), isSigner: false, isWritable: true })
    }


    {
        let tx = new Transaction();
        let { instructions, destinationAccounts } = await getATokenAccountsNeedCreate(
            solConnection,
            player,
            player,
            nfts.map((nft) => nft.mintInfo)
        );
        if (instructions.length > 0) {
            instructions.map((ix) => tx.add(ix));
            await provider.sendAndConfirm(tx);
        }
    }

    for (var nft of nfts) {
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(globalAuthority, nft.mintInfo), isSigner: false, isWritable: true })
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(player, nft.mintInfo), isSigner: false, isWritable: true })
    }

    let txId = await program.methods
        .openBox(openTimes)
        .accounts({
            player,
            globalPool: globalAuthority,
            boxPool: boxAddress,
            prizePool: prizeAddress,
            solVault,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
        })
        .remainingAccounts(remainingAccounts)
        .rpc();

    console.log("txHash =", txId);    
}

export const getGlobalPool = async (): Promise<GlobalPool | null> => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    try {
        return await program.account.globalPool.fetch(globalAuthority) as unknown as GlobalPool;
    } catch {
        return null;
    }
}

export const getBoxPool = async (boxAddress): Promise<BoxPool | null> => {
    try {
        return await program.account.boxPool.fetch(boxAddress) as unknown as BoxPool;
    } catch {
        return null;
    }
}

export const getPrizePool = async (prizeAddress): Promise<PrizePool | null> => {
    try {
        return await program.account.prizePool.fetch(prizeAddress) as unknown as PrizePool;
    } catch {
        return null;
    }
}

export const getDecimals = async (owner: PublicKey, tokenMint: PublicKey): Promise<number | null> => {
    try {
        let ownerTokenAccount = await getAssociatedTokenAccount(owner, tokenMint);
        const tokenAccount = await solConnection.getParsedAccountInfo(ownerTokenAccount);
        let decimal = (tokenAccount.value?.data as ParsedAccountData).parsed.info.tokenAmount.decimals;
        let DECIMALS = Math.pow(10, decimal);
        return DECIMALS;
    } catch {
        return null;
    }
}

export const getATokenAccountsNeedCreate = async (
    connection: anchor.web3.Connection,
    walletAddress: anchor.web3.PublicKey,
    owner: anchor.web3.PublicKey,
    nfts: anchor.web3.PublicKey[],
) => {
    let instructions = [], destinationAccounts = [];
    for (const mint of nfts) {
        const destinationPubkey = await getAssociatedTokenAccount(owner, mint);
        let response = await connection.getAccountInfo(destinationPubkey);
        if (!response) {
            const createATAIx = createAssociatedTokenAccountInstruction(
                destinationPubkey,
                walletAddress,
                owner,
                mint,
            );
            instructions.push(createATAIx);
        }
        destinationAccounts.push(destinationPubkey);
        if (walletAddress != owner) {
            const userAccount = await getAssociatedTokenAccount(walletAddress, mint);
            response = await connection.getAccountInfo(userAccount);
            if (!response) {
                const createATAIx = createAssociatedTokenAccountInstruction(
                    userAccount,
                    walletAddress,
                    walletAddress,
                    mint,
                );
                instructions.push(createATAIx);
            }
        }
    }
    return {
        instructions,
        destinationAccounts,
    };
}

export const createAssociatedTokenAccountInstruction = (
    associatedTokenAddress: anchor.web3.PublicKey,
    payer: anchor.web3.PublicKey,
    walletAddress: anchor.web3.PublicKey,
    splTokenMintAddress: anchor.web3.PublicKey
) => {
    const keys = [
        { pubkey: payer, isSigner: true, isWritable: true },
        { pubkey: associatedTokenAddress, isSigner: false, isWritable: true },
        { pubkey: walletAddress, isSigner: false, isWritable: false },
        { pubkey: splTokenMintAddress, isSigner: false, isWritable: false },
        {
            pubkey: anchor.web3.SystemProgram.programId,
            isSigner: false,
            isWritable: false,
        },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        {
            pubkey: anchor.web3.SYSVAR_RENT_PUBKEY,
            isSigner: false,
            isWritable: false,
        },
    ];
    return new anchor.web3.TransactionInstruction({
        keys,
        programId: ASSOCIATED_TOKEN_PROGRAM_ID,
        data: Buffer.from([]),
    });
}

const getAssociatedTokenAccount = async (ownerPubkey: PublicKey, mintPk: PublicKey): Promise<PublicKey> => {
    let associatedTokenAccountPubkey = (await PublicKey.findProgramAddress(
        [
            ownerPubkey.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            mintPk.toBuffer(), // mint address
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID
    ))[0];
    return associatedTokenAccountPubkey;
}

main()
