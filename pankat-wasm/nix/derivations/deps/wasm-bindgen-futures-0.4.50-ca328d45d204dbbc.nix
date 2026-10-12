# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "wasm-bindgen-futures-0_4_50-ca328d45d204dbbc";
    meta.cargo_crate_info = {
      name = "wasm-bindgen-futures";
      version = "0.4.50";
      crate_hash = "ca328d45d204dbbc";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [cfg-if-1_0_0-616d46354eebf850 js-sys-0_3_77-d34ceb3da0dda202 once_cell-1_20_3-60992a3834e62ae0 wasm-bindgen-0_2_100-77c8057a5f6a6e61];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/wasm-bindgen-futures/0.4.50/download";
      sha256 = "555d470ec0bc3bb57890405e5d4322cc9ea83cebb085523ced7be4144dac1e61";
    };

    unpackPhase = ''
      tar xf $src
      cd wasm-bindgen-futures-0.4.50
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "wasm_bindgen_futures";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "The wasm-bindgen Developers";
    CARGO_PKG_DESCRIPTION = "Bridging the gap between Rust Futures and JavaScript Promises";
    CARGO_PKG_HOMEPAGE = "https://rustwasm.github.io/wasm-bindgen/";
    CARGO_PKG_LICENSE = "MIT OR Apache-2.0";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "wasm-bindgen-futures";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/rustwasm/wasm-bindgen/tree/master/crates/futures";
    CARGO_PKG_RUST_VERSION = "1.57";
    CARGO_PKG_VERSION = "0.4.50";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "4";
    CARGO_PKG_VERSION_PATCH = "50";
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
              --crate-name wasm_bindgen_futures \
              --edition=2021 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              --allow=clippy::type_complexity \
              --allow=clippy::too_many_arguments \
              --allow=clippy::overly_complex_bool_expr \
              --allow=clippy::new_without_default \
              --allow=clippy::large_enum_variant \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --cfg 'feature="default"' \
              --cfg 'feature="std"' \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("default", "futures-core", "futures-core-03-stream", "std"))' \
              -C metadata=62084164c2c697d2 \
              -C extra-filename=-ca328d45d204dbbc \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern cfg_if=${cfg-if-1_0_0-616d46354eebf850}/libcfg_if-616d46354eebf850.rmeta \
              --extern js_sys=${js-sys-0_3_77-d34ceb3da0dda202}/libjs_sys-d34ceb3da0dda202.rmeta \
              --extern once_cell=${once_cell-1_20_3-60992a3834e62ae0}/libonce_cell-60992a3834e62ae0.rmeta \
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
