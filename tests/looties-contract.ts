import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as metaplex from "@metaplex/js";
import { AccountLayout, TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction, _default } from "@solana/web3.js";
import { LootiesContract } from "../target/types/looties_contract";
import { assert } from "chai";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

const MetadataDataData = metaplex.programs.metadata.MetadataDataData;
const CreateMetadata = metaplex.programs.metadata.CreateMetadata;

describe("looties-contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.LootiesContract as Program<LootiesContract>;
  
  // Constant accounts.
  const wallet = provider.wallet as anchor.Wallet;
  const payer = wallet.payer;
  const authority = wallet.publicKey;
  const escrowAccount1 = Keypair.generate();
  const escrowAccount2 = Keypair.generate();
  const player = Keypair.generate();
  const escrowAssociateAccount1 = Keypair.generate();
  const escrowAssociateAccount2 = Keypair.generate();
  const ESCROW_PDA_SEED = Buffer.from("escrow_pda_seed");
  const ESCROW_TOKEN_PDA_SEED = Buffer.from("escrow_token_pda_seed");
  const ESCROW_NFT_PDA_SEED = Buffer.from("escrow_nft_pda_seed");

  // Uninitialized constant accounts.
  let metadata: PublicKey = null;
  let mintA: Token = null;
  let mintB: Token = null;
  let initializerReceiveTokenAccount: PublicKey = null;
  let playerReceiveTokenAccount: PublicKey = null;

  it("Initialise escrow state", async () => {
    // Airdropping tokens to a payer.
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        payer.publicKey,
        10000000000
      ),
      "confirmed"
    );

    // Airdropping tokens to a player.
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        player.publicKey,
        10000000000
      ),
      "confirmed"
    );

    mintA = await Token.createMint(
      provider.connection,
      payer,
      authority,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    mintB = await Token.createMint(
      provider.connection,
      payer,
      authority,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    initializerReceiveTokenAccount = await mintA.createAccount(
      authority
    );

    playerReceiveTokenAccount = await mintA.createAccount(
      player.publicKey
    );
  });

  it("Initialize escrow1", async () => {
    let rewards = [
      {
        name: "1 SOL",
        description: "1 SOL",
        imageUrl: "https://no_url",
        rewardType: 0,
        chance: 10 * 10 ** 3,
        price: new anchor.BN(10 ** 9),
        mintToken: null,
        escrowAssociateTokenAccount: null,
        prizeAccount: null,
      },
      {
        name: "2 SOL",
        description: "2 SOL",
        imageUrl: "https://no_url",
        rewardType: 0,
        chance: 10 * 10 ** 3,
        price: new anchor.BN(2 * 10 ** 9),
        mintToken: null,
        escrowAssociateTokenAccount: null,
        prizeAccount: null,
      },
      {
        name: "1 tokenA",
        description: "1 tokenA",
        imageUrl: "https://no_url",
        rewardType: 1,
        chance: 10 * 10 ** 3,
        price: new anchor.BN(10 ** 9),
        mintToken: mintA.publicKey,
        escrowAssociateTokenAccount: null,
        prizeAccount: null,
      },
      {
        name: "2 tokenA",
        description: "2 tokenA",
        imageUrl: "https://no_url",
        rewardType: 1,
        chance: 10 * 10 ** 3,
        price: new anchor.BN(20 ** 9),
        mintToken: mintA.publicKey,
        escrowAssociateTokenAccount: null,
        prizeAccount: null,
      },
      {
        name: "1 tokenB",
        description: "1 tokenB",
        imageUrl: "https://no_url",
        rewardType: 1,
        chance: 10 * 10 ** 3,
        price: new anchor.BN(10 ** 9),
        mintToken: mintB.publicKey,
        escrowAssociateTokenAccount: null,
        prizeAccount: null,
      },
      {
        name: "2 tokenB",
        description: "2 tokenB",
        imageUrl: "https://no_url",
        rewardType: 1,
        chance: 10 * 10 ** 3,
        price: new anchor.BN(20 ** 9),
        mintToken: mintB.publicKey,
        escrowAssociateTokenAccount: null,
        prizeAccount: null,
      },
      {
        name: "NFT",
        description: "NFT",
        imageUrl: "https://no_url",
        rewardType: 2,
        chance: 40 * 10 ** 3,
        price: new anchor.BN(1),
        mintToken: null,
        escrowAssociateTokenAccount: null,
        prizeAccount: null,
      },
    ];

    let array = [];
    rewards.forEach((reward) => {
      if (reward.rewardType == 1) {
        let [pda,] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            ESCROW_TOKEN_PDA_SEED,
          ],
          program.programId
        );
        // escrow_associate_token_account
        array.push({ pubkey: pda, isWritable: true, isSigner: false });
      } else if (reward.rewardType == 2) {
        let [pda,] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            ESCROW_NFT_PDA_SEED,
            escrowAccount1.publicKey.toBuffer(),
            // reward.mintToken.toBuffer(),
          ],
          program.programId
        );
        // prize_account
        array.push({ pubkey: pda, isWritable: true, isSigner: false });
      }
    });

    await program.methods.initializeEscrow(
      "firstBox",
      "this is test box",
      new anchor.BN(2 * 10 ** 9),  // 2 SOL
      "https://imaga_url",
      rewards)
      .accounts({
          initializer: authority,
          initializerReceiveTokenAccount: initializerReceiveTokenAccount,
          escrowAccount: escrowAccount1.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
      })
      .remainingAccounts(array)
      .signers([payer, escrowAccount1])
      .rpc();

    // balance = await provider.connection.getAccountInfo(escrowAccount.publicKey);
    // console.log(balance);

    let escrow = await program.account.escrowAccount.fetch(escrowAccount1.publicKey);

    assert.ok(escrow.name === "firstBox");
    assert.ok(escrow.description === "this is test box");
    assert.ok(escrow.price.toString() === new anchor.BN("2000000000").toString());
    assert.ok(escrow.imageUrl === "https://imaga_url");
    assert.ok(escrow.rewards.length === rewards.length);
    assert.ok(escrow.rewards.toString() == rewards.toString());
    escrow.rewards.forEach((reward, index) => {
      assert.ok(reward.name === rewards[index].name);
      assert.ok(reward.description === rewards[index].description);
      assert.ok(reward.imageUrl === rewards[index].imageUrl);
      assert.ok(reward.rewardType === rewards[index].rewardType);
      assert.ok(reward.chance === rewards[index].chance);
      assert.ok(reward.price.toString() === rewards[index].price.toString());
      assert.ok(reward.mintToken?.toString() === rewards[index].mintToken?.toString());
      console.log("reward ", index, "'s escrow_associate_token_account : ", reward.escrowAssociateTokenAccount?.toString())
      console.log("reward ", index, "'s prize_account : ", reward.prizeAccount?.toString())
    })
  });
});
