import * as anchor from "@coral-xyz/anchor";
// import * as solana from "@solana/web3.js";

import { Program } from "@coral-xyz/anchor";
import { ModularToken } from "../target/types/modular_token";
import { BasicTokenBackend } from "../target/types/basic_token_backend";

describe("modular-token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const token = anchor.workspace.ModularToken as Program<ModularToken>;
  const backend = anchor.workspace.BasicTokenBackend as Program<BasicTokenBackend>;

  it("Can initialize token accounts", async () => {
    const tokenAccountFrontKP = anchor.web3.Keypair.generate();
    const mint = new anchor.BN(0);

    const [tokenAccountFront] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("token_account_front"),
        Buffer.from(mint.toArray("le")),
        token.provider.publicKey.toBuffer()
      ],
      token.programId
    );

    console.log(tokenAccountFront);

    // anchor.web3.PublicKey.findProgramAddressSync([
    //   anchor.utils.bytes.utf8.encode("token_account_back"),

    // ])

    // anchor.web3.SystemProgram.createAccountWithSeed({
    //   fromPubkey: token.provider.publicKey,
    //   basePubkey: token.provider.publicKey,
    //   seed: "TEST",
    //   space: 300,
    //   lamports: 100000,
    //   programId: backend.programId,
    // })


    // await program.methods
    //   .initializeTokenAccount(program.provider.publicKey, mint)
    //   .accounts({
    //     // backe
    //     tokenAccountFront: tokenAccountFrontKP.publicKey,
    //     initializer: program.provider.publicKey,
    //     systemProgram: anchor.web3.SystemProgram.programId,
    //   })
    //   .signers([tokenAccountFrontKP])
    //   .rpc();

    // const storedTokenAccount = await program.account.tokenAccountFront.fetch(
    //   tokenAccountFrontKP.publicKey
    // );

    // console.log(storedTokenAccount);
  });
});
