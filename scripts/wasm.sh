cargo build --release --target wasm32-unknown-unknown --package icrc7
ic-wasm target/wasm32-unknown-unknown/release/icrc7.wasm -o target/wasm32-unknown-unknown/release/icrc7.wasm shrink
gzip -f target/wasm32-unknown-unknown/release/icrc7.wasm > wasm/icrc7.wasm.gz