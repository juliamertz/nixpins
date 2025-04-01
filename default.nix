{
  rustPlatform,
  lib,
  makeWrapper,
  nix,
  nixfmt-rfc-style,
  ...
}:
let
  runtimeDeps = lib.makeBinPath [
    nix
    nixfmt-rfc-style
  ];
in
rustPlatform.buildRustPackage {
    pname = "nixpins";
    version = "0.1.1";
  src = ./.;

  nativeBuildInputs = [ makeWrapper ];
  postInstall = ''
    wrapProgram "$out/bin/nixpins" --set PATH "${runtimeDeps}"
  '';

  cargoLock = {
    lockFile = ./Cargo.lock;
    allowBuiltinFetchGit = true;
  };

  meta.mainProgram = "nixpins";
}
