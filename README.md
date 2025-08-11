# wasm dwarf lib

## build

```bash
cargo install wit-bindgen-cli
cargo build -r --target wasm32-unknown-unknown
```

output wasm is placed in `target/wasm32-unknown-unknown/release/binding.wasm`

## test

```bash
cargo test --package dwo_parser_impl
```
