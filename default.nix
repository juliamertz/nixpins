{
  rustPlatform,
  lib,
  makeWrapper,
  nix,
  nixfmt-rfc-style,
  ...
}:
rustPlatform.buildRustPackage rec {
  pname = "nixpins";
  version = "0.1.1";
  src = ./.;

  nativeBuildInputs = [ makeWrapper ];
  buildInputs = [
    nix
    nixfmt-rfc-style
  ];

  postInstall = ''
    wrapProgram "$out/bin/nixpins" --set PATH "${lib.makeBinPath buildInputs}"
  '';

  cargoLock = {
    lockFile = ./Cargo.lock;
    allowBuiltinFetchGit = true;
  };

  meta.mainProgram = "nixpins";
}
