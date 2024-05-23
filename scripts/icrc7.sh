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
icrc7_max_update_batch_size= opt 5;
icrc7_max_query_batch_size= opt 5;
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

dfx canister call icrc7 icrc37_approve_tokens '(vec{ 
    record{
        token_id= 1;                               
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
