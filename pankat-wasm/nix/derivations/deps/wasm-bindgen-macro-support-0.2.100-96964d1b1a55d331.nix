# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "wasm-bindgen-macro-support-0_2_100-96964d1b1a55d331";
    meta.cargo_crate_info = {
      name = "wasm-bindgen-macro-support";
      version = "0.2.100";
      crate_hash = "96964d1b1a55d331";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [proc-macro2-1_0_93-cfe81a59cf98819f quote-1_0_38-12b99e3192e30e82 syn-2_0_98-93aa0f13dad61a07 wasm-bindgen-backend-0_2_100-d0c488311c1205dd wasm-bindgen-shared-0_2_100-acd810d9b451b936];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/wasm-bindgen-macro-support/0.2.100/download";
      sha256 = "8ae87ea40c9f689fc23f209965b6fb8a99ad69aeeb0231408be24920604395de";
    };

    unpackPhase = ''
      tar xf $src
      cd wasm-bindgen-macro-support-0.2.100
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "wasm_bindgen_macro_support";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "The wasm-bindgen Developers";
    CARGO_PKG_DESCRIPTION = "The part of the implementation of the `#[wasm_bindgen]` attribute that is not in the shared backend crate";
    CARGO_PKG_HOMEPAGE = "https://rustwasm.github.io/wasm-bindgen/";
    CARGO_PKG_LICENSE = "MIT OR Apache-2.0";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "wasm-bindgen-macro-support";
    CARGO_PKG_README = "";
    CARGO_PKG_REPOSITORY = "https://github.com/rustwasm/wasm-bindgen/tree/master/crates/macro-support";
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
              --crate-name wasm_bindgen_macro_support \
              --edition=2021 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              --allow=clippy::type_complexity \
              --allow=clippy::too_many_arguments \
              --allow=clippy::overly_complex_bool_expr \
              --allow=clippy::new_without_default \
              --allow=clippy::large_enum_variant \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("extra-traits", "strict-macro"))' \
              -C metadata=29ab8c5368c05981 \
              -C extra-filename=-96964d1b1a55d331 \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern proc_macro2=${proc-macro2-1_0_93-cfe81a59cf98819f}/libproc_macro2-cfe81a59cf98819f.rmeta \
              --extern quote=${quote-1_0_38-12b99e3192e30e82}/libquote-12b99e3192e30e82.rmeta \
              --extern syn=${syn-2_0_98-93aa0f13dad61a07}/libsyn-93aa0f13dad61a07.rmeta \
              --extern wasm_bindgen_backend=${wasm-bindgen-backend-0_2_100-d0c488311c1205dd}/libwasm_bindgen_backend-d0c488311c1205dd.rmeta \
              --extern wasm_bindgen_shared=${wasm-bindgen-shared-0_2_100-acd810d9b451b936}/libwasm_bindgen_shared-acd810d9b451b936.rmeta \
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
