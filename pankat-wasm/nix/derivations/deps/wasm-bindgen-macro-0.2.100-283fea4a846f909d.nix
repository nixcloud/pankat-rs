# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "wasm-bindgen-macro-0_2_100-283fea4a846f909d";
    meta.cargo_crate_info = {
      name = "wasm-bindgen-macro";
      version = "0.2.100";
      crate_hash = "283fea4a846f909d";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [quote-1_0_38-12b99e3192e30e82 wasm-bindgen-macro-support-0_2_100-96964d1b1a55d331];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/wasm-bindgen-macro/0.2.100/download";
      sha256 = "7fe63fc6d09ed3792bd0897b314f53de8e16568c2b3f7982f468c0bf9bd0b407";
    };

    unpackPhase = ''
      tar xf $src
      cd wasm-bindgen-macro-0.2.100
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "wasm_bindgen_macro";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "The wasm-bindgen Developers";
    CARGO_PKG_DESCRIPTION = "Definition of the `#[wasm_bindgen]` attribute, an internal dependency";
    CARGO_PKG_HOMEPAGE = "https://rustwasm.github.io/wasm-bindgen/";
    CARGO_PKG_LICENSE = "MIT OR Apache-2.0";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "wasm-bindgen-macro";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/rustwasm/wasm-bindgen/tree/master/crates/macro";
    CARGO_PKG_RUST_VERSION = "1.57";
    CARGO_PKG_VERSION = "0.2.100";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "2";
    CARGO_PKG_VERSION_PATCH = "100";
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
              --crate-name wasm_bindgen_macro \
              --edition=2021 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type proc-macro \
              --emit=dep-info,link \
              -C prefer-dynamic \
              -C embed-bitcode=no \
              --allow=clippy::type_complexity \
              --allow=clippy::too_many_arguments \
              --allow=clippy::overly_complex_bool_expr \
              --allow=clippy::new_without_default \
              --allow=clippy::large_enum_variant \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("strict-macro", "xxx_debug_only_print_generated_code"))' \
              -C metadata=a734596341c5d865 \
              -C extra-filename=-283fea4a846f909d \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern quote=${quote-1_0_38-12b99e3192e30e82}/libquote-12b99e3192e30e82.rlib \
              --extern wasm_bindgen_macro_support=${wasm-bindgen-macro-support-0_2_100-96964d1b1a55d331}/libwasm_bindgen_macro_support-96964d1b1a55d331.rlib \
              --extern proc_macro \
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
