build-frontend:
  cd frontend && wasm-pack build --target web --debug

serve-frontend:
  cd frontend && trunk serve --port 5001

copy-frontend:
  mkdir -p documents/pkg && cp -R frontend/pkg/* documents/pkg

build:
  cargo build

run: build-frontend copy-frontend build
  cargo run