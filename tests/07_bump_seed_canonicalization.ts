import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BumpSeedCanonical ization } from "../target/types/bump_seed_canonicalization";
import { expect } from "chai";

describe("bump_seed_canonicalization", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.BumpSeedCanonical ization as Program<BumpSeedCanonical ization >;

    it("demonstrates insecure bump seed usage", async () => {
        const authority = provider.wallet.publicKey;
        const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("vault"), authority.toBuffer()],
            program.programId
        );

        await program.methods
            .insecureInit(255)
            .accounts({
                vault: vaultPda,
                authority: authority,
            })
            .rpc();
    });

    it("demonstrates secure bump seed validation", async () => {
        const authority = provider.wallet.publicKey;
        const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("vault"), authority.toBuffer()],
            program.programId
        );

        await program.methods
            .secureInit()
            .accounts({
                vault: vaultPda,
                authority: authority,
            })
            .rpc();
    });
});
