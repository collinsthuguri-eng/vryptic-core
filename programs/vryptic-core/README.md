# vryptic-core — Production Ledger Program

**Role:** Execution. Audit-ready. Mainnet-bound.

This is the VRYPTIC production on-chain program. It contains the clean, minimal logic required to register a hardware-rooted Truth Seal on the Solana ledger — and nothing else.

---

## What Lives Here

- `RegisterSeal` — the core instruction that validates an NPCC attestation and commits the provenance hash to Solana
- `TruthSeal` — the on-chain account structure storing the hardware unit authority, provenance hash, and timestamp

## Design Philosophy

This program is deliberately minimal. Every line of code here will be submitted to a professional security audit before Solana Mainnet deployment. Scope is kept narrow by design — additional protocol economics (e.g. Sentinel rewards) will be handled by sibling programs, not by expanding this one.

---

## Relationship to `vryptic-anchor`

Experimental and integration work lives in `programs/vryptic-anchor/`. Logic is only promoted here once it has been validated in the sandbox, reviewed by the Strike Team, and deemed ready for the audit pipeline.

---

## Program ID

```
Vryptic11111111111111111111111111111111111
```

> This is a placeholder ID for the Genesis Sprint development phase. A verified program ID will be issued upon Devnet deployment at Milestone 1.
