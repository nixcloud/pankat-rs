{
  description = "pankat static blog generator";
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
          rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
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
          pankat = pkgs.callPackage ./pankat.nix {};
        in
        with pkgs;
        rec {
          packages.default = pankat;
          devShells.default = mkShell {
            buildInputs = [
              rust
              cargo
              cargo-binutils
              just
              sqlite
              pandoc
              
              cmake
              clang
              lld
              pkg-config
              openssl

              binaryen  # required to minify WASM files with wasm-opt
              wasm-pack

              #zig
              #cargo-zigbuild
            ];
          };
        }
      );
}
