{
  description = "pankat static blog generator";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    cargo-libnix.url = "github:nixcloud/cargo";
    fenix.url   = "github:nix-community/fenix";
  };
  outputs =
  { self, nixpkgs, flake-utils, fenix, cargo-libnix }:
    (flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [
            fenix.overlays.default
          ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };

          fp = fenix.packages.${system};
          # host std is already inside stable.rustc; listing it again is harmless
          rust_toolchain = fp.combine [
            cargo-libnix.packages.${system}.rust-1_99
            #fp.targets.wasm32-unknown-unknown.stable.rust-std
          ];

          project_root = ./.;

          src = builtins.filterSource
            (path: type:
                let base = baseNameOf path;
                in !(base == "target" || base == "result" || builtins.match "result-*" base != null)
            ) project_root;

          lock = builtins.fromTOML (builtins.readFile ./Cargo.lock);
          crateTarballs = map (p: pkgs.fetchurl {
              name = "crate-${p.name}-${p.version}.tar.gz";
              url = "https://crates.io/api/v1/crates/${p.name}/${p.version}/download";
              sha256 = p.checksum;
            })
            (builtins.filter (p: (p.source or "") == "registry+https://github.com/rust-lang/crates.io-index") lock.package);

          generated = pkgs.runCommand "cargo-nix-generated" { inherit crateTarballs; nativeBuildInputs = [ cargo-libnix.packages.${system}.rust-1_99 ]; } ''
            export HOME=$TMPDIR CARGO_BACKEND=nix
            cp -r ${src} project && chmod -R u+w project && cd project
            ${cargo-libnix.packages.${system}.rust-1_99}/bin/cargo build --release --offline write-nix-buildsystem --out-dir $out
          '';

          # the cargo binary unit of a generated build system; its hash isn't known in advance
          pankatBin = units: units.${pkgs.lib.findFirst (n: pkgs.lib.hasPrefix "pankat-0_1_1-bin-" n)
            (throw "no pankat-0_1_1-bin-* unit in the generated build system") (builtins.attrNames units)};

          pankat-ifd = pankatBin (import "${generated}/cargo_build_caller.nix" { inherit system; project_root = src; });

          pankat_wasm = pkgs.callPackage ./pankat-wasm.nix {
             rustc = rust_toolchain;
             cargo = rust_toolchain;
          };

          # pankat_wasm_deps = {
          #   deps = {
          #       "openssl-sys" = [ pkgs.pkg-config pkgs.openssl ];
          #   };
          #   envs = {};
          # };
        in
        with pkgs;
        rec {
          packages.pankat = pankat-ifd;
          packages.pankat_wasm = pankat_wasm;
          # bin/pankat + share/pankat/wasm and share/pankat/assets (pass them via --wasm and --assets),
          # all a deployment needs besides the documents, see nixos-module.nix
          packages.default = symlinkJoin {
            name = "pankat-with-wasm";
            paths = [ packages.pankat ];
            postBuild = ''
              mkdir -p $out/share/pankat
              ln -s ${packages.pankat_wasm} $out/share/pankat/wasm
              ln -s ${./documents/assets} $out/share/pankat/assets
            '';
          };
          devShells.default = mkShell {
            buildInputs = [
              # pankat-rs requires those (build)
              cargo-libnix.packages.${system}.rust-1_99

              patchelf
              just
              cargo-binutils

              # pankat-rs requires those (runtime)
              pandoc

              # openssl-sys (used in pankat-wasm)
              #pkg-config
              #openssl
              #sqlite

              # required to minify WASM files with wasm-opt
              binaryen
              wasm-pack

              # vanilla rust (nixpkgs)
              # rust
              # cargo
              # patching rust-bin
              # (rust-bin.stable.latest.default.override {  extensions = [ "rust-src" ];  targets = [ "wasm32-unknown-unknown" ];})

              # 'just zig' creates a standalone binary (not using glibc)
              zig
              cargo-zigbuild
            ];
            shellHook = ''
              export CARGO_BACKEND=nix
            '';
          };
        }
      )) // {
        # services.pankat: the pankat service, nginx and optionally a git checkout of the documents
        nixosModules.pankat = import ./nixos-module.nix self;
        nixosModules.default = self.nixosModules.pankat;
      };
}
