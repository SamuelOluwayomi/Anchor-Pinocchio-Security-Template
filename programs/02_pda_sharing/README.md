# Lesson 2: PDA Sharing / Missing Verification

## 💀 The Vulnerability
If your program expects a specific Account (like a global vault or a user-specific settings account) but fails to verify its address (via `seeds` constraints in Anchor), an attacker can pass a **fake account** with data they control.

## 💥 The Exploit
1. Create a fake "Vault" account (just a normal Token Account they own).
2. Call `insecure_withdraw` passing their fake vault as the `vault` account.
3. The program transfers tokens from the fake vault to the destination. 
   - Wait, if they own the fake vault, they are just moving their own money.
   - The REAL danger is usually the other way around: **Writing** to a fake account to trick the indexer, or if the program logic relies on data inside the account to make decisions (e.g., `vault.admin_is_super_user = true`).
   - OR, if multiple PDAs share the same structure (e.g. `UserStats`), an attacker might swap one user's stats for another's to bypass checks.

## 🛡️ The Fix
Use Anchor's `seeds` and `bump` constraints. This forces the runtime to derive the address and check if it matches the passed account.
