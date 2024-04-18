use std::collections::HashMap;

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use icrc_ledger_types::{
    icrc::generic_metadata_value::MetadataValue,
    icrc1::account::{Account, Subaccount},
};
use serde::{Deserialize, Serialize};

use crate::errors::{BurnError, InsertTransactionError, MintError, TransferError};

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum TransactionType {
    Mint {
        tid: u128,
        from: Account,
        to: Account,
    },
    Burn {
        tid: u128,
        from: Account,
        to: Account,
    },
    Transfer {
        tid: u128,
        from: Account,
        to: Account,
    },
    Approval {
        tid: u128,
        from: Account,
        to: Account,
    },
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub at: u64,
    pub txn_id: u128,
    pub op: String,
    pub txn_type: TransactionType,
    pub memo: Option<Vec<u8>>,
}

impl Storable for Transaction {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Transaction {
    pub fn new(txn_id: u128, txn_type: TransactionType, at: u64, memo: Option<Vec<u8>>) -> Self {
        let op = match &txn_type {
            TransactionType::Transfer {
                tid: _,
                from: _,
                to: _,
            } => "transfer".into(),
            TransactionType::Mint {
                tid: _,
                from: _,
                to: _,
            } => "mint".into(),
            TransactionType::Burn {
                tid: _,
                from: _,
                to: _,
            } => "burn".into(),
            TransactionType::Approval {
                tid: _,
                from: _,
                to: _,
            } => "approve".into(),
        };
        Self {
            op,
            txn_id,
            at,
            txn_type,
            memo,
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArg {
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    pub token_id: u128,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

pub type TransferResult = Result<u128, TransferError>;

pub type Icrc7TokenMetadata = HashMap<String, MetadataValue>;

#[derive(CandidType, Deserialize, Clone)]
pub struct MintArg {
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    pub token_id: u128,
    pub memo: Option<Vec<u8>>,
    // if None, then the combination of Collection's symbol and token's id will be provided
    // for e.g.: "ICRC7 100"
    pub token_name: Option<String>,
    pub token_description: Option<String>,
    pub token_logo: Option<String>,
}

pub type MintResult = Result<u128, MintError>;

#[derive(CandidType, Deserialize, Clone)]
pub struct BurnArg {
    pub from_subaccount: Option<Subaccount>,
    pub token_id: u128,
    pub memo: Option<Vec<u8>>,
}

pub type BurnResult = Result<u128, BurnError>;

#[derive(CandidType, Deserialize)]
pub struct InitArg {
    pub minting_account: Option<Account>,
    pub icrc7_symbol: String,
    pub icrc7_name: String,
    pub icrc7_description: Option<String>,
    pub icrc7_logo: Option<String>,
    pub icrc7_supply_cap: Option<u128>,
    pub icrc7_max_query_batch_size: Option<u128>,
    pub icrc7_max_update_batch_size: Option<u128>,
    pub icrc7_max_take_value: Option<u128>,
    pub icrc7_default_take_value: Option<u128>,
    pub icrc7_max_memo_size: Option<u128>,
    pub icrc7_atomic_batch_transfers: Option<bool>,
    pub tx_window: Option<u64>,
    pub permitted_drift: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct InitApprovalsArg {
    pub deployer: Account,
    pub max_approvals: u128,
    pub max_approvals_per_token_or_collection: Option<u128>,
    pub max_revoke_approvals: Option<u128>,
    pub settle_to_approvals: Option<u128>,
    pub collection_approval_requires_token: Option<bool>,
}

#[derive(CandidType)]
pub struct Standard {
    pub name: String,
    pub url: String,
}

#[derive(CandidType, Deserialize)]
pub struct ApprovalArg {
    pub from_subaccount: Option<Subaccount>,
    pub spender: Account,
    pub token_id: u128,
    pub expires_at: Option<u64>,
    pub memo: Option<Vec<u8>>,
}

pub type SyncReceipt = Result<u32, InsertTransactionError>;
