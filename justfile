build-frontend:
  cd frontend && wasm-pack build --target web

serve-frontend:
  cd frontend && trunk serve --port 5000

build:
  cargo build