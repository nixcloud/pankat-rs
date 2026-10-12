# generated from target.nix.handlebars using cargo (manual edits won't be persistent)
{ pkgs, pankat-wasm-0_1_0-c4a3d03613b2b446 }:

pkgs.writeShellScriptBin "create-symlinks" ''
  if [[ -f "${pankat-wasm-0_1_0-c4a3d03613b2b446}/libpankat_wasm-c4a3d03613b2b446.rlib" ]]; then
    ln -fs ${pankat-wasm-0_1_0-c4a3d03613b2b446}/libpankat_wasm-c4a3d03613b2b446.rlib target/debug/libpankat_wasm.rlib
  fi
  if [[ -f "${pankat-wasm-0_1_0-c4a3d03613b2b446}/libpankat_wasm-c4a3d03613b2b446.so" ]]; then
    ln -fs ${pankat-wasm-0_1_0-c4a3d03613b2b446}/libpankat_wasm-c4a3d03613b2b446.so target/debug/libpankat_wasm.so
  fi
  if [[ -f "${pankat-wasm-0_1_0-c4a3d03613b2b446}/libpankat_wasm-c4a3d03613b2b446.a" ]]; then
    ln -fs ${pankat-wasm-0_1_0-c4a3d03613b2b446}/libpankat_wasm-c4a3d03613b2b446.a target/debug/libpankat_wasm.a
  fi
  if [[ -f "${pankat-wasm-0_1_0-c4a3d03613b2b446}/libpankat_wasm-c4a3d03613b2b446.rmeta" ]]; then
    ln -fs ${pankat-wasm-0_1_0-c4a3d03613b2b446}/libpankat_wasm-c4a3d03613b2b446.rmeta target/debug/libpankat_wasm.rmeta
  fi
  if [[ -f "${pankat-wasm-0_1_0-c4a3d03613b2b446}/libpankat_wasm-c4a3d03613b2b446.d" ]]; then
    ln -fs ${pankat-wasm-0_1_0-c4a3d03613b2b446}/libpankat_wasm-c4a3d03613b2b446.d target/debug/libpankat_wasm.d
  fi
''