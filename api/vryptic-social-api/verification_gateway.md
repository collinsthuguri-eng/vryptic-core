# VRYPTIC Social Verification API
**Bridging the Gap between Hardware and Social Platforms.**

This API allows social media platforms (Centralized & Federated) to query the VRYPTIC Ledger for content authenticity.

### Key Endpoints:
* **GET /verify/{provenance_hash}:** Returns the full "Truth Seal" history, including hardware unit ID and Arweave storage anchor.
* **POST /check-integrity:** Compares a social media upload against the original CMOS-level hash to detect AI-generated deepfakes or unauthorized edits.
* **OEmbed Support:** Provides a "VRYPTIC Verified" badge for embedded content on web platforms.

### Integration Standards:
* Supports **C2PA** (Coalition for Content Provenance and Authenticity) metadata standards.
* Compatible with **ActivityPub** for decentralized social networks (Mastodon, Bluesky).
