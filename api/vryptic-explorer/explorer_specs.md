# VRYPTIC Explorer: The Provenance Dashboard

The VRYPTIC Explorer is a unified data visualizer that aggregates on-chain evidence from the VRYPTIC Protocol.

### Key Visual Components:
* **The Provenance Timeline:** A step-by-step visual map showing:
    1. **Capture:** Sensor Unit ID & Timestamp.
    2. **Attestation:** Solana Transaction ID.
    3. **Persistence:** Arweave AO Process ID.
* **Integrity Status:** A real-time check against the original NPCC hash to confirm "No AI Alteration Detected."
* **Global Sentinel Map:** A decentralized map showing the distribution of active VRYPTIC Sentinels.

### Tech Stack:
* **Frontend:** Next.js / Tailwind CSS.
* **Indexing:** Custom Subgraph for Solana & AO-Connect for Arweave data fetching.
