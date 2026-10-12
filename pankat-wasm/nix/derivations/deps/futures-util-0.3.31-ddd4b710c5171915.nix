# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "futures-util-0_3_31-ddd4b710c5171915";
    meta.cargo_crate_info = {
      name = "futures-util";
      version = "0.3.31";
      crate_hash = "ddd4b710c5171915";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [futures-channel-0_3_31-c19bd6ab8876ed10 futures-core-0_3_31-f6a43156147538e7 futures-io-0_3_31-5ba7d6bdea5b3277 futures-macro-0_3_31-30147c265a9b2972 futures-sink-0_3_31-120d6b5c0eab5386 futures-task-0_3_31-4256e2b27fdd8ebe memchr-2_7_4-3cee6db17bbe0dde pin-project-lite-0_2_16-dbb7ad05ebc1034f pin-utils-0_1_0-25df2e9bf3ca1c40 slab-0_4_9-703e5769016427df];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/futures-util/0.3.31/download";
      sha256 = "9fa08315bb612088cc391249efdc3bc77536f16c91f6cf495e6fbe85b20a4a81";
    };

    unpackPhase = ''
      tar xf $src
      cd futures-util-0.3.31
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "futures_util";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "";
    CARGO_PKG_DESCRIPTION = "Common utilities and extension traits for the futures-rs library.";
    CARGO_PKG_HOMEPAGE = "https://rust-lang.github.io/futures-rs";
    CARGO_PKG_LICENSE = "MIT OR Apache-2.0";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "futures-util";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/rust-lang/futures-rs";
    CARGO_PKG_RUST_VERSION = "1.56";
    CARGO_PKG_VERSION = "0.3.31";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "3";
    CARGO_PKG_VERSION_PATCH = "31";
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
              --crate-name futures_util \
              --edition=2018 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              --warn=unreachable_pub \
              --warn=unexpected_cfgs \
              --warn=single_use_lifetimes \
              --warn=rust_2018_idioms \
              --warn=missing_debug_implementations \
              --check-cfg 'cfg(futures_sanitizer)' \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --cfg 'feature="alloc"' \
              --cfg 'feature="async-await"' \
              --cfg 'feature="async-await-macro"' \
              --cfg 'feature="channel"' \
              --cfg 'feature="futures-channel"' \
              --cfg 'feature="futures-io"' \
              --cfg 'feature="futures-macro"' \
              --cfg 'feature="futures-sink"' \
              --cfg 'feature="io"' \
              --cfg 'feature="memchr"' \
              --cfg 'feature="sink"' \
              --cfg 'feature="slab"' \
              --cfg 'feature="std"' \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("alloc", "async-await", "async-await-macro", "bilock", "cfg-target-has-atomic", "channel", "compat", "default", "futures-channel", "futures-io", "futures-macro", "futures-sink", "futures_01", "io", "io-compat", "memchr", "portable-atomic", "sink", "slab", "std", "tokio-io", "unstable", "write-all-vectored"))' \
              -C metadata=9a56693c34e9ee7c \
              -C extra-filename=-ddd4b710c5171915 \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern futures_channel=${futures-channel-0_3_31-c19bd6ab8876ed10}/libfutures_channel-c19bd6ab8876ed10.rmeta \
              --extern futures_core=${futures-core-0_3_31-f6a43156147538e7}/libfutures_core-f6a43156147538e7.rmeta \
              --extern futures_io=${futures-io-0_3_31-5ba7d6bdea5b3277}/libfutures_io-5ba7d6bdea5b3277.rmeta \
              --extern futures_macro=${futures-macro-0_3_31-30147c265a9b2972}/libfutures_macro-30147c265a9b2972.so \
              --extern futures_sink=${futures-sink-0_3_31-120d6b5c0eab5386}/libfutures_sink-120d6b5c0eab5386.rmeta \
              --extern futures_task=${futures-task-0_3_31-4256e2b27fdd8ebe}/libfutures_task-4256e2b27fdd8ebe.rmeta \
              --extern memchr=${memchr-2_7_4-3cee6db17bbe0dde}/libmemchr-3cee6db17bbe0dde.rmeta \
              --extern pin_project_lite=${pin-project-lite-0_2_16-dbb7ad05ebc1034f}/libpin_project_lite-dbb7ad05ebc1034f.rmeta \
              --extern pin_utils=${pin-utils-0_1_0-25df2e9bf3ca1c40}/libpin_utils-25df2e9bf3ca1c40.rmeta \
              --extern slab=${slab-0_4_9-703e5769016427df}/libslab-703e5769016427df.rmeta \
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
