dfx deploy icrc7_archive --argument '(record{   
    first_index= 0;
    index_type= variant {Stable};
    max_pages= 10;
    max_records= 100
})'

dfx canister call icrc7_archive append_blocks '(vec{
    variant {
        Int= 1
    };
    variant {
        Int= 1
    }
})'