# icrc7_launchpad

## Init

ICRC7 Init args:

```
pub struct InitArg {
    pub minting_account: Option<Account>,
    pub icrc7_symbol: String,
    pub icrc7_name: String,
    pub icrc7_description: Option<String>,
    pub icrc7_logo: Option<String>,
    pub icrc7_supply_cap: Option<u128>,
    pub icrc7_max_query_batch_size: Option<u16>,
    pub icrc7_max_update_batch_size: Option<u16>,
    pub icrc7_max_take_value: Option<u128>,
    pub icrc7_default_take_value: Option<u128>,
    pub icrc7_max_memo_size: Option<u32>,
    pub icrc7_atomic_batch_transfers: Option<bool>,
    pub tx_window: Option<u64>,
    pub permitted_drift: Option<u64>,
    pub approval_init: Option<InitApprovalsArg>,    // ICRC37 Init args
    pub archive_init: Option<InitArchiveArg>,       // ICRC3 Init args
}
```

ICRC37 Init args:

```
type InitApprovalsArg = record {
    max_approvals : opt nat16;
    max_approvals_per_token_or_collection : opt nat16;
    settle_to_approvals : opt nat16;
    max_revoke_approvals : opt nat16;
    collection_approval_requires_token : opt bool;
}
```

ICRC3 Init args:

```
type InitArchiveArg = record {
    maxRecordsToArchive : nat;                    //Max number of archive items to archive in one round
    archiveIndexType : IndexType;                 //Index type to use for the memory of the archive
    maxArchivePages : nat;                        //Max number of pages allowed on the archivserver
    settleToRecords : nat;                        //number of records to settle to during the clean up process
    archiveCycles : nat;                          //number of cycles to sent to a new archive canister;
    maxActiveRecords : nat;                       //allowed max active records on this canister
    maxRecordsInArchiveInstance : nat;            //specify the max number of archive items to put on an archive instance
    archiveControllers : opt opt vec principal;   //override the default controllers. The canister will always add itself to this group;
}
```

## ICIC7

### ICRC-7

ICRC-7 is the minimal standard for the implementation of Non-Fungible Tokens (NFTs) on the Internet Computer.

A token ledger implementation following this standard hosts an NFT collection (collection), which is a set of NFTs.

ICRC-7 does not handle approval-related operations such as approve and transfer_from itself. Those operations are specified by ICRC-37 which extends ICRC-7 with approval semantics.

[ICRC-7](https://github.com/dfinity/ICRC/blob/icrc_7_and_37/ICRCs/ICRC-7/ICRC-7.md)

### ICRC-37

[ICRC-37](https://github.com/dfinity/ICRC/blob/icrc_7_and_37/ICRCs/ICRC-37/ICRC-37.md)

### ICRC-3

[ICRC-3](https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-3/README.md)


### Scripts

#### Deploying Icrc7 Canister

```bash
dfx deploy icrc7 --argument '(record{                                  
minting_account= opt record {
    owner = principal "3yyxm-t5fpe-v32em-ac6lr-xyort-wuscb-dvl4x-3wnwi-hqkyj-xortw-oqe";                                     
    subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
  };                  
icrc7_supply_cap= null;
icrc7_description= opt "ICP Flower Collection";
tx_window= null;                        
permitted_drift= null;                  
icrc7_max_take_value= null;
icrc7_max_memo_size= null;
icrc7_symbol= "ICFL";
icrc7_max_update_batch_size= null;
icrc7_max_query_batch_size= null;
icrc7_atomic_batch_transfers= null;
icrc7_default_take_value= null;
icrc7_logo= null;
icrc7_name= "ICP Flower";
approval_init= null;
archive_init= null
})'
```

```bash
dfx deploy icrc7 --argument '(record{                                  
minting_account= opt record {
        owner = principal "3yyxm-t5fpe-v32em-ac6lr-xyort-wuscb-dvl4x-3wnwi-hqkyj-xortw-oqe";                                     
        subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
    };                  
icrc7_supply_cap= null;
icrc7_description= opt "ICP Flower Collection";
tx_window= null;                        
permitted_drift= null;                  
icrc7_max_take_value= null;
icrc7_max_memo_size= null;
icrc7_symbol= "ICFL";
icrc7_max_update_batch_size= null;
icrc7_max_query_batch_size= null;
icrc7_atomic_batch_transfers= null;
icrc7_default_take_value= null;
icrc7_logo= null;
icrc7_name= "ICP Flower";
approval_init= null;
archive_init= opt record {
        maxRecordsToArchive= 2;
        archiveIndexType= variant {Stable};
        maxArchivePages= 3;
        settleToRecords= 2;
        archiveCycles= 1000000000000;
        maxActiveRecords= 4;
        maxRecordsInArchiveInstance= 4;
        archiveControllers= null
    }
})'
```

#### Minting NFT

```bash
dfx canister call icrc7 icrc7_mint '(record{                                  
to= record {
    owner = principal "3yyxm-t5fpe-v32em-ac6lr-xyort-wuscb-dvl4x-3wnwi-hqkyj-xortw-oqe";                                     
    subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
  };          
token_id=1;
memo= null;
from_subaccount= null;                  
token_description= opt "Token Number 1";
token_logo= null;
token_name= null
})'
```