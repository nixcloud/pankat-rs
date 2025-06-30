{ lib, stdenv, rustPlatform, fetchFromGitHub, pkg-config, openssl, libiconv, sqlite }:

rustPlatform.buildRustPackage rec {
  pname = "pankat";
  version = "0.1.1";

  src = ./.;

  buildInputs = [ openssl sqlite pkg-config ];

  useFetchCargoVendor = true;
  cargoHash = "sha256-/lTxF1Sqj98ae6hZUlWD52xEFVEE0uWp42gTOv8nqn8=";

  meta = with lib; {
    homepage = "https://github.com/nixcloud/pankat-rs";
    description = "";
    maintainers = with maintainers; [ qknight ];
    license = with licenses; [ agpl3Plus ];
  };
}
