// VRYPTIC Explorer Aggregator
// Purpose: Merging Solana Ledger data and Arweave Storage data into a single view.

pub struct ProvenanceReport {
    pub truth_seal: [u8; 32],
    pub solana_block_height: u64,
    pub arweave_data_root: String,
    pub sensor_health_status: bool,
}

pub trait IExplorerAggregator {
    /// Fetches the "Complete Truth" by querying multiple chains.
    fn get_full_provenance_chain(&self, seal_id: [u8; 32]) -> Result<ProvenanceReport, ExplorerError>;
}
