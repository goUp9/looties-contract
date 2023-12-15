import { Program, web3 } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Keypair, ParsedAccountData, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction } from "@solana/web3.js";
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
const NFTcollection = new anchor.web3.PublicKey("7kZwsnGkJKRkit73yQC96XCzrEDBBzo7iD2s8FME5HtL");
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
    // super admin  : CPvqXDUJBwGDH9e2SadrQzFYqCaKiF2UXmxgqkcQdYTZ
    // admin        : 5RoELXPzGfPFJ8DqHXX6QmgLguYERWfptPC3SUkwCBGz
    // player       : Bw6cx4qzWygKDLtbThv8TxESxsUsEXmcv1X4gUPbSycc

    // ===========> methods related to global pool < =========== //
    // await initProject();
    // await changeSuperAdmin(new anchor.web3.PublicKey("CPvqXDUJBwGDH9e2SadrQzFYqCaKiF2UXmxgqkcQdYTZ"));
    // await addTokenAddress(tokenAAddress);
    // console.log(await getGlobalPool());
    
    let boxPool1 = new web3.PublicKey("6DJhBvvxenGKbz2uFXEgmQdRcbLoB3EJZ6Acwdqk2QD7");
    let boxPool2 = new web3.PublicKey("gJPEXEh9xagJJ8iivqeBEjfpqP8BDLfgkeaBVg4QzDG");

    // ===========> methods related to box pool < =========== //
    // await initBoxTest();
    // await updateBoxTest(boxPool1);
    // await changeAdmin(boxPool2, new anchor.web3.PublicKey("5RoELXPzGfPFJ8DqHXX6QmgLguYERWfptPC3SUkwCBGz"))
    // await removeBox(boxPool2);
    // await deposit(boxPool2, new anchor.BN(10 ** 9), new anchor.BN(0 * 10 ** 6), tokenAAddress);
    // await withdraw(boxPool2, new anchor.BN(10 ** 9), new anchor.BN(0), tokenBAddress);
    // console.log(await getBoxPool(boxPool2));
    
    // let nfts = [
    //     new anchor.web3.PublicKey("9ARBwxAsTtNiJv3UYjrvNvJH81HUvqCSd7TeThNiTmKi"),
    //     new anchor.web3.PublicKey("4TV9nj18LTdnaRrPweGR4ZAuo9FzyxYrSwoHtnA7p6ny"),
    // ];
    
    // await depositNfts(boxPool2, NFTcollection, nfts);
    // await withdrawNfts(boxPool2, nfts);
    // console.log(await getPrizePool((await getBoxPool(boxPool2)).prizes));

    
    // ===========> methods related to player pool < =========== //

    // await initPlayer();                              // This function must be called when the user registers the platform.
    // await openBox(boxPool2, 3);
    // await claimReward();
    // console.log(await getPlayerPool());

    /*
        when player open the box, you should call functions step by step.
        1. await openBox(boxPool2, 3);
        2. get reward indexes(you can display rewards with index)
            let lastRewardIdxs = (await getPlayerPool()).lastRewardIdxs;
            lastRewardIdxs.map((reward_idx) => {
                console.log(reward_idx);
            })
        3. await claimReward();
    */
}

/////////////////////
/// Test funciton ///
/////////////////////

