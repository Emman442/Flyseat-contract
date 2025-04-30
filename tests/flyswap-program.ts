import * as anchor from "@coral-xyz/anchor";
import { FlyswapProgram } from "../target/types/flyswap_program";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { createCollection, MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";
import { generateSigner, keypairIdentity } from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { fromWeb3JsKeypair } from "@metaplex-foundation/umi-web3js-adapters";
import { assert } from "chai";
import {
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAssociatedTokenAddress
} from "@solana/spl-token";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("flyswap-program", () => {
const provider = anchor.AnchorProvider.env();
  const wallet = anchor.Wallet.local()

  const umi = createUmi('http://127.0.0.1:8899').use(keypairIdentity(fromWeb3JsKeypair(wallet.payer)));
  anchor.setProvider(provider);
  const program = anchor.workspace.flyswapProgram as anchor.Program<FlyswapProgram>;
  const [seat, seatbump] = PublicKey.findProgramAddressSync(
    [Buffer.from("seat"), wallet.publicKey.toBuffer()],
    program.programId
  )
  console.log("Seat PDA", seat)
  let asset = Keypair.generate();

  const collectionSigner = generateSigner(umi); // ✅ move here so both tests can reuse it

  before(async () => {
    await createCollection(umi, {
      collection: collectionSigner,
      name: 'Newest Collection',
      uri: 'https://github.com/Emman442/Quiz-application-with-leaderboard-feature/blob/main/mpl.json',
    }).sendAndConfirm(umi);
  });


  it("Mint Seat", async () => {   
    const tx = await program.methods.mintSeat(1, new anchor.BN(1745791232), new anchor.BN(1745791239), new anchor.BN(1745791264), "Arik Air", "https://github.com/Emman442/Quiz-application-with-leaderboard-feature/blob/main/mpl.json").accountsPartial({
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


  it("Buy Seat", async()=>{
    const mint = await createMint(provider.connection, wallet.payer, wallet.publicKey, null, 0);
    const buyerTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      mint,
      wallet.publicKey
    );
    const seller = Keypair.generate();
    const sellerTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      mint,
      seller.publicKey
    );

    // Mint 100 tokens to buyer
    await mintTo(
      provider.connection,
      wallet.payer,
      mint,
      buyerTokenAccount,
      wallet.payer,
      100
    );
  
    const tx = await program.methods.buySeat(new anchor.BN(10)).accountsPartial({
      buyer: wallet.publicKey,
      seat,
      asset: asset.publicKey,
      systemProgram: SystemProgram.programId,
      collection: collectionSigner.publicKey,
      mint,
      buyerTokenAccount,
      seller: sellerTokenAccount,
      mplCoreProgram: MPL_CORE_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID
    }).signers([wallet.payer]).rpc();

    console.log("Buy Seat transaction signature", tx);
  })

  it("Redeem Seat", async()=>{
    const tx = await program.methods.redeemSeat().accountsPartial({ 
      redeemer: wallet.publicKey,
      seat,
      systemProgram: SystemProgram.programId,
    })
  })
});

