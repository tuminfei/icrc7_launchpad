use ic_cdk_macros::query;

use crate::icrc3_types::{BlockType, GetArchiveArgs, GetBlocksArgs, GetBlocksResult, Tip};
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
    STATE.with(|s| s.borrow().icrc3_get_tip_certificate())
}

// Get icrc3 blocks information
#[query]
pub fn icrc3_get_blocks(_arg: GetBlocksArgs) -> GetBlocksResult {
    return GetBlocksResult {
        blocks: vec![],
        log_length: 0,
        archived_blocks: vec![],
    };
}

// Returns the latest hash and lastest index along with a witness
#[query]
pub fn get_tip() -> Tip {
    STATE.with(|s| s.borrow().icrc3_get_tip())
}