const initBoxTest = async () => {
    let defaultKey = PublicKey.default;

    let rewards = [
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

    let rewards_2 = [
        {
            name: "0.005 SOL",
            description: "0.005 SOL",
            imageUrl: "https://looties-next-app.vercel.app/assets/cases/case-1.svg",
            rewardType: 1,
            chance: 25 * 10 ** 2,
            sol: new anchor.BN(5 * 10 ** 6),
            token: new anchor.BN(0),
            tokenAddress: defaultKey,
            collectionAddress: defaultKey,
        },
        {
            name: "1000 TokenA",
            description: "1000 TokenA",
            imageUrl: "https://looties-next-app.vercel.app/assets/cases/case-2.svg",
            rewardType: 2,
            chance: 25 * 10 ** 2,
            sol: new anchor.BN(0),
            token: new anchor.BN(1000),
            tokenAddress: tokenAAddress,
            collectionAddress: defaultKey,
        },
        {
            name: "1000 TokenB",
            description: "1000 TokenB",
            imageUrl: "https://looties-next-app.vercel.app/assets/cases/case-3.svg",
            rewardType: 2,
            chance: 25 * 10 ** 2,
            sol: new anchor.BN(0),
            token: new anchor.BN(1000),
            tokenAddress: tokenBAddress,
            collectionAddress: defaultKey,
        },
        {
            name: "NFT",
            description: "NFT",
            imageUrl: "https://looties-next-app.vercel.app/assets/cases/case-4.svg",
            rewardType: 3,
            chance: 25 * 10 ** 2,
            sol: new anchor.BN(0),
            token: new anchor.BN(0),
            tokenAddress: defaultKey,
            collectionAddress: NFTcollection,
        },
    ]

    await initBox(
        new anchor.web3.PublicKey("5RoELXPzGfPFJ8DqHXX6QmgLguYERWfptPC3SUkwCBGz"),
        "Complex Case",
        "Complex Case",
        "https://looties-next-app.vercel.app/assets/cases/case-1.svg", new anchor.BN(0.11 * 10 ** 9),
        rewards_2,
    );
}

const updateBoxTest = async (boxAddress: PublicKey) => {
    let defaultKey = PublicKey.default;

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

    await updateBox(
        boxAddress,
        "Radioactive Case",
        "Radioactive Case",
        "https://looties-next-app.vercel.app/sample-case.svg", new anchor.BN(0.11 * 10 ** 9),
        rewards_2,
    );
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

    // make associated token account for PDA
    {
        let tx = new Transaction();
        let { instructions, destinationAccounts } = await getATokenAccountsNeedCreate(
            solConnection,
            authority,
            globalAuthority,
            [tokenAddress]
        );
    
        if (instructions.length > 0) {
            instructions.map((ix) => tx.add(ix));
            await provider.sendAndConfirm(tx);
        }
    }

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

    {
        let tx = new Transaction();
        let { instructions, destinationAccounts } = await getATokenAccountsNeedCreate(
            solConnection,
            authority,
            authority,
            [tokenAddress]
        );
        if (instructions.length > 0) {
            instructions.map((ix) => tx.add(ix));
            await provider.sendAndConfirm(tx);
        }
    }

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

    {
        let tx = new Transaction();
        let { instructions, destinationAccounts } = await getATokenAccountsNeedCreate(
            solConnection,
            authority,
            authority,
            [tokenAddress]
        );
        if (instructions.length > 0) {
            instructions.map((ix) => tx.add(ix));
            await provider.sendAndConfirm(tx);
        }
    }

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
    console.log('==> deposit NFTs')
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [prizeAuthority, pBump] = await PublicKey.findProgramAddress(
        [Buffer.from(PRIZE_POOL_SEED), boxAddress.toBuffer()],
        program.programId
    );

    {
        let tx = new Transaction();
        let { instructions, destinationAccounts } = await getATokenAccountsNeedCreate(
            solConnection,
            authority,
            globalAuthority,
            nfts
        );
        if (instructions.length > 0) {
            instructions.map((ix) => tx.add(ix));
            await provider.sendAndConfirm(tx);
        }
    }

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
    console.log('==> withdraw NFTs');
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [prizeAuthority, pBump] = await PublicKey.findProgramAddress(
        [Buffer.from(PRIZE_POOL_SEED), boxAddress.toBuffer()],
        program.programId
    );

    {
        let tx = new Transaction();
        let { instructions, destinationAccounts } = await getATokenAccountsNeedCreate(
            solConnection,
            authority,
            authority,
            nfts
        );
        if (instructions.length > 0) {
            instructions.map((ix) => tx.add(ix));
            await provider.sendAndConfirm(tx);
        }
    }

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
export const initPlayer = async () => {
    console.log('==> initPlayer : ', authority.toString());

    const [playerAuthority, pBump] = await PublicKey.findProgramAddress(
        [Buffer.from(PLAYER_POOL_SEED), authority.toBuffer()],
        program.programId
    );

    let txId = await program.methods
        .initPlayer()
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


/**
 * Open box
 * 
 * @access  - player
 */
export const claimReward = async () => {
    const [globalAuthority, gBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );

    const [solVault, svBump] = await PublicKey.findProgramAddress(
        [Buffer.from(SOL_VAULT_SEED)],
        program.programId
    );

    const [playerAuthority, pBump2] = await PublicKey.findProgramAddress(
        [Buffer.from(PLAYER_POOL_SEED), authority.toBuffer()],
        program.programId
    );

    const globalPool = await getGlobalPool();
    const playerPool = await getPlayerPool();

    console.log(playerPool)
    console.log(playerPool.boxAddr);

    const [prizeAuthority, pBump1] = await PublicKey.findProgramAddress(
        [Buffer.from(PRIZE_POOL_SEED), playerPool.boxAddr.toBuffer()],
        program.programId
    );
    
    let tokenAddresses = globalPool.tokenAddress.concat(playerPool.claimableNfts);

    {
        let tx = new Transaction();

        let { instructions, destinationAccounts } = await getATokenAccountsNeedCreate(
            solConnection,
            authority,
            authority,
            tokenAddresses
        );
        if (instructions.length > 0) {
            instructions.map((ix) => tx.add(ix));
            await provider.sendAndConfirm(tx);
        }
    }

    let remainingAccounts = [];

    for (var nft of tokenAddresses) {
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(globalAuthority, nft), isSigner: false, isWritable: true })
        remainingAccounts.push({ pubkey: await getAssociatedTokenAccount(authority, nft), isSigner: false, isWritable: true })
    }


    let txId = await program.methods
        .claimReward()
        .accounts({
            player: authority,
            globalPool: globalAuthority,
            boxPool: playerPool.boxAddr,
            prizePool: prizeAuthority,
            playerPool: playerAuthority,
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
