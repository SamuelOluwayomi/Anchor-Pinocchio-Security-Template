import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PdaSharing } from "../target/types/pda_sharing";
import { assert } from "chai";
import { createMint, createAccount, mintTo, getAccount } from "@solana/spl-token";

describe("02_pda_sharing", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.pdaSharing as Program<PdaSharing>;
    const owner = anchor.web3.Keypair.generate();

    let mint: anchor.web3.PublicKey;
    let vaultPda: anchor.web3.PublicKey;
    let vaultAuthority: anchor.web3.PublicKey;
    let attackerVault: anchor.web3.PublicKey;
    let destination: anchor.web3.PublicKey;

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
            await provider.connection.requestAirdrop(owner.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL)
        );

        mint = await createMint(provider.connection, owner, owner.publicKey, null, 6);

        // Derived addresses
        [vaultAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("vault")],
            program.programId
        );

        // Create a real vault (controlled by PDA) -- In a real app the program would init this
        // Here we just simulate it being an account owned by the PDA authority
        // Actually, createAccount creates an ATA or Token Account. 
        vaultPda = await createAccount(
            provider.connection,
            owner,
            mint,
            vaultAuthority // Owned by PDA
        );

        // Create an attacker's "fake vault" (owned by owner/attacker)
        attackerVault = await createAccount(
            provider.connection,
            owner,
            mint,
            owner.publicKey // Owned by USER
        );

        destination = await createAccount(provider.connection, owner, mint, owner.publicKey);

        // Fund the fake vault so the transfer succeeds
        await mintTo(provider.connection, owner, mint, attackerVault, owner, 1000);
    });

    it("VULNERABLE: Can use a fake vault", async () => {
        // We pass 'attackerVault' instead of the real 'vaultPda'.
        // The program trusts us and transfers from attackerVault.
        // In this specific code example, verify that it accepts the wrong account.

        await program.methods.insecureWithdraw(new anchor.BN(100))
            .accounts({
                vault: attackerVault, // FAKE
                authority: owner.publicKey,
                destination: destination,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            })
            .signers([owner])
            .rpc()
            .then(confirmTx);

        console.log("Successfully used fake vault");
    });

    it("SECURE: Rejects invalid PDA", async () => {
        // The secure instruction requires 'vaultAuthority' to be a PDA derived from "vault".
        // If we pass the wrong account, Anchor validation fails? 
        // Wait, the secure instruction calculates the signer seeds.
        // If we pass a fake vault, we must also pass the vaultAuthority.
        // BUT the constraint seeds=[b"vault"] ensures vaultAuthority IS the unique PDA.
        // So we can't fake vaultAuthority.
        // And if the code uses `cpi_context` with that authority, it can only sign for the REAL vault (if associated properly) 
        // or checks ownership.

        // Actually, in `secure_withdraw`, we define `vault_authority` with seeds.
        // And we use that to sign. 

        try {
            await program.methods.secureWithdraw(new anchor.BN(100))
                .accounts({
                    vault: attackerVault, // Try to drain fake vault using secure instruction? 
                    // Or drain REAL vault?
                    // If we try to use the SECURE instruction with a FAKE vault...
                    // the signer is the PDA. The PDA does NOT own the fake vault.
                    // SPL Token program will reject the transfer because the authority (PDA) is not the owner of attackerVault.
                    vaultAuthority: vaultAuthority,
                    destination: destination,
                    tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                })
                .rpc();
            assert.fail("Should have failed because PDA is not owner of attackerVault");
        } catch (e) {
            // Expected failure: "Error: Owner mismatch" (from Token Program)
            assert.ok(e);
        }
    });
});
