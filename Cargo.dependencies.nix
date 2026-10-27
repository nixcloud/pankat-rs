{ pkgs }:
with pkgs;
{
    deps = {
        #"openssl-sys" = [ pkg-config openssl ];
        #"cargo" = [ git ];
    };
    envs = {};
    project_src = {
        #include = [ ".git" ];
        #exclude = [ "nix" "nix_" "nix__" ];
    };
}
