build-pankat-wasm:
  cd pankat-wasm && wasm-pack build --target web --release --manifest-path ./Cargo.toml 

copy-pankat-wasm:
  mkdir -p documents/wasm && cp -R pankat-wasm/pkg/* documents/wasm

build-backend:
  cargo build

run-backend-only: build-backend copy-pankat-wasm
  cargo run -- --input documents/blog.lastlog.de --flat

run: build-pankat-wasm copy-pankat-wasm build-backend
  cargo run -- --input documents/blog.lastlog.de --flat

test:
  cargo test

fmt:
  cargo fmt