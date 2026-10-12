# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "tokio-1_43_0-1ce995a0fb66b2ee";
    meta.cargo_crate_info = {
      name = "tokio";
      version = "1.43.0";
      crate_hash = "1ce995a0fb66b2ee";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [bytes-1_10_0-64a68773187e68cf libc-0_2_170-00cb1c0a8601ad49 mio-1_0_3-8d8e48f08bd95b08 pin-project-lite-0_2_16-dbb7ad05ebc1034f socket2-0_5_8-b832ef87df39fa74 tokio-macros-2_5_0-e9096740d4cd33e0];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/tokio/1.43.0/download";
      sha256 = "3d61fa4ffa3de412bfea335c6ecff681de2b609ba3c77ef3e00e521813a9ed9e";
    };

    unpackPhase = ''
      tar xf $src
      cd tokio-1.43.0
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "tokio";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "Tokio Contributors <team@tokio.rs>";
    CARGO_PKG_DESCRIPTION = "An event-driven, non-blocking I/O platform for writing asynchronous I/O
backed applications.";
    CARGO_PKG_HOMEPAGE = "https://tokio.rs";
    CARGO_PKG_LICENSE = "MIT";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "tokio";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/tokio-rs/tokio";
    CARGO_PKG_RUST_VERSION = "1.70";
    CARGO_PKG_VERSION = "1.43.0";
    CARGO_PKG_VERSION_MAJOR = "1";
    CARGO_PKG_VERSION_MINOR = "43";
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
              --crate-name tokio \
              --edition=2021 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --cfg 'feature="bytes"' \
              --cfg 'feature="default"' \
              --cfg 'feature="io-util"' \
              --cfg 'feature="libc"' \
              --cfg 'feature="macros"' \
              --cfg 'feature="mio"' \
              --cfg 'feature="net"' \
              --cfg 'feature="rt"' \
              --cfg 'feature="socket2"' \
              --cfg 'feature="sync"' \
              --cfg 'feature="time"' \
              --cfg 'feature="tokio-macros"' \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("bytes", "default", "fs", "full", "io-std", "io-util", "libc", "macros", "mio", "net", "parking_lot", "process", "rt", "rt-multi-thread", "signal", "signal-hook-registry", "socket2", "sync", "test-util", "time", "tokio-macros", "tracing", "windows-sys"))' \
              -C metadata=d7abe3a3a7e7ccfa \
              -C extra-filename=-1ce995a0fb66b2ee \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern bytes=${bytes-1_10_0-64a68773187e68cf}/libbytes-64a68773187e68cf.rmeta \
              --extern libc=${libc-0_2_170-00cb1c0a8601ad49}/liblibc-00cb1c0a8601ad49.rmeta \
              --extern mio=${mio-1_0_3-8d8e48f08bd95b08}/libmio-8d8e48f08bd95b08.rmeta \
              --extern pin_project_lite=${pin-project-lite-0_2_16-dbb7ad05ebc1034f}/libpin_project_lite-dbb7ad05ebc1034f.rmeta \
              --extern socket2=${socket2-0_5_8-b832ef87df39fa74}/libsocket2-b832ef87df39fa74.rmeta \
              --extern tokio_macros=${tokio-macros-2_5_0-e9096740d4cd33e0}/libtokio_macros-e9096740d4cd33e0.so \
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
