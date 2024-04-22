use ic_cdk_macros::query;

use crate::icrc3_types::BlockType;

// Returns the approval-related metadata of the ledger implementation.
#[query]
pub fn icrc3_supported_block_types() -> Vec<BlockType> {
    vec![
        BlockType {
            block_type: "7mint".into(),
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-7/ICRC-7.md".into(),
        },
        BlockType {
            block_type: "7burn".into(),
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-7/ICRC-7.md".into(),
        },
        BlockType {
            block_type: "7xfer".into(),
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-7/ICRC-7.md".into(),
        },
        BlockType {
            block_type: "7update".into(),
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-7/ICRC-7.md".into(),
        },
        BlockType {
            block_type: "37appr".into(),
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-37/ICRC-37.md".into(),
        },
        BlockType {
            block_type: "37appr_coll".into(),
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-37/ICRC-37.md".into(),
        },
        BlockType {
            block_type: "37revoke".into(),
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-37/ICRC-37.md".into(),
        },
        BlockType {
            block_type: "37revoke_coll".into(),
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-37/ICRC-37.md".into(),
        },
        BlockType {
            block_type: "37xfer".into(),
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-37/ICRC-37.md".into(),
        },
    ]
}
