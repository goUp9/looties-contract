import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LootiesContract } from "../target/types/looties_contract";



describe("looties-contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.LootiesContract as Program<LootiesContract>;
  const wallet = provider.wallet as anchor.Wallet;
  const payer = wallet.payer;
  const authority = wallet.publicKey;

  it("Initialise global pool", async () => {
    await program.methods
      .initialize()
      .accounts({
        superAdmin: payer.publicKey,
      })
      .signers([payer])
      .rpc();
  });
});
