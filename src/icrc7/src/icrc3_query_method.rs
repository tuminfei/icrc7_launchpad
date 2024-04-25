use ic_cdk_macros::query;

use crate::icrc3_types::{BlockType, GetArchiveArgs};
use crate::state::STATE;
use icrc_ledger_types::icrc3::{archive::ArchiveInfo, blocks::DataCertificate};

// Returns all the supported block types.
#[query]
pub fn icrc3_supported_block_types() -> Vec<BlockType> {
    STATE.with(|s| s.borrow().archive_ledger_info.supported_blocks.clone())
}

// Listing all the canisters containing its blocks
#[query]
pub fn icrc3_get_archives(_arg: GetArchiveArgs) -> Vec<ArchiveInfo> {
    vec![]
}

// The Ledger MUST certify the last block (tip) recorded
#[query]
pub fn icrc3_get_tip_certificate() -> Option<DataCertificate> {
    // Only the Ledger certifies the tip of the chain.
    None
}
