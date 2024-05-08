use candid::{CandidType, Deserialize, Encode, Principal};
use ic_cdk::api::management_canister::{
    main::{create_canister, install_code, CreateCanisterArgument, InstallCodeArgument},
    provisional::CanisterSettings,
};
use serde::Serialize;

use crate::icrc3_types::{ArchiveCreateArgs, IndexType};

pub const ARCHIVE_WASM: &[u8] = std::include_bytes!("./../../archive/archive.wasm.gz");

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ArchiveInitArgs {
    #[serde(rename = "firstIndex")]
    pub first_index: u128,
    #[serde(rename = "indexType")]
    pub index_type: IndexType,
    #[serde(rename = "maxPages")]
    pub max_pages: u128,
    #[serde(rename = "maxRecords")]
    pub max_records: u128,
}

impl ArchiveInitArgs {
    fn new(max_pages: u128, max_records: u128) -> Self {
        Self {
            first_index: 0,
            index_type: IndexType::Stable,
            max_pages,
            max_records,
        }
    }
}

#[allow(unused)]
async fn create_archive_canister(arg: ArchiveCreateArgs) -> Result<Principal, String> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err("Anonymous Caller".into());
    }

    let principal = match create_canister(
        CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(vec![ic_cdk::id(), caller.clone()]),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
                reserved_cycles_limit: None,
            }),
        },
        10_000_000_000_000,
    )
    .await
    {
        Err((code, msg)) => return Err(format!("Rejection Code: {:?}, Message: {:?}", code, msg)),
        Ok((principal,)) => principal.canister_id,
    };
    let init_arg = ArchiveInitArgs::new(arg.max_pages, arg.max_records);
    let init_arg = Encode!(&init_arg).unwrap();
    match install_code(InstallCodeArgument {
        mode: ic_cdk::api::management_canister::main::CanisterInstallMode::Install,
        canister_id: principal,
        wasm_module: ARCHIVE_WASM.to_vec(),
        arg: init_arg,
    })
    .await
    {
        Ok(()) => Ok(principal),
        Err((code, msg)) => Err(format!("Code: {:?}, Message: {:?}", code, msg)),
    }
}
