# VRYPTIC System Architecture

This document outlines the Sensor-to-Ledger pipeline for the VRYPTIC Protocol.

## 1. Capture Layer (AOSP/Mobile)
* **CMOS Interaction:** The VRYPTIC-hardened camera interface requests a RAW frame buffer.
* **PRNU Extraction:** The NPCC module identifies unique silicon noise patterns within the Trusted Execution Environment (TEE).
* **TEE Handshake:** The PRNU profile is hashed and signed by the device's unique hardware key (StrongBox/Seed Vault).

## 2. Validation Layer (NPCC)
* **Correlation Check:** The generated "Truth Seal" is compared against the device's registered hardware profile.
* **Timestamping:** A sub-second finality timestamp is applied to the metadata.

## 3. Ledger Layer (Solana/Arweave)
* **Metadata Anchoring:** The "Truth Seal" and asset URI are submitted to a Solana Anchor program.
* **State Compression:** Utilizing Solana State Compression (Concurrent Merkle Trees) to ensure low-cost, high-scale provenance for millions of assets.
* **Permanent Storage:** The encrypted asset is pushed to Arweave for immutable long-term availability.
