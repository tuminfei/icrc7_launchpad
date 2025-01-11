# icrc7_launchpad

**ICRC7 Launchpad** is a powerful tool designed to streamline the creation of ICRC7 NFT canisters. With the **ICRC7 Launchpad**, users can easily deploy ICRC7-based NFT canisters tailored to their needs by simply modifying the `icrc7_launchpad.sh` script.

## Key Components

1. **ICRC7 Launchpad**:  
   A canister-based launcher that simplifies the deployment of ICRC7 NFT canisters. It abstracts the complexities of configuration and setup, enabling users to focus on their projects.

2. **ICRC7**:  
   The core ICRC canister contract code. This serves as the foundation for ICRC7-based NFTs, providing essential functionality for token operations.

3. **ICRC7 Types**:  
   A dedicated type library for ICRC7. This library defines the structures and data types required to interact with ICRC7 canisters, ensuring consistency and ease of integration.

## Use Cases

- Launch new NFT collections with ease.
- Experiment with ICRC7 token features and functionalities.
- Provide an infrastructure for decentralized applications leveraging NFTs.

## How It Works

1. Modify the `icrc7_launchpad.sh` script to include your desired configuration, such as token name, symbol, metadata, and supply parameters.
2. Execute the script to deploy a new ICRC7 canister on the Internet Computer.
3. Interact with the newly created NFT canister using the ICRC7 Types library for seamless integration into your application.

---

Get started with the **ICRC7 Launchpad** and unlock the potential of ICRC7 NFTs on the Internet Computer!


```

dfx start --background

dfx deploy icrc7_launchpad

sh icrc7_launchpad.sh

```

```bash

#!/bin/bash

ICRC7_LAUNCHPAD_CANISTER_ID="icrc7_launchpad"

# Arguments for the `mint_collection_canister` method
ARG=$(cat <<EOF
(
  record {
    icrc7_supply_cap = opt 1000000;
    icrc7_description = opt "An example collection description";
    tx_window = opt 86400;
    icrc7_max_query_batch_size = opt 500;
    ...
    ...
  }
)
EOF
)

# Call the canister method
dfx canister call "$ICRC7_LAUNCHPAD_CANISTER_ID" mint_collection_canister "$ARG"

```

## Creating Asset Storage and Uploading Assets

The **ICRC7 Launchpad** also supports asset storage for uploading and managing images or files associated with your NFTs. 

Follow the steps below to create asset storage and upload your assets:

### 1. Deploy the Asset Storage Canister
Run the following command to create an asset storage canister:  
```bash
dfx deploy ic_canister_assets
```

### 2. Organize Local Assets

Place all your local assets (e.g., images, files) into the `assets` directory in your project.

Supports directory synchronization, which ensures that the entire assets directory, including all subdirectories and files, is uploaded and synchronized to the canister.

This process maintains the directory structure, allowing seamless access to files and directories stored in the canister.

、、、

assets/
└── nft/
    ├── nft1.png
    ├── nft2.png
└── nft_image.jpg

、、、

### 3. Set Upload Parameters
Modify the relevant parameters in `tests/src/lib.rs` to specify details for the upload process, such as:
- Path to the `assets` directory.
- Metadata or configurations related to the assets.

### 4. Execute the Upload Test
Run the following command to upload the assets to the canister:
```bash
cargo test --package tests --lib -- --show-output
```

### 5. Retrieve Asset URLs
After the upload is complete, the script will output the paths of the uploaded assets. Use these paths to link the uploaded images or files to your NFTs.

```
example: nft_image.jpg
         /nft/nft1.jpg
         /nft/nft2.jpg

local network: http://{ic_canister_assets_canister_id}.raw.localhost:4943/nft_image.jpg
               http://{ic_canister_assets_canister_id}.raw.localhost:4943/nft/nft1.jpg
               http://{ic_canister_assets_canister_id}.raw.localhost:4943/nft/nft2.jpg

ic network: https://{ic_canister_assets_canister_id}.raw.icp0.io/nft_image.jpg
            https://{ic_canister_assets_canister_id}.raw.icp0.io/nft/nft1.jpg
            https://{ic_canister_assets_canister_id}.raw.icp0.io/nft/nft2.jpg  

```

## ICRC7 Init

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
icrc7_max_take_value= opt 100;
icrc7_max_memo_size= opt 1000;
icrc7_symbol= "ICFL";
icrc7_max_update_batch_size= opt 100;
icrc7_max_query_batch_size= opt 100;
icrc7_atomic_batch_transfers= null;
icrc7_default_take_value= opt 100;
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
dfx canister call icrc7 mint '(record{                                  
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


#### Transfer NFT

```bash
dfx canister call icrc7 icrc7_transfer '(vec{
  record{
    to=record {
      owner = principal "t4egw-clf4w-qbpli-svryg-7yqq6-jt2yj-7v755-mabir-zmx6i-vp4fr-fqe";
      subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
    };
    token_id= 1;
    from_subaccount= null;
    memo= null;
    created_at_time= null
  }
})'
```

#### Approve NFT

```bash
dfx canister call icrc7 icrc37_approve_tokens '(vec{ 
  record{
    token_id= 2;                               
    approval_info= record {
      memo= null;
      from_subaccount= null;
      created_at_time= null;
      expires_at= null;
      spender= record {
          owner = principal "o2zom-piy75-ifbnk-nhhlq-362su-4vsx5-ptl2s-ec4jw-osbv4-nygtw-dae";                                     
          subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
      }
  }
}     
})'

dfx canister call icrc7 icrc37_approve_collection '(vec{ 
  record{
    token_id= 2;                               
    approval_info= record {
      memo= null;
      from_subaccount= null;
      created_at_time= null;
      expires_at= null;
      spender= record {
          owner = principal "o2zom-piy75-ifbnk-nhhlq-362su-4vsx5-ptl2s-ec4jw-osbv4-nygtw-dae";                                     
          subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
      }
  }
}     
})'
```

#### Transfer From NFT

```bash
dfx canister call icrc7 icrc37_transfer_from '(vec{
  record{
    from= record {
        owner = principal "3yyxm-t5fpe-v32em-ac6lr-xyort-wuscb-dvl4x-3wnwi-hqkyj-xortw-oqe";                                     
        subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
    };
    to= record {
        owner = principal "t4egw-clf4w-qbpli-svryg-7yqq6-jt2yj-7v755-mabir-zmx6i-vp4fr-fqe";                                     
        subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
    };
    spender_subaccount= null;
    token_id= 2;
    memo= opt blob "123";
    created_at_time= null
  }
})'
```

#### Burn NFT

The implementation of the burn method does not delete the token; rather, it transfers the token to a burn_address (akin to a zero address).

```bash
dfx canister call icrc7 burn '(vec {
  record {
    token_id = 1 : nat;
    memo = opt blob "Burning token 1";
    from_subaccount = null
  }
})'
```