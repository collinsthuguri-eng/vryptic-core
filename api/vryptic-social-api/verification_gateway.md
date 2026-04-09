# VRYPTIC Social Verification API
**Bridging the Gap between Hardware and Social Platforms.**

This API allows social media platforms — both centralized and federated — to query the VRYPTIC Ledger for content authenticity, enabling hardware-rooted verified badges on user-generated content at the platform level.

### Key Endpoints:
* **GET /verify/{provenance_hash}:** Returns the full Truth Seal history, including hardware unit ID, Arweave storage anchor, and edit manifest.
* **POST /check-integrity:** Compares a social media upload against the original CMOS-level hash to detect AI-generated deepfakes or unauthorized edits.
* **OEmbed Support:** Provides a "VRYPTIC Verified" badge for embedded content on web platforms — a visual Blue Checkmark for physical reality.

### Integration Standards:
* **C2PA Compliant:** All VRYPTIC Truth Seals are injected with C2PA-standard metadata, ensuring provenance is automatically recognized by Google, X, and Adobe's content authenticity tooling.
* **ActivityPub (Roadmap):** Planned integration with the ActivityPub protocol to extend VRYPTIC verification to federated social networks (Mastodon, Pixelfed). This allows decentralized platforms to display hardware-verified badges natively — without routing through a centralized verification authority.

### Target Platforms:
| Platform | Type | Integration Path |
|---|---|---|
| X (Twitter) | Centralized | Integration API — per-call verification fee |
| Instagram | Centralized | Integration API — per-call verification fee |
| Mastodon | Federated (ActivityPub) | Roadmap — Phase II SDK Beta |
| Pixelfed | Federated (ActivityPub) | Roadmap — Phase II SDK Beta |
