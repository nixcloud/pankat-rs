# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "gloo-net-0_5_0-6ac8e7851b0f0d3b";
    meta.cargo_crate_info = {
      name = "gloo-net";
      version = "0.5.0";
      crate_hash = "6ac8e7851b0f0d3b";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [futures-channel-0_3_31-c19bd6ab8876ed10 futures-core-0_3_31-f6a43156147538e7 futures-sink-0_3_31-120d6b5c0eab5386 gloo-utils-0_2_0-40da9fc51db2c995 http-0_2_12-83e90f39f49f8f94 js-sys-0_3_77-d34ceb3da0dda202 pin-project-1_1_9-91cb6bfb138aa153 thiserror-1_0_69-19f3044926d2abbd wasm-bindgen-0_2_100-77c8057a5f6a6e61 wasm-bindgen-futures-0_4_50-ca328d45d204dbbc web-sys-0_3_77-c55ef6dc80720d2e];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/gloo-net/0.5.0/download";
      sha256 = "43aaa242d1239a8822c15c645f02166398da4f8b5c4bae795c1f5b44e9eee173";
    };

    unpackPhase = ''
      tar xf $src
      cd gloo-net-0.5.0
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "gloo_net";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "Rust and WebAssembly Working Group:Muhammad Hamza <muhammadhamza1311@gmail.com>";
    CARGO_PKG_DESCRIPTION = "HTTP requests library for WASM Apps";
    CARGO_PKG_HOMEPAGE = "";
    CARGO_PKG_LICENSE = "MIT OR Apache-2.0";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "gloo-net";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/rustwasm/gloo";
    CARGO_PKG_RUST_VERSION = "1.64";
    CARGO_PKG_VERSION = "0.5.0";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "5";
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
              --crate-name gloo_net \
              --edition=2021 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --cfg 'feature="futures-channel"' \
              --cfg 'feature="futures-core"' \
              --cfg 'feature="futures-sink"' \
              --cfg 'feature="pin-project"' \
              --cfg 'feature="websocket"' \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("default", "eventsource", "futures-channel", "futures-core", "futures-io", "futures-sink", "http", "io-util", "json", "pin-project", "serde", "serde_json", "websocket"))' \
              -C metadata=dec9ee0d4f418a77 \
              -C extra-filename=-6ac8e7851b0f0d3b \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern futures_channel=${futures-channel-0_3_31-c19bd6ab8876ed10}/libfutures_channel-c19bd6ab8876ed10.rmeta \
              --extern futures_core=${futures-core-0_3_31-f6a43156147538e7}/libfutures_core-f6a43156147538e7.rmeta \
              --extern futures_sink=${futures-sink-0_3_31-120d6b5c0eab5386}/libfutures_sink-120d6b5c0eab5386.rmeta \
              --extern gloo_utils=${gloo-utils-0_2_0-40da9fc51db2c995}/libgloo_utils-40da9fc51db2c995.rmeta \
              --extern http=${http-0_2_12-83e90f39f49f8f94}/libhttp-83e90f39f49f8f94.rmeta \
              --extern js_sys=${js-sys-0_3_77-d34ceb3da0dda202}/libjs_sys-d34ceb3da0dda202.rmeta \
              --extern pin_project=${pin-project-1_1_9-91cb6bfb138aa153}/libpin_project-91cb6bfb138aa153.rmeta \
              --extern thiserror=${thiserror-1_0_69-19f3044926d2abbd}/libthiserror-19f3044926d2abbd.rmeta \
              --extern wasm_bindgen=${wasm-bindgen-0_2_100-77c8057a5f6a6e61}/libwasm_bindgen-77c8057a5f6a6e61.rmeta \
              --extern wasm_bindgen_futures=${wasm-bindgen-futures-0_4_50-ca328d45d204dbbc}/libwasm_bindgen_futures-ca328d45d204dbbc.rmeta \
              --extern web_sys=${web-sys-0_3_77-c55ef6dc80720d2e}/libweb_sys-c55ef6dc80720d2e.rmeta \
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
