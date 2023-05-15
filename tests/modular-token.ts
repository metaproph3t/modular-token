import * as anchor from "@coral-xyz/anchor";

import { Program } from "@coral-xyz/anchor";
import { Token } from "../target/types/token";
import { BasicTokenHandler } from "../target/types/basic_token_handler";
import { assert } from "chai";

describe("modular-token", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const token = anchor.workspace.Token as Program<Token>;
  const tokenHandler = anchor.workspace
    .BasicTokenHandler as Program<BasicTokenHandler>;

  it("Passes tests", async () => {
    const tokenAccountBytes = 8 + 32 + 8 + 8; // discriminator + authority + mint + balance
    const mintBytes = 8 + 32 + 8 + 1; // discriminator + mintAuthority + supply + decimals
    const mintAuthority = anchor.web3.Keypair.generate();

    // Users can recognize tokens by their nonces. 
    // In this case, we are creating 'token 4.'
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

    const nonceBytes = Buffer.from(nonce.toArray('le', 8))

    const [mint, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("mint"), nonceBytes],
      token.programId
    );


    const initializeMintData = tokenHandler.coder.instruction.encode("initialize_mint", {
      mintAuthority: mintAuthority.publicKey,
      decimals: 6,
    }).slice(8); // slice off the first 8 bytes because they're already hardcoded inside the program

    await token.methods
      .initializeMint(nonce, initializeMintData)
      .accounts({
        handler,
        handlerProgram: tokenHandler.programId,
        mint,
        payer: token.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

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

    assert(storedTokenAccount.balance.toNumber() == 0);

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

    assert(storedTokenAccount.balance.toNumber() == 1000);
  });
});
