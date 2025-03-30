{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import (nixpkgs) { inherit system overlays; };

        rust-bin = pkgs.rust-bin.stable.latest;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust-bin.minimal;
          rustc = rust-bin.minimal;
        };
      in
      {
        packages = {
          default = pkgs.callPackage ./default.nix { inherit rustPlatform; };
        };

        devShells.default = pkgs.mkShell {
          packages = with rust-bin; [
            (minimal.override {
              extensions = [
                "clippy"
                "rust-src"
              ];
            })

            rustfmt
            rust-analyzer
          ];
        };
      }
    );
}
