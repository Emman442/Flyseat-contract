import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FlyswapProgram } from "../target/types/flyswap_program";

describe("flyswap-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.flyswapProgram as Program<FlyswapProgram>;

  it("Mint Seat", async () => {
    // Add your test here.
    const tx = await program.methods.processMintSeat(1, new anchor.BN(1745791232), new anchor.BN(1745791239), new anchor.BN(1745791264), "Arik Air", "https://github.com/Emman442/Quiz-application-with-leaderboard-feature/blob/main/mpl.json").rpc();
    console.log("Your transaction signature", tx);
  });
});
