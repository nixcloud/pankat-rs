build-pankat-wasm:
  cd pankat-wasm && wasm-pack build --target web --release

copy-pankat-wasm:
  mkdir -p documents/assets/pankat-wasm && cp -R pankat-wasm/pkg/* documents/assets/pankat-wasm

build:
  cargo build

run: build-pankat-wasm copy-pankat-wasm build
  cargo run -- --input documents/blog.lastlog.de --output documents/output --assets documents/assets