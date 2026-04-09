// VRYPTIC Hardware Attestation Interface
// Phase 0: Architectural Draft for Grant Milestone 1

pub trait VrypticAttestor {
    /// Step 1: Requests raw entropy from the proprietary NPCC CMOS engine.
    /// This is a placeholder for the private hardware driver.
    fn get_npcc_correlation(&self) -> Result<[u8; 64], AttestationError>;

    /// Step 2: Signs the hardware attestation using the Solana Mobile Stack (SMS) Seed Vault.
    /// This ensures the 'Truth' never leaves the secure element.
    fn sign_with_seed_vault(&self, payload: &[u8]) -> Result<Signature, VaultError>;
}

#[derive(Debug)]
pub enum AttestationError {
    /// PRNU fingerprint extracted from sensor does not match the registered hardware profile.
    HardwareSensorMismatch,
    /// Neural Floor (Ψ) verification timeout — the deep learning forensic analysis layer
    /// failed to complete within the required window. Media is held pending retry.
    NeuralSyncTimeout,
    /// AHA Protocol active — device is operating in a zero-connectivity environment.
    /// Truth Seal is sealed locally in the TEE and will sync to the ledger on reconnection.
    ZeroConnectivityHold,
}
