{
  description = "A flake to run erasmus";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    devshell = {
      url = "github:numtide/devshell";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, ... }:
    let
      inherit (flake-utils.lib) eachDefaultSystem eachSystem;
    in
    eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            inputs.devshell.overlays.default
          ];
        };
      in
      {
        devShells = rec {
          default = erasmus;
          erasmus = pkgs.devshell.mkShell {
            name = "erasmus";
            packages = [
              # Nix related packaged
              pkgs.nixpkgs-fmt

              # Node
              pkgs.nodejs_20
              pkgs.yarn

              # Rust
              pkgs.rustc
              pkgs.cargo
              pkgs.rustfmt
              pkgs.rust-analyzer
              pkgs.clippy
              pkgs.diesel-cli

              # SQLite
              pkgs.sqlite
            ];
            commands = [
              {
                name = "lint:check";
                category = "Linting";
                help = "Check for linting errors";
                command = ''
                  set +e
                  yarn lint:js
                  yarn lint:css
                  cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
                  ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check flake.nix
                '';
              }
              {
                name = "lint:fix";
                category = "Linting";
                help = "Fix linting errors";
                command = ''
                  set +e
                  yarn lint:js --fix
                  yarn lint:css --fix
                  cargo fmt --manifest-path src-tauri/Cargo.toml --all
                  ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt flake.nix
                '';
              }
            ];
            env = [
              {
                name = "DIESEL_CONFIG_FILE";
                value = "src-tauri/diesel.toml";
              }
            ];
          };
        };
      }
    );
}
