{
  rustPlatform,
  lib,
  makeWrapper,
  nix,
  ...
}:
rustPlatform.buildRustPackage rec {
  pname = "nixpins";
  version = "0.1.2";
  src = ./.;

  nativeBuildInputs = [ makeWrapper ];
  buildInputs = [
    nix
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
