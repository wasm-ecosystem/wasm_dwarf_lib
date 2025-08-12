set -e

cargo build -r --target wasm32-unknown-unknown
wasm2wat target/wasm32-unknown-unknown/release/binding.wasm -o target/wasm32-unknown-unknown/release/binding.wat
cp target/wasm32-unknown-unknown/release/binding.wasm ../wasm-compiler/scripts/wasm_dwarf_lib.wasm

ls target/wasm32-unknown-unknown/release/binding.wat