import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as metaplex from "@metaplex/js";
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram } from "@solana/web3.js";
import { LootiesContract } from "../target/types/looties_contract";

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
  
  // type Reward = program.idl.types[0];

  // Constant accounts.
  const payer = Keypair.generate();
  const mintAuthority = Keypair.generate();
  const escrowAccount = Keypair.generate();

  // Uninitialized constant accounts.
  let metadata: PublicKey = null;
  let mintA: Token = null;
  let initializerTokenAccountA: PublicKey = null;
  let nftMintClient: Token = null;
  
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
      mintAuthority.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    initializerTokenAccountA = await mintA.createAccount(
      provider.wallet.publicKey
    );
  });

  // it("Creates an NFT mint", async () => {
  //   // Create the mint.
  //   nftMintClient = await Token.createMint(
  //     provider.connection,
  //     payer,
  //     payer.publicKey,
  //     null,
  //     6,
  //     TOKEN_PROGRAM_ID,
  //   );

  //   // Create the metadata.
  //   const [_metadata] = await PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("metadata"),
  //       TOKEN_METADATA_PROGRAM_ID.toBuffer(),
  //       nftMintClient.publicKey.toBuffer(),
  //     ],
  //     TOKEN_METADATA_PROGRAM_ID
  //   );
  //   metadata = _metadata;
  //   const tx = new CreateMetadata(
  //     { feePayer: payer.publicKey },
  //     {
  //       metadata,
  //       metadataData: new MetadataDataData({
  //         name: "test-nft",
  //         symbol: "TEST",
  //         uri: "https://nothing.com",
  //         sellerFeeBasisPoints: 1,
  //         creators: null,
  //       }),
  //       updateAuthority: payer.publicKey,
  //       mint: nftMintClient.publicKey,
  //       mintAuthority: payer.publicKey,
  //     }
  //   );
  //   await provider.sendAndConfirm(tx);
  // });

  it("Initialize escrow", async () => {

    let rewards = [
      {
        name: "Reward 1",
        description: "Description 1",
        imageUrl: "Image url 1",
        rewardType: 0,
        key: payer.publicKey,
        chance: 50000,
        price: 0,
        prizes: [],
      },
      {
        name: "Reward 2",
        description: "Description 2",
        imageUrl: "Image url 2",
        rewardType: 1,
        key: payer.publicKey,
        chance: 50000,
        price: 1000000000, // 1 SOL
        prizes: [],
      },
    ];

    // let balance = await provider.connection.getAccountInfo(provider.wallet.publicKey);
    // console.log(balance);

    await program.rpc.initializeEscrow(
      "firstBox",
      "this is test box",
      1000,
      "https:/imaga_url",
      rewards,
      {
        accounts: {
          initializer: provider.wallet.publicKey,
          initializerReceiveTokenAccount: initializerTokenAccountA,
          escrowAccount: escrowAccount.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        },
        signers: [escrowAccount],
      }
    );

    // balance = await provider.connection.getAccountInfo(escrowAccount.publicKey);
    // console.log(balance);

    let escrow = await program.account.escrowAccount.fetch(escrowAccount.publicKey);
    console.log(escrow);
  });
});
