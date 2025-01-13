{pkgs}: {
  deps = [
    pkgs.pandoc
    pkgs.tig
    pkgs.diesel-cli
    pkgs.openssl
    pkgs.pkg-config
    pkgs.tmux
    pkgs.sqlite
  ];
}
