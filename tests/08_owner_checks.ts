import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OwnerChecks } from "../target/types/owner_checks";
import { expect } from "chai";

describe("owner_checks", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.OwnerChecks as Program<OwnerChecks>;

    it("demonstrates insecure owner check vulnerability", async () => {
        const config = anchor.web3.Keypair.generate();

        await program.methods
            .insecureUpdate(new anchor.BN(42))
            .accounts({
                config: config.publicKey,
                authority: provider.wallet.publicKey,
            })
            .rpc();
    });

    it("demonstrates secure owner validation", async () => {
        const config = anchor.web3.Keypair.generate();

        await program.methods
            .secureUpdate(new anchor.BN(42))
            .accounts({
                config: config.publicKey,
                authority: provider.wallet.publicKey,
            })
            .rpc();
    });
});
