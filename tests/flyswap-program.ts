import * as anchor from "@coral-xyz/anchor";
import { FlyswapProgram } from "../target/types/flyswap_program";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { createCollection, MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";
import { generateSigner, keypairIdentity } from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { fromWeb3JsKeypair } from "@metaplex-foundation/umi-web3js-adapters";
import { assert } from "chai";

describe("flyswap-program", () => {

  const wallet = anchor.Wallet.local()

  const umi = createUmi('http://127.0.0.1:8899').use(keypairIdentity(fromWeb3JsKeypair(wallet.payer)));
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.flyswapProgram as anchor.Program<FlyswapProgram>;
  const [seat, seatbump] = PublicKey.findProgramAddressSync(
    [Buffer.from("seat"), wallet.publicKey.toBuffer()],
    program.programId
  )
  console.log("Seat PDA", seat)
  let asset = Keypair.generate();

  it("Mint Seat", async () => {
    const collectionSigner = generateSigner(umi);
    await createCollection(umi, {
      collection: collectionSigner,
      name: 'Newest Collection',
      uri: 'https://github.com/Emman442/Quiz-application-with-leaderboard-feature/blob/main/mpl.json',
    }).sendAndConfirm(umi);
    // const collectionSigner = generateSigner(umi)
    // await createCollection(umi, {
    //   collection: collectionSigner,
    //   name: 'Newest Collection',
    //   uri: 'https://github.com/Emman442/Quiz-application-with-leaderboard-feature/blob/main/mpl.json',
    // })
    const tx = await program.methods.processMintSeat(1, new anchor.BN(1745791232), new anchor.BN(1745791239), new anchor.BN(1745791264), "Arik Air", "https://github.com/Emman442/Quiz-application-with-leaderboard-feature/blob/main/mpl.json").accountsPartial({
      signer: wallet.publicKey,
      payer: wallet.publicKey,
      seat,
      collection: collectionSigner.publicKey,
      mplCoreProgram: MPL_CORE_PROGRAM_ID,
      updateAuthority: wallet.publicKey,
      asset: asset.publicKey,
      systemProgram: SystemProgram.programId
    }).signers([asset, wallet.payer]).rpc()
    console.log("Your transaction signature", tx);;

    const seatAccount = await program.account.seat.fetch(seat);
    assert.equal(seatAccount.isOccupied, false);
    assert.equal(seatAccount.seatNumber, 1);
  });
});

