use candid::Nat;
use ic_cdk_macros::query;

use crate::{icrc37_types::Metadata, state::STATE};

// Returns the approval-related metadata of the ledger implementation.
#[query]
pub fn icrc37_metadata() -> Metadata {
    STATE.with(|s| s.borrow().icrc37_metadata())
}

// Returns the maximum number of approvals this ledger implementation allows to be active per token or per principal for the collection.
#[query]
pub fn icrc37_max_approvals_per_token_or_collection() -> Option<Nat> {
    STATE.with(|s| {
        Some(Nat::from(
            s.borrow()
                .approval_ledger_info
                .max_approvals_per_token_or_collection,
        ))
    })
}

// Returns the maximum number of approvals that may be revoked in a single invocation of `icrc37_revoke_token_approvals` or `icrc37_revoke_collection_approvals`.
#[query]
pub fn icrc37_max_revoke_approvals() -> Option<Nat> {
    STATE.with(|s| {
        Some(Nat::from(
            s.borrow().approval_ledger_info.max_revoke_approvals,
        ))
    })
}
