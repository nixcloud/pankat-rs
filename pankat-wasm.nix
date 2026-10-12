# default.nix
{ pkgs ? import <nixpkgs> {}, rustc, cargo }:

pkgs.stdenv.mkDerivation {
  pname = "pankat-wasm";
  version = "0.1.0";
  src = pkgs.lib.cleanSource ./pankat-wasm;

  # vendored crates; cargoSetupHook writes .cargo/config.toml pointing cargo at them
  cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
    src = pkgs.lib.cleanSource ./pankat-wasm;
    hash = "sha256-9mMOiUMjitkLCaNqy20t3WYYY3xdVSKXsBEy1xnMno4=";
  };

  nativeBuildInputs = [
    pkgs.rustPlatform.cargoSetupHook
    pkgs.wasm-pack
    # must match the wasm-bindgen version in pankat-wasm/Cargo.lock exactly
    pkgs.wasm-bindgen-cli_0_2_100
    pkgs.binaryen
    rustc
    cargo
  ];

  buildPhase = ''
    runHook preBuild
    export HOME=$TMPDIR
    wasm-pack build --mode no-install --release --target web -- --frozen
    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall
    mkdir -p $out
    cp -r pkg/* $out/
    runHook postInstall
  '';
}
