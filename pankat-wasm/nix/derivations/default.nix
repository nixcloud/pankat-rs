# generated from default.nix.handlebars using cargo (manual edits won't be persistent)
{ pkgs, lib ? pkgs.lib, rustc, cargo, external_crate_dependencies, project_root }:
let
  build_rs_libnix' = pkgs.callPackage ./build_rs_libnix.nix {
    inherit pkgs;
  };
  callPackage' = lib.callPackageWith (pkgs // lib // self // { inherit fn rustc cargo project_root; });
  fn = rec {
    build_rs_libnix = "${build_rs_libnix'}/bin/build-rs-libnix";
    allCollectedInputs = a: lib.unique (
      builtins.foldl' (acc: el: acc ++ [el] ++ (allCollectedInputs el.rust_crate_libraries)) [] a
    );
    rustc_linker_arguments = rust_crate_libraries: builtins.concatStringsSep " " (map (lib: "-L ${lib}") (allCollectedInputs rust_crate_libraries));
    rustc_linker_arguments_dir = rust_crate_libraries: 
      pkgs.stdenv.mkDerivation {
        name = "rustc-linker-arguments-dir";
        # anyhow-139173be5e005a44.d  libanyhow-139173be5e005a44.rlib  libanyhow-139173be5e005a44.rmeta  nix
        buildCommand = ''
          mkdir -p $out/deps
          for lib in ${builtins.concatStringsSep " " (map (lib: "${lib}") (allCollectedInputs rust_crate_libraries))}; do
            if [ -d "$lib" ]; then
              for f in "$lib"/*.rlib "$lib"/*.rmeta "$lib"/*.so; do
                if [ -e "$f" ]; then
                  ln -s "$f" "$out/deps/" || true
                fi
              done
            fi
          done
          echo ${builtins.concatStringsSep " " (map (lib: "-L ${lib}") (allCollectedInputs rust_crate_libraries))} > $out/rustc-linker-arguments
        '';
      };
    rustc_arguments = rust_crate_parent: assert lib.assertMsg (builtins.length rust_crate_parent <= 1) "passthru.rust_crate_parent must have 0 or 1 element at maxium" ;
      lib.replaceStrings ["\n"] [""] (builtins.concatStringsSep " " (map (crate:
        if builtins.pathExists "${crate}/nix/rustc-arguments" then
          builtins.readFile "${crate}/nix/rustc-arguments"
        else
          ""
        ) rust_crate_parent));
    rustc_propagated_arguments = rust_crate_libraries: lib.replaceStrings ["\n"] [""] (builtins.concatStringsSep " " (map (crate:
      if builtins.pathExists "${crate}/nix/rustc-propagated-arguments" then
        builtins.readFile "${crate}/nix/rustc-propagated-arguments"
      else
        ""
      ) (allCollectedInputs rust_crate_libraries)));
    environment_variables = rust_script_build_run: builtins.concatStringsSep " " (map (d: "${d}/nix/environment-variables") rust_script_build_run);
    get_rust_crate_parent = parent_list:
      assert lib.assertMsg (builtins.length parent_list == 1) "item is supposed to have exactly one parent, while it has 0 or more than 1";
      builtins.head parent_list;
    import_bash_function_helpers = ''
      print_compiling_message() {
        local fullname="$1"
        printf '\033[0;32mCompiling\033[0m %s\n' "$fullname"
      }

      # to indicate that a crate build inside a mkDerivation has started of type
      print_cargo_message_type_0() {
        local fullname="$1"         # "serde-1_21_3-bfbc21afc6e0b538"
        local name="$2"             # "serde" 
        local type="$3"             # "(lib)"
        echo "@cargo { \"type\":0, \"crate_name\":\"$name\", \"crate_type\":\"$type\", \"id\":\"$fullname\" }"
      }

      # to indicate that a mkDerivation has finished compiling a crate of type rustc call counterpart: finishes a type 0 message
      print_cargo_message_type_2() {
        local fullname="$1"         # "serde-1_21_3-bfbc21afc6e0b538"
        local name="$2"             # "serde"
        local type="$3"             # "(lib)"
        local rustc_exit_code="$4"
        local rustc_json_output_lines="$5"
        output=$(${pkgs.jq}/bin/jq -s -r -c \
          --arg fullname "$fullname" \
          --arg name "$name" \
          --arg type "$type" \
          --arg exit_code "$rustc_exit_code" \
          '{type: 2, crate_name: $name, crate_type: $type, id: $fullname, rustc_exit_code: ($exit_code|tonumber), rustc_messages: .}' \
          "$rustc_json_output_lines")
        printf '@cargo %s\n' "$output"
      }

      # "build.rs run" calls this
      print_cargo_message_type_3() {
        local name="$1"
        local type="$2"
        local notice="$3"
        local exit_code="$4"
        local build_script_build_output_lines="$5"
        output=$(${pkgs.jq}/bin/jq -c -n \
          --arg name "$name" \
          --arg type "$type" \
          --arg notice "$notice" \
          --arg exit_code "$exit_code" \
          --rawfile msg $build_script_build_output_lines \
          '{type: 3, crate_name: $name, crate_type: $type, exit_code: ($exit_code|tonumber), messages: [ $notice, $msg ] }')
        printf '@cargo %s\n' "$output"
      }

      # rustc outputs json, from that we extracted the .rendered part and print it to stdout 
      # (so we see useful messages during 'nix build')
      print_rustc_rendered_messages() {
        local rustc_json_output_lines="$1"
        while IFS= read -r line
        do
            tmpFile=$(${pkgs.mktemp}/bin/mktemp)
            echo "$line" > $tmpFile
            ${pkgs.jq}/bin/jq -r -c 'select(."$message_type"=="diagnostic") | .rendered' $tmpFile
        done < $rustc_json_output_lines
      }

      # set env variables from the passthru propagation, used for DEP_* and similar from build.rs
      load_environment_variables_from_files() {
        local files="$1"
        for file in $files; do
          if [ -f $file ]; then
            set -a
              while read -r line; do
                echo -e "\033[38;5;208m$line\033[0m"
              done < "$file"
              source $file
              set +a
          fi
        done
      }

      copy_build_script_run_results_over_with_nix() {
        local build_script_run_out_dir="$1"
        cp -r "$build_script_run_out_dir"/* $OUT_DIR
      }

      copy_build_script_run_results_over_without_nix() {
        local build_script_run_out_dir="$1"
        cp -r "$build_script_run_out_dir"/* $OUT_DIR
        rm -Rf $OUT_DIR/nix
      }
    '';
    inject_deps = cargo_crate_info:
      let
        deps = external_crate_dependencies.deps;
        name = cargo_crate_info.name;
        version = cargo_crate_info.version;
        crate_hash = cargo_crate_info.hash;
        entry = deps.${name} or null;
      in
        if builtins.isAttrs entry then
          let
            verEntry = entry.${version} or null;
          in
            if builtins.isAttrs verEntry then
              verEntry.${crate_hash} or (throw "No deps found for ${name} version ${version} hash ${crate_hash}")
            else if builtins.isList verEntry then
              verEntry
            else
              throw "No deps found for '${name}' version ${version}"
        else if builtins.isList entry then
          entry
        else
          [];
    inject_envs = cargo_crate_info:
      let
        envs = external_crate_dependencies.envs;
        name = cargo_crate_info.name;
        version = cargo_crate_info.version;
        crate_hash = cargo_crate_info.crate_hash;
        entry = envs.${name} or null;
      in
        if builtins.isAttrs entry then
          let
            verEntry = entry.${version} or null;
          in
            if builtins.isAttrs verEntry then
              if builtins.isAttrs verEntry then
                if verEntry ? "${crate_hash}" then verEntry.${crate_hash} else verEntry
              else
                {}
            else
              entry
        else
          {};
    relativeFileset = project_root: relPaths: lib.fileset.unions (map (p: project_root + "/${p}") relPaths);
  };
  self = {
    target = callPackage' ./target.nix {};
    pankat-wasm-0_1_0-c4a3d03613b2b446 = callPackage' ./pankat-wasm-0.1.0-c4a3d03613b2b446.nix { };
    deps = {
      autocfg-1_4_0-30cc6115a8e43b39 = callPackage' ./deps/autocfg-1.4.0-30cc6115a8e43b39.nix { };
      base64-0_21_7-9aa53967577f990a = callPackage' ./deps/base64-0.21.7-9aa53967577f990a.nix { };
      bitflags-2_8_0-3292edd04786f4ea = callPackage' ./deps/bitflags-2.8.0-3292edd04786f4ea.nix { };
      bumpalo-3_17_0-8957db2a71cbd636 = callPackage' ./deps/bumpalo-3.17.0-8957db2a71cbd636.nix { };
      bytes-1_10_0-64a68773187e68cf = callPackage' ./deps/bytes-1.10.0-64a68773187e68cf.nix { };
      cc-1_2_15-04a27c387b0d6562 = callPackage' ./deps/cc-1.2.15-04a27c387b0d6562.nix { };
      cfg-if-1_0_0-616d46354eebf850 = callPackage' ./deps/cfg-if-1.0.0-616d46354eebf850.nix { };
      console_log-1_0_0-72e8f99a2a6e7480 = callPackage' ./deps/console_log-1.0.0-72e8f99a2a6e7480.nix { };
      displaydoc-0_2_5-a3932aec884d58b9 = callPackage' ./deps/displaydoc-0.2.5-a3932aec884d58b9.nix { };
      encoding_rs-0_8_35-92f62b88e615318e = callPackage' ./deps/encoding_rs-0.8.35-92f62b88e615318e.nix { };
      equivalent-1_0_2-742118f59cddfc47 = callPackage' ./deps/equivalent-1.0.2-742118f59cddfc47.nix { };
      fnv-1_0_7-e86d9923dc926ea6 = callPackage' ./deps/fnv-1.0.7-e86d9923dc926ea6.nix { };
      foreign-types-0_3_2-97dc570000d8ad39 = callPackage' ./deps/foreign-types-0.3.2-97dc570000d8ad39.nix { };
      foreign-types-shared-0_1_1-cd57af38e80cf627 = callPackage' ./deps/foreign-types-shared-0.1.1-cd57af38e80cf627.nix { };
      form_urlencoded-1_2_1-ce91ddb5b964d841 = callPackage' ./deps/form_urlencoded-1.2.1-ce91ddb5b964d841.nix { };
      futures-0_3_30-e4b5d46ddca185d9 = callPackage' ./deps/futures-0.3.30-e4b5d46ddca185d9.nix { };
      futures-channel-0_3_31-c19bd6ab8876ed10 = callPackage' ./deps/futures-channel-0.3.31-c19bd6ab8876ed10.nix { };
      futures-core-0_3_31-f6a43156147538e7 = callPackage' ./deps/futures-core-0.3.31-f6a43156147538e7.nix { };
      futures-executor-0_3_31-510e5ef294b2bbd4 = callPackage' ./deps/futures-executor-0.3.31-510e5ef294b2bbd4.nix { };
      futures-io-0_3_31-5ba7d6bdea5b3277 = callPackage' ./deps/futures-io-0.3.31-5ba7d6bdea5b3277.nix { };
      futures-macro-0_3_31-30147c265a9b2972 = callPackage' ./deps/futures-macro-0.3.31-30147c265a9b2972.nix { };
      futures-sink-0_3_31-120d6b5c0eab5386 = callPackage' ./deps/futures-sink-0.3.31-120d6b5c0eab5386.nix { };
      futures-task-0_3_31-4256e2b27fdd8ebe = callPackage' ./deps/futures-task-0.3.31-4256e2b27fdd8ebe.nix { };
      futures-util-0_3_31-ddd4b710c5171915 = callPackage' ./deps/futures-util-0.3.31-ddd4b710c5171915.nix { };
      gloo-net-0_5_0-6ac8e7851b0f0d3b = callPackage' ./deps/gloo-net-0.5.0-6ac8e7851b0f0d3b.nix { };
      gloo-timers-0_3_0-d391f5f244d34fb3 = callPackage' ./deps/gloo-timers-0.3.0-d391f5f244d34fb3.nix { };
      gloo-utils-0_2_0-40da9fc51db2c995 = callPackage' ./deps/gloo-utils-0.2.0-40da9fc51db2c995.nix { };
      h2-0_3_26-6ae3cdc03d4408e3 = callPackage' ./deps/h2-0.3.26-6ae3cdc03d4408e3.nix { };
      hashbrown-0_15_2-472f190b0100fb8e = callPackage' ./deps/hashbrown-0.15.2-472f190b0100fb8e.nix { };
      http-0_2_12-83e90f39f49f8f94 = callPackage' ./deps/http-0.2.12-83e90f39f49f8f94.nix { };
      http-body-0_4_6-604b1fb4509892f9 = callPackage' ./deps/http-body-0.4.6-604b1fb4509892f9.nix { };
      httparse-1_10_0-6ca9fa58eef50542 = callPackage' ./deps/httparse-1.10.0-6ca9fa58eef50542.nix { };
      httparse-1_10_0-script_build-6ae0d85f4f9411c7 = callPackage' ./deps/httparse-1.10.0-script_build-6ae0d85f4f9411c7.nix { };
      httparse-1_10_0-script_build_run-fd184386b39acc0e = callPackage' ./deps/httparse-1.10.0-script_build_run-fd184386b39acc0e.nix { };
      httpdate-1_0_3-0986d952b4b79a01 = callPackage' ./deps/httpdate-1.0.3-0986d952b4b79a01.nix { };
      hyper-0_14_32-78cc5e777f9503a3 = callPackage' ./deps/hyper-0.14.32-78cc5e777f9503a3.nix { };
      hyper-tls-0_5_0-0ce46cc06ae98dd5 = callPackage' ./deps/hyper-tls-0.5.0-0ce46cc06ae98dd5.nix { };
      icu_collections-1_5_0-4d027eaae4cb9000 = callPackage' ./deps/icu_collections-1.5.0-4d027eaae4cb9000.nix { };
      icu_locid-1_5_0-294b75cdeaebd083 = callPackage' ./deps/icu_locid-1.5.0-294b75cdeaebd083.nix { };
      icu_locid_transform-1_5_0-291edb4addc628ca = callPackage' ./deps/icu_locid_transform-1.5.0-291edb4addc628ca.nix { };
      icu_locid_transform_data-1_5_0-13b6af6aebf3cbc0 = callPackage' ./deps/icu_locid_transform_data-1.5.0-13b6af6aebf3cbc0.nix { };
      icu_normalizer-1_5_0-c59449698a6720d0 = callPackage' ./deps/icu_normalizer-1.5.0-c59449698a6720d0.nix { };
      icu_normalizer_data-1_5_0-d2c229093da62481 = callPackage' ./deps/icu_normalizer_data-1.5.0-d2c229093da62481.nix { };
      icu_properties-1_5_1-36033132eb86e488 = callPackage' ./deps/icu_properties-1.5.1-36033132eb86e488.nix { };
      icu_properties_data-1_5_0-4f08bfd4250192ce = callPackage' ./deps/icu_properties_data-1.5.0-4f08bfd4250192ce.nix { };
      icu_provider-1_5_0-df823559bbe57634 = callPackage' ./deps/icu_provider-1.5.0-df823559bbe57634.nix { };
      icu_provider_macros-1_5_0-4a62175d1a529a39 = callPackage' ./deps/icu_provider_macros-1.5.0-4a62175d1a529a39.nix { };
      idna-1_0_3-052b40b7203eb6a2 = callPackage' ./deps/idna-1.0.3-052b40b7203eb6a2.nix { };
      idna_adapter-1_2_0-38922f4c446972e8 = callPackage' ./deps/idna_adapter-1.2.0-38922f4c446972e8.nix { };
      indexmap-2_7_1-e565cbeccfc36840 = callPackage' ./deps/indexmap-2.7.1-e565cbeccfc36840.nix { };
      ipnet-2_11_0-2aac75a0fbf913b7 = callPackage' ./deps/ipnet-2.11.0-2aac75a0fbf913b7.nix { };
      itoa-1_0_14-6ae7bc765ab6d9fa = callPackage' ./deps/itoa-1.0.14-6ae7bc765ab6d9fa.nix { };
      js-sys-0_3_77-d34ceb3da0dda202 = callPackage' ./deps/js-sys-0.3.77-d34ceb3da0dda202.nix { };
      lazy_static-1_5_0-ffebef2c0e161fa7 = callPackage' ./deps/lazy_static-1.5.0-ffebef2c0e161fa7.nix { };
      libc-0_2_170-00cb1c0a8601ad49 = callPackage' ./deps/libc-0.2.170-00cb1c0a8601ad49.nix { };
      libc-0_2_170-script_build-b36be536ffa06313 = callPackage' ./deps/libc-0.2.170-script_build-b36be536ffa06313.nix { };
      libc-0_2_170-script_build_run-894aecc637a6f18a = callPackage' ./deps/libc-0.2.170-script_build_run-894aecc637a6f18a.nix { };
      litemap-0_7_4-1f4d69d5dc04ca70 = callPackage' ./deps/litemap-0.7.4-1f4d69d5dc04ca70.nix { };
      log-0_4_26-d46dd80a9e51535a = callPackage' ./deps/log-0.4.26-d46dd80a9e51535a.nix { };
      memchr-2_7_4-3cee6db17bbe0dde = callPackage' ./deps/memchr-2.7.4-3cee6db17bbe0dde.nix { };
      mime-0_3_17-19dbee6f0f8f1382 = callPackage' ./deps/mime-0.3.17-19dbee6f0f8f1382.nix { };
      mio-1_0_3-8d8e48f08bd95b08 = callPackage' ./deps/mio-1.0.3-8d8e48f08bd95b08.nix { };
      native-tls-0_2_14-cbf881b3f45228cd = callPackage' ./deps/native-tls-0.2.14-cbf881b3f45228cd.nix { };
      native-tls-0_2_14-script_build-dddcfe18f672e5af = callPackage' ./deps/native-tls-0.2.14-script_build-dddcfe18f672e5af.nix { };
      native-tls-0_2_14-script_build_run-883158839f9643b4 = callPackage' ./deps/native-tls-0.2.14-script_build_run-883158839f9643b4.nix { };
      once_cell-1_20_3-60992a3834e62ae0 = callPackage' ./deps/once_cell-1.20.3-60992a3834e62ae0.nix { };
      openssl-0_10_71-f908109d8e630048 = callPackage' ./deps/openssl-0.10.71-f908109d8e630048.nix { };
      openssl-0_10_71-script_build-97f35a2abcfa1e3b = callPackage' ./deps/openssl-0.10.71-script_build-97f35a2abcfa1e3b.nix { };
      openssl-0_10_71-script_build_run-90ea5b99940e3374 = callPackage' ./deps/openssl-0.10.71-script_build_run-90ea5b99940e3374.nix { };
      openssl-macros-0_1_1-bb87a3ae5c01794f = callPackage' ./deps/openssl-macros-0.1.1-bb87a3ae5c01794f.nix { };
      openssl-probe-0_1_6-6d5d5ac82de655f5 = callPackage' ./deps/openssl-probe-0.1.6-6d5d5ac82de655f5.nix { };
      openssl-sys-0_9_106-833fd3e71ff3fad0 = callPackage' ./deps/openssl-sys-0.9.106-833fd3e71ff3fad0.nix { };
      openssl-sys-0_9_106-script_build-50940dff65501832 = callPackage' ./deps/openssl-sys-0.9.106-script_build-50940dff65501832.nix { };
      openssl-sys-0_9_106-script_build_run-eec0b9a918121863 = callPackage' ./deps/openssl-sys-0.9.106-script_build_run-eec0b9a918121863.nix { };
      percent-encoding-2_3_1-e8b9e34a5db857ef = callPackage' ./deps/percent-encoding-2.3.1-e8b9e34a5db857ef.nix { };
      pin-project-1_1_9-91cb6bfb138aa153 = callPackage' ./deps/pin-project-1.1.9-91cb6bfb138aa153.nix { };
      pin-project-internal-1_1_9-69f489b3df026b03 = callPackage' ./deps/pin-project-internal-1.1.9-69f489b3df026b03.nix { };
      pin-project-lite-0_2_16-dbb7ad05ebc1034f = callPackage' ./deps/pin-project-lite-0.2.16-dbb7ad05ebc1034f.nix { };
      pin-utils-0_1_0-25df2e9bf3ca1c40 = callPackage' ./deps/pin-utils-0.1.0-25df2e9bf3ca1c40.nix { };
      pkg-config-0_3_31-9f951027c189d1c8 = callPackage' ./deps/pkg-config-0.3.31-9f951027c189d1c8.nix { };
      proc-macro2-1_0_93-cfe81a59cf98819f = callPackage' ./deps/proc-macro2-1.0.93-cfe81a59cf98819f.nix { };
      proc-macro2-1_0_93-script_build-44658edbe8e1d4ac = callPackage' ./deps/proc-macro2-1.0.93-script_build-44658edbe8e1d4ac.nix { };
      proc-macro2-1_0_93-script_build_run-a36de409238acb5c = callPackage' ./deps/proc-macro2-1.0.93-script_build_run-a36de409238acb5c.nix { };
      quote-1_0_38-12b99e3192e30e82 = callPackage' ./deps/quote-1.0.38-12b99e3192e30e82.nix { };
      reqwest-0_11_27-e0792ed017152126 = callPackage' ./deps/reqwest-0.11.27-e0792ed017152126.nix { };
      rustls-pemfile-1_0_4-8daa84e95740c37e = callPackage' ./deps/rustls-pemfile-1.0.4-8daa84e95740c37e.nix { };
      rustversion-1_0_19-44471f16741f7333 = callPackage' ./deps/rustversion-1.0.19-44471f16741f7333.nix { };
      rustversion-1_0_19-script_build-ead4695dbebdc35b = callPackage' ./deps/rustversion-1.0.19-script_build-ead4695dbebdc35b.nix { };
      rustversion-1_0_19-script_build_run-b22133fe627e2be1 = callPackage' ./deps/rustversion-1.0.19-script_build_run-b22133fe627e2be1.nix { };
      ryu-1_0_19-e43e7d32117cb8d6 = callPackage' ./deps/ryu-1.0.19-e43e7d32117cb8d6.nix { };
      serde-1_0_218-eb19f02d9fcfde0c = callPackage' ./deps/serde-1.0.218-eb19f02d9fcfde0c.nix { };
      serde-1_0_218-script_build-a589e6ac3b3b6f6e = callPackage' ./deps/serde-1.0.218-script_build-a589e6ac3b3b6f6e.nix { };
      serde-1_0_218-script_build_run-e4c9f613c8361860 = callPackage' ./deps/serde-1.0.218-script_build_run-e4c9f613c8361860.nix { };
      serde_json-1_0_140-46bc37dceda4ba0d = callPackage' ./deps/serde_json-1.0.140-46bc37dceda4ba0d.nix { };
      serde_json-1_0_140-script_build-c432054f227df43d = callPackage' ./deps/serde_json-1.0.140-script_build-c432054f227df43d.nix { };
      serde_json-1_0_140-script_build_run-770f79f91f797b97 = callPackage' ./deps/serde_json-1.0.140-script_build_run-770f79f91f797b97.nix { };
      serde_urlencoded-0_7_1-2d4ded72aa609d7b = callPackage' ./deps/serde_urlencoded-0.7.1-2d4ded72aa609d7b.nix { };
      shlex-1_3_0-9ec73c791a70e40d = callPackage' ./deps/shlex-1.3.0-9ec73c791a70e40d.nix { };
      slab-0_4_9-703e5769016427df = callPackage' ./deps/slab-0.4.9-703e5769016427df.nix { };
      slab-0_4_9-script_build-c5c7ac36caba863e = callPackage' ./deps/slab-0.4.9-script_build-c5c7ac36caba863e.nix { };
      slab-0_4_9-script_build_run-5d3e9bb72a7bf225 = callPackage' ./deps/slab-0.4.9-script_build_run-5d3e9bb72a7bf225.nix { };
      smallvec-1_14_0-597ef9a5b577a51b = callPackage' ./deps/smallvec-1.14.0-597ef9a5b577a51b.nix { };
      socket2-0_5_8-b832ef87df39fa74 = callPackage' ./deps/socket2-0.5.8-b832ef87df39fa74.nix { };
      stable_deref_trait-1_2_0-62798fe62205e471 = callPackage' ./deps/stable_deref_trait-1.2.0-62798fe62205e471.nix { };
      syn-2_0_98-93aa0f13dad61a07 = callPackage' ./deps/syn-2.0.98-93aa0f13dad61a07.nix { };
      sync_wrapper-0_1_2-56943e2929f0b498 = callPackage' ./deps/sync_wrapper-0.1.2-56943e2929f0b498.nix { };
      synstructure-0_13_1-b52c6310fdbcc413 = callPackage' ./deps/synstructure-0.13.1-b52c6310fdbcc413.nix { };
      thiserror-1_0_69-19f3044926d2abbd = callPackage' ./deps/thiserror-1.0.69-19f3044926d2abbd.nix { };
      thiserror-1_0_69-script_build-6cadb049729033d9 = callPackage' ./deps/thiserror-1.0.69-script_build-6cadb049729033d9.nix { };
      thiserror-1_0_69-script_build_run-ac159c9a9043953b = callPackage' ./deps/thiserror-1.0.69-script_build_run-ac159c9a9043953b.nix { };
      thiserror-impl-1_0_69-284cfbbe4814f2b5 = callPackage' ./deps/thiserror-impl-1.0.69-284cfbbe4814f2b5.nix { };
      tinystr-0_7_6-887be2ca1a583610 = callPackage' ./deps/tinystr-0.7.6-887be2ca1a583610.nix { };
      tokio-1_43_0-1ce995a0fb66b2ee = callPackage' ./deps/tokio-1.43.0-1ce995a0fb66b2ee.nix { };
      tokio-macros-2_5_0-e9096740d4cd33e0 = callPackage' ./deps/tokio-macros-2.5.0-e9096740d4cd33e0.nix { };
      tokio-native-tls-0_3_1-35c9a5a3ee237cd6 = callPackage' ./deps/tokio-native-tls-0.3.1-35c9a5a3ee237cd6.nix { };
      tokio-util-0_7_13-a55504be0ec4869b = callPackage' ./deps/tokio-util-0.7.13-a55504be0ec4869b.nix { };
      tower-service-0_3_3-f2bb19b3a855b555 = callPackage' ./deps/tower-service-0.3.3-f2bb19b3a855b555.nix { };
      tracing-0_1_41-4e706e46b3a66672 = callPackage' ./deps/tracing-0.1.41-4e706e46b3a66672.nix { };
      tracing-core-0_1_33-983fd01c842b7b60 = callPackage' ./deps/tracing-core-0.1.33-983fd01c842b7b60.nix { };
      try-lock-0_2_5-85edf1e11d3a5298 = callPackage' ./deps/try-lock-0.2.5-85edf1e11d3a5298.nix { };
      unicode-ident-1_0_17-19b9bf45cc26c223 = callPackage' ./deps/unicode-ident-1.0.17-19b9bf45cc26c223.nix { };
      url-2_5_4-9f9e587302d1e06c = callPackage' ./deps/url-2.5.4-9f9e587302d1e06c.nix { };
      utf16_iter-1_0_5-e2c4d90fc6d42e47 = callPackage' ./deps/utf16_iter-1.0.5-e2c4d90fc6d42e47.nix { };
      utf8_iter-1_0_4-09be6fe1c1163a1d = callPackage' ./deps/utf8_iter-1.0.4-09be6fe1c1163a1d.nix { };
      vcpkg-0_2_15-586bd5fdef1a0fde = callPackage' ./deps/vcpkg-0.2.15-586bd5fdef1a0fde.nix { };
      want-0_3_1-65d33fb3976c0b7a = callPackage' ./deps/want-0.3.1-65d33fb3976c0b7a.nix { };
      wasm-bindgen-0_2_100-77c8057a5f6a6e61 = callPackage' ./deps/wasm-bindgen-0.2.100-77c8057a5f6a6e61.nix { };
      wasm-bindgen-0_2_100-script_build-3397ca532321e0d8 = callPackage' ./deps/wasm-bindgen-0.2.100-script_build-3397ca532321e0d8.nix { };
      wasm-bindgen-0_2_100-script_build_run-414fd6a6e909b616 = callPackage' ./deps/wasm-bindgen-0.2.100-script_build_run-414fd6a6e909b616.nix { };
      wasm-bindgen-backend-0_2_100-d0c488311c1205dd = callPackage' ./deps/wasm-bindgen-backend-0.2.100-d0c488311c1205dd.nix { };
      wasm-bindgen-futures-0_4_50-ca328d45d204dbbc = callPackage' ./deps/wasm-bindgen-futures-0.4.50-ca328d45d204dbbc.nix { };
      wasm-bindgen-macro-0_2_100-283fea4a846f909d = callPackage' ./deps/wasm-bindgen-macro-0.2.100-283fea4a846f909d.nix { };
      wasm-bindgen-macro-support-0_2_100-96964d1b1a55d331 = callPackage' ./deps/wasm-bindgen-macro-support-0.2.100-96964d1b1a55d331.nix { };
      wasm-bindgen-shared-0_2_100-acd810d9b451b936 = callPackage' ./deps/wasm-bindgen-shared-0.2.100-acd810d9b451b936.nix { };
      wasm-bindgen-shared-0_2_100-script_build-fff16c94402b1949 = callPackage' ./deps/wasm-bindgen-shared-0.2.100-script_build-fff16c94402b1949.nix { };
      wasm-bindgen-shared-0_2_100-script_build_run-c1ec16c7ecc896b4 = callPackage' ./deps/wasm-bindgen-shared-0.2.100-script_build_run-c1ec16c7ecc896b4.nix { };
      web-sys-0_3_77-c55ef6dc80720d2e = callPackage' ./deps/web-sys-0.3.77-c55ef6dc80720d2e.nix { };
      write16-1_0_0-26b1931c23e70fcd = callPackage' ./deps/write16-1.0.0-26b1931c23e70fcd.nix { };
      writeable-0_5_5-126ae34b487ba069 = callPackage' ./deps/writeable-0.5.5-126ae34b487ba069.nix { };
      yoke-0_7_5-06c6b7b4d71d5784 = callPackage' ./deps/yoke-0.7.5-06c6b7b4d71d5784.nix { };
      yoke-derive-0_7_5-a7212319c1d2a9df = callPackage' ./deps/yoke-derive-0.7.5-a7212319c1d2a9df.nix { };
      zerofrom-0_1_5-67ba692659162de9 = callPackage' ./deps/zerofrom-0.1.5-67ba692659162de9.nix { };
      zerofrom-derive-0_1_5-6de1175abc985e61 = callPackage' ./deps/zerofrom-derive-0.1.5-6de1175abc985e61.nix { };
      zerovec-0_10_4-e3e96206699c2afa = callPackage' ./deps/zerovec-0.10.4-e3e96206699c2afa.nix { };
      zerovec-derive-0_10_3-229ca38161287853 = callPackage' ./deps/zerovec-derive-0.10.3-229ca38161287853.nix { };
    };
  };
in
self
