# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "hyper-tls-0_5_0-0ce46cc06ae98dd5";
    meta.cargo_crate_info = {
      name = "hyper-tls";
      version = "0.5.0";
      crate_hash = "0ce46cc06ae98dd5";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [bytes-1_10_0-64a68773187e68cf hyper-0_14_32-78cc5e777f9503a3 native-tls-0_2_14-cbf881b3f45228cd tokio-1_43_0-1ce995a0fb66b2ee tokio-native-tls-0_3_1-35c9a5a3ee237cd6];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/hyper-tls/0.5.0/download";
      sha256 = "d6183ddfa99b85da61a140bea0efc93fdf56ceaa041b37d553518030827f9905";
    };

    unpackPhase = ''
      tar xf $src
      cd hyper-tls-0.5.0
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "hyper_tls";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "Sean McArthur <sean@seanmonstar.com>";
    CARGO_PKG_DESCRIPTION = "Default TLS implementation for use with hyper";
    CARGO_PKG_HOMEPAGE = "https://hyper.rs";
    CARGO_PKG_LICENSE = "MIT/Apache-2.0";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "hyper-tls";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/hyperium/hyper-tls";
    CARGO_PKG_RUST_VERSION = "";
    CARGO_PKG_VERSION = "0.5.0";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "5";
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
              --crate-name hyper_tls \
              --edition=2018 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("vendored"))' \
              -C metadata=0dd7ebe05ccd7cd2 \
              -C extra-filename=-0ce46cc06ae98dd5 \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern bytes=${bytes-1_10_0-64a68773187e68cf}/libbytes-64a68773187e68cf.rmeta \
              --extern hyper=${hyper-0_14_32-78cc5e777f9503a3}/libhyper-78cc5e777f9503a3.rmeta \
              --extern native_tls=${native-tls-0_2_14-cbf881b3f45228cd}/libnative_tls-cbf881b3f45228cd.rmeta \
              --extern tokio=${tokio-1_43_0-1ce995a0fb66b2ee}/libtokio-1ce995a0fb66b2ee.rmeta \
              --extern tokio_native_tls=${tokio-native-tls-0_3_1-35c9a5a3ee237cd6}/libtokio_native_tls-35c9a5a3ee237cd6.rmeta \
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
