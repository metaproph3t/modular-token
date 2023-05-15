import * as anchor from "@coral-xyz/anchor";
// import * as solana from "@solana/web3.js";

import { Program } from "@coral-xyz/anchor";
import { Token } from "../target/types/token";
import { BasicTokenHandler } from "../target/types/basic_token_handler";

describe("modular-token", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const token = anchor.workspace.Token as Program<Token>;
  const tokenHandler = anchor.workspace
    .BasicTokenHandler as Program<BasicTokenHandler>;

  it("Can initialize token accounts", async () => {
    const tokenAccountBytes = 8 + 32 + 8 + 8; // disc + authority + mint + balance
    const mintBytes = 8 + 32 + 8 + 1; // disc + mintAuthority + supply + decimals

    const nonce = new anchor.BN(4);

    const [handler, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("handler"), tokenHandler.programId.toBuffer()],
      token.programId
    );


    await token.methods
      .registerHandler(tokenAccountBytes, mintBytes)
      .accounts({
        handler,
        payer: token.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        handlerProgram: tokenHandler.programId,
      })
      .rpc();

    const storedHandler = await token.account.tokenHandler.fetch(
      handler
    );

    console.log(storedHandler);

    const nonceBytes = Buffer.from(nonce.toArray('le', 8))

    const [mint, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("mint"), nonceBytes],
      token.programId
    );

    console.log("mint: ", mint);
    console.log(bump);
    // console.log(leBump);


    const mintAuthority = anchor.web3.Keypair.generate();

    const data = tokenHandler.coder.instruction.encode("initialize_mint", {
      mintAuthority: mintAuthority.publicKey,
      decimals: 6,
    }).slice(8); // slice off the first 8 bytes because they're already hardcoded inside the program

    console.log(data);

    await token.methods
      .initializeMint(nonce, data)
      .accounts({
        handler,
        handlerProgram: tokenHandler.programId,
        mint,
        payer: token.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const storedMint = await connection.getAccountInfoAndContext(mint);

    console.log(storedMint);

    const [tokenAccount, tokenAccountBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("token"), nonceBytes, provider.publicKey.toBuffer()],
      token.programId
    );

    await token.methods.initializeTokenAccount(provider.publicKey, nonce, Buffer.from([]))
      .accounts({
        handler,
        handlerProgram: tokenHandler.programId,
        tokenAccount,
        payer: provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    let storedTokenAccount = await tokenHandler.account.tokenAccount.fetch(tokenAccount);

    console.log(storedTokenAccount.balance.toNumber());

    await token.methods.mintTo(new anchor.BN(1000))
      .accounts({
        handler,
        handlerProgram: tokenHandler.programId,
        to: tokenAccount,
        mint,
        mintAuthority: mintAuthority.publicKey,
      })
      .signers([mintAuthority])
      .rpc();

    storedTokenAccount = await tokenHandler.account.tokenAccount.fetch(tokenAccount);

    console.log(storedTokenAccount.balance.toNumber());



  });
});
