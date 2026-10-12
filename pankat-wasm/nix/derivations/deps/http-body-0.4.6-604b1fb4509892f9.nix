# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "http-body-0_4_6-604b1fb4509892f9";
    meta.cargo_crate_info = {
      name = "http-body";
      version = "0.4.6";
      crate_hash = "604b1fb4509892f9";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [bytes-1_10_0-64a68773187e68cf http-0_2_12-83e90f39f49f8f94 pin-project-lite-0_2_16-dbb7ad05ebc1034f];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/http-body/0.4.6/download";
      sha256 = "7ceab25649e9960c0311ea418d17bee82c0dcec1bd053b5f9a66e265a693bed2";
    };

    unpackPhase = ''
      tar xf $src
      cd http-body-0.4.6
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "http_body";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "Carl Lerche <me@carllerche.com>:Lucio Franco <luciofranco14@gmail.com>:Sean McArthur <sean@seanmonstar.com>";
    CARGO_PKG_DESCRIPTION = "Trait representing an asynchronous, streaming, HTTP request or response body.";
    CARGO_PKG_HOMEPAGE = "";
    CARGO_PKG_LICENSE = "MIT";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "http-body";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/hyperium/http-body";
    CARGO_PKG_RUST_VERSION = "";
    CARGO_PKG_VERSION = "0.4.6";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "4";
    CARGO_PKG_VERSION_PATCH = "6";
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
              --crate-name http_body \
              --edition=2018 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values())' \
              -C metadata=5e9280b9269a568c \
              -C extra-filename=-604b1fb4509892f9 \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern bytes=${bytes-1_10_0-64a68773187e68cf}/libbytes-64a68773187e68cf.rmeta \
              --extern http=${http-0_2_12-83e90f39f49f8f94}/libhttp-83e90f39f49f8f94.rmeta \
              --extern pin_project_lite=${pin-project-lite-0_2_16-dbb7ad05ebc1034f}/libpin_project_lite-dbb7ad05ebc1034f.rmeta \
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
