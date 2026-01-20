import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PinocchioVsAnchor } from "../target/types/pinocchio_vs_anchor";

describe("05_pinocchio_vs_anchor", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.pinocchioVsAnchor as Program<PinocchioVsAnchor>;

    it("Anchor: Works implicitly", async () => {
        // Simple RPC call to verify the anchor instruction works
        const tx = await program.methods.anchorSayHello().rpc();
        console.log("Anchor Hello Signature:", tx);
    });

    // We can't easily test the Pinocchio entrypoint here because it's not hooked up to the main entrypoint
    // (Anchor owns it). This lesson is primarily for Code Review / Educational comparison.
});
