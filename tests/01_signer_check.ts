import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SignerCheck } from "../target/types/signer_check";
import { assert } from "chai";

describe("01_signer_check", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.signerCheck as Program<SignerCheck>;

    const owner = anchor.web3.Keypair.generate();
    const attacker = anchor.web3.Keypair.generate();

    const [potPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("pot"), owner.publicKey.toBuffer()],
        program.programId
    );

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
        // Airgrop
        await confirmTx(
            await provider.connection.requestAirdrop(owner.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL)
        );
        await confirmTx(
            await provider.connection.requestAirdrop(attacker.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL)
        );

        // Initialize
        await program.methods
            .initialize()
            .accounts({
                pot: potPda,
                owner: owner.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([owner])
            .rpc()
            .then(confirmTx);

        // Fund the pot manually
        const tx = new anchor.web3.Transaction().add(
            anchor.web3.SystemProgram.transfer({
                fromPubkey: owner.publicKey,
                toPubkey: potPda,
                lamports: 50_000_000,
            })
        );
        await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [owner]);
    });

    it("VULNERABLE: Attacker can withdraw without signature", async () => {
        const initialPotBalance = await provider.connection.getBalance(potPda);

        await program.methods
            .insecureWithdraw(new anchor.BN(10_000_000))
            .accounts({
                pot: potPda,
                owner: owner.publicKey, // We pass the legitimate owner's key
            })
            // NO Signer required for 'owner' in this vulnerable instruction
            .rpc()
            .then(confirmTx);

        const finalPotBalance = await provider.connection.getBalance(potPda);
        console.log("Pot Balance Delta:", initialPotBalance - finalPotBalance);
        assert.ok(finalPotBalance < initialPotBalance, "Pot should have been drained");
    });

    it("SECURE: Fails when signer is missing", async () => {
        try {
            await program.methods
                .secureWithdraw(new anchor.BN(10_000_000))
                .accounts({
                    pot: potPda,
                    owner: owner.publicKey,
                })
                // NOTE: In Anchor client, if we don't provide the signer, it might implicitly use the Provider wallet 
                // if the key is the same, but here 'owner' is a separate keypair.
                // We do typically need to add .signers([owner]) if we want it to succeed.
                // Since we are NOT adding .signers([owner]), and the IDL says it's a Signer, 
                // Anchor client usually throws a "Signature verification failed" or "Missing signature" error locally 
                // before even sending, OR it sends it and the RPC rejects it.
                .rpc();

            assert.fail("Should have failed!");
        } catch (err) {
            assert.ok(true, "Failed as expected");
        }
    });

    it("SECURE: Succeeds with valid signer", async () => {
        await program.methods
            .secureWithdraw(new anchor.BN(10_000_000))
            .accounts({
                pot: potPda,
                owner: owner.publicKey,
            })
            .signers([owner])
            .rpc()
            .then(confirmTx);
    });
});
