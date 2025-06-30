# default.nix
{ pkgs ? import <nixpkgs> {} }:

pkgs.stdenv.mkDerivation {
  pname = "my-wasm-project";
  version = "1.0.0";

  src = pkgs.lib.cleanSource ./pankat-wasm;

  buildInputs = [
    pkgs.wasm-pack
    pkgs.rustc
    pkgs.cargo
  ];

  buildPhase = ''
    wasm-pack build --mode no-install --offline --target web --release --manifest-path ./Cargo.toml
  '';

  installPhase = ''
    mkdir -p $out
    cp -r pkg/* $out/
  '';
}