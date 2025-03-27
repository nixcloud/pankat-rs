build-pankat-wasm:
  cd pankat-wasm && wasm-pack build --target web --release --manifest-path ./Cargo.toml 

copy-pankat-wasm:
  mkdir -p documents/wasm && cp -R pankat-wasm/pkg/* documents/wasm

build-backend:
  cargo build

build-backend-release:
  cargo zigbuild --release

run-backend-only: build-backend copy-pankat-wasm
  cargo run

run: build-pankat-wasm copy-pankat-wasm build-backend
  cargo run

test:
  cargo test

fmt:
  cargo fmt