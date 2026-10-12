# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "tokio-util-0_7_13-a55504be0ec4869b";
    meta.cargo_crate_info = {
      name = "tokio-util";
      version = "0.7.13";
      crate_hash = "a55504be0ec4869b";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [bytes-1_10_0-64a68773187e68cf futures-core-0_3_31-f6a43156147538e7 futures-sink-0_3_31-120d6b5c0eab5386 pin-project-lite-0_2_16-dbb7ad05ebc1034f tokio-1_43_0-1ce995a0fb66b2ee];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/tokio-util/0.7.13/download";
      sha256 = "d7fcaa8d55a2bdd6b83ace262b016eca0d79ee02818c5c1bcdf0305114081078";
    };

    unpackPhase = ''
      tar xf $src
      cd tokio-util-0.7.13
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "tokio_util";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "Tokio Contributors <team@tokio.rs>";
    CARGO_PKG_DESCRIPTION = "Additional utilities for working with Tokio.";
    CARGO_PKG_HOMEPAGE = "https://tokio.rs";
    CARGO_PKG_LICENSE = "MIT";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "tokio-util";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/tokio-rs/tokio";
    CARGO_PKG_RUST_VERSION = "1.70";
    CARGO_PKG_VERSION = "0.7.13";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "7";
    CARGO_PKG_VERSION_PATCH = "13";
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
              --crate-name tokio_util \
              --edition=2021 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --cfg 'feature="codec"' \
              --cfg 'feature="default"' \
              --cfg 'feature="io"' \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("__docs_rs", "codec", "compat", "default", "full", "futures-io", "futures-util", "hashbrown", "io", "io-util", "net", "rt", "slab", "time", "tracing"))' \
              -C metadata=870b98091f0a24b6 \
              -C extra-filename=-a55504be0ec4869b \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern bytes=${bytes-1_10_0-64a68773187e68cf}/libbytes-64a68773187e68cf.rmeta \
              --extern futures_core=${futures-core-0_3_31-f6a43156147538e7}/libfutures_core-f6a43156147538e7.rmeta \
              --extern futures_sink=${futures-sink-0_3_31-120d6b5c0eab5386}/libfutures_sink-120d6b5c0eab5386.rmeta \
              --extern pin_project_lite=${pin-project-lite-0_2_16-dbb7ad05ebc1034f}/libpin_project_lite-dbb7ad05ebc1034f.rmeta \
              --extern tokio=${tokio-1_43_0-1ce995a0fb66b2ee}/libtokio-1ce995a0fb66b2ee.rmeta \
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
