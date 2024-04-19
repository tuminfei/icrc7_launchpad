use std::collections::BTreeMap;

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use icrc_ledger_types::{
    icrc::generic_value::Map,
    icrc1::account::{Account, Subaccount},
};
use serde::{Deserialize, Serialize};

use crate::errors::{
    ApproveCollectionError, ApproveTokenError, RevokeCollectionApprovalError,
    RevokeTokenApprovalError,
};

pub type Metadata = Map;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserAccount(Account);

impl Storable for UserAccount {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl UserAccount {
    pub fn new(user: Account) -> Self {
        UserAccount(user)
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct LedgerInfo {
    pub max_approvals_per_token_or_collection: u16,
    pub max_revoke_approvals: u16,
    pub max_approvals: u16,
    pub settle_to_approvals: u16,
    pub collection_approval_requires_token: bool,
}

impl Default for LedgerInfo {
    fn default() -> Self {
        Self {
            max_approvals_per_token_or_collection: 10000,
            max_revoke_approvals: 10000,
            max_approvals: crate::state::State::DEFAULT_MAX_UPDATE_BATCH_SIZE,
            settle_to_approvals: 9975,
            collection_approval_requires_token: true,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct ApprovalInfo {
    pub from_subaccount: Option<Subaccount>,
    pub spender: Account, // Approval is given to an ICRC Account
    pub memo: Option<Vec<u8>>,
    pub expires_at: Option<u64>,
    pub created_at_time: Option<u64>,
}

impl Storable for ApprovalInfo {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl ApprovalInfo {
    pub fn new(
        from_subaccount: Option<Subaccount>,
        spender: Account,
        memo: Option<Vec<u8>>,
        expires_at: Option<u64>,
        created_at_time: Option<u64>,
    ) -> Self {
        Self {
            from_subaccount,
            spender,
            memo,
            expires_at,
            created_at_time,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct TokenApprovalInfo(BTreeMap<Account, Vec<ApprovalInfo>>);

impl Storable for TokenApprovalInfo {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl TokenApprovalInfo {
    pub fn new(owner: Account, approval: ApprovalInfo) -> Self {
        let mut token_approval = BTreeMap::new();
        token_approval.insert(owner, vec![approval]);
        TokenApprovalInfo(token_approval)
    }

    pub fn approve(&mut self, owner: Account, approval: ApprovalInfo) {
        match self.0.get_mut(&owner) {
            None => {
                self.0.insert(owner, vec![approval]);
            }
            Some(approvals) => {
                approvals.push(approval);
            }
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct CollectionApprovalInfo(BTreeMap<Account, ApprovalInfo>);

impl Storable for CollectionApprovalInfo {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl CollectionApprovalInfo {
    pub fn new(spender: Account, approval: ApprovalInfo) -> Self {
        let mut collection_approval = BTreeMap::new();
        collection_approval.insert(spender, approval);
        CollectionApprovalInfo(collection_approval)
    }

    pub fn approve(&mut self, spender: Account, approval: ApprovalInfo) {
        self.0.insert(spender, approval);
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CollectionApprovalAccount {
    pub owner: Account,
    pub spender: Account,
}

impl Storable for CollectionApprovalAccount {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize)]
pub struct InitApprovalsArg {
    pub deployer: Account,
    pub max_approvals: u16,
    pub max_approvals_per_token_or_collection: Option<u16>,
    pub max_revoke_approvals: Option<u16>,
    pub settle_to_approvals: Option<u16>,
    pub collection_approval_requires_token: Option<bool>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct ApproveTokenArg {
    pub token_id: u128,
    pub approval_info: ApprovalInfo,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct TokenApproval {
    token_id: u128,
    approval_info: ApprovalInfo,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct ApproveCollectionArg {
    pub approval_info: ApprovalInfo,
}

pub type ApproveTokenResult = Result<u128, ApproveTokenError>;

pub type ApproveCollectionResult = Result<u128, ApproveCollectionError>;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct RevokeTokenApprovalArg {
    token_id: u128,
    from_subaccount: Option<Subaccount>,
    spender: Option<Account>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

pub type RevokeTokenApprovalResult = Result<u128, RevokeTokenApprovalError>;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct RevokeCollectionApprovalArg {
    from_subaccount: Option<Subaccount>,
    spender: Option<Account>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

pub type RevokeCollectionApprovalResult = Result<u128, RevokeCollectionApprovalError>;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct IsApprovedArg {
    spender: Account,
    from_subaccount: Option<Subaccount>,
    token_id: u128,
}
