# services.pankat: pankat as a systemd service and nginx in front of it, which serves the static files (output,
# posts, media, assets, wasm) itself and passes only the websocket, the login, the drafts, the special pages and the
# session state to pankat; optionally keeps a git checkout of the documents up to date
#
# NOTE: untested on a real host, so far only checked in a NixOS VM test (no TLS/ACME, no git.url)
#
#   inputs.pankat.url = "github:nixcloud/pankat-rs";
#   nixosConfigurations.webservi = nixpkgs.lib.nixosSystem {
#     modules = [ pankat.nixosModules.default {
#       services.pankat = {
#         enable = true;
#         siteUrl = "https://lastlog.de/blog";
#         subdir = "/blog";
#         git.url = "https://github.com/you/your-blog.git";
#         # PANKAT_JWT_TOKEN=..., PANKAT_ADMIN_PASSWORD=... and PANKAT_GITHUB_WEBHOOK_SECRET=..., outside of the nix store
#         environmentFile = "/etc/pankat.env";
#         nginx.virtualHost = "lastlog.de";
#       };
#       services.nginx.virtualHosts."lastlog.de" = { forceSSL = true; enableACME = true; };
#     } ];
#   };
self:
{ config, lib, pkgs, ... }:
let
  cfg = config.services.pankat;
  inherit (lib) mkOption mkEnableOption mkIf mkMerge types optionalAttrs;
  s = cfg.subdir;
  proxy = {
    proxyPass = "http://127.0.0.1:${toString cfg.port}";
    recommendedProxySettings = true;
  };
  # nginx shows pankat's 404 page for the files it serves itself
  static = path: {
    alias = "${path}/";
    extraConfig = "error_page 404 ${s}/404.html;";
  };
