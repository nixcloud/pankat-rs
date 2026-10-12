# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "pin-project-internal-1_1_9-69f489b3df026b03";
    meta.cargo_crate_info = {
      name = "pin-project-internal";
      version = "1.1.9";
      crate_hash = "69f489b3df026b03";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [proc-macro2-1_0_93-cfe81a59cf98819f quote-1_0_38-12b99e3192e30e82 syn-2_0_98-93aa0f13dad61a07];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/pin-project-internal/1.1.9/download";
      sha256 = "f6e859e6e5bd50440ab63c47e3ebabc90f26251f7c73c3d3e837b74a1cc3fa67";
    };

    unpackPhase = ''
      tar xf $src
      cd pin-project-internal-1.1.9
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "pin_project_internal";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "";
    CARGO_PKG_DESCRIPTION = "Implementation detail of the `pin-project` crate.";
    CARGO_PKG_HOMEPAGE = "";
    CARGO_PKG_LICENSE = "Apache-2.0 OR MIT";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "pin-project-internal";
    CARGO_PKG_README = "";
    CARGO_PKG_REPOSITORY = "https://github.com/taiki-e/pin-project";
    CARGO_PKG_RUST_VERSION = "1.56";
    CARGO_PKG_VERSION = "1.1.9";
    CARGO_PKG_VERSION_MAJOR = "1";
    CARGO_PKG_VERSION_MINOR = "1";
    CARGO_PKG_VERSION_PATCH = "9";
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
              --crate-name pin_project_internal \
              --edition=2021 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type proc-macro \
              --emit=dep-info,link \
              -C prefer-dynamic \
              -C embed-bitcode=no \
              --warn=clippy::unused_trait_names \
              --warn=unreachable_pub \
              --warn=unnameable_types \
              --warn=unexpected_cfgs \
              --warn=clippy::undocumented_unsafe_blocks \
              --warn=clippy::transmute_undefined_repr \
              --warn=clippy::trailing_empty_array \
              --warn=single_use_lifetimes \
              --warn=rust_2018_idioms \
              --warn=clippy::pedantic \
              --warn=non_ascii_idents \
              --warn=clippy::inline_asm_x86_att_syntax \
              --warn=improper_ctypes_definitions \
              --warn=improper_ctypes \
              --warn=deprecated_safe \
              --warn=clippy::default_union_representation \
              --warn=clippy::as_underscore \
              --warn=clippy::as_ptr_cast_mut \
              --warn=clippy::all \
              --allow=clippy::unreadable_literal \
              --allow=clippy::type_complexity \
              --allow=clippy::too_many_lines \
              --allow=clippy::too_many_arguments \
              --allow=clippy::struct_field_names \
              --allow=clippy::struct_excessive_bools \
              --allow=clippy::single_match_else \
              --allow=clippy::single_match \
              --allow=clippy::similar_names \
              --allow=clippy::range_plus_one \
              --allow=clippy::nonminimal_bool \
              --allow=clippy::naive_bytecount \
              --allow=clippy::module_name_repetitions \
              --allow=clippy::missing_errors_doc \
              --allow=clippy::manual_range_contains \
              --allow=clippy::manual_assert \
              --allow=clippy::lint_groups_priority \
              --allow=clippy::incompatible_msrv \
              --allow=clippy::float_cmp \
              --allow=clippy::doc_markdown \
              --allow=clippy::declare_interior_mutable_const \
              --allow=clippy::cast_lossless \
              --allow=clippy::borrow_as_ptr \
              --allow=clippy::bool_assert_comparison \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values())' \
              -C metadata=1dd5f96b5d225724 \
              -C extra-filename=-69f489b3df026b03 \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern proc_macro2=${proc-macro2-1_0_93-cfe81a59cf98819f}/libproc_macro2-cfe81a59cf98819f.rlib \
              --extern quote=${quote-1_0_38-12b99e3192e30e82}/libquote-12b99e3192e30e82.rlib \
              --extern syn=${syn-2_0_98-93aa0f13dad61a07}/libsyn-93aa0f13dad61a07.rlib \
              --extern proc_macro \
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
