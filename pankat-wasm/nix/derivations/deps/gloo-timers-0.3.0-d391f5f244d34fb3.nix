# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "gloo-timers-0_3_0-d391f5f244d34fb3";
    meta.cargo_crate_info = {
      name = "gloo-timers";
      version = "0.3.0";
      crate_hash = "d391f5f244d34fb3";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [futures-channel-0_3_31-c19bd6ab8876ed10 futures-core-0_3_31-f6a43156147538e7 js-sys-0_3_77-d34ceb3da0dda202 wasm-bindgen-0_2_100-77c8057a5f6a6e61];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/gloo-timers/0.3.0/download";
      sha256 = "bbb143cf96099802033e0d4f4963b19fd2e0b728bcf076cd9cf7f6634f092994";
    };

    unpackPhase = ''
      tar xf $src
      cd gloo-timers-0.3.0
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "gloo_timers";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "Rust and WebAssembly Working Group";
    CARGO_PKG_DESCRIPTION = "Convenience crate for working with JavaScript timers";
    CARGO_PKG_HOMEPAGE = "https://github.com/rustwasm/gloo";
    CARGO_PKG_LICENSE = "MIT OR Apache-2.0";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "gloo-timers";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/rustwasm/gloo/tree/master/crates/timers";
    CARGO_PKG_RUST_VERSION = "1.64";
    CARGO_PKG_VERSION = "0.3.0";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "3";
    CARGO_PKG_VERSION_PATCH = "0";
    CARGO_PKG_VERSION_PRE = "";

    buildPhase = ''
      ${fn.import_bash_function_helpers}
      export CARGO_MANIFEST_DIR=$(realpath $PWD/$CARGO_MANIFEST_DIR)
      export CARGO_MANIFEST_PATH=$(realpath $PWD/$CARGO_MANIFEST_PATH)
      
      mkdir -p $out/nix
      export OUT_DIR=$out

      print_compiling_message "${name}"
      print_cargo_message_type_0 "${name}" "${meta.cargo_crate_info.name}" "${meta.cargo_crate_info.type}"

      rustc_json_output_lines=$(${pkgs.mktemp}/bin/mktemp)
      set -x +e
      ${RUSTC} \
              --crate-name gloo_timers \
              --edition=2021 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --cfg 'feature="default"' \
              --cfg 'feature="futures"' \
              --cfg 'feature="futures-channel"' \
              --cfg 'feature="futures-core"' \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("default", "futures", "futures-channel", "futures-core"))' \
              -C metadata=1ce6028c0d6abc40 \
              -C extra-filename=-d391f5f244d34fb3 \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern futures_channel=${futures-channel-0_3_31-c19bd6ab8876ed10}/libfutures_channel-c19bd6ab8876ed10.rmeta \
              --extern futures_core=${futures-core-0_3_31-f6a43156147538e7}/libfutures_core-f6a43156147538e7.rmeta \
              --extern js_sys=${js-sys-0_3_77-d34ceb3da0dda202}/libjs_sys-d34ceb3da0dda202.rmeta \
              --extern wasm_bindgen=${wasm-bindgen-0_2_100-77c8057a5f6a6e61}/libwasm_bindgen-77c8057a5f6a6e61.rmeta \
              --cap-lints allow 2> $rustc_json_output_lines
      rustc_exit_value=$?
      set +x -e
           
      print_rustc_rendered_messages $rustc_json_output_lines
      
      print_cargo_message_type_2 "${name}" "${meta.cargo_crate_info.name}" "${meta.cargo_crate_info.type}" $rustc_exit_value $rustc_json_output_lines
      
      if [ "$rustc_exit_value" -ne 0 ]; then
          exit $rustc_exit_value
      fi
    '';

}
