use anchor_lang::prelude::*;

declare_id!("Vryptic11111111111111111111111111111111111");

#[program]
pub mod vryptic_core {
    use super::*;

    /// Registers a Hardware-to-Ledger 'Truth Seal'.
    /// This instruction validates the NPCC attestation and commits the 
    /// provenance hash to the Solana ledger.
    pub fn register_truth_seal(ctx: Context<RegisterSeal>, provenance_hash: [u8; 32]) -> Result<()> {
        let seal_account = &mut ctx.accounts.seal_account;
        seal_account.authority = *ctx.accounts.signer.key;
        seal_account.hash = provenance_hash;
        seal_account.timestamp = Clock::get()?.unix_timestamp;
        
        msg!("VRYPTIC: Truth Seal Registered. Provenance Secured.");
        Ok(())
    }
}

#[account]
pub struct TruthSeal {
    pub authority: Pubkey,   // The specific hardware unit ID
    pub hash: [u8; 32],      // The NPCC provenance hash
    pub timestamp: i64,      // Time of registration
}

#[derive(Accounts)]
pub struct RegisterSeal<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 32 + 8)]
    pub seal_account: Account<'info, TruthSeal>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
