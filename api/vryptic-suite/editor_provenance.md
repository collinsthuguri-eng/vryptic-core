# VRYPTIC Editor
**Preserving Integrity through Transformation.**

The VRYPTIC editing layer maintains an unbroken chain of custody from the
moment of capture through to final export. Every transformation is
cryptographically attested. No edit can be concealed. No provenance can
be broken.

---

## Two Editing Surfaces

VRYPTIC ships editing capabilities across two distinct products targeting
different users and use cases.

### Light Editing Suite (In-App)

Embedded inside the VRYPTIC Camera App. Ships at launch. Always free.

Target users: citizen journalists, field contributors, enthusiasts.

Covers essential transformations — crop, colour correction, trim, filter —
with full provenance continuity. Every operation generates a TEE-signed
Derivative Attestation appended to the original Truth Seal. The source
capture is never modified.

### Vryptic Editing Suite (Desktop — Windows + macOS)

Standalone professional application. Ships post-launch.

- **V1** (Q3 Year 2 → Q1 Year 3): Professional photo editing with complete
  Derivative Attestation pipeline.
- **V2** (Q2 Year 3): Video editing with dual Arweave archival model.

Target users: professional journalists, legal professionals, newsrooms,
insurance investigators, NGOs.

Derivative Attestation runs on a dedicated CPU thread in parallel with the
GPU render pipeline (Metal on macOS, DirectX/CUDA on Windows). The
provenance record completes within the same window as the export render —
no additional wait time for the user.

Paid product. Individual subscriptions and seat-based enterprise licensing.
All fees feed the $VRYP burn-and-mint equilibrium.

---

## Non-Destructive Provenance Engine

The VRYPTIC editing layer operates on a non-destructive architecture. The
original Truth Seal is never overwritten. Every edit is appended as a
cryptographically signed event — a **Derivative Attestation** — linked
to the original Truth Record on the Solana ledger.

The full edit history is:
- **Human-readable:** A verifier can follow every transformation from lens
  to final export in plain language.
- **Cryptographically verifiable:** Each Derivative Attestation is TEE-signed.
  No step in the chain can be silently removed or altered.
- **Permanently archived:** The complete chain is pushed to Arweave at export,
  retrievable indefinitely via the Permaweb.

---

## Tiered Delta Recording

The delta between the raw capture and any edited state is recorded at a
resolution appropriate to the verification tier of the requesting party.

| Tier | Delta Recording Method | Use Case |
|---|---|---|
| **Standard** | Perceptual hash diff + processing metadata | Default. Lightweight. Consumer and enthusiast use. |
| **Verified Partner** | Perceptual hash diff + metadata + regional pixel sampling | SDK partners, platform integrations. |
| **Enterprise / Legal** | Full pixel-level diff on demand | Courtroom evidence, insurance audit, enterprise compliance. Cost passed to enterprise client. |

Standard tier is the default for all captures. Partners and enterprise clients
access higher tiers through the TaaS Gateway.

---

## Dual Arweave Archival (Video)

For video assets, VRYPTIC operates a two-stage permanent archival model:

- **TR#1 — Raw Archive:** The unedited raw file is archived to Arweave at
  capture. The file hash is anchored to the Solana ledger instantly.
- **TR#2 — Edited Archive:** The final export is archived to Arweave at the
  point of export. TR#2 is cryptographically linked to TR#1 via the
  Derivative Attestation chain.

Both copies are permanently retrievable. A downstream verifier — journalist,
court, platform — can retrieve the raw original and the published edit
simultaneously and verify every transformation between them on the ledger.

---

## Verifiable Exports

All exports from either editing surface carry a VRYPTIC provenance record
that links directly to the Solana and Arweave ledger entries. Any verifier
with access to the VRYPTIC Explorer or Social API can confirm:

- The original capture hardware and timestamp.
- The complete Derivative Attestation chain.
- The Tf score of the source Truth Seal.
- The delta between raw capture and final export at the appropriate tier.

---

*VRYPTIC Protocol — Nairobi, Kenya.*
*"Two founders. Four incidents. One infrastructure layer that makes truth undeniable from the first microsecond."*
