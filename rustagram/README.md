# Rustagram

### Command

```bash
cargo build --target wasm32-wasi
```

```bash
wasmtime target/wasm32-wasi/debug/rustagram.wasm
```

```bash
wasm2wat target/wasm32-wasi/debug/rustagram.wasm
```

### Command for CLI

```bash
cargo build --target wasm32-wasi
```

```bash
wasmtime --dir . target/wasm32-wasi/debug/rustagram.wasm "96023.jpg" 1977
```
