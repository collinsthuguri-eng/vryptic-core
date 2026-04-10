# VRYPTIC Protocol: Technical Positioning

A technical comparison of the VRYPTIC Hardware-to-Ledger provenance standard
against existing media verification approaches.

---

## The Core Problem

Every existing media verification solution operates at or above the OS layer.
This is the fundamental vulnerability — by the time any software-based solution
touches a media file, the opportunity for manipulation has already passed.

VRYPTIC is the only protocol that operates **below the OS**, at the silicon level,
at the moment of capture.

---

## Technical Comparison

### Numbers Protocol

**Approach:** Blockchain-based media registration. Captures metadata and
registers a hash on-chain at the OS layer.

**Gap:** OS-layer capture means the image has already passed through the
device's image signal processor (ISP). Any manipulation occurring before
registration — including AI-generated content injected at the driver level —
is invisible to the system.

**VRYPTIC difference:** NPCC extraction occurs before the OS image processing
pipeline. The RAW frame buffer is intercepted directly from the CMOS sensor
inside the TEE. The physics of the sensor are verified, not the pixels.

---

### C2PA (Coalition for Content Provenance and Authenticity)

**Approach:** An industry standard for attaching signed metadata to media
files. Adopted by Adobe, Microsoft, Google, and others.

**Gap:** C2PA is a labelling standard, not a provenance standard. It attests
that a file was signed by a particular software tool — not that the underlying
capture was physically real. C2PA credentials can be attached to AI-generated
content. The standard is also OS-dependent: it relies on software signing,
which can be spoofed at the application layer.

**VRYPTIC difference:** VRYPTIC is C2PA-compliant — Truth Seals are structured
to map to C2PA manifests. However, VRYPTIC operates as the silicon-rooted
layer beneath C2PA. Where C2PA verifies the software signature, VRYPTIC
verifies the hardware origin. The two are complementary, not competing.
VRYPTIC provides the physical ground truth that C2PA cannot.

---

### Truepic

**Approach:** Hardware-rooted image capture using device TEEs. Produces
cryptographically signed images with verified timestamps.

**Gap:** Truepic is a centralised service. Verification depends on Truepic's
infrastructure remaining operational, trusted, and accessible. There is no
decentralised network, no token economy, and no node operator layer. A single
point of trust is a single point of failure.

**VRYPTIC difference:** VRYPTIC operates a decentralised two-layer node
network — PC-based Sentinel Nodes for permanent validation and mobile
contributors for event-driven capture. No single entity controls the
verification layer. The protocol is trustless by architecture, not by
policy.

---

### Very.org

**Approach:** Biometric identity verification using palm-print scanning
combined with media capture. Ties verified media to a confirmed human identity.

**Gap:** Identity-bound verification creates a fundamental privacy problem.
Whistleblowers, journalists in hostile environments, and citizen witnesses
cannot use identity-linked systems without placing themselves at risk.
Requiring biometric enrollment also creates significant barriers to adoption
in the markets where verified media is most urgently needed.

**VRYPTIC difference:** VRYPTIC is pseudonymous by design. The protocol
verifies the **device**, not the person. Zero-knowledge provenance means a
journalist in Nairobi, a witness in a conflict zone, or a citizen documenting
state violence can produce cryptographically verified media without revealing
their identity. Truth does not require sacrifice of anonymity.

---

## Summary Table

| Capability | Numbers | C2PA | Truepic | Very.org | **VRYPTIC** |
|---|---|---|---|---|---|
| Below-OS capture | ❌ | ❌ | Partial | ❌ | ✅ |
| Silicon fingerprint (PRNU) | ❌ | ❌ | ❌ | ❌ | ✅ |
| Decentralised node network | ❌ | ❌ | ❌ | ❌ | ✅ |
| Offline attestation (AHA) | ❌ | ❌ | ❌ | ❌ | ✅ |
| Pseudonymous / zero-knowledge | ✅ | ✅ | ✅ | ❌ | ✅ |
| C2PA compatible | ❌ | ✅ | Partial | ❌ | ✅ |
| Provenance-preserving editor | ❌ | ❌ | ❌ | ❌ | ✅ |
| Permanent storage layer | ❌ | ❌ | ❌ | ❌ | ✅ |

---

## The Moat

No existing solution combines:

1. Silicon-level capture (below OS, inside TEE)
2. Decentralised validation network
3. Offline truth sealing with ledger sync
4. Pseudonymous provenance
5. Non-destructive provenance-preserving editing
6. Dual-layer permanent storage (Solana + Arweave)

VRYPTIC is not a better version of an existing tool. It is a different
layer of the stack entirely — the hardware truth layer that every
upstream verification standard depends on but none provides.

---

*VRYPTIC Protocol — Nairobi, Kenya.*
*"We don't verify the pixels. We verify the physics."*
