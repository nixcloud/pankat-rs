# motivation

pankat-replit is a reimplementation of https://github.com/nixcloud/pankat in rust (conversion from go).

the roadmap is here:

https://github.com/nixcloud/pankat/issues/7

# run

    just run

This command executes the WASM build, copies artefacts and starts the webserver. This is at times slow since it always checks WASM targets as well. So when not working on the WASM changes, run:

    just run

# architecture

![architecture](internals.svg)

# extend schema

    diesel print-schema > src/db/schema.rs