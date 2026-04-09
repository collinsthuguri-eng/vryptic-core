# VRYPTIC On-Chain Programs

This directory contains all Solana Anchor programs for the VRYPTIC Protocol, organized by purpose.

---

## Program Architecture

VRYPTIC follows the standard multi-program Anchor workspace pattern, separating exploration from execution across distinct program directories:

| Directory | Role | Purpose |
|---|---|---|
| `vryptic-anchor/` | R&D Sandbox | Integration testing, early PoC work, experimental builds |
| `vryptic-core/` | Production Ledger | Audit-ready, Mainnet-bound Truth Seal logic |
| `vryptic-governance/` | Hardware Oracle | Device registry and hardware unit revocation |
| `vryptic-sentinel/` | Arweave AO Process | Permanent provenance anchoring on the Permaweb |

---

## `vryptic-anchor` — The R&D Sandbox

This is the engineering workspace. It is where the Strike Team tests new Merkle tree structures for Solana State Compression, runs heavy integration tests against the Solana Devnet, and models early SMS hardware handshakes before the architecture is fully formalized.

**Nothing in this directory goes to Mainnet.** It exists to protect `vryptic-core` from experimental code.

## `vryptic-core` — The Production Ledger Program

This is the clean, deployable program that will be submitted to a professional security audit before Solana Mainnet deployment. It contains only the `RegisterSeal` instruction and the `TruthSeal` account structure — the minimal, hardened logic required to commit a hardware-rooted provenance hash to the Solana ledger.

As the protocol scales, additional production programs (e.g. `vryptic-rewards` for Sentinel uptime economics) will be added as sibling directories here — not folded into `vryptic-core`.

---

## Design Principle

> Separate Exploration from Execution. The physics moat is only as strong as the code protecting it.
