# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "serde_urlencoded-0_7_1-2d4ded72aa609d7b";
    meta.cargo_crate_info = {
      name = "serde_urlencoded";
      version = "0.7.1";
      crate_hash = "2d4ded72aa609d7b";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [form_urlencoded-1_2_1-ce91ddb5b964d841 itoa-1_0_14-6ae7bc765ab6d9fa ryu-1_0_19-e43e7d32117cb8d6 serde-1_0_218-eb19f02d9fcfde0c];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/serde_urlencoded/0.7.1/download";
      sha256 = "d3491c14715ca2294c4d6a88f15e84739788c1d030eed8c110436aafdaa2f3fd";
    };

    unpackPhase = ''
      tar xf $src
      cd serde_urlencoded-0.7.1
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "serde_urlencoded";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "Anthony Ramine <n.oxyde@gmail.com>";
    CARGO_PKG_DESCRIPTION = "`x-www-form-urlencoded` meets Serde";
    CARGO_PKG_HOMEPAGE = "";
    CARGO_PKG_LICENSE = "MIT/Apache-2.0";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "serde_urlencoded";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/nox/serde_urlencoded";
    CARGO_PKG_RUST_VERSION = "";
    CARGO_PKG_VERSION = "0.7.1";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "7";
    CARGO_PKG_VERSION_PATCH = "1";
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
              --crate-name serde_urlencoded \
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
              -C metadata=e1ddb26048d44d77 \
              -C extra-filename=-2d4ded72aa609d7b \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern form_urlencoded=${form_urlencoded-1_2_1-ce91ddb5b964d841}/libform_urlencoded-ce91ddb5b964d841.rmeta \
              --extern itoa=${itoa-1_0_14-6ae7bc765ab6d9fa}/libitoa-6ae7bc765ab6d9fa.rmeta \
              --extern ryu=${ryu-1_0_19-e43e7d32117cb8d6}/libryu-e43e7d32117cb8d6.rmeta \
              --extern serde=${serde-1_0_218-eb19f02d9fcfde0c}/libserde-eb19f02d9fcfde0c.rmeta \
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
