# motivation

# what
**pankat** is a **static blog generator** inspired by [joey hess's ikiwiki](https://ikiwiki.info/users/joey/).

![A screenshot featuring pankat](./.screenshots/pankat.jpg)

notable features:

* writing / reading aid
  * **markdown** syntax for writing articles
  * **table of contents** using jquery.tocify.min.js
  * **dynamic page anchors** (similar to anchor.js)
  * `[[!draft]]` mode support
  * **live updates** of article changes via websocket using **file system changes monitoring** in the documents folder
  * full git support
* minimalistic approach:
  * **pankat instance creates static html** documents
  * uses **sqlite database**
  * manage **documents with git**
  * hosting of documents with **nginx**
* **out of source** document builds

* full **theme-support**, asset folder contains
  * templates
  * css
  * js
  * fonts

# run

    nix develop
    just run

this command executes the WASM build, copies artefacts and starts the webserver. 

## configuration file

pankat supports:

* **command line** (see `pankat --help` for details)
* **environment variables** (most likely only used for these)
  * PANKAT_JWT_SECRET
  * PANKAT_ADMIN_PASSWORD
* config file `pankat.toml`
  * see the [pankat.toml](pankat.toml) for documentation

# architecture

![architecture](internals.svg)

# supported platforms

* linux (with nix)
* windows (untested)

## dependencies

* rust
* pandoc 3.x
* (see flake.nix dependencies)

# sqlite database

used for:

* storing parsed article structure
* article cache (so we don't need to run pandoc each tim)

pankat uses `diesel` to query the database.

## extend schema

    diesel print-schema > src/db/schema.rs

# development

* run the tests `just test`
* `just run`, then connect to localhost:5000 with any webbrowser
* you can delete **documents/output** and **documents/pankat.sqlite** then start `pankat` again
* use the `documents/example.de` documents, theme contributions are welcome

# deployment

currently: `just run`, then `cp target/debug/pankat /home/pankat-app/pankat`.

ideally: `just zig`, then `cp target/release/pankat /home/pankat-app/pankat` and later using a nix expression to build and use a read-only /nix/store for both pankat binary and artifacts.

## directory structure

```bash
/etc/nixos> ls /home/pankat-app/
╭───┬──────────────────────────────┬──────┬──────────┬─────────────╮
│ # │             name             │ type │   size   │  modified   │
├───┼──────────────────────────────┼──────┼──────────┼─────────────┤
│ 0 │ /home/pankat-app/documents   │ dir  │    108 B │ 8 hours ago │
│ 1 │ /home/pankat-app/pankat      │ file │ 117.6 MB │ 9 hours ago │
│ 2 │ /home/pankat-app/pankat.toml │ file │    948 B │ 9 hours ago │
╰───┴──────────────────────────────┴──────┴──────────┴─────────────╯

/etc/nixos> ls /home/pankat-app/documents/
╭───┬────────────────────────────────────────────┬──────┬─────────┬─────────────╮
│ # │                    name                    │ type │  size   │  modified   │
├───┼────────────────────────────────────────────┼──────┼─────────┼─────────────┤
│ 0 │ /home/pankat-app/documents/assets          │ dir  │    58 B │ 9 hours ago │
│ 1 │ /home/pankat-app/documents/blog.lastlog.de │ dir  │    48 B │ a day ago   │
│ 2 │ /home/pankat-app/documents/lastlog.de      │ dir  │    10 B │ a day ago   │
│ 3 │ /home/pankat-app/documents/output          │ dir  │ 11.1 kB │ 8 hours ago │
│ 4 │ /home/pankat-app/documents/pankat.sqlite   │ file │  2.4 MB │ 8 hours ago │
│ 5 │ /home/pankat-app/documents/wasm            │ dir  │   188 B │ 9 hours ago │
╰───┴────────────────────────────────────────────┴──────┴─────────┴─────────────╯
```

# configuration.nix

```nix
    lastlogblog = {
        serverName = "lastlog.de";
        serverAliases = [ "www.lastlog.de" ];
        forceSSL = true;
        enableACME = true;
        locations = {
          "/" = {
            proxyPass = "http://127.0.0.1:5000/";
          };
        };
        extraConfig = ''
            location /blog/api/ws {
              proxy_pass http://127.0.0.1:5000;
              proxy_set_header Host $host;
              proxy_http_version 1.1;
              proxy_set_header Upgrade $http_upgrade;
              proxy_set_header Connection "upgrade";
              proxy_read_timeout 86400;
            }
            location = / {
              return 301 https://lastlog.de/blog/index.html;
            }
            location = /blog {
              return 301 https://lastlog.de/blog/index.html;
            }
        '';
    };

    ...    

    systemd.services.pankat-app = {
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ];
        description = "Start the pankat server backend";
        environment = { RUST_LOG = "info"; };
        path = with pkgs; [ pandoc ];
        serviceConfig = {
            Restart = "always";
            Type = "simple";
            User = "pankat-app";
            ExecStart = "/home/pankat-app/pankat";
            WorkingDirectory = "/home/pankat-app";
        };
    };
    
    systemd.services.pankat-git-pull = {
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ];
        description = "Git pull for blog.lastlog.de every 10 minutes";
        path = with pkgs; [ git bash ];
        serviceConfig = {
            Type = "simple";
            User = "pankat-app";
            ExecStart =
            "${pkgs.bash}/bin/bash -c 'while true; do git -C /home/pankat-app/documents/blog.lastlog.de pull; sleep 600; done'";
            WorkingDirectory = "/home/pankat-app/documents/blog.lastlog.de";
        };
    };
```
