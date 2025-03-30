{
  rustPlatform,
  lib,
  makeWrapper,
  nix,
  nixfmt-rfc-style,
  ...
}:
let
  manifest = lib.importTOML ./Cargo.toml;
  runtimeDeps = lib.makeBinPath [
    nix
    nixfmt-rfc-style
  ];
in
rustPlatform.buildRustPackage {
  inherit (manifest.package) name version;
  src = ./.;

  nativeBuildInputs = [ makeWrapper ];
  postInstall = ''
    wrapProgram "$out/bin/nixpins" --set PATH "${runtimeDeps}"
  '';

  cargoLock = {
    lockFile = ./Cargo.lock;
    allowBuiltinFetchGit = true;
  };

  meta.mainProgram = manifest.package.name;
}
