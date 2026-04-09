# vryptic-anchor — R&D Sandbox

**Role:** Exploration. Not for Mainnet deployment.

This is the VRYPTIC engineering sandbox. It serves as the testing ground for early Solana Mobile Stack (SMS) interactions, Concurrent Merkle Tree experiments for State Compression, local validator scripts, and initial hardware handshake modeling — before any logic is promoted to `vryptic-core`.

---

## What Lives Here

- Early proof-of-concept scaffolding for the Hardware-to-Ledger pipeline
- Integration tests against Solana Devnet
- Experimental Truth Seal logic that has not yet been hardened for production

## What Does NOT Live Here

- Any code intended for Solana Mainnet deployment
- Any logic that will be submitted to a security audit

---

## Relationship to `vryptic-core`

Once logic in this sandbox is validated and hardened, it is promoted to `programs/vryptic-core/` — the production program. The two directories are intentionally separate to ensure the audit surface of `vryptic-core` remains clean and minimal.

> Think of `vryptic-anchor` as the lab. `vryptic-core` is the operating room.
