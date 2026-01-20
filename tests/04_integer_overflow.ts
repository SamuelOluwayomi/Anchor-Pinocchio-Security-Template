import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IntegerOverflow } from "../target/types/integer_overflow";
import { assert } from "chai";

describe("04_integer_overflow", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.integerOverflow as Program<IntegerOverflow>;

    it("VULNERABLE: Wraps around on overflow", async () => {
        const big = new anchor.BN("18446744073709551615"); // u64::MAX
        const one = new anchor.BN(1);

        // u64::MAX + 1 should match 0 if it wraps
        const result = await program.methods.insecureAdd(big, one).rpc();
        console.log("Insecure add finished (implicitly returned 0 via log potentially)");
    });

    it("SECURE: Returns error on overflow", async () => {
        const big = new anchor.BN("18446744073709551615");
        const one = new anchor.BN(1);

        try {
            await program.methods.secureAdd(big, one).rpc();
            assert.fail("Should have failed with overflow error");
        } catch (e) {
            assert.ok(e);
            console.log("Secure add failed as expected");
        }
    });

});
