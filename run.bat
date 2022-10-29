cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-dir ./out/ --target web .\target\wasm32-unknown-unknown\release\rs_granite.wasm

basic-http-server out