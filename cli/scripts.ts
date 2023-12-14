import { Program, web3 } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Keypair, ParsedAccountData, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction } from "@solana/web3.js";
import fs from 'fs';
import { GlobalPool, BoxPool, PrizePool, Reward, PlayerPool } from './types';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
const idl = require('../target/idl/looties_contract.json');

const PROGRAM_ID = "t1ynC7jhTJfZD8idR58Yz6EW8XiwajKzNXusf2tguBV";
const BOX_AUTHORITY_SEED = "box-authority";
const GLOBAL_AUTHORITY_SEED = "global-authority";
const PRIZE_POOL_SEED = "prize-pool";
const PLAYER_POOL_SEED = "player-pool";
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
const admin1 = new anchor.web3.PublicKey("CPvqXDUJBwGDH9e2SadrQzFYqCaKiF2UXmxgqkcQdYTZ");
const admin2 = new anchor.web3.PublicKey("CPvqXDUJBwGDH9e2SadrQzFYqCaKiF2UXmxgqkcQdYTZ");
const admin3 = new anchor.web3.PublicKey("CPvqXDUJBwGDH9e2SadrQzFYqCaKiF2UXmxgqkcQdYTZ");

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

/////////////////////
/// Test funciton ///
/////////////////////

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
        new anchor.web3.PublicKey("5RoELXPzGfPFJ8DqHXX6QmgLguYERWfptPC3SUkwCBGz"),
        "Radioactive Case",
        "Radioactive Case",
        "https://looties-next-app.vercel.app/sample-case.svg", new anchor.BN(0.11 * 10 ** 9),
        rewards_2,
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

//////////////////////////////////////////////////
/// Interface to integrate with smart contract ///
//////////////////////////////////////////////////

/**
 * Initialize Global Pool
 * 
 * @access      - super_admin
 */
