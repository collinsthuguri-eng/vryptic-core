# VRYPTIC Camera App
**The Source of Truth.**

The Camera App is the primary ingest point for the VRYPTIC Protocol. Unlike standard camera apps, it operates at the kernel level — bypassing OS image processing to extract raw sensor data directly from the CMOS hardware before any software can interfere.

### Key Features:
* **Hardware-Linked Capture:** Every frame is timestamped and hashed at the moment of photon-to-digital conversion. The NPCC engine extracts the unique PRNU silicon fingerprint from any standard CMOS sensor — no proprietary hardware required.
* **SMS Integration:** The app calls the Solana Mobile Stack (SMS) to sign the initial capture metadata inside the Seed Vault, anchoring the Truth Seal to the Solana ledger at the moment of capture.
* **Zero-Knowledge Provenance:** Generates a cryptographic proof that the image was captured by a verified physical sensor in a real gravitational environment — without revealing the user's identity or precise location.

### Supported Hardware:
The NPCC engine is hardware-agnostic by design. A Solana Seeker, a Google Pixel, a Samsung Galaxy, or any AOSP-compatible device with a standard CMOS sensor can participate in the VRYPTIC network. Sensor-specific calibration profiles are maintained across Sony IMX, Samsung ISOCELL, MediaTek, and Google Tensor chipset families.
