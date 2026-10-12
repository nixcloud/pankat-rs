# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "futures-0_3_30-e4b5d46ddca185d9";
    meta.cargo_crate_info = {
      name = "futures";
      version = "0.3.30";
      crate_hash = "e4b5d46ddca185d9";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [futures-channel-0_3_31-c19bd6ab8876ed10 futures-core-0_3_31-f6a43156147538e7 futures-executor-0_3_31-510e5ef294b2bbd4 futures-io-0_3_31-5ba7d6bdea5b3277 futures-sink-0_3_31-120d6b5c0eab5386 futures-task-0_3_31-4256e2b27fdd8ebe futures-util-0_3_31-ddd4b710c5171915];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/futures/0.3.30/download";
      sha256 = "645c6916888f6cb6350d2550b80fb63e734897a8498abe35cfb732b6487804b0";
    };

    unpackPhase = ''
      tar xf $src
      cd futures-0.3.30
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "futures";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "";
    CARGO_PKG_DESCRIPTION = "An implementation of futures and streams featuring zero allocations,
composability, and iterator-like interfaces.";
    CARGO_PKG_HOMEPAGE = "https://rust-lang.github.io/futures-rs";
    CARGO_PKG_LICENSE = "MIT OR Apache-2.0";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "futures";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/rust-lang/futures-rs";
    CARGO_PKG_RUST_VERSION = "1.56";
    CARGO_PKG_VERSION = "0.3.30";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "3";
    CARGO_PKG_VERSION_PATCH = "30";
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
              --crate-name futures \
              --edition=2018 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --cfg 'feature="alloc"' \
              --cfg 'feature="async-await"' \
              --cfg 'feature="default"' \
              --cfg 'feature="executor"' \
              --cfg 'feature="futures-executor"' \
              --cfg 'feature="std"' \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("alloc", "async-await", "bilock", "cfg-target-has-atomic", "compat", "default", "executor", "futures-executor", "io-compat", "std", "thread-pool", "unstable", "write-all-vectored"))' \
              -C metadata=1f2ca57cb3665823 \
              -C extra-filename=-e4b5d46ddca185d9 \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern futures_channel=${futures-channel-0_3_31-c19bd6ab8876ed10}/libfutures_channel-c19bd6ab8876ed10.rmeta \
              --extern futures_core=${futures-core-0_3_31-f6a43156147538e7}/libfutures_core-f6a43156147538e7.rmeta \
              --extern futures_executor=${futures-executor-0_3_31-510e5ef294b2bbd4}/libfutures_executor-510e5ef294b2bbd4.rmeta \
              --extern futures_io=${futures-io-0_3_31-5ba7d6bdea5b3277}/libfutures_io-5ba7d6bdea5b3277.rmeta \
              --extern futures_sink=${futures-sink-0_3_31-120d6b5c0eab5386}/libfutures_sink-120d6b5c0eab5386.rmeta \
              --extern futures_task=${futures-task-0_3_31-4256e2b27fdd8ebe}/libfutures_task-4256e2b27fdd8ebe.rmeta \
              --extern futures_util=${futures-util-0_3_31-ddd4b710c5171915}/libfutures_util-ddd4b710c5171915.rmeta \
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
