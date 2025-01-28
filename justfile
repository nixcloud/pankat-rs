build-frontend:
  cd frontend && wasm-pack build --target web --release

copy-frontend:
  mkdir -p documents/static/wasm && cp -R frontend/pkg/* documents/static/wasm

build:
  cargo build

run: build-frontend copy-frontend build
  cargo run