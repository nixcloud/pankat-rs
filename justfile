build-pankat-wasm:
  cd pankat-wasm && wasm-pack build --target web --release --manifest-path ./Cargo.toml 

copy-pankat-wasm:
  mkdir -p documents/assets/pankat-wasm && cp -R pankat-wasm/pkg/* documents/assets/pankat-wasm

build-backend:
  cargo build

run: build-backend copy-pankat-wasm
  cargo run -- --input documents/lastlog.de --output documents/output/ --assets documents/assets/ --database documents/

run-all: build-pankat-wasm copy-pankat-wasm build-backend
  cargo run -- --input documents/blog.lastlog.de --output documents/output/ --assets documents/assets/ --database documents/

test:
  cargo test

fmt:
  cargo fmt

  