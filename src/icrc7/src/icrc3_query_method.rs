use ic_cdk_macros::query;

use crate::icrc3_types::BlockType;
use crate::state::STATE;

// Returns all the supported block types.
#[query]
pub fn icrc3_supported_block_types() -> Vec<BlockType> {
    STATE.with(|s| s.borrow().archive_ledger_info.supported_blocks.clone())
}
