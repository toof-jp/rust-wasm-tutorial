cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target no-modules --no-typescript --out-dir . ../target/wasm32-unknown-unknown/release/rust_wasm_tutorial.wasm