in
{
  options.services.pankat = {
    enable = mkEnableOption "the pankat blog";

    package = mkOption {
      type = types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      defaultText = lib.literalExpression "pankat.packages.\${system}.default";
      description = "pankat with share/pankat/assets and share/pankat/wasm.";
    };

    user = mkOption {
      type = types.str;
      default = "pankat";
      description = "User pankat runs as, created when it's the default.";
    };

    group = mkOption {
      type = types.str;
      default = "pankat";
      description = "Group of the data directory, nginx is added to it to read the documents.";
    };

    dataDir = mkOption {
      type = types.path;
      default = "/var/lib/pankat";
      description = "Holds the database (pankat.sqlite) and, by default, the input and output directories.";
    };

    input = mkOption {
      type = types.path;
      default = "${cfg.dataDir}/input";
      defaultText = lib.literalExpression ''"''${config.services.pankat.dataDir}/input"'';
      description = "The documents of the blog (posts/, media/), a checkout of git.url when that is set.";
    };

    output = mkOption {
      type = types.path;
      default = "${cfg.dataDir}/output";
      defaultText = lib.literalExpression ''"''${config.services.pankat.dataDir}/output"'';
      description = "Where pankat writes the html, served by nginx; it can be deleted, pankat writes it again.";
    };

    assets = mkOption {
      type = types.path;
      default = "${cfg.package}/share/pankat/assets";
      defaultText = lib.literalExpression ''"''${config.services.pankat.package}/share/pankat/assets"'';
      description = "Templates, pandoc lua filters, css and js.";
    };

    wasm = mkOption {
      type = types.path;
      default = "${cfg.package}/share/pankat/wasm";
      defaultText = lib.literalExpression ''"''${config.services.pankat.package}/share/pankat/wasm"'';
      description = "The pankat-wasm runtime for the live updates in the browser.";
    };

    subdir = mkOption {
      type = types.str;
      default = "/blog";
      example = "";
      description = ''The path the blog is served below, like "/blog" or "" for the root.'';
    };

    siteUrl = mkOption {
      type = types.str;
      example = "https://lastlog.de/blog";
      description = "Public base URL incl. subdir, for the label top left and the Open Graph tags.";
    };

    port = mkOption {
      type = types.port;
      default = 5000;
      description = "Port pankat listens on (all interfaces, keep it closed in the firewall), nginx proxies to it.";
    };

    flat = mkOption {
      type = types.bool;
      default = true;
      description = "Flatten the output directory (foo/bar.mdwn -> bar.html).";
    };

    environmentFile = mkOption {
      type = types.nullOr types.path;
      default = null;
      example = "/etc/pankat.env";
      description = ''
        File with PANKAT_JWT_TOKEN=... (`openssl rand -hex 32`) and PANKAT_ADMIN_PASSWORD=..., kept out of the
        nix store. Without it the admin login (drafts, special pages) is disabled. With
        PANKAT_GITHUB_WEBHOOK_SECRET=... a github webhook (push events, application/json) to <siteUrl>/github-webhook
        runs git pull in `input`, which has to be a git checkout (git.url).
      '';
    };

    git = {
      url = mkOption {
        type = types.nullOr types.str;
        default = null;
        example = "https://github.com/you/your-blog.git";
        description = "Clone the documents into `input` and pull them regularly, pankat rebuilds what changed.";
      };
      branch = mkOption {
        type = types.nullOr types.str;
        default = null;
        description = "Branch to check out, the default branch of the repository when null.";
      };
      interval = mkOption {
        type = types.str;
        default = "10min";
        description = "How often to pull, a systemd time span.";
      };
    };

    nginx = {
      enable = mkOption {
        type = types.bool;
        default = true;
        description = "Serve the blog with nginx in the virtual host `nginx.virtualHost`.";
      };
      virtualHost = mkOption {
        type = types.str;
        default = "localhost";
        example = "lastlog.de";
        description = ''
          The nginx virtual host the locations are added to; set its serverName, serverAliases, forceSSL and
          enableACME in services.nginx.virtualHosts."<name>".
        '';
      };
      redirectRoot = mkOption {
        type = types.bool;
        default = s != "";
        defaultText = lib.literalExpression ''config.services.pankat.subdir != ""'';
        description = "Redirect / of the virtual host to the blog.";
      };
    };
  };

  config = mkIf cfg.enable (mkMerge [
    {
      assertions = [
        {
          assertion = s == "" || (lib.hasPrefix "/" s && !lib.hasSuffix "/" s);
          message = ''services.pankat.subdir must be "" or start with / and not end with /, like "/blog".'';
        }
      ];

      users.users = optionalAttrs (cfg.user == "pankat") {
        pankat = {
          isSystemUser = true;
          group = cfg.group;
          home = cfg.dataDir;
          description = "pankat blog";
        };
      };
      users.groups = optionalAttrs (cfg.group == "pankat") { pankat = { }; };

      # group readable for nginx; pankat stops when input or output is missing
      systemd.tmpfiles.settings.pankat = lib.genAttrs [ cfg.dataDir cfg.input cfg.output ] (_: {
        d = {
          user = cfg.user;
          group = cfg.group;
          mode = "0750";
        };
      });

      systemd.services.pankat = {
        description = "pankat blog";
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ] ++ lib.optional (cfg.git.url != null) "pankat-git.service";
        wants = lib.optional (cfg.git.url != null) "pankat-git.service";
        # git for the github webhook (git pull in input)
        path = [ pkgs.pandoc pkgs.git pkgs.openssh ];
        # everything but the secrets (environmentFile), pankat.toml isn't needed
        environment = {
          RUST_LOG = "info";
          PANKAT_INPUT = cfg.input;
          PANKAT_OUTPUT = cfg.output;
          PANKAT_ASSETS = cfg.assets;
          PANKAT_WASM = cfg.wasm;
          PANKAT_DATABASE = cfg.dataDir;
          PANKAT_SUBDIR = s;
          PANKAT_SITE_URL = cfg.siteUrl;
          PANKAT_PORT = toString cfg.port;
          PANKAT_FLAT = lib.boolToString cfg.flat;
        };
        serviceConfig = {
          ExecStart = "${cfg.package}/bin/pankat";
          User = cfg.user;
          Group = cfg.group;
          WorkingDirectory = cfg.dataDir;
          EnvironmentFile = mkIf (cfg.environmentFile != null) cfg.environmentFile;
          Restart = "always";
          # new files group readable for nginx
          UMask = "0027";
          NoNewPrivileges = true;
          PrivateTmp = true;
          ProtectSystem = "strict";
          ReadWritePaths = lib.unique [ cfg.dataDir cfg.output ];
        };
      };
    }

    (mkIf (cfg.git.url != null) {
      systemd.services.pankat-git = {
        description = "Clone or pull the pankat documents";
        after = [ "network-online.target" ];
        wants = [ "network-online.target" ];
        path = [ pkgs.git pkgs.openssh ];
        serviceConfig = {
          Type = "oneshot";
          User = cfg.user;
          Group = cfg.group;
          UMask = "0027";
        };
        script = ''
          if [ -d ${lib.escapeShellArg cfg.input}/.git ]; then
            git -C ${lib.escapeShellArg cfg.input} pull --ff-only
          else
            git clone ${lib.optionalString (cfg.git.branch != null) "--branch ${lib.escapeShellArg cfg.git.branch}"} \
              ${lib.escapeShellArg cfg.git.url} ${lib.escapeShellArg cfg.input}
          fi
        '';
      };
      systemd.timers.pankat-git = {
        wantedBy = [ "timers.target" ];
        timerConfig = {
          OnUnitActiveSec = cfg.git.interval;
          OnBootSec = cfg.git.interval;
        };
      };
    })

    (mkIf cfg.nginx.enable {
      services.nginx.enable = true;
      users.users.nginx.extraGroups = [ cfg.group ];
      services.nginx.virtualHosts.${cfg.nginx.virtualHost}.locations = {
        "${s}/api/ws" = proxy // {
          proxyWebsockets = true;
          extraConfig = "proxy_read_timeout 86400;";
        };
        # longer prefixes than ^~ ${s}/ below, so they win
        "^~ ${s}/login" = proxy;
        "^~ ${s}/logout" = proxy;
        # /draft and /drafts/
        "^~ ${s}/draft" = proxy;
        "^~ ${s}/specialpages" = proxy;
        "= ${s}/api/session" = proxy;
        "= ${s}/github-webhook" = proxy;
        "^~ ${s}/posts/" = static "${cfg.input}/posts";
        "^~ ${s}/media/" = static "${cfg.input}/media";
        "^~ ${s}/assets/" = static cfg.assets;
        "^~ ${s}/wasm/" = static cfg.wasm;
        "^~ ${s}/" = static cfg.output;
      } // optionalAttrs (s != "") {
        "= ${s}".return = "301 ${s}/index.html";
      } // optionalAttrs cfg.nginx.redirectRoot {
        "= /".return = "301 ${s}/index.html";
      };
    })
  ]);
}
