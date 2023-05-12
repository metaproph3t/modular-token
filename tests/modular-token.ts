import * as anchor from "@coral-xyz/anchor";
// import * as solana from "@solana/web3.js";

import { Program } from "@coral-xyz/anchor";
import { ModularToken } from "../target/types/modular_token";
import { TokenFrontend } from "../target/types/token_frontend";
import { BasicTokenBackend } from "../target/types/basic_token_backend";

describe("modular-token", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const frontend = anchor.workspace.TokenFrontend as Program<TokenFrontend>;
  const backend = anchor.workspace
    .BasicTokenBackend as Program<BasicTokenBackend>;

  it("Can initialize token accounts", async () => {
    const tokenAccNeededSpc = new anchor.BN(8 + 8 + 8); // disc + balance + mint
    const mintNeededSpc = new anchor.BN(8 + 8); // disc + mintAuthority + supply + decimals

    const nonce = new anchor.BN(4);

    const backendAcc = anchor.web3.Keypair.generate();
    await frontend.methods
      .registerBackend(8 + 8 + 8, 8 + 32 + 8 + 1)
      .accounts({
        backend: backendAcc.publicKey,
        payer: frontend.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        backendProgram: backend.programId,
      })
      .signers([backendAcc])
      .rpc();

    const storedBackend = await frontend.account.backend.fetch(
      backendAcc.publicKey
    );

    console.log(storedBackend);

    const leBump = Buffer.from(nonce.toArray('le', 8));

    const [mint, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("mint"), Buffer.from(nonce.toArray('le', 8))],
      frontend.programId
    );

    console.log("mint: ", mint);
    console.log(bump);
    console.log(leBump);


    const mintAuthority = anchor.web3.Keypair.generate();

    const data = backend.coder.instruction.encode("initialize_mint", {
      mintAuthority: mintAuthority.publicKey,
      decimals: 6,
    }).slice(8); // slice off the first 8 bytes because they're already hardcoded inside the program

    console.log(data);

    await frontend.methods
      .initializeMint(nonce, data)
      .accounts({
        backend: backendAcc.publicKey,
        backendProgram: backend.programId,
        mint,
        payer: frontend.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const storedMint = await connection.getAccountInfoAndContext(mint);
    const providerBal = await connection.getAccountInfoAndContext(
      frontend.provider.publicKey
    );

    console.log(providerBal);

    console.log("frontend program", frontend.programId);
    console.log("backend program", backend.programId);

    console.log(storedMint);


    // const tokenAccountFrontKP = anchor.web3.Keypair.generate();
    // const mint = new anchor.BN(0);

    // const [tokenAccountFront] = anchor.web3.PublicKey.findProgramAddressSync(
    //   [
    //     anchor.utils.bytes.utf8.encode("token_account_front"),
    //     Buffer.from(mint.toArray("le")),
    //     token.provider.publicKey.toBuffer()
    //   ],
    //   token.programId
    // );

    // console.log(tokenAccountFront);

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
