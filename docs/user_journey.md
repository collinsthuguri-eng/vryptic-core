# The VRYPTIC User Journey

A complete end-to-end walkthrough of the Hardware-to-Ledger provenance pipeline —
from the first microsecond of capture to third-party verification.

---

## Stage 1: Capture

The user opens the **VRYPTIC Camera App** on their mobile device.

At the moment of capture, the NPCC (Neural-Physical Cross-Correlation) engine
intercepts the RAW frame buffer directly from the CMOS sensor — before the OS
image processing pipeline can alter the data. The unique PRNU (Photo-Response
Non-Uniformity) silicon fingerprint of the sensor is extracted within the
Trusted Execution Environment (TEE), isolated from any software-layer
interference.

The NPCC engine computes the **Truth Floor Index (Tf)**:

```
Tf = w₁·ρ(K,W) + w₂·Ψ(I) + w₃·Λ(τ)
```

- `ρ(K,W)` — Physical Floor: PRNU silicon fingerprint match
- `Ψ(I)`   — Neural Floor: deep learning forensic analysis
- `Λ(τ)`   — Temporal Floor: hardware-rooted timestamp

**Verdict:**
- `Tf ≥ 0.85` → ✅ AUTHENTIC — Truth Seal issued
- `0.30 < Tf < 0.85` → ⚠️ FLAGGED — Secondary review triggered
- `Tf ≤ 0.30` → ❌ REJECTED — Synthetic media detected

A **Truth Seal** is generated: a cryptographic envelope binding the image to its
physical hardware origin.

---

## Stage 2: Attest

The Truth Seal is passed to the **Solana Mobile Stack (SMS)**.

The device's unique hardware key — secured inside the **AOSP StrongBox** and
**Solana Seed Vault** — signs the Truth Seal within the TEE. The signed seal
is then submitted to the **vryptic-core** Anchor program on the Solana ledger
via the `register_truth_seal` instruction.

A permanent, timestamped **Truth Record** is committed on-chain at a cost of
approximately `$0.0001` per record using Solana State Compression (Concurrent
Merkle Trees).

**Zero-Connectivity Scenario — AHA Protocol:**
If the device has no network connection at the time of capture, the
**Asynchronous Hardware Attestation (AHA) Protocol** activates. The Truth Seal
is signed and sealed locally within the TEE, stored in an encapsulated manifest
on-device. The moment connectivity is restored, the manifest is pushed to the
Solana Ledger with the original capture timestamp intact. The ledger never
receives unverified data.

---

## Stage 3: Enhance (Edit)

The user opens the captured asset inside the **VRYPTIC Editor**.

Every edit operation — crop, colour grade, trim, filter, export — generates a
**TEE-signed Derivative Attestation**. The editor operates on a
**Non-Destructive Provenance Engine**: the original Truth Seal is never
overwritten. Instead, each edit is appended as a provenance-safe event,
maintaining an unbroken chain of custody from lens to final export.

The edit history is human-readable and cryptographically verifiable. A
downstream verifier can reconstruct every transformation applied to the asset.

---

## Stage 4: Permanentize

Once the user finalises the asset, the complete provenance record — original
Truth Seal plus the full Derivative Attestation chain — is pushed to
**Arweave** via the **VRYPTIC Sentinel Network**.

Sentinel Nodes running the `sentinel_process` AO handler receive the
`RegisterProvenance` action, verify the Solana signature, and commit the
encrypted asset and forensic audit trail to the **Permaweb** for immutable,
permanent archival. The asset now has a dual anchor: Solana (fast, indexed,
queryable) and Arweave (permanent, tamper-proof storage).

---

## Stage 5: Verify

Any third party — a journalist, platform, court, or API consumer — can verify
the asset's authenticity and chain of custody.

**Direct Verification:**
The verifier submits the asset to the **VRYPTIC Verifier**. The system queries
the Solana ledger for the registered Truth Seal, reconstructs the Tf score, and
confirms the hardware origin against the device registry maintained by the
**Hardware Oracle** (`vryptic-governance`).

**Platform Integration:**
Social platforms and media organisations can integrate the **VRYPTIC Social API**
(`vryptic-social-api`) to perform automated origin checks on uploaded content.
The `SocialAuthenticator` interface returns a `VerificationStatus` struct
confirming hardware verification, edit history integrity, and the original
ledger timestamp.

**C2PA Compatibility:**
VRYPTIC Truth Seals are structured to be C2PA-compliant. Platforms already
operating within the C2PA standard can ingest VRYPTIC provenance records
without additional integration work — VRYPTIC operates as the silicon-rooted
layer beneath C2PA, not a competing standard.

---

## Pipeline Summary

```
[CMOS Sensor]
     │
     ▼
[NPCC Engine — TEE]          ← PRNU extraction, Tf scoring
     │
     ▼
[Truth Seal — SMS Seed Vault] ← Hardware-signed attestation
     │
     ├──── [AHA Protocol]    ← Offline fallback, batch sync on reconnect
     │
     ▼
[Solana Ledger]              ← On-chain Truth Record (~$0.0001)
     │
     ▼
[VRYPTIC Editor]             ← Derivative Attestation chain (non-destructive)
     │
     ▼
[Arweave Sentinel Network]   ← Permanent Permaweb archival
     │
     ▼
[VRYPTIC Verifier / Social API / C2PA Layer] ← Third-party verification
```

---

*VRYPTIC Protocol — Nairobi, Kenya.*
*"Two founders. Four incidents. One infrastructure layer that makes truth undeniable from the first microsecond."*
