{ pkgs }:
with pkgs;
{
    deps = {
        #"pankat" = [ git ];
    };
    envs = {};
    project_src = {
        # include = [ ".git" ];
        exclude = [ "pankat-wasm" ];
    };
}
