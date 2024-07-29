{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      naersk,
      nixpkgs-mozilla,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import nixpkgs-mozilla) ];
        };
        toolchain =
          (pkgs.rustChannelOf {
            rustToolchain = ./rust-toolchain.toml;
            sha256 = "sha256-oumiacOMp70g/4FHsbBdFdCy3OUiv1HSvnHAaQWfQOY=";
          }).rust;
        naersk-lib = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };
      in
      {
        packages.default = naersk-lib.buildPackage {
          src = ./.;
          nativeBuildInputs = [ pkgs.makeWrapper ];
          postInstall = '''';
        };
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              rustPackages.clippy
            ];
            nativeBuildInputs = [
              toolchain
              taplo
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      }
    );
}
