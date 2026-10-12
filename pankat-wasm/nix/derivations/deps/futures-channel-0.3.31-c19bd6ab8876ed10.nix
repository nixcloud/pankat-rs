# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "futures-channel-0_3_31-c19bd6ab8876ed10";
    meta.cargo_crate_info = {
      name = "futures-channel";
      version = "0.3.31";
      crate_hash = "c19bd6ab8876ed10";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [futures-core-0_3_31-f6a43156147538e7 futures-sink-0_3_31-120d6b5c0eab5386];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/futures-channel/0.3.31/download";
      sha256 = "2dff15bf788c671c1934e366d07e30c1814a8ef514e1af724a602e8a2fbe1b10";
    };

    unpackPhase = ''
      tar xf $src
      cd futures-channel-0.3.31
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "futures_channel";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "";
    CARGO_PKG_DESCRIPTION = "Channels for asynchronous communication using futures-rs.";
    CARGO_PKG_HOMEPAGE = "https://rust-lang.github.io/futures-rs";
    CARGO_PKG_LICENSE = "MIT OR Apache-2.0";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "futures-channel";
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
              --crate-name futures_channel \
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
              --cfg 'feature="default"' \
              --cfg 'feature="futures-sink"' \
              --cfg 'feature="sink"' \
              --cfg 'feature="std"' \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("alloc", "cfg-target-has-atomic", "default", "futures-sink", "sink", "std", "unstable"))' \
              -C metadata=4fa17d030ea0f5ce \
              -C extra-filename=-c19bd6ab8876ed10 \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern futures_core=${futures-core-0_3_31-f6a43156147538e7}/libfutures_core-f6a43156147538e7.rmeta \
              --extern futures_sink=${futures-sink-0_3_31-120d6b5c0eab5386}/libfutures_sink-120d6b5c0eab5386.rmeta \
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
