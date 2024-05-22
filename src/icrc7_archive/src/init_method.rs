use candid::candid_method;
use ic_cdk_macros::init;

use crate::{
    state::{State, STATE, DEFAULT_MAX_TRANSACTIONS_PER_GET_TRANSACTION_RESPONSE},
    types::ArchiveInitArgs,
};

#[init]
#[candid_method(init)]
pub fn init(arg: ArchiveInitArgs) {
    let ledger_id = ic_cdk::caller();
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        let state = State {
            max_pages: arg.max_pages,
            max_records: arg.max_records,
            block_index_offset: arg.first_index,
            block_index: arg.first_index,
            max_transactions_per_response: DEFAULT_MAX_TRANSACTIONS_PER_GET_TRANSACTION_RESPONSE,
            index_type: arg.index_type,
            ledger_id,
        };
        *s = state;
    });
}
