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
    crane = {
      # Use a fork until https://github.com/ipetkov/crane/pull/358 is merged
      url = "github:shimunn/crane?ref=tauri-package";
      inputs.nixpkgs.follows = "nixpkgs";
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
        craneLib = inputs.crane.lib.${system};

        frontend = pkgs.mkYarnPackage {
          pname = "erasmus-frontend";
          version = "unstable";

          src = pkgs.lib.cleanSource ./src;

          packageJSON = ./src/package.json;
          yarnLock = ./yarn.lock;

          buildPhase = ''
            yarn run build
          '';

          installPhase = ''
            cp -r deps/frontend/dist $out
          '';

          distPhase = "true";
        };

        tauriArgs = {
          src = pkgs.lib.cleanSourceWith {
            src = ./src-tauri; # The original, unfiltered source
            filter = path: type:
              (pkgs.lib.hasSuffix "\.html" path) ||
              (pkgs.lib.hasSuffix "\.scss" path) ||
              (pkgs.lib.hasSuffix "\.css" path) ||
              (pkgs.lib.hasSuffix "\.json" path) ||
              (pkgs.lib.hasSuffix "\.png" path) ||
              # Example of a folder for images, icons, etc
              (pkgs.lib.hasInfix "/assets/" path) ||
              # Default filter from crane (allow .rs files)
              (craneLib.filterCargoSources path type)
            ;
          };
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [
            # Add additional build inputs here
          ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
            pkgs.webkitgtk
            pkgs.gtk3
            pkgs.cairo
            pkgs.gdk-pixbuf
            pkgs.glib
            pkgs.dbus
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
            pkgs.darwin.apple_sdk.frameworks.AppKit
            pkgs.darwin.apple_sdk.frameworks.WebKit
          ];
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly (tauriArgs);

      in
      {
        packages = rec {
          default = erasmus;
          erasmus = craneLib.buildTauriPackage (tauriArgs // {
            pname = "erasmus";
            version = "unstable";
            inherit cargoArtifacts;
            tauriConfigPath = ./src-tauri/tauri.conf.json;
            tauriDistDir = frontend;
          });
        };
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
              {
                name = "DATABASE_URL";
                eval = "$PRJ_DATA_DIR/erasmus.sqlite";
              }
            ];
          };
        };
      }
    );
}
