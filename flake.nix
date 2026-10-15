{
  description = "pankat static blog generator";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    fenix.url   = "github:nix-community/fenix";
  };
  outputs =
  { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem
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
            fp.stable.rustc
            fp.stable.cargo
            fp.targets.wasm32-unknown-unknown.stable.rust-std
          ];

          ######################## cargo-libnix-1_87_0_plus_v4 ###########################

          rustc_1_87_vanilla_pin = (fenix.packages.${system}.toolchainOf {
            channel = "1.87.0";
            sha256 = "sha256-KUm16pHj+cRedf8vxs/Hd2YWxpOrWZ7UOrwhILdSJBU=";
          }).rustc;
          cargo_1_87_vanilla_pin = (fenix.packages.${system}.toolchainOf {
            channel = "1.87.0";
            sha256 = "sha256-KUm16pHj+cRedf8vxs/Hd2YWxpOrWZ7UOrwhILdSJBU=";
          }).cargo;

          project_root = ./.;

          cargo-libnix-1_87_0_plus_v4_src = builtins.fetchTarball {
            url = "https://github.com/nixcloud/cargo/releases/download/libnix-1.87.0%2Bv4/libnix-1.87.0+v4.tar.bz2";
            sha256 = "sha256:0zhj8sl05g861c7pkwgd1ij26mf68qj1flbad2ivwj113vac9chq";
          };
          # the generated cargo_build_caller.nix (used by `cargo build` in `nix develop` and by pankat-ifd) builds
          # with fenix.stable.toolchain, which ships rust-src: rustc then maps std's /rustc/<commit>/ paths in panic
          # locations back to that toolchain's store path, pulling the whole toolchain (4 GB) into pankat's runtime
          # closure (#24). build cargo (libnix) from a source whose template uses rustc_1_87_vanilla_pin instead,
          # which has no rust-src; appendContext and not builtins.storePath, so it also works in pure evaluation
          cargo-libnix-1_87_0_plus_v4_cargo_src_url = "https://github.com/nixcloud/cargo/archive/refs/tags/libnix-1.87.0+v4.tar.gz";
          cargo-libnix-1_87_0_plus_v4_cargo_src = pkgs.runCommand "cargo-libnix-1.87.0-v4-src" {} ''
            mkdir $out
            tar xzf ${pkgs.fetchurl {
              url = cargo-libnix-1_87_0_plus_v4_cargo_src_url;
              sha256 = "0cy6nff1arj08xiddpv4r3jr8306ch7lvi8najyngr3zaiby1mkg";
            }} -C $out --strip-components=1
            substituteInPlace $out/src/cargo/core/compiler/nix_build/templates/cargo_build_caller.nix.handlebars \
              --replace-fail 'toolchain = fenix.stable.toolchain;' \
                'toolchain = builtins.appendContext "${rustc_1_87_vanilla_pin}" { "${rustc_1_87_vanilla_pin}" = { path = true; }; };'
          '';
          # the cargo units fetch their source with pkgs.fetchurl, all other crates are passed through
          pkgs_cargo_libnix = pkgs // {
            pkgs = pkgs_cargo_libnix;
            fetchurl = args: if (args.url or null) == cargo-libnix-1_87_0_plus_v4_cargo_src_url
              then cargo-libnix-1_87_0_plus_v4_cargo_src else pkgs.fetchurl args;
          };
          cargo-libnix-1_87_0_plus_v4 = (import (cargo-libnix-1_87_0_plus_v4_src + "/derivations/default.nix"){
            inherit project_root;
            pkgs = pkgs_cargo_libnix;
            external_crate_dependencies = import (cargo-libnix-1_87_0_plus_v4_src + "/Cargo.dependencies.nix") { inherit pkgs; };
            rustc = rustc_1_87_vanilla_pin;
            cargo = cargo_1_87_vanilla_pin;
          }).cargo-0_88_0-bin-8cad88ffd54a16db;

          ######################################################################################

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

          generated = pkgs.runCommand "cargo-nix-generated" { inherit crateTarballs; nativeBuildInputs = [ cargo-libnix-1_87_0_plus_v4 rustc_1_87_vanilla_pin ]; } ''
            export HOME=$TMPDIR CARGO_BACKEND=nix
            cp -r ${src} project && chmod -R u+w project && cd project
            ${cargo-libnix-1_87_0_plus_v4}/bin/cargo build --release --offline write-nix-buildsystem --out-dir $out
          '';

          # the cargo binary unit of a generated build system; its hash isn't known in advance
          pankatBin = units: units.${pkgs.lib.findFirst (n: pkgs.lib.hasPrefix "pankat-0_1_1-bin-" n)
            (throw "no cargo-0_88_0-bin-* unit in the generated build system") (builtins.attrNames units)};

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
          # bin/pankat + share/pankat/wasm (pass the latter via --wasm)
          packages.default = symlinkJoin {
            name = "pankat-with-wasm";
            paths = [ packages.pankat ];
            postBuild = ''
              mkdir -p $out/share/pankat
              ln -s ${packages.pankat_wasm} $out/share/pankat/wasm
            '';
          };
          devShells.default = mkShell {
            buildInputs = [
              # pankat-rs requires those (build)
              cargo-libnix-1_87_0_plus_v4
              rustc_1_87_vanilla_pin
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
      );
}

#  users.users = {
# pankat-app = {
#   isSystemUser = true;
#   home = "/pankat-app";
#   group = "pankat-app";
#   description = "pankat app users";
# };
#  };
# users.groups.pankat-app = {};

# security.acme = {
#   acceptTerms = true;
#   defaults.email = "info@nixcloud.io";
# };

# services.nginx = {
#   enable = true;
#   recommendedGzipSettings = true;
#   recommendedOptimisation = true;
#   virtualHosts = {
#     lastlogblog = {
#       serverName = "lastlog.de";
#       serverAliases = [ "www.lastlog.de" ];
#       forceSSL = true;
#       enableACME = true;
#       extraConfig = ''
#         location /blog/api/ws {
#           proxy_pass http://127.0.0.1:5000;
#           proxy_set_header Host $host;
#           proxy_http_version 1.1;
#           proxy_set_header Upgrade $http_upgrade;
#           proxy_set_header Connection "upgrade";
#           proxy_read_timeout 86400;
#         }
#         # admin login, drafts and the session state for the navbar are served by pankat,
#         # longer prefixes than ^~ /blog/ below so they win
#         location ^~ /blog/login { proxy_pass http://127.0.0.1:5000; proxy_set_header Host $host; }
#         location ^~ /blog/logout { proxy_pass http://127.0.0.1:5000; proxy_set_header Host $host; }
#         location ^~ /blog/draft { proxy_pass http://127.0.0.1:5000; proxy_set_header Host $host; }
#         location ^~ /blog/specialpages { proxy_pass http://127.0.0.1:5000; proxy_set_header Host $host; }
#         location = /blog/api/session { proxy_pass http://127.0.0.1:5000; proxy_set_header Host $host; }
#         # written by pankat to the output directory
#         error_page 404 /blog/404.html;
#         location ^~ /blog/posts/ {
#           alias /pankat-app/documents/blog.lastlog.de/posts/;
#           autoindex on;
#         }
#         location ^~ /blog/media/ {
#           alias /pankat-app/documents/blog.lastlog.de/media/;
#           autoindex on;
#         }
#         location ^~ /blog/assets/ {
#           alias /pankat-app/documents/assets/;
#           autoindex on;
#         }
#         location ^~ /blog/wasm/ {
#           alias /pankat-app/documents/wasm/;
#           autoindex on;
#         }
#         location ^~ /blog/ {
#           alias /pankat-app/documents/output/;
#           autoindex on;
#         }
#         location = / {
#           return 301 https://lastlog.de/blog/index.html;
#         }
#         location = /blog {
#           return 301 https://lastlog.de/blog/index.html;
#         }
#       '';
#     };
#   };
# };

# systemd.services.pankat-app = {
#   wantedBy = [ "multi-user.target" ];
#   after = [ "network.target" ];
#   description = "Start the pankat server backend";
#   environment = { RUST_LOG = "info"; };
#   path = with pkgs; [ pandoc ];
#   serviceConfig = {
#     Restart = "always";
#     Type = "simple";
#     User = "pankat-app";
#     ExecStart = "/pankat-app/pankat";
#     WorkingDirectory = "/pankat-app";
#   };
# };

# systemd.services.pankat-git-pull = {
#   wantedBy = [ "multi-user.target" ];
#   after = [ "network.target" ];
#   description = "Git pull for blog.lastlog.de every 10 minutes";
#   path = with pkgs; [ git bash ];
#   serviceConfig = {
#     Type = "simple";
#     User = "pankat-app";
#     ExecStart =
#       "${pkgs.bash}/bin/bash -c 'while true; do git -C /pankat-app/documents/blog.lastlog.de pull; sleep 600; done'";
#     WorkingDirectory = "/pankat-app/documents/blog.lastlog.de";
#   };
# };