export const initProject = async () => {
    console.log('==>Initializing program');

    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    let txId = await program.methods
        .initialize()
        .accounts({
            super_admin: authority,
            globalPool: globalAuthority,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .rpc();
    console.log('super admin for game: ', authority.toString());

    console.log("txHash =", txId);    
}

/**
 * Change super_admin
 * 
 * @access - super_admin
 * 
 * @param - newSuperAdmin   :  new super_admin for game
 */
export const changeSuperAdmin = async (
    newSuperAdmin: PublicKey,
) => {
    console.log('==>changeSuperAdmin to : ', newSuperAdmin.toString());

    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    let txId = await program.methods
        .changeSuperAdmin(newSuperAdmin)
        .accounts({
            super_admin: authority,
            globalPool: globalAuthority,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .rpc();

    console.log("txHash =", txId);    
}

/**
 * Add spl token to use for game
 * 
 * @access - super_admin
 * 
 * @param - tokenAddress: spl token address to add to game newly
 */
export const addTokenAddress = async (
    tokenAddress: PublicKey,
) => {
    console.log('==>addTokenAddress : ', tokenAddress.toString());

    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    let txId = await program.methods
        .addTokenAddress(tokenAddress)
        .accounts({
            super_admin: authority,
            globalPool: globalAuthority,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .rpc();

    console.log("txHash =", txId);    
}

/**
 * Add new box
 * 
 * @access  - super_admin
 * 
 * @param   - admin         : admin for box.
 *          - name          : name for box
 *          - description   : description for box
 *          - imageUrl      : image_url for box
 *          - priceInSol    : price in sol that the player must pay to open the box
 *          - rewards       : reward array that the player can get when play on box
 */
export const initBox = async (
    admin: PublicKey,
    name: String,
    description: String,
    imageUrl: String,
    priceInSol: anchor.BN,
    rewards: Reward[],
) => {
    console.log('==>initBox : ');
    const _rand = Keypair.generate().publicKey;

    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [boxAuthority, bBump] = await PublicKey.findProgramAddress(
        [Buffer.from(BOX_AUTHORITY_SEED), _rand.toBuffer()],
        program.programId
    );

    const [prizeAuthority, pBump] = await PublicKey.findProgramAddress(
        [Buffer.from(PRIZE_POOL_SEED), boxAuthority.toBuffer()],
        program.programId
    );

    let txId = await program.methods
        .initBox(admin, name, description, imageUrl, priceInSol, rewards)
        .accounts({
            authority,
            globalPool: globalAuthority,
            boxPool: boxAuthority,
            prizePool: prizeAuthority,
            randKey: _rand,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .rpc();

    console.log('added new box: address: ', boxAuthority.toString(), ' admin: ', admin.toString());
    
    console.log("txHash =", txId);    
}

/**
 * Update the box
 * 
 * @access  - super_admin
 * 
 * @param   - boxAddress    : box address to update.
 *          - name          : name for box
 *          - description   : description for box
 *          - imageUrl      : image_url for box
 *          - priceInSol    : price in sol that the player must pay to open the box
 *          - rewards       : reward array that the player can get when play on box
 */
export const updateBox = async (
    boxAddress: PublicKey,
    name: String,
    description: String,
    imageUrl: String,
    priceInSol: anchor.BN,
    rewards: Reward[],
) => {
    console.log('==>updateBox : ', boxAddress.toString());

    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    let txId = await program.methods
        .updateBox(name, description, imageUrl, priceInSol, rewards)
        .accounts({
            super_admin: authority,
            globalPool: globalAuthority,
            boxPool: boxAddress,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .rpc();
    
    console.log("txHash =", txId);    
}

/**
 * Change box admin
 * 
 * @access  - super_admin
 * 
 * @param   - boxAddress    : box address to update.
 *          - newAdmin      : new admin for box
 */
export const changeAdmin = async (
    boxAddress: PublicKey,
    newAdmin: PublicKey,
) => {
    console.log('==>changeAdmin : boxAddress: ', boxAddress.toString(), ' newAdmin: ', newAdmin.toString());

    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    let txId = await program.methods
        .changeAdmin(newAdmin)
        .accounts({
            super_admin: authority,
            globalPool: globalAuthority,
            boxPool: boxAddress,
        })
        .rpc();
    
    console.log("txHash =", txId);    
}

/**
 * Remove box
 * 
 * @access  - super_admin
 * 
 * @param   - boxAddress    : box address to remove.
 */
export const removeBox = async (
    boxAddress: PublicKey,
) => {
    console.log('==>removeBox : boxAddress: ', boxAddress.toString());

    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [prizeAuthority, pBump] = await PublicKey.findProgramAddress(
        [Buffer.from(PRIZE_POOL_SEED), boxAddress.toBuffer()],
        program.programId
    );

    let txId = await program.methods
        .removeBox()
        .accounts({
            super_admin: authority,
            globalPool: globalAuthority,
            boxPool: boxAddress,
            prizePool: prizeAuthority,
        })
        .rpc();
    
    console.log("txHash =", txId);    
}

/**
 * Deposit Sol and Spl token
 * 
 * @access  - super_admin, admin(admin for box)
 * 
 * @param   - boxAddress    : box address to deposit
 *          - solAmount     : Sol amount to deposit
 *          - tokenAmount   : Token amount to deposit
 *          - tokenAddress  : Token mint address to deposit
 */
export const deposit = async (
    boxAddress: PublicKey,
    solAmount: anchor.BN,
    tokenAmount: anchor.BN,
    tokenAddress: PublicKey,
) => {
    console.log('==>deposit : SOL: ', solAmount, ' Token(', tokenAddress.toString(), '): ' , tokenAmount);

    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [solVault, svBump] = await PublicKey.findProgramAddress(
        [Buffer.from(SOL_VAULT_SEED)],
        program.programId
    );

    let adminTokenAccount = await getAssociatedTokenAccount(authority, tokenAddress);
    let gameTokenAccount = await getAssociatedTokenAccount(globalAuthority, tokenAddress);

    let txId = await program.methods
        .deposit(solAmount, tokenAmount)
        .accounts({
            admin: authority,
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
}

/**
 * Withdraw Sol and Spl token
 * 
 * @access  - admin(admin for box)
 * 
 * @param   - boxAddress    : box address to withdraw
 *          - solAmount     : Sol amount to withdraw
 *          - tokenAmount   : Token amount to withdraw
 *          - tokenAddress  : Token mint address to withdraw
 */
export const withdraw = async (
    boxAddress: PublicKey,
    solAmount: anchor.BN,
    tokenAmount: anchor.BN,
    tokenAddress: PublicKey,
) => {
    console.log('==>withdraw : SOL: ', solAmount, ' Token(', tokenAddress.toString(), '): ' , tokenAmount);

    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [solVault, svBump] = await PublicKey.findProgramAddress(
        [Buffer.from(SOL_VAULT_SEED)],
        program.programId
    );

    let adminTokenAccount = await getAssociatedTokenAccount(authority, tokenAddress);
    let gameTokenAccount = await getAssociatedTokenAccount(globalAuthority, tokenAddress);

    let txId = await program.methods
        .withdraw(solAmount, tokenAmount)
        .accounts({
            admin: authority,
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
}

/**
 * Deposit Nfts
 * 
 * @access  - super_admin, admin(admin for box)
 * 
 * @param   - boxAddress    : box address to deposit
 *          - collection    : collection NFT address
 *          - nfts          : NFT address into collection to deposit
 */
export const depositNfts = async (
    boxAddress: PublicKey,
    collection: PublicKey,
    nfts: PublicKey[],
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [prizeAuthority, pBump] = await PublicKey.findProgramAddress(
        [Buffer.from(PRIZE_POOL_SEED), boxAddress.toBuffer()],
        program.programId
    );

    let remainingAccounts = [];

    for (var nft of nfts) {
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(authority, nft), isSigner: false, isWritable: true })
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(globalAuthority, nft), isSigner: false, isWritable: true })
    }

    let txId = await program.methods
        .depositNfts(collection, nfts)
        .accounts({
            admin: authority,
            globalPool: globalAuthority,
            boxPool: boxAddress,
            prizePool: prizeAuthority,
            tokenProgram: TOKEN_PROGRAM_ID,
        })
        .remainingAccounts(remainingAccounts)
        .rpc();

    console.log("txHash =", txId);    
}

/**
 * Withdraw Nfts
 * 
 * @access  - admin(admin for box)
 * 
 * @param   - boxAddress    : box address to withdraw
 *          - nfts          : NFT address into collection to withdraw
 */
export const withdrawNfts = async (
    boxAddress: PublicKey,
    nfts: PublicKey[],
) => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [prizeAuthority, pBump] = await PublicKey.findProgramAddress(
        [Buffer.from(PRIZE_POOL_SEED), boxAddress.toBuffer()],
        program.programId
    );

    let remainingAccounts = [];

    for (var nft of nfts) {
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(globalAuthority, nft), isSigner: false, isWritable: true })
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(authority, nft), isSigner: false, isWritable: true })
    }

    let txId = await program.methods
        .withdrawNfts(nfts)
        .accounts({
            admin: authority,
            globalPool: globalAuthority,
            boxPool: boxAddress,
            prizePool: prizeAuthority,
            tokenProgram: TOKEN_PROGRAM_ID,
        })
        .remainingAccounts(remainingAccounts)
        .rpc();

    console.log("txHash =", txId);    
}

/**
 * Initialize player pool
 * 
 * @access  - player
 */
export const initializePlayerPool = async () => {
    console.log('==> initializePlayerPool : ', authority.toString());

    const [playerAuthority, pBump] = await PublicKey.findProgramAddress(
        [Buffer.from(PLAYER_POOL_SEED), authority.toBuffer()],
        program.programId
    );

    let txId = await program.methods
        .withdrawNfts()
        .accounts({
            player: authority,
            playerPool: playerAuthority,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .rpc();

    console.log("txHash =", txId);    
}

/**
 * Open box
 * 
 * @access  - player
 * 
 * @param   - boxAddress
 *          - openTimes
 */
export const openBox = async (
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

    const [prizeAuthority, pBump1] = await PublicKey.findProgramAddress(
        [Buffer.from(PRIZE_POOL_SEED), boxAddress.toBuffer()],
        program.programId
    );

    const [playerAuthority, pBump2] = await PublicKey.findProgramAddress(
        [Buffer.from(PLAYER_POOL_SEED), authority.toBuffer()],
        program.programId
    );

    let txId = await program.methods
        .openBox(openTimes)
        .accounts({
            player: authority,
            globalPool: globalAuthority,
            boxPool: boxAddress,
            prizePool: prizeAuthority,
            playerPool: playerAuthority,
            solVault,
            admin1,
            admin2,
            admin3,
            systemProgram: SystemProgram.programId,
        })
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

export const getPlayerPool = async (): Promise<PlayerPool | null> => {
    const [playerAuthority, pBump] = await PublicKey.findProgramAddress(
        [Buffer.from(PLAYER_POOL_SEED), authority.toBuffer()],
        program.programId
    );

    try {
        return await program.account.playerPool.fetch(playerAuthority) as unknown as PlayerPool;
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
