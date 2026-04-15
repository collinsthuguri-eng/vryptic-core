# VRYPTIC Speed Architecture

Field deployment is not a controlled environment. Network connectivity is
unreliable. Devices range from flagship to entry-level. Events move faster
than infrastructure. This document specifies how VRYPTIC performs under
real-world conditions — not ideal ones.

---

## Capture to Truth Record: Under 2 Seconds

The full pipeline from shutter press to confirmed Truth Record on the Solana
ledger completes in under 2 seconds via Optimistic Confirmation.

```
⚡ Sealing      — NPCC engine computes Tf, Truth Seal generated inside TEE
🔄 Anchoring    — Truth Seal submitted to Solana ledger
✅ Verified     — Optimistic Confirmation received, Truth Record live
```

The user never waits for full blockchain finality. Optimistic Confirmation
provides immediate on-chain acknowledgement. The ledger finalises in the
background while the user continues capturing.

---

## NPCC on Mid-Range Devices: Under 500ms

The NPCC engine is designed for the devices that matter in the markets where
verified media is most urgently needed — not flagship hardware.

On Tecno, Infinix, and equivalent mid-range AOSP devices, the NPCC
computation completes in under 500ms via:

- **Lightweight NPCC Profile:** A calibrated low-overhead variant of the
  NPCC algorithm optimised for constrained hardware without compromising
  Truth Floor scoring accuracy.
- **Rust/C Acceleration:** The NPCC engine is implemented in Rust and C,
  compiled via Android NDK for direct hardware-layer execution. No JVM
  overhead in the critical path.
- **Chipset-Aware Calibration:** Sensor-specific calibration profiles are
  maintained across the three dominant chipset families in the target market:
  - MediaTek Helio (Tecno, Infinix, OPPO)
  - Qualcomm Snapdragon (Samsung, Motorola, OnePlus)
  - Google Tensor (Pixel devices)

Calibration profiles ensure PRNU extraction accuracy is consistent across
sensor manufacturers — Sony IMX, Samsung ISOCELL, OmniVision — regardless
of chipset.

---

## Dual Attestation: Parallel Threads, Same Window

When the Camera SDK is active, two Truth Records are produced per capture —
TR#1 (pre-processing) and TR#2 (post-processing). Both run on parallel
threads and complete within the same 2-second window.

Dual attestation adds no perceptible latency. The parallel thread architecture
ensures the NPCC computation and the partner app's processing pipeline execute
concurrently, not sequentially.

---

## AHA Offline Sync: Disciplined Batching

In zero-connectivity environments, the AHA (Asynchronous Hardware Attestation)
Protocol seals Truth Records locally inside the TEE and queues them for
ledger sync the moment connectivity is restored.

Sync discipline prevents network flooding on reconnection:

- **Batch size:** 10 Truth Records per sync cycle.
- **Pause between cycles:** 30 seconds.
- **Execution mode:** Background only — via Android WorkManager. Never
  interrupts active capture or foreground use.
- **Order preservation:** Chronological capture sequence is maintained
  across all batches. The ledger receives records in the order they
  were captured, regardless of sync timing.
- **Video upload:** WiFi-first. Large video files queue for WiFi
  availability before upload to Arweave.

---

## Mass Event Load: Scalable by Architecture

High-volume scenarios — protests, elections, sporting events, breaking news —
generate simultaneous Truth Record submissions from large numbers of mobile
contributors in a concentrated geographic area.

VRYPTIC handles this through layered scaling:

- **Solana State Compression:** Concurrent Merkle Tree batching absorbs
  high-volume submission bursts at ~$0.0001 per Truth Record regardless
  of load.
- **Sentinel Node Aggregation:** PC-based Sentinel Nodes aggregate and
  batch Truth Records from mobile contributors before ledger submission,
  reducing per-device Solana interaction load during peak events.
- **Enterprise Priority Fee Mechanism:** Enterprise clients posting
  Reality Bounties during high-demand events can attach priority fees
  to ensure their capture requests are processed ahead of standard queue
  traffic.

---

## Editing Suite Export: Zero Added Wait

The Vryptic Editing Suite (desktop) generates Derivative Attestations
at export without adding to the user's wait time.

Attestation generation runs on a dedicated CPU thread in parallel with
the GPU render pipeline:

- **macOS:** Metal API handles GPU render. CPU thread handles attestation
  in parallel.
- **Windows:** DirectX or CUDA handles GPU render. CPU thread handles
  attestation in parallel.

From the user's perspective, the export completes in the same time it would
without provenance. The chain of custody is built while the render runs.

---

## Summary

| Operation | Target | Method |
|---|---|---|
| Capture → Truth Record | < 2 seconds | Optimistic Confirmation |
| NPCC on mid-range devices | < 500ms | Lightweight profile, Rust/C NDK, chipset calibration |
| Dual attestation (TR#1 + TR#2) | Same 2-second window | Parallel thread execution |
| AHA offline sync | Non-disruptive | 10 records/cycle, 30s pause, background WorkManager |
| Mass event load | Linearly scalable | State Compression + Sentinel aggregation + priority fees |
| Editing Suite export | No added wait | Parallel CPU/GPU thread execution |

---

*VRYPTIC Protocol — Nairobi, Kenya.*
*"Two founders. Four incidents. One infrastructure layer that makes truth undeniable from the first microsecond."*
