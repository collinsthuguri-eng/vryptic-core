# VRYPTIC System Architecture

This document outlines the Sensor-to-Ledger pipeline for the VRYPTIC Protocol.

## 1. Capture Layer (AOSP/Mobile)

- **CMOS Interaction:** The VRYPTIC-hardened camera interface requests a RAW
  frame buffer directly from the sensor — before the OS image processing
  pipeline can alter the data.
- **PRNU Extraction:** The NPCC (Neural-Physical Cross-Correlation) module
  identifies unique silicon noise patterns (Photo-Response Non-Uniformity)
  within the Trusted Execution Environment (TEE), isolated from any
  software-layer interference.
- **TEE Handshake:** The PRNU profile is hashed and signed by the device's
  unique hardware key via AOSP StrongBox and Solana Seed Vault.

### Platform Scope: Android Only

The VRYPTIC capture layer is Android-exclusive. This is a deliberate
architectural decision, not a scope limitation.

Apple's AVFoundation framework does not expose raw sensor data at the depth
required by the NPCC engine. Android's Camera2 API provides direct access to
the RAW frame buffer before ISP processing — the exact interception point
required for reliable PRNU extraction. VRYPTIC will not issue Truth Records
it cannot fully stand behind. macOS is supported for the Vryptic Editing Suite
only. At scale, if market pressure causes Apple to open sensor-level access,
VRYPTIC integrates on its own terms and timeline.

## 2. Validation Layer (NPCC Engine)

- **Truth Floor Scoring:** The NPCC engine computes a composite Truth Floor
  Index (Tf) across three independent verification dimensions — Physical
  (PRNU match), Neural (deep learning forensic analysis), and Temporal
  (hardware-rooted timestamp). A score of Tf ≥ 0.85 is required for a
  Truth Seal to be issued.
- **Correlation Check:** The generated Truth Seal is compared against the
  device's registered hardware profile in the Arweave-stored device registry.
- **Verdict:** Media is classified as AUTHENTIC (Tf ≥ 0.85), FLAGGED
  (0.30 < Tf < 0.85), or REJECTED (Tf ≤ 0.30 — synthetic media detected).

## 3. Asynchronous Hardware Attestation (AHA Protocol)

- **Offline Truth Sealing:** In zero-connectivity environments, the AHA
  Protocol enables the device to produce a hardware-rooted Proof of Reality
  locally — signed within the TEE without requiring a live network connection.
- **Encapsulated Manifest:** The PRNU profile, Tf score, and hardware
  timestamp are packaged into a signed manifest stored securely on-device.
- **Ledger Sync:** The moment connectivity is restored, the manifest is
  pushed to the Solana Ledger with the original capture timestamp intact.
  The ledger never receives unverified data.
- **Sync Discipline:** AHA sync is throttled — 10 Truth Records per cycle,
  30-second pauses between cycles, background-only via Android WorkManager.
  Chronological capture order is preserved across all batches.

## 4. Ledger Layer (Solana + Arweave)

- **Metadata Anchoring:** The Truth Seal and asset URI are submitted to a
  Solana Anchor program for on-chain provenance indexing.
- **State Compression:** Utilizing Solana State Compression (Concurrent
  Merkle Trees) to ensure low-cost, high-scale provenance records
  (~$0.0001 per Truth Record).
- **Permanent Storage:** The encrypted asset and forensic audit trail are
  pushed to Arweave for immutable, permanent archival via the Permaweb.

### Dual Arweave Archival (Video)

For video assets, VRYPTIC operates a two-stage archival model:

- **Truth Record #1 (TR#1) — Raw Archive:** The raw video file is archived
  to Arweave at the moment of capture. The file hash is anchored to the
  Solana ledger instantly as TR#1.
- **Truth Record #2 (TR#2) — Edited Archive:** The edited export is archived
  to Arweave at the point of final export. TR#2 is cryptographically linked
  to TR#1 via the Derivative Attestation chain.

Both copies are permanently retrievable and independently comparable on the
ledger. A downstream verifier can retrieve the raw original and the published
edit simultaneously and audit every transformation between them.

## 5. Editing Layer

The VRYPTIC editing surface is split across two distinct products:

### Light Editing Suite (In-App)
- Embedded directly inside the VRYPTIC Camera App.
- Ships at launch (Q1/Q2 2028). Always free.
- Target users: citizen journalists, field contributors, enthusiasts.
- Covers essential transformations: crop, colour correction, trim.
- Every edit generates a TEE-signed Derivative Attestation. Chain of custody
  is maintained from capture through export.

### Vryptic Editing Suite (Desktop — Windows + macOS)
- Standalone professional application. Ships post-launch.
- V1 (Q3 Year 2 → Q1 Year 3): Photo editing with full provenance pipeline.
- V2 (Q2 Year 3): Video editing with dual Arweave archival model live.
- Target users: professional journalists, legal professionals, newsrooms,
  insurance investigators, NGOs.
- Derivative Attestation runs on a dedicated CPU thread in parallel with GPU
  render (Metal on macOS, DirectX/CUDA on Windows). No additional wait time
  felt by the user at export.
- Paid product. Subscription and seat-based licensing. Feeds the $VRYP
  burn-and-mint economy.

---

*VRYPTIC Protocol — Nairobi, Kenya.*
*"We don't verify the pixels. We verify the physics."*
