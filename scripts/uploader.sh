cargo install icx-proxy

icx-proxy --fetch-root-key --address 127.0.0.1:8453 --dns-alias myproject.localhost:bkyz2-fmaaa-aaaaa-qaaaq-cai -v -v


dfx canister call ic_canister_assets permission_set_admin '(principal "3yyxm-t5fpe-v32em-ac6lr-xyort-wuscb-dvl4x-3wnwi-hqkyj-xortw-oqe")'

dfx canister call ic_canister_assets permission_is_admin '(principal "3yyxm-t5fpe-v32em-ac6lr-xyort-wuscb-dvl4x-3wnwi-hqkyj-xortw-oqe")'


 : (principal) -> (bool) query;