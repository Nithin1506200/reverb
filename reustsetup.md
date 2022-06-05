# Rust setup

1. Create new library

```sh
cargo new --lib reverb
```

2. install wasm-pack

```sh
cargo install wasm-pack
```

3. update Cargo.toml to add `[lib]` and `[dependencies]`

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["***"]
edition = "***"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2.74"
```

4. To test create `bin\main.rs`

```sh
cargo run --bin main
```

5. To build wasm

```sh
wasm-pack build --target web
```
