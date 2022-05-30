import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Autodca } from "../target/types/autodca";

describe("autodca", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Autodca as Program<Autodca>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
