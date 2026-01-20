# Lesson 1: Missing Signer Check

## 💀 The Vulnerability
In Solana programs, simply passing an account's public key is not enough proof of ownership. You must verify that the user **signed** the transaction.

If you use `AccountInfo` instead of `Signer` in Anchor (or forget `is_signer` check in raw Rust), anyone can pretend to be that user.

## 💥 The Exploit
An attacker can call `insecure_withdraw` passing the **victim's** public key as the `owner` account.
The program checks `if pot.owner == owner.key()`, which is true (the public keys match).
The program then transfers funds to that account.
Wait... does this steal funds?
- If the withdrawal destination is the owner's wallet, the attacker just forced a withdrawal (griefing).
- BUT, if the logic allowed withdrawing to *another* account provided by the authority, the attacker could steal everything. 
- In this specific example, it demonstrates **Identity Spoofing**. The attacker pretends to be the owner to trigger an action the owner didn't authorize.

## 🛡️ The Fix
Use the `Signer<'info>` type in your Anchor validation struct. This ensures the runtime checked the signature.
