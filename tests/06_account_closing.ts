import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountClosing } from "../target/types/account_closing";
import { expect } from "chai";

describe("account_closing", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.AccountClosing as Program<AccountClosing>;

  it("demonstrates insecure account closing vulnerability", async () => {
    const vault = anchor.web3.Keypair.generate();
    
    await program.methods
      .insecureClose()
      .accounts({
        vault: vault.publicKey,
        destination: provider.wallet.publicKey,
      })
      .rpc();
  });

  it("demonstrates secure account closing", async () => {
    const vault = anchor.web3.Keypair.generate();
    
    await program.methods
      .secureClose()
      .accounts({
        vault: vault.publicKey,
        destination: provider.wallet.publicKey,
      })
      .rpc();
  });
});
