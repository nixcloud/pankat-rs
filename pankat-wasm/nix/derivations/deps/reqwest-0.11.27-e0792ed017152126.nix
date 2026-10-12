# generated from rustc-call.nix.handlebars using cargo (manual edits won't be persistent)
{ fn, pkgs, rustc, cargo, deps, project_root }: with deps;
  pkgs.stdenv.mkDerivation rec {
    name = "reqwest-0_11_27-e0792ed017152126";
    meta.cargo_crate_info = {
      name = "reqwest";
      version = "0.11.27";
      crate_hash = "e0792ed017152126";
      type = "";
    };
    buildInputs = [] ++ fn.inject_deps meta.cargo_crate_info;
    env = fn.inject_envs meta.cargo_crate_info;

    passthru.rust_crate_libraries = [base64-0_21_7-9aa53967577f990a bytes-1_10_0-64a68773187e68cf encoding_rs-0_8_35-92f62b88e615318e futures-core-0_3_31-f6a43156147538e7 futures-util-0_3_31-ddd4b710c5171915 h2-0_3_26-6ae3cdc03d4408e3 http-0_2_12-83e90f39f49f8f94 http-body-0_4_6-604b1fb4509892f9 hyper-0_14_32-78cc5e777f9503a3 hyper-tls-0_5_0-0ce46cc06ae98dd5 ipnet-2_11_0-2aac75a0fbf913b7 log-0_4_26-d46dd80a9e51535a mime-0_3_17-19dbee6f0f8f1382 native-tls-0_2_14-cbf881b3f45228cd once_cell-1_20_3-60992a3834e62ae0 percent-encoding-2_3_1-e8b9e34a5db857ef pin-project-lite-0_2_16-dbb7ad05ebc1034f rustls-pemfile-1_0_4-8daa84e95740c37e serde-1_0_218-eb19f02d9fcfde0c serde_json-1_0_140-46bc37dceda4ba0d serde_urlencoded-0_7_1-2d4ded72aa609d7b sync_wrapper-0_1_2-56943e2929f0b498 tokio-1_43_0-1ce995a0fb66b2ee tokio-native-tls-0_3_1-35c9a5a3ee237cd6 tower-service-0_3_3-f2bb19b3a855b555 url-2_5_4-9f9e587302d1e06c];
    passthru.rust_crate_parent = [];
    passthru.rust_script_build_run = [];
    phases = "unpackPhase buildPhase";

    src = pkgs.fetchurl {
      url = "https://crates.io/api/v1/crates/reqwest/0.11.27/download";
      sha256 = "dd67538700a17451e7cba03ac727fb961abb7607553461627b97de0b89cf4a62";
    };

    unpackPhase = ''
      tar xf $src
      cd reqwest-0.11.27
    '';

    RUSTC = "${rustc}/bin/rustc";
    CARGO_CRATE_NAME = "reqwest";
    CARGO_MANIFEST_DIR = "./";
    CARGO_MANIFEST_PATH = "./Cargo.toml";
    CARGO_PKG_AUTHORS = "Sean McArthur <sean@seanmonstar.com>";
    CARGO_PKG_DESCRIPTION = "higher level HTTP client library";
    CARGO_PKG_HOMEPAGE = "";
    CARGO_PKG_LICENSE = "MIT OR Apache-2.0";
    CARGO_PKG_LICENSE_FILE = "";
    CARGO_PKG_NAME = "reqwest";
    CARGO_PKG_README = "README.md";
    CARGO_PKG_REPOSITORY = "https://github.com/seanmonstar/reqwest";
    CARGO_PKG_RUST_VERSION = "1.63.0";
    CARGO_PKG_VERSION = "0.11.27";
    CARGO_PKG_VERSION_MAJOR = "0";
    CARGO_PKG_VERSION_MINOR = "11";
    CARGO_PKG_VERSION_PATCH = "27";
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
              --crate-name reqwest \
              --edition=2021 src/lib.rs \
              --error-format=json \
              --json=diagnostic-rendered-ansi,artifacts,future-incompat \
              --crate-type lib \
              --emit=dep-info,metadata,link \
              -C embed-bitcode=no \
              -C debuginfo=2 \
              ${fn.rustc_arguments passthru.rust_crate_parent} \
              --cfg 'feature="__tls"' \
              --cfg 'feature="default"' \
              --cfg 'feature="default-tls"' \
              --cfg 'feature="hyper-tls"' \
              --cfg 'feature="json"' \
              --cfg 'feature="native-tls-crate"' \
              --cfg 'feature="serde_json"' \
              --cfg 'feature="tokio-native-tls"' \
              --check-cfg 'cfg(docsrs,test)' \
              --check-cfg 'cfg(feature, values("__internal_proxy_sys_no_cache", "__rustls", "__tls", "async-compression", "blocking", "brotli", "cookie_crate", "cookie_store", "cookies", "default", "default-tls", "deflate", "futures-channel", "gzip", "h3", "h3-quinn", "hickory-dns", "hickory-resolver", "http3", "hyper-rustls", "hyper-tls", "json", "mime_guess", "multipart", "native-tls", "native-tls-alpn", "native-tls-crate", "native-tls-vendored", "quinn", "rustls", "rustls-native-certs", "rustls-tls", "rustls-tls-manual-roots", "rustls-tls-native-roots", "rustls-tls-webpki-roots", "serde_json", "socks", "stream", "tokio-native-tls", "tokio-rustls", "tokio-socks", "tokio-util", "trust-dns", "wasm-streams", "webpki-roots"))' \
              -C metadata=283d29bf10f5c6c6 \
              -C extra-filename=-e0792ed017152126 \
              --out-dir $OUT_DIR \
              -L dependency=${fn.rustc_linker_arguments_dir passthru.rust_crate_libraries}/deps \
              ${fn.rustc_propagated_arguments passthru.rust_script_build_run} \
              ${fn.rustc_propagated_arguments passthru.rust_crate_libraries} \
              --extern base64=${base64-0_21_7-9aa53967577f990a}/libbase64-9aa53967577f990a.rmeta \
              --extern bytes=${bytes-1_10_0-64a68773187e68cf}/libbytes-64a68773187e68cf.rmeta \
              --extern encoding_rs=${encoding_rs-0_8_35-92f62b88e615318e}/libencoding_rs-92f62b88e615318e.rmeta \
              --extern futures_core=${futures-core-0_3_31-f6a43156147538e7}/libfutures_core-f6a43156147538e7.rmeta \
              --extern futures_util=${futures-util-0_3_31-ddd4b710c5171915}/libfutures_util-ddd4b710c5171915.rmeta \
              --extern h2=${h2-0_3_26-6ae3cdc03d4408e3}/libh2-6ae3cdc03d4408e3.rmeta \
              --extern http=${http-0_2_12-83e90f39f49f8f94}/libhttp-83e90f39f49f8f94.rmeta \
              --extern http_body=${http-body-0_4_6-604b1fb4509892f9}/libhttp_body-604b1fb4509892f9.rmeta \
              --extern hyper=${hyper-0_14_32-78cc5e777f9503a3}/libhyper-78cc5e777f9503a3.rmeta \
              --extern hyper_tls=${hyper-tls-0_5_0-0ce46cc06ae98dd5}/libhyper_tls-0ce46cc06ae98dd5.rmeta \
              --extern ipnet=${ipnet-2_11_0-2aac75a0fbf913b7}/libipnet-2aac75a0fbf913b7.rmeta \
              --extern log=${log-0_4_26-d46dd80a9e51535a}/liblog-d46dd80a9e51535a.rmeta \
              --extern mime=${mime-0_3_17-19dbee6f0f8f1382}/libmime-19dbee6f0f8f1382.rmeta \
              --extern native_tls_crate=${native-tls-0_2_14-cbf881b3f45228cd}/libnative_tls-cbf881b3f45228cd.rmeta \
              --extern once_cell=${once_cell-1_20_3-60992a3834e62ae0}/libonce_cell-60992a3834e62ae0.rmeta \
              --extern percent_encoding=${percent-encoding-2_3_1-e8b9e34a5db857ef}/libpercent_encoding-e8b9e34a5db857ef.rmeta \
              --extern pin_project_lite=${pin-project-lite-0_2_16-dbb7ad05ebc1034f}/libpin_project_lite-dbb7ad05ebc1034f.rmeta \
              --extern rustls_pemfile=${rustls-pemfile-1_0_4-8daa84e95740c37e}/librustls_pemfile-8daa84e95740c37e.rmeta \
              --extern serde=${serde-1_0_218-eb19f02d9fcfde0c}/libserde-eb19f02d9fcfde0c.rmeta \
              --extern serde_json=${serde_json-1_0_140-46bc37dceda4ba0d}/libserde_json-46bc37dceda4ba0d.rmeta \
              --extern serde_urlencoded=${serde_urlencoded-0_7_1-2d4ded72aa609d7b}/libserde_urlencoded-2d4ded72aa609d7b.rmeta \
              --extern sync_wrapper=${sync_wrapper-0_1_2-56943e2929f0b498}/libsync_wrapper-56943e2929f0b498.rmeta \
              --extern tokio=${tokio-1_43_0-1ce995a0fb66b2ee}/libtokio-1ce995a0fb66b2ee.rmeta \
              --extern tokio_native_tls=${tokio-native-tls-0_3_1-35c9a5a3ee237cd6}/libtokio_native_tls-35c9a5a3ee237cd6.rmeta \
              --extern tower_service=${tower-service-0_3_3-f2bb19b3a855b555}/libtower_service-f2bb19b3a855b555.rmeta \
              --extern url=${url-2_5_4-9f9e587302d1e06c}/liburl-9f9e587302d1e06c.rmeta \
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
