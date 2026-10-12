# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "openssl-0_10_71-f908109d8e630048";
    meta.cargo_crate_info = {
      name = "openssl";
      version = "0.10.71";
      crate_hash = "f908109d8e630048";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [bitflags-2_8_0-3292edd04786f4ea cfg-if-1_0_0-616d46354eebf850 foreign-types-0_3_2-97dc570000d8ad39 libc-0_2_170-00cb1c0a8601ad49 once_cell-1_20_3-60992a3834e62ae0 openssl-macros-0_1_1-bb87a3ae5c01794f openssl-sys-0_9_106-833fd3e71ff3fad0];
    passthru.rust_crate_parent = [openssl-0_10_71-script_build_run-90ea5b99940e3374];
    passthru.rust_script_build_run = [openssl-0_10_71-script_build_run-90ea5b99940e3374];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/openssl/0.10.71/download";
      sha256 = "5e14130c6a98cd258fdcb0fb6d744152343ff729cbfcb28c656a9d12b999fbcd";
    };

    unpackPhase = ''
      tar xf $src
      cd openssl-0.10.71
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "openssl";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "Steven Fackler <sfackler@gmail.com>";
    CARGO_PKG_DESCRIPTION = "OpenSSL bindings";
    CARGO_PKG_HOMEPAGE = "";
    CARGO_PKG_LICENSE = "Apache-2.0";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "openssl";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/sfackler/rust-openssl";
    CARGO_PKG_RUST_VERSION = "1.63.0";
    CARGO_PKG_VERSION = "0.10.71";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "10";
    CARGO_PKG_VERSION_PATCH = "71";
    CARGO_PKG_VERSION_PRE = "";

    buildPhase = ''
      ${fn.import_bash_function_helpers}
      export CARGO_MANIFEST_DIR=$(realpath $PWD/$CARGO_MANIFEST_DIR)
      export CARGO_MANIFEST_PATH=$(realpath $PWD/$CARGO_MANIFEST_PATH)
      
      mkdir -p $out/nix
      export OUT_DIR=$out

      print_compiling_message "${name}"
      print_cargo_message_type_0 "${name}" "${meta.cargo_crate_info.name}" "${meta.cargo_crate_info.type}"
      copy_build_script_run_results_over_with_nix "${fn.get_rust_crate_parent passthru.rust_crate_parent}"
      for file in $out/environment-variables $out/rustc-arguments $out/rustc-propagated-arguments; do
          if [ -f "$file" ]; then
          sed -i "s|${fn.get_rust_crate_parent passthru.rust_crate_parent}|$out|g" "$file"
          fi
      done
      load_environment_variables_from_files "${fn.environment_variables passthru.rust_script_build_run}"
      rustc_json_output_lines=$(${pkgs.mktemp}/bin/mktemp)
      set -x +e
      ${RUSTC} \
              --crate-name openssl \
              --edition=2021 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --cfg 'feature="default"' \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("bindgen", "default", "unstable_boringssl", "v101", "v102", "v110", "v111", "vendored"))' \
              -C metadata=10c0080f549d29b8 \
              -C extra-filename=-f908109d8e630048 \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern bitflags=${bitflags-2_8_0-3292edd04786f4ea}/libbitflags-3292edd04786f4ea.rmeta \
              --extern cfg_if=${cfg-if-1_0_0-616d46354eebf850}/libcfg_if-616d46354eebf850.rmeta \
              --extern foreign_types=${foreign-types-0_3_2-97dc570000d8ad39}/libforeign_types-97dc570000d8ad39.rmeta \
              --extern libc=${libc-0_2_170-00cb1c0a8601ad49}/liblibc-00cb1c0a8601ad49.rmeta \
              --extern once_cell=${once_cell-1_20_3-60992a3834e62ae0}/libonce_cell-60992a3834e62ae0.rmeta \
              --extern openssl_macros=${openssl-macros-0_1_1-bb87a3ae5c01794f}/libopenssl_macros-bb87a3ae5c01794f.so \
              --extern ffi=${openssl-sys-0_9_106-833fd3e71ff3fad0}/libopenssl_sys-833fd3e71ff3fad0.rmeta \
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
