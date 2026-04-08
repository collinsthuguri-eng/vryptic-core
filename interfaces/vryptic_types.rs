// VRYPTIC Protocol: Universal Data Types
// This ensures hardware, ledger, and storage all speak the same language.

pub struct TruthSealEnvelope {
    /// Unique ID of the CMOS sensor unit
    pub unit_id: [u8; 32],
    
    /// The NPCC-generated provenance hash
    pub provenance_hash: [u8; 32],
    
    /// The Solana transaction signature (Proof of Ledger)
    pub ledger_signature: [u8; 64],
    
    /// The Arweave AO process ID (Proof of Storage)
    pub sentinel_node_id: [u8; 32],
}

/// Status of the seal as it moves through the pipeline
pub enum SealStatus {
    Captured,    // Hardware level
    Attested,    // SMS Seed Vault signed
    Anchored,    // Solana ledger confirmed
    Permanent,   // Arweave sentinel confirmed
}
