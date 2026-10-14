# what

**pankat** is a **static blog generator** with a live preview: it renders markdown articles to
static html, which nginx (or pankat itself) serves, and while you write, pankat watches the
documents folder and updates open browser tabs via websocket.

![A screenshot featuring pankat](./.screenshots/pankat.jpg)

# features

* writing
  * **markdown** articles (`*.mdwn`), rendered with **pandoc 3.x**
  * **plugins** in the articles for metadata, images, drafts, ... (see [plugins](#plugins))
  * **live updates**: a file system watcher (inotify) rebuilds changed articles and pushes the
    new html to all open browser tabs via websocket
    * neighbouring articles (prev/next, series) and the timeline are rebuilt as well
    * renaming an article shows readers of the old page a link to the new location, removing
      it a link to the blog's index
    * removing a plugin call (e.g. `[[!series ...]]`) removes its effect again
  * **drafts** (`[[!draft]]`) are only visible to the admin, rendered on request and never
    written to the public output
  * documents are plain files, manage them with **git**
* reading
  * **table of contents** (jquery.tocify) and **page anchors** for headings
  * **timeline** of all articles with a **tag** and **series** filter
  * **prev/next** navigation and **series** navigation
  * the index redirects to the most recent article
  * **Open Graph / twitter card** tags for social previews (title, summary, image, tags)
  * a **404 page** in the look of the blog
* admin
  * a single user, the admin: `/login` asks for the `admin_password` only
  * `/drafts/` lists all drafts newest first, `/specialpages/` all special pages
  * the session lasts 50 days (http-only cookie with a JWT signed by `jwt_token`)
* minimalistic
  * the output is **static html**, hosted with **nginx** in production
  * **sqlite** database with a cache of the pandoc output, unchanged articles aren't parsed
    again on startup
  * **out of source** builds: input, output, assets and database are separate directories
  * stale files in the output directory are removed on startup
  * all templates and pandoc filters are checked on startup, a missing or broken one stops
    pankat with a list of all problems instead of failing in the middle of a build
* full **theme support**, the assets folder contains
  * templates (handlebars)
  * css
  * js
  * fonts
  * pandoc lua filters

# plugins

plugins are written as `[[!name argument]]` into an article, names are case insensitive.

| plugin | example | effect |
|---|---|---|
| `meta` | `[[!meta date="2026-10-12 12:00"]]` | date of the article, orders the timeline and prev/next |
| `title` | `[[!title my article]]` | title, defaults to the file name |
| `summary` | `[[!summary one sentence about it]]` | shown in the timeline and as social preview description |
| `tag` | `[[!tag nix rust]]` | tags, whitespace separated |
| `series` | `[[!series libnix]]` | adds the article to a series with its own navigation |
| `img` | `[[!img media/logo.png class="noFancy" style="float: right"]]` | image linking to itself, further attributes are passed on |
| `draft` | `[[!draft]]` | only the admin can see the article below `/drafts/` |
| `SpecialPage` | `[[!SpecialPage]]` | a page like about or imprint: not in the timeline, no prev/next |
| `OpenGraphImage` | `[[!OpenGraphImage posts/libnix/card.png]]` | image for social previews, without it a small card is used |
| `hiddentext` | `[[!hiddentext the solution]]` | text hidden behind a "show" toggle |

# run

    nix develop
    cargo build
    cargo run

then open http://localhost:5000 (see `port` and `subdir` in [pankat.toml](pankat.toml)).

`just run` additionally builds the wasm client first, see below for what builds where.

## building

`nix develop` provides **cargo+libnix**, a cargo which builds every crate as its own nix
derivation (`CARGO_BACKEND=nix` is set in the dev shell). at the moment it can build the main
webserver, but not the static musl binary of `just zig` nor `pankat-wasm`.

| what | command | with cargo+libnix |
|---|---|---|
| webserver | `cargo build` | yes |
| webserver as nix package | `nix build .#pankat` | yes |
| tests | `CARGO_BACKEND=legacy cargo test` | no, `cargo test` needs the legacy backend |
| static musl binary for deployment | `just zig` | no, not at the moment |
| wasm client (live updates in the browser) | `just build-pankat-wasm copy-pankat-wasm` | no, not at the moment |

`CARGO_BACKEND=legacy` switches to the regular cargo behaviour. the wasm client ends up in
`documents/wasm`, pankat serves it from the `wasm` directory.

## configuration

pankat reads, in this order (later ones win):

* defaults of the **command line** options (see `pankat --help`)
* the config file **`pankat.toml`** in the working directory, documented in
  [pankat.toml](pankat.toml)
* **environment variables** `PANKAT_<OPTION>`, e.g. `PANKAT_JWT_TOKEN`,
  `PANKAT_ADMIN_PASSWORD` or `PANKAT_SITE_URL`; use these for the secrets
* **command line** options which were set explicitly

| option | meaning |
|---|---|
| `input` | the documents (`*.mdwn`, `posts/`, `media/`) |
| `output` | generated html, can be deleted and regenerated |
| `assets` | templates, css, js, fonts, pandoc lua filters |
| `wasm` | the wasm client |
| `database` | directory of `pankat.sqlite` |
| `site_url` | public URL incl. subdir, e.g. `https://lastlog.de/blog`, for the label top left and the social previews |
| `subdir` | path the blog lives below, e.g. `/blog` or `""` |
| `port` | port of the webserver |
| `flat` | `documents/mydir/myarticle.mdwn` becomes `output/myarticle.html` instead of `output/mydir/myarticle.html` |
| `static_build_only` | only build the documents and exit |
| `jwt_token` | server side secret signing the session cookies, changing it logs out all sessions |
| `admin_password` | password of the admin for `/login` |

the login is disabled while `jwt_token` or `admin_password` is `"not set"`.

## admin login

there are no user accounts, pankat has a single user: the admin. `/login` shows a password field
only and compares the input with `admin_password`. a successful login stores a session token (a
JWT signed with `jwt_token`, valid for 50 days) in an http-only cookie, and the navbar shows the
links to the drafts, the special pages and a logout button. a failed login waits a second
before answering.

opening `/drafts/` or `/specialpages/` without a session redirects to the login and back to the
requested page afterwards.

## urls

below `subdir`:

| url | |
|---|---|
| `/`, `/index.html` | redirect to the most recent article |
| `/<article>.html` | an article |
| `/timeline.html` | all articles with tag/series filter |
| `/posts/...`, `/media/...` | files from the input directory |
| `/assets/...`, `/wasm/...` | theme and wasm client |
| `/login`, `/logout` | admin login (`POST /logout`) |
| `/drafts/`, `/drafts/<article>.html` | drafts, admin only |
| `/specialpages/` | list of the special pages, admin only |
| `/api/ws` | websocket for live updates |
| `/api/session` | `{"admin": true/false}`, used by the navbar |

everything else answers with the 404 page.

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
* article cache (so we don't need to run pandoc each time)

pankat uses `diesel` to query the database, migrations are embedded into the binary and run on
startup.

## extend schema

    diesel print-schema > src/db/schema.rs

# development

* run the tests with `CARGO_BACKEND=legacy cargo test`
* `cargo run`, then connect to localhost:5000 with any webbrowser
* you can delete **documents/output** and **documents/pankat.sqlite** then start `pankat` again
* use the `documents/example.de` documents, theme contributions are welcome

# deployment

    just zig
    scp target/x86_64-unknown-linux-musl/release/pankat webservi:/tmp

`just zig` builds a static musl binary with cargo-zigbuild, it doesn't work with cargo+libnix at
the moment (see [building](#building)).

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

in production nginx serves the static files from `documents/output`, `documents/assets`, ...
directly and passes only the websocket, the login, the drafts, the special pages and the
session state to `pankat`. live updates still work, pankat rewrites the html documents in
`documents/output` on file system changes.

```nix
users.users = {
  pankat-app = {
    isSystemUser = true;
    home = "/pankat-app";
    group = "pankat-app";
    description = "pankat app users";
  };
};
users.groups.pankat-app = {};

security.acme = {
  acceptTerms = true;
  defaults.email = "info@nixcloud.io";
};

services.nginx = {
  enable = true;
  recommendedGzipSettings = true;
  recommendedOptimisation = true;
  virtualHosts = {
    lastlogblog = {
      serverName = "lastlog.de";
      serverAliases = [ "www.lastlog.de" ];
      forceSSL = true;
      enableACME = true;
      extraConfig = ''
        location /blog/api/ws {
          proxy_pass http://127.0.0.1:5000;
          proxy_set_header Host $host;
          proxy_http_version 1.1;
          proxy_set_header Upgrade $http_upgrade;
          proxy_set_header Connection "upgrade";
          proxy_read_timeout 86400;
        }
        # admin login, drafts, special pages and the session state for the navbar are
        # served by pankat, longer prefixes than ^~ /blog/ below so they win
        location ^~ /blog/login { proxy_pass http://127.0.0.1:5000; proxy_set_header Host $host; }
        location ^~ /blog/logout { proxy_pass http://127.0.0.1:5000; proxy_set_header Host $host; }
        location ^~ /blog/draft { proxy_pass http://127.0.0.1:5000; proxy_set_header Host $host; }
        location ^~ /blog/specialpages { proxy_pass http://127.0.0.1:5000; proxy_set_header Host $host; }
        location = /blog/api/session { proxy_pass http://127.0.0.1:5000; proxy_set_header Host $host; }
        # written by pankat to the output directory
        error_page 404 /blog/404.html;
        location ^~ /blog/posts/ {
          alias /pankat-app/documents/blog.lastlog.de/posts/;
          autoindex on;
        }
        location ^~ /blog/media/ {
          alias /pankat-app/documents/blog.lastlog.de/media/;
          autoindex on;
        }
        location ^~ /blog/assets/ {
          alias /pankat-app/documents/assets/;
          autoindex on;
        }
        location ^~ /blog/wasm/ {
          alias /pankat-app/documents/wasm/;
          autoindex on;
        }
        location ^~ /blog/ {
          alias /pankat-app/documents/output/;
          autoindex on;
        }
        location = / {
          return 301 https://lastlog.de/blog/index.html;
        }
        location = /blog {
          return 301 https://lastlog.de/blog/index.html;
        }
      '';
    };
  };
};
```

# commands

```bash
setfacl -d -m u:joachim:rwx /pankat-app
setfacl -R -m u:joachim:rx /pankat-app
setfacl -R -m u:nginx:rx /pankat-app
setfacl -d -m u:nginx:rx /pankat-app
```
