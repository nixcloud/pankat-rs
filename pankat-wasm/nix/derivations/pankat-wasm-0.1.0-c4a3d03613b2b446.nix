# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "pankat-wasm-0_1_0-c4a3d03613b2b446";
    meta.cargo_crate_info = {
      name = "pankat-wasm";
      version = "0.1.0";
      crate_hash = "c4a3d03613b2b446";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [console_log-1_0_0-72e8f99a2a6e7480 futures-0_3_30-e4b5d46ddca185d9 gloo-net-0_5_0-6ac8e7851b0f0d3b gloo-timers-0_3_0-d391f5f244d34fb3 gloo-utils-0_2_0-40da9fc51db2c995 lazy_static-1_5_0-ffebef2c0e161fa7 log-0_4_26-d46dd80a9e51535a reqwest-0_11_27-e0792ed017152126 serde-1_0_218-eb19f02d9fcfde0c serde_json-1_0_140-46bc37dceda4ba0d tokio-1_43_0-1ce995a0fb66b2ee wasm-bindgen-0_2_100-77c8057a5f6a6e61 wasm-bindgen-futures-0_4_50-ca328d45d204dbbc web-sys-0_3_77-c55ef6dc80720d2e];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = builtins.filterSource
    (path: type:
        let base = baseNameOf path;
        in !(base == "target" || base == "result" || builtins.match "result-*" base != null)
    ) ../..;

    unpackPhase = "";

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "pankat_wasm";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "";
    CARGO_PKG_DESCRIPTION = "";
    CARGO_PKG_HOMEPAGE = "";
    CARGO_PKG_LICENSE = "";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "pankat-wasm";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "";
    CARGO_PKG_RUST_VERSION = "";
    CARGO_PKG_VERSION = "0.1.0";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "1";
    CARGO_PKG_VERSION_PATCH = "0";
    CARGO_PKG_VERSION_PRE = "";
    CARGO_PRIMARY_PACKAGE = "1";
    CARGO_SBOM_PATH = "";

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
              --crate-name pankat_wasm \
              --edition=2021 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type cdylib \
              --crate-type rlib \
              --emit=dep-info,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values())' \
              -C metadata=29059d29987193cb \
              --out-dir $OUT_DIR \
              -C incremental=$INC_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern console_log=${console_log-1_0_0-72e8f99a2a6e7480}/libconsole_log-72e8f99a2a6e7480.rlib \
              --extern futures=${futures-0_3_30-e4b5d46ddca185d9}/libfutures-e4b5d46ddca185d9.rlib \
              --extern gloo_net=${gloo-net-0_5_0-6ac8e7851b0f0d3b}/libgloo_net-6ac8e7851b0f0d3b.rlib \
              --extern gloo_timers=${gloo-timers-0_3_0-d391f5f244d34fb3}/libgloo_timers-d391f5f244d34fb3.rlib \
              --extern gloo_utils=${gloo-utils-0_2_0-40da9fc51db2c995}/libgloo_utils-40da9fc51db2c995.rlib \
              --extern lazy_static=${lazy_static-1_5_0-ffebef2c0e161fa7}/liblazy_static-ffebef2c0e161fa7.rlib \
              --extern log=${log-0_4_26-d46dd80a9e51535a}/liblog-d46dd80a9e51535a.rlib \
              --extern reqwest=${reqwest-0_11_27-e0792ed017152126}/libreqwest-e0792ed017152126.rlib \
              --extern serde=${serde-1_0_218-eb19f02d9fcfde0c}/libserde-eb19f02d9fcfde0c.rlib \
              --extern serde_json=${serde_json-1_0_140-46bc37dceda4ba0d}/libserde_json-46bc37dceda4ba0d.rlib \
              --extern tokio=${tokio-1_43_0-1ce995a0fb66b2ee}/libtokio-1ce995a0fb66b2ee.rlib \
              --extern wasm_bindgen=${wasm-bindgen-0_2_100-77c8057a5f6a6e61}/libwasm_bindgen-77c8057a5f6a6e61.rlib \
              --extern wasm_bindgen_futures=${wasm-bindgen-futures-0_4_50-ca328d45d204dbbc}/libwasm_bindgen_futures-ca328d45d204dbbc.rlib \
              --extern web_sys=${web-sys-0_3_77-c55ef6dc80720d2e}/libweb_sys-c55ef6dc80720d2e.rlib 2> $rustc_json_output_lines
      rustc_exit_value=$?
      set +x -e

      ls -la
           
      print_rustc_rendered_messages $rustc_json_output_lines
      
      print_cargo_message_type_2 "${name}" "${meta.cargo_crate_info.name}" "${meta.cargo_crate_info.type}" $rustc_exit_value $rustc_json_output_lines
      
      if [ "$rustc_exit_value" -ne 0 ]; then
          exit $rustc_exit_value
      fi
    '';

}
