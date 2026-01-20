import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Reinitialization } from "../target/types/reinitialization";
import { assert } from "chai";

describe("03_reinitialization", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.reinitialization as Program<Reinitialization>;
    const user = anchor.web3.Keypair.generate();
    const attacker = anchor.web3.Keypair.generate();

    // Helper for deprecated confirmTransaction
    const confirmTx = async (signature: string) => {
        const latestBlockhash = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction(
            {
                signature,
                ...latestBlockhash,
            },
            "confirmed"
        );
        return signature;
    };

    before(async () => {
        await confirmTx(
            await provider.connection.requestAirdrop(user.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL)
        );
        await confirmTx(
            await provider.connection.requestAirdrop(attacker.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL)
        );
    });

    it("VULNERABLE: Account can be re-initialized (overwritten)", async () => {
        // First, legitimate init
        const stateKeypair = anchor.web3.Keypair.generate();

        // Use a "manual" init flow for the vulnerable example since it expects an existing account
        // We first create the account with system program, then call the program to "init" its data.

        const space = 8 + 32;
        const createTx = new anchor.web3.Transaction().add(
            anchor.web3.SystemProgram.createAccount({
                fromPubkey: user.publicKey,
                newAccountPubkey: stateKeypair.publicKey,
                lamports: await provider.connection.getMinimumBalanceForRentExemption(space),
                space,
                programId: program.programId
            })
        );
        await anchor.web3.sendAndConfirmTransaction(provider.connection, createTx, [user, stateKeypair]);

        // Admin sets themself
        await program.methods.insecureInit()
            .accounts({
                state: stateKeypair.publicKey,
                user: user.publicKey
            })
            .signers([user])
            .rpc()
            .then(confirmTx);

        let state = await program.account.state.fetch(stateKeypair.publicKey);
        assert.equal(state.admin.toBase58(), user.publicKey.toBase58());

        // ATTACK: Attacker calls the same function
        await program.methods.insecureInit()
            .accounts({
                state: stateKeypair.publicKey,
                user: attacker.publicKey
            })
            .signers([attacker])
            .rpc()
            .then(confirmTx);

        state = await program.account.state.fetch(stateKeypair.publicKey);
        assert.equal(state.admin.toBase58(), attacker.publicKey.toBase58(), "Admin should have been overwritten");
    });

    it("SECURE: Initialization fails if already initialized", async () => {
        const [statePda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("state"), user.publicKey.toBuffer()],
            program.programId
        );

        // First init should work
        await program.methods.secureInit()
            .accounts({
                state: statePda,
                user: user.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([user])
            .rpc()
            .then(confirmTx);

        // Second init should fail (Anchor Constraint init fails if account exists)
        try {
            await program.methods.secureInit()
                .accounts({
                    state: statePda,
                    user: user.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([user])
                .rpc();
            assert.fail("Should have failed to re-init");
        } catch (e) {
            // Expected
            assert.ok(e);
        }
    });

});
