import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as metaplex from "@metaplex/js";
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram } from "@solana/web3.js";
import { LootiesContract } from "../target/types/looties_contract";
import { assert } from "chai";

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

  // Uninitialized constant accounts.
  let metadata: PublicKey = null;
  let mintA: Token = null;
  let pdaAssociateTokenAccount1: PublicKey = null;
  let pdaAssociateTokenAccount2: PublicKey = null;
  let initializerReceiveTokenAccount: PublicKey = null;
  let nftMintClient: Token = null;

  it("Creates an NFT mint", async () => {
    return;
    // Create the mint.
    nftMintClient = await Token.createMint(
      provider.connection,
      payer,
      authority,
      null,
      6,
      TOKEN_PROGRAM_ID
    );

    // Create the metadata.
    const [_metadata] = await PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        nftMintClient.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );
    metadata = _metadata;
    const tx = new CreateMetadata(
      { feePayer: authority },
      {
        metadata,
        metadataData: new MetadataDataData({
          name: "test-nft",
          symbol: "TEST",
          uri: "https://nothing.com",
          sellerFeeBasisPoints: 1,
          creators: null,
        }),
        updateAuthority: authority,
        mint: nftMintClient.publicKey,
        mintAuthority: authority,
      }
    );
    await provider.sendAndConfirm(tx);
  });
  
  it("Initialise escrow state", async () => {
    // Airdropping tokens to a payer.
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        payer.publicKey,
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

    pdaAssociateTokenAccount1 = await mintA.createAccount(
      authority
    );

    pdaAssociateTokenAccount2 = await mintA.createAccount(
      authority
    );

    initializerReceiveTokenAccount = await mintA.createAccount(
      authority
    );
  });

  it("Initialize escrow1", async () => {
    let rewards = [
      {
        name: "Reward 1",
        description: "Description 1",
        imageUrl: "Image url 1",
        rewardType: 0,
        key: payer.publicKey,
        chance: 50 * 10 ** 3,
        price: new anchor.BN(0),
        prizes: [],
      },
      {
        name: "Reward 2",
        description: "Description 2",
        imageUrl: "Image url 2",
        rewardType: 1,
        key: payer.publicKey,
        chance: 50 * 10 ** 3,
        price: new anchor.BN(10 ** 9), // 1 SOL
        prizes: [],
      },
    ];

    await program.rpc.initializeEscrow(
      "firstBox",
      "this is test box",
      new anchor.BN(2 * 10 ** 9),  // 2 SOL
      "https://imaga_url",
      rewards,
      {
        accounts: {
          initializer: authority,
          pdaAssociateTokenAccount: pdaAssociateTokenAccount1,
          initializerReceiveTokenAccount: initializerReceiveTokenAccount,
          escrowAccount: escrowAccount1.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        },
        signers: [escrowAccount1],
      }
    );

    // balance = await provider.connection.getAccountInfo(escrowAccount.publicKey);
    // console.log(balance);

    let escrow = await program.account.escrowAccount.fetch(escrowAccount1.publicKey);

    assert.ok(escrow.name === "firstBox");
    assert.ok(escrow.description === "this is test box");
    assert.ok(escrow.price.toString() === new anchor.BN("2000000000").toString());
    assert.ok(escrow.imageUrl === "https://imaga_url");
    assert.ok(escrow.rewards.length === 2);
  });

  it("Initialize escrow2", async () => {
    let rewards = [
      {
        name: "Reward 1",
        description: "Description 1",
        imageUrl: "Image url 1",
        rewardType: 0,
        key: payer.publicKey,
        chance: 100 * 10 ** 3,
        price: new anchor.BN(0),
        prizes: [],
      }
    ];

    await program.rpc.initializeEscrow(
      "second",
      "this is test box",
      new anchor.BN(0), 
      "https://imaga_url",
      rewards,
      {
        accounts: {
          initializer: authority,
          pdaAssociateTokenAccount: pdaAssociateTokenAccount2,
          initializerReceiveTokenAccount: initializerReceiveTokenAccount,
          escrowAccount: escrowAccount2.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        },
        signers: [escrowAccount2],
      }
    );

    let escrow = await program.account.escrowAccount.fetch(escrowAccount2.publicKey);

    assert.ok(escrow.name === "second");
    assert.ok(escrow.description === "this is test box");
    assert.ok(escrow.price.toString() === new anchor.BN(0).toString());
    assert.ok(escrow.imageUrl === "https://imaga_url");
    assert.ok(escrow.rewards.length === 1);
  });
  
  it("Update escrow2", async () => {
    let rewards = [
      {
        name: "Reward 1",
        description: "Description 1",
        imageUrl: "Image url 1",
        rewardType: 1,
        key: payer.publicKey,
        chance: 25 * 10 ** 3,
        price: new anchor.BN(2 * 10 ** 9),
        prizes: [],
      },
      {
        name: "Reward 1",
        description: "Description 1",
        imageUrl: "Image url 1",
        rewardType: 1,
        key: payer.publicKey,
        chance: 25 * 10 ** 3,
        price: new anchor.BN(10 ** 9),
        prizes: [],
      },
      {
        name: "Reward 1",
        description: "Description 1",
        imageUrl: "Image url 1",
        rewardType: 1,
        key: payer.publicKey,
        chance: 25 * 10 ** 3,
        price: new anchor.BN(5 * 10 ** 8),
        prizes: [],
      },
      {
        name: "Reward 1",
        description: "Description 1",
        imageUrl: "Image url 1",
        rewardType: 1,
        key: payer.publicKey,
        chance: 25 * 10 ** 3,
        price: new anchor.BN(10 ** 8),
        prizes: [],
      }
    ];

    await program.rpc.updateEscrow(
      "second",
      "this is test box",
      new anchor.BN(10 ** 9),  // 1 SOL
      "https://imaga_url",
      rewards,
      {
        accounts: {
          initializer: authority,
          initializerReceiveTokenAccount: initializerReceiveTokenAccount,
          escrowAccount: escrowAccount2.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [],
      }
    );

    let escrow = await program.account.escrowAccount.fetch(escrowAccount2.publicKey);

    assert.ok(escrow.name === "second");
    assert.ok(escrow.description === "this is test box");
    assert.ok(escrow.price.toString() === new anchor.BN(10 ** 9).toString());
    assert.ok(escrow.imageUrl === "https://imaga_url");
    assert.ok(escrow.rewards.length === 4);
  });

  it("Deposit SOL", async () => {
    await program.rpc.depositSol(
      new anchor.BN(10 ** 9),
      {
        accounts: {
          from: authority,
          to: pdaAssociateTokenAccount1,
          systemProgram: SystemProgram.programId,
        },
        signers: [],
      }
    );
  });
});
