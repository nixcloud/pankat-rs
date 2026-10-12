# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "hyper-0_14_32-78cc5e777f9503a3";
    meta.cargo_crate_info = {
      name = "hyper";
      version = "0.14.32";
      crate_hash = "78cc5e777f9503a3";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [bytes-1_10_0-64a68773187e68cf futures-channel-0_3_31-c19bd6ab8876ed10 futures-core-0_3_31-f6a43156147538e7 futures-util-0_3_31-ddd4b710c5171915 h2-0_3_26-6ae3cdc03d4408e3 http-0_2_12-83e90f39f49f8f94 http-body-0_4_6-604b1fb4509892f9 httparse-1_10_0-6ca9fa58eef50542 httpdate-1_0_3-0986d952b4b79a01 itoa-1_0_14-6ae7bc765ab6d9fa pin-project-lite-0_2_16-dbb7ad05ebc1034f socket2-0_5_8-b832ef87df39fa74 tokio-1_43_0-1ce995a0fb66b2ee tower-service-0_3_3-f2bb19b3a855b555 tracing-0_1_41-4e706e46b3a66672 want-0_3_1-65d33fb3976c0b7a];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/hyper/0.14.32/download";
      sha256 = "41dfc780fdec9373c01bae43289ea34c972e40ee3c9f6b3c8801a35f35586ce7";
    };

    unpackPhase = ''
      tar xf $src
      cd hyper-0.14.32
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "hyper";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "Sean McArthur <sean@seanmonstar.com>";
    CARGO_PKG_DESCRIPTION = "A fast and correct HTTP library.";
    CARGO_PKG_HOMEPAGE = "https://hyper.rs";
    CARGO_PKG_LICENSE = "MIT";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "hyper";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/hyperium/hyper";
    CARGO_PKG_RUST_VERSION = "";
    CARGO_PKG_VERSION = "0.14.32";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "14";
    CARGO_PKG_VERSION_PATCH = "32";
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
              --crate-name hyper \
              --edition=2018 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --cfg 'feature="client"' \
              --cfg 'feature="h2"' \
              --cfg 'feature="http1"' \
              --cfg 'feature="http2"' \
              --cfg 'feature="runtime"' \
              --cfg 'feature="socket2"' \
              --cfg 'feature="tcp"' \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("__internal_happy_eyeballs_tests", "backports", "client", "default", "deprecated", "ffi", "full", "h2", "http1", "http2", "libc", "nightly", "runtime", "server", "socket2", "stream", "tcp"))' \
              -C metadata=8983126635e0bfa9 \
              -C extra-filename=-78cc5e777f9503a3 \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern bytes=${bytes-1_10_0-64a68773187e68cf}/libbytes-64a68773187e68cf.rmeta \
              --extern futures_channel=${futures-channel-0_3_31-c19bd6ab8876ed10}/libfutures_channel-c19bd6ab8876ed10.rmeta \
              --extern futures_core=${futures-core-0_3_31-f6a43156147538e7}/libfutures_core-f6a43156147538e7.rmeta \
              --extern futures_util=${futures-util-0_3_31-ddd4b710c5171915}/libfutures_util-ddd4b710c5171915.rmeta \
              --extern h2=${h2-0_3_26-6ae3cdc03d4408e3}/libh2-6ae3cdc03d4408e3.rmeta \
              --extern http=${http-0_2_12-83e90f39f49f8f94}/libhttp-83e90f39f49f8f94.rmeta \
              --extern http_body=${http-body-0_4_6-604b1fb4509892f9}/libhttp_body-604b1fb4509892f9.rmeta \
              --extern httparse=${httparse-1_10_0-6ca9fa58eef50542}/libhttparse-6ca9fa58eef50542.rmeta \
              --extern httpdate=${httpdate-1_0_3-0986d952b4b79a01}/libhttpdate-0986d952b4b79a01.rmeta \
              --extern itoa=${itoa-1_0_14-6ae7bc765ab6d9fa}/libitoa-6ae7bc765ab6d9fa.rmeta \
              --extern pin_project_lite=${pin-project-lite-0_2_16-dbb7ad05ebc1034f}/libpin_project_lite-dbb7ad05ebc1034f.rmeta \
              --extern socket2=${socket2-0_5_8-b832ef87df39fa74}/libsocket2-b832ef87df39fa74.rmeta \
              --extern tokio=${tokio-1_43_0-1ce995a0fb66b2ee}/libtokio-1ce995a0fb66b2ee.rmeta \
              --extern tower_service=${tower-service-0_3_3-f2bb19b3a855b555}/libtower_service-f2bb19b3a855b555.rmeta \
              --extern tracing=${tracing-0_1_41-4e706e46b3a66672}/libtracing-4e706e46b3a66672.rmeta \
              --extern want=${want-0_3_1-65d33fb3976c0b7a}/libwant-65d33fb3976c0b7a.rmeta \
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
