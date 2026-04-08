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
    HardwareSensorMismatch,
    NeuralSyncTimeout,
    ZeroConnectivityHold,
}
