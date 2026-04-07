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

## 4. Ledger Layer (Solana + Arweave)

- **Metadata Anchoring:** The Truth Seal and asset URI are submitted to a 
  Solana Anchor program for on-chain provenance indexing.
- **State Compression:** Utilizing Solana State Compression (Concurrent 
  Merkle Trees) to ensure low-cost, high-scale provenance records 
  (~$0.0001 per Truth Record).
- **Permanent Storage:** The encrypted asset and forensic audit trail are 
  pushed to Arweave for immutable, permanent archival via the Permaweb.
