cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./web/ --no-typescript --target web ./target/wasm32-unknown-unknown/release/birds-of-a-feather.wasm