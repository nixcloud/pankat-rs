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
          pankat-wasm = pkgs.callPackage ./pankat-wasm.nix {};
        in
        with pkgs;
        rec {
          packages.default = pankat-wasm; #[ pankat pankat-wasm ];
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
              (rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" ];
                targets = [ "wasm32-unknown-unknown" ];
              })

              #zig
              #cargo-zigbuild
            ];
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
