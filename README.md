# icrc7_launchpad

## Init

ICRC37 Init args:

```
    type InitApprovalsArg = record {
        max_approvals : opt nat16;
        max_approvals_per_token_or_collection : opt nat16;
        settle_to_approvals : opt nat16;
        max_revoke_approvals : opt nat16;
        collection_approval_requires_token : opt bool;
    };
```

ICRC3 Init args:

```
    type InitArchiveArg = record {
        maxRecordsToArchive : nat;   //Max number of archive items to archive in one round
        archiveIndexType : IndexType;  //Index type to use for the memory of the archive
        maxArchivePages : nat;   //Max number of pages allowed on the archivserver
        settleToRecords : nat;  //number of records to settle to during the clean up process
        archiveCycles : nat;  //number of cycles to sent to a new archive canister;
        maxActiveRecords : nat;  //allowed max active records on this canister
        maxRecordsInArchiveInstance : nat;  //specify the max number of archive items to put on an archive instance
        archiveControllers : opt opt vec principal;   //override the default controllers. The canister will always add itself to this group;
    };
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
