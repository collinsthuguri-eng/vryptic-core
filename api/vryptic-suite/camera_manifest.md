# VRYPTIC Camera App
**The Source of Truth.**

The Camera App is the primary ingest point for the VRYPTIC Protocol. Unlike standard camera apps, it operates at the kernel level to interface with NPCC-enabled sensors.

### Key Features:
* **Hardware-Linked Capture:** Every frame is timestamped and hashed at the moment of photon-to-digital conversion.
* **SMS Integration:** The app calls the Solana Mobile Stack (SMS) to sign the initial capture metadata.
* **Zero-Knowledge Proofs:** Generates a proof that the image was captured by a VRYPTIC-certified sensor without revealing the user's private biometric data.
