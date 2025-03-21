# please read flake introduction here:
# https://fasterthanli.me/series/building-a-rust-service-with-nix/part-10#a-flake-with-a-dev-shell
{
  description = "The fairsync importer prototype flake";
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };
  outputs =
  { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rust = pkgs.rust-bin.fromRustupToolchainFile ./pankat-wasm/rust-toolchain.toml;
          platform_packages =
            if pkgs.stdenv.isLinux then
              with pkgs; [ ]
            else if pkgs.stdenv.isDarwin then
              with pkgs.darwin.apple_sdk.frameworks; [
                CoreFoundation
                Security
                SystemConfiguration
              ]
            else
              throw "unsupported platform";
        in
        with pkgs;
        rec {
          #trunk = pkgs.callPackage ./trunk.nix {
          #  inherit (darwin.apple_sdk.frameworks) CoreServices Security SystemConfiguration;
          #};
          #leptosfmt = pkgs.callPackage ./leptosfmt.nix {};

          devShells.default = mkShell {
            buildInputs = [
              cargo
              sqlite
              cargo-binutils
              lld
              pandoc
              pkg-config
              nushell
              just
              rust
              binaryen                 # required to minify WASM files with wasm-opt
              wasm-pack
              #cmake
              #clang
              #trunk                    # required to bundle the frontend
              #git
              #nodejs                   # required to install tailwind plugins
            ];
          };
        }
      );
}
