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
    permitted_drift = opt 300;
    archive_init = opt record {
      maxRecordsToArchive = 10000;
      archiveIndexType = variant { Stable };
      maxArchivePages = 10;
      settleToRecords = 1000;
      archiveCycles = 1000000000;
      maxActiveRecords = 100;
      maxRecordsInArchiveInstance = 1000;
      archiveControllers = null;
    };
    icrc7_max_take_value = opt 1000;
    icrc7_max_memo_size = opt 256;
    icrc7_symbol = "EXM";
    icrc7_max_update_batch_size = opt 200;
    icrc7_atomic_batch_transfers = opt true;
    approval_init = opt record {
      max_approvals = opt 100;
      max_approvals_per_token_or_collection = opt 10;
      settle_to_approvals = opt 5;
      max_revoke_approvals = opt 50;
      collection_approval_requires_token = opt false;
    };
    icrc7_default_take_value = opt 500;
    icrc7_logo = opt "https://example.com/logo.png";
    icrc7_name = "Example Collection";
  }
)
EOF
)

# Call the canister method
dfx canister call "$ICRC7_LAUNCHPAD_CANISTER_ID" mint_collection_canister "$ARG"
