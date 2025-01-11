{pkgs}: {
  deps = [
    pkgs.diesel-cli
    pkgs.openssl
    pkgs.pkg-config
    pkgs.tmux
    pkgs.sqlite
  ];
}
