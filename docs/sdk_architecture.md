# VRYPTIC SDK Architecture

This document specifies the Camera SDK and Integration API — the two external
interfaces through which third-party developers and platforms connect to the
VRYPTIC provenance layer.

---

## Dual API Stack

VRYPTIC exposes two distinct APIs serving fundamentally different integration
scenarios.

### Camera API / Camera SDK

Embeds the full VRYPTIC capture pipeline inside third-party Android camera
applications.

An SDK partner integrates the Camera SDK into their existing app. From that
point, every capture made through that app runs through the NPCC engine,
generates a Truth Seal, and produces a Truth Record on the Solana ledger —
all without the end user leaving the partner application.

This is the deeper integration. It extends VRYPTIC's hardware attestation
layer across the Android ecosystem without requiring users to switch to the
native VRYPTIC Camera App.

Target integrators: Android camera apps, journalism tools, field documentation
platforms, legal evidence capture software.

### Integration API

Allows platforms to verify existing content against the VRYPTIC ledger.

A platform submits a media asset and receives back a verification result:
whether a Truth Record exists, the Tf score, the original capture timestamp,
and the complete Derivative Attestation chain if edits were made. The platform
never needs to have been part of the original capture workflow.

This is the lighter integration. It enables retroactive verification — useful
for content moderation pipelines, newsroom workflows, and social platforms
performing origin checks on uploaded content.

Target integrators: social media platforms, newsrooms, insurance platforms,
court evidence management systems.

---

## Dual Attestation Framework (Camera SDK)

When the Camera SDK is active, every capture produces two Truth Records
generated on parallel threads within the same 2-second window.

### Truth Record #1 — Pre-Processing Attestation

Generated from the RAW frame buffer before any application pipeline runs.

- PRNU silicon fingerprint extracted directly from the CMOS sensor.
- Tf score computed by the NPCC engine inside the TEE.
- Truth Seal signed by the device hardware key via AOSP StrongBox and
  Solana Seed Vault.
- Anchored to the Solana ledger as TR#1.

This record captures the state of physical reality at the moment of capture,
before any software has touched the image.

### Truth Record #2 — Post-Processing Attestation

Generated after the partner app's processing pipeline completes.

- Records the delta between the raw sensor state (TR#1) and the processed
  output using Tiered Delta Recording.
- Cryptographically linked to TR#1.
- Anchored to the Solana ledger as TR#2.

This record documents exactly what the partner application did to the image
after capture — making the full transformation chain auditable.

### Tiered Delta Recording

The delta between TR#1 and TR#2 is recorded at a resolution appropriate to
the verification tier of the SDK partner.

| Tier | Method | Access |
|---|---|---|
| Standard | Perceptual hash diff + processing metadata | Default — all SDK integrations |
| Verified Partner | Hash diff + metadata + regional pixel sampling | SDK partners with Verified Partner badge |
| Enterprise / Legal | Full pixel-level diff on demand | Enterprise clients via TaaS Gateway |

Both Truth Records are completed within the same 2-second capture window.
The dual attestation process runs on parallel threads and imposes no
perceptible latency on the user experience.

---

## SDK Partner Programme

### Certification

VRYPTIC operates a hybrid certification model:

- **Automated:** An automated compliance test suite ships with the SDK.
  Applications that pass the suite receive production API keys immediately
  — no manual review required. This ensures indie developers can integrate
  and ship without gatekeeping delays.
- **Manual:** Enterprise partners seeking the **VRYPTIC Verified SDK Partner**
  badge undergo a manual certification review. The badge signals to end users
  that the integration has been audited for full pipeline integrity.

### Pricing Tiers

All fees are denominated in $VRYP and feed directly into the
burn-and-mint equilibrium mechanism.

| Tier | Volume | Cost | Target |
|---|---|---|---|
| **Free** | Up to 10,000 Truth Records/month | No fee | Indie developers, open source projects |
| **Pro** | Above 10,000 Truth Records/month | Volume-based $VRYP micro-fee per Truth Record | Commercial applications |
| **Enterprise** | Negotiated | Annual $VRYP commitment | Large platforms, newsrooms, legal integrations |

Enterprise tier includes the Verified SDK Partner badge and access to
Enterprise/Legal-tier delta recording.

### $VRYP Flow from SDK Fees

- 50% burned — reduces circulating $VRYP supply via BME mechanism.
- 30% to Protocol Treasury — funds ongoing development and security audits.
- 20% to Sentinel Rewards — distributed to active Sentinel Nodes.

---

## Platform Coverage

The Camera SDK targets the Android ecosystem exclusively.

The NPCC engine requires direct access to the RAW frame buffer before ISP
processing — a capability exposed by Android's Camera2 API. This level of
sensor access is not available on iOS via Apple's AVFoundation framework.
VRYPTIC will not issue Truth Records it cannot fully stand behind.

The Integration API is platform-agnostic. Any system capable of making HTTP
requests can query the VRYPTIC ledger for verification results, regardless
of the original capture platform.

---

*VRYPTIC Protocol — Nairobi, Kenya.*
*"We don't verify the pixels. We verify the physics."*
