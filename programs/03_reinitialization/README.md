# Lesson 3: Double Initialization / Type Confusion

## 💀 The Vulnerability
If an instruction initializes account state (e.g. "sets the admin") but doesn't check if it was **already** initialized, an attacker can call it again on an existing account.

## 💥 The Exploit
1. Admin deploys and initializes the Global Config, setting themselves as Admin.
2. Attacker calls `insecure_init` passing the Global Config account.
3. The program blindly overwrites `state.admin` with the attacker's key.
4. Attacker now controls the protocol.

## 🛡️ The Fix
Use Anchor's `init` constraint. This performs 3 checks:
1. The account is owned by the program.
2. The account discriminator matches (or is zero/unset if new).
3. The system program created it (if using PDAs).

If you must re-use an account (without `init`), manually check a boolean flag `if state.is_initialized { return Err(...) }`.
