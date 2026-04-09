// VRYPTIC Hardware Oracle
// Purpose: Managing the Global Hardware Registry and Unit Revocation.

pub fn verify_unit_status(ctx: Context<CheckUnit>, unit_id: [u8; 32]) -> Result<()> {
    let registry = &ctx.accounts.hardware_registry;
    
    // Check if the hardware unit has been flagged as compromised and revoked
    if registry.is_revoked(unit_id) {
        return Err(ErrorCode::HardwareCompromised.into());
    }
    
    msg!("VRYPTIC Oracle: Unit ID verified and active.");
    Ok(())
}

#[account]
pub struct HardwareRegistry {
    pub admin: Pubkey,        // VRYPTIC Core Governance
    pub revoked_units: Vec<[u8; 32]>,  // Hardware units flagged as compromised and revoked
}
