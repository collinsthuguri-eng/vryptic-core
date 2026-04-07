# VRYPTIC Protocol: Hardware-to-Ledger Provenance

> Establishing a Universal Standard of Truth (UST) for the Generative AI Era.

VRYPTIC is a Decentralized Physical Infrastructure (DePIN) protocol that anchors digital media to physical reality. By extracting unique **PRNU (Photo-Response Non-Uniformity)** silicon fingerprints from CMOS sensors at the point of capture — below the OS layer, before any software can interfere — VRYPTIC creates an immutable provenance record secured by the Solana Ledger.

**We don't verify the pixels. We verify the physics.**

---

## 🛠 The Architecture

VRYPTIC operates a hybrid-source model to ensure hardware-level integrity:

- **Open Interface:** A public HAL (Hardware Abstraction Layer) bridge for mobile dApps to request and verify Truth Seals.
- **On-Chain Validation:** Rust-based Anchor programs utilizing Solana State Compression for scalable, low-cost provenance indexing (~$0.0001 per Truth Record).
- **Secure Module (Proprietary):** The **NPCC (Neural-Physical Cross-Correlation)** engine, executed within the AOSP StrongBox and Solana Seed Vault Trusted Execution Environments (TEE). This module is not open-sourced to protect protocol integrity.

### The NPCC Algorithm

The core identity:
I = x ∘ (1 + K) + Ξ

Where:
- `I` = captured image
- `x` = true scene radiance
- `∘` = Hadamard product (element-wise multiplication)
- `K` = PRNU factor — the unique silicon fingerprint of the sensor
- `Ξ` = additive Gaussian noise

The **Truth Floor Index (Tf)** is a composite verification score:
Tf = w₁·ρ(K,W) + w₂·Ψ(I) + w₃·Λ(τ)

- `ρ(K,W)` — Physical Floor: PRNU match
- `Ψ(I)` — Neural Floor: deep learning forensic analysis
- `Λ(τ)` — Temporal Floor: hardware-rooted timestamp

**Verdict thresholds:**
- `Tf ≥ 0.85` → ✅ AUTHENTIC — Truth Seal issued
- `0.30 < Tf < 0.85` → ⚠️ FLAGGED — Secondary review
- `Tf ≤ 0.30` → ❌ REJECTED — Synthetic media detected

---

## 🚀 Key Features

- **Hardware-Rooted Trust:** Validates the physical sensor, not just the pixels.
- **Zero-Connectivity Resilience:** The **AHA (Asynchronous Hardware Attestation)** Protocol enables offline truth sealing — proofs are signed at source and anchored to the ledger the moment connectivity is restored.
- **Deepfake Immunity:** Neutralizes AI-generated imagery by verifying PRNU silicon fingerprints that no generative model can replicate.
- **Solana Native:** Integrated with the Solana Mobile Stack (SMS), AOSP StrongBox, and Seed Vault for flagship-grade security.
- **Pseudonymous by Design:** Zero-knowledge provenance — we verify the device, not the person.

---

## 📂 Repository Structure
├── docs/                        # Technical schematics and UST protocol specifications
│   └── ARCHITECTURE.md          # Full protocol architecture and system design
├── interfaces/                  # Public HAL for TEE integration
│   └── vryptic_hal.h            # Hardware Abstraction Layer interface definitions
├── programs/vryptic-anchor/src/ # Solana Anchor programs for on-chain state and verification
│   └── lib.rs                   # Initial Anchor scaffold for state compression
├── .gitignore
├── LICENSE                      # Apache 2.0
└── README.md

---

## 🏗 Phase 0: Genesis Sprint

We are currently in the **Genesis Sprint** — a 6-month build period moving VRYPTIC from validated architecture to a working end-to-end Hardware-to-Ledger prototype.

**Milestone 1 (Months 1–2): Security Scaffolding & Device Handshake**
- Build the AOSP-to-Seed Vault bridge
- Deliverable: PoC cryptographic handshake between a mobile TEE and a remote Sentinel Node

**Milestone 2 (Months 3–4): NPCC Logic & Global Fleet Calibration**
- Calibrate NPCC weights across Sony, Samsung, MediaTek, and Tensor architectures
- Deliverable: Provenance Report across 5+ OEM hardware profiles

**Milestone 3 (Months 5–6): AHA Protocol & Scaling Prototype**
- Integrate Asynchronous Hardware Attestation with on-chain anchoring
- Deliverable: Functional end-to-end Hardware-to-Ledger demo

---

## ⚖️ License

The open-source components of this repository are licensed under the **Apache License 2.0**. The core NPCC Extraction Engine remains proprietary to VRYPTIC to ensure protocol integrity and safeguard hardware-level security standards.

---

*Architected by the VRYPTIC Strike Team — Nairobi, Kenya.*
*"Two founders. Four incidents. One infrastructure layer that makes truth undeniable from the first microsecond."*
