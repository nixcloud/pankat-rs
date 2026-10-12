# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "h2-0_3_26-6ae3cdc03d4408e3";
    meta.cargo_crate_info = {
      name = "h2";
      version = "0.3.26";
      crate_hash = "6ae3cdc03d4408e3";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [bytes-1_10_0-64a68773187e68cf fnv-1_0_7-e86d9923dc926ea6 futures-core-0_3_31-f6a43156147538e7 futures-sink-0_3_31-120d6b5c0eab5386 futures-util-0_3_31-ddd4b710c5171915 http-0_2_12-83e90f39f49f8f94 indexmap-2_7_1-e565cbeccfc36840 slab-0_4_9-703e5769016427df tokio-1_43_0-1ce995a0fb66b2ee tokio-util-0_7_13-a55504be0ec4869b tracing-0_1_41-4e706e46b3a66672];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/h2/0.3.26/download";
      sha256 = "81fe527a889e1532da5c525686d96d4c2e74cdd345badf8dfef9f6b39dd5f5e8";
    };

    unpackPhase = ''
      tar xf $src
      cd h2-0.3.26
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "h2";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "Carl Lerche <me@carllerche.com>:Sean McArthur <sean@seanmonstar.com>";
    CARGO_PKG_DESCRIPTION = "An HTTP/2 client and server";
    CARGO_PKG_HOMEPAGE = "";
    CARGO_PKG_LICENSE = "MIT";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "h2";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/hyperium/h2";
    CARGO_PKG_RUST_VERSION = "1.63";
    CARGO_PKG_VERSION = "0.3.26";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "3";
    CARGO_PKG_VERSION_PATCH = "26";
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
              --crate-name h2 \
              --edition=2018 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("stream", "unstable"))' \
              -C metadata=d6129b0175a7ffcc \
              -C extra-filename=-6ae3cdc03d4408e3 \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern bytes=${bytes-1_10_0-64a68773187e68cf}/libbytes-64a68773187e68cf.rmeta \
              --extern fnv=${fnv-1_0_7-e86d9923dc926ea6}/libfnv-e86d9923dc926ea6.rmeta \
              --extern futures_core=${futures-core-0_3_31-f6a43156147538e7}/libfutures_core-f6a43156147538e7.rmeta \
              --extern futures_sink=${futures-sink-0_3_31-120d6b5c0eab5386}/libfutures_sink-120d6b5c0eab5386.rmeta \
              --extern futures_util=${futures-util-0_3_31-ddd4b710c5171915}/libfutures_util-ddd4b710c5171915.rmeta \
              --extern http=${http-0_2_12-83e90f39f49f8f94}/libhttp-83e90f39f49f8f94.rmeta \
              --extern indexmap=${indexmap-2_7_1-e565cbeccfc36840}/libindexmap-e565cbeccfc36840.rmeta \
              --extern slab=${slab-0_4_9-703e5769016427df}/libslab-703e5769016427df.rmeta \
              --extern tokio=${tokio-1_43_0-1ce995a0fb66b2ee}/libtokio-1ce995a0fb66b2ee.rmeta \
              --extern tokio_util=${tokio-util-0_7_13-a55504be0ec4869b}/libtokio_util-a55504be0ec4869b.rmeta \
              --extern tracing=${tracing-0_1_41-4e706e46b3a66672}/libtracing-4e706e46b3a66672.rmeta \
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
