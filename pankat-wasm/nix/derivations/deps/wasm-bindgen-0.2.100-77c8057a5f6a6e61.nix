# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "wasm-bindgen-0_2_100-77c8057a5f6a6e61";
    meta.cargo_crate_info = {
      name = "wasm-bindgen";
      version = "0.2.100";
      crate_hash = "77c8057a5f6a6e61";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [cfg-if-1_0_0-616d46354eebf850 once_cell-1_20_3-60992a3834e62ae0 rustversion-1_0_19-44471f16741f7333 wasm-bindgen-macro-0_2_100-283fea4a846f909d];
    passthru.rust_crate_parent = [wasm-bindgen-0_2_100-script_build_run-414fd6a6e909b616];
    passthru.rust_script_build_run = [wasm-bindgen-0_2_100-script_build_run-414fd6a6e909b616];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/wasm-bindgen/0.2.100/download";
      sha256 = "1edc8929d7499fc4e8f0be2262a241556cfc54a0bea223790e71446f2aab1ef5";
    };

    unpackPhase = ''
      tar xf $src
      cd wasm-bindgen-0.2.100
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "wasm_bindgen";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "The wasm-bindgen Developers";
    CARGO_PKG_DESCRIPTION = "Easy support for interacting between JS and Rust.";
    CARGO_PKG_HOMEPAGE = "https://rustwasm.github.io/";
    CARGO_PKG_LICENSE = "MIT OR Apache-2.0";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "wasm-bindgen";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/rustwasm/wasm-bindgen";
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
      copy_build_script_run_results_over_with_nix "${fn.get_rust_crate_parent passthru.rust_crate_parent}"
      for file in $out/environment-variables $out/rustc-arguments $out/rustc-propagated-arguments; do
          if [ -f "$file" ]; then
          sed -i "s|${fn.get_rust_crate_parent passthru.rust_crate_parent}|$out|g" "$file"
          fi
      done
      load_environment_variables_from_files "${fn.environment_variables passthru.rust_script_build_run}"
      rustc_json_output_lines=$(${pkgs.mktemp}/bin/mktemp)
      set -x +e
      ${RUSTC} \
              --crate-name wasm_bindgen \
              --edition=2021 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              --warn=unexpected_cfgs \
              --allow=clippy::type_complexity \
              --allow=clippy::too_many_arguments \
              --allow=clippy::overly_complex_bool_expr \
              --allow=clippy::new_without_default \
              --allow=clippy::large_enum_variant \
              --check-cfg 'cfg(wasm_bindgen_unstable_test_coverage)' \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --cfg 'feature="default"' \
              --cfg 'feature="msrv"' \
              --cfg 'feature="rustversion"' \
              --cfg 'feature="std"' \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("default", "enable-interning", "gg-alloc", "msrv", "rustversion", "serde", "serde-serialize", "serde_json", "spans", "std", "strict-macro", "xxx_debug_only_print_generated_code"))' \
              -C metadata=33b69f9d41272472 \
              -C extra-filename=-77c8057a5f6a6e61 \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern cfg_if=${cfg-if-1_0_0-616d46354eebf850}/libcfg_if-616d46354eebf850.rmeta \
              --extern once_cell=${once_cell-1_20_3-60992a3834e62ae0}/libonce_cell-60992a3834e62ae0.rmeta \
              --extern rustversion=${rustversion-1_0_19-44471f16741f7333}/librustversion-44471f16741f7333.so \
              --extern wasm_bindgen_macro=${wasm-bindgen-macro-0_2_100-283fea4a846f909d}/libwasm_bindgen_macro-283fea4a846f909d.so \
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
