import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TypeCosplay } from "../target/types/type_cosplay";
import { expect } from "chai";

describe("type_cosplay", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.TypeCosplay as Program<TypeCosplay>;

    it("demonstrates account discrimination vulnerability", async () => {
        const userAccount = anchor.web3.Keypair.generate();

        await program.methods
            .insecureWithdraw(new anchor.BN(100))
            .accounts({
                userAccount: userAccount.publicKey,
                authority: provider.wallet.publicKey,
            })
            .rpc();
    });

    it("demonstrates secure account type validation", async () => {
        const userAccount = anchor.web3.Keypair.generate();

        await program.methods
            .secureWithdraw(new anchor.BN(100))
            .accounts({
                userAccount: userAccount.publicKey,
                authority: provider.wallet.publicKey,
            })
            .rpc();
    });
});
