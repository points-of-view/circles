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
            src = ./main; # The original, unfiltered source
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
            tauriConfigPath = ./main/tauri.conf.json;
            tauriDistDir = frontend;
          });
          reader = pkgs.stdenv.mkDerivation {
            pname = "erasmus-reader";
            version = "unstable";
            # This filters with the default `cleanSourceFilter` with one exception for `.so` files
            src = pkgs.lib.cleanSourceWith {
              filter = path: type: (pkgs.lib.hasSuffix "\.so" path) || (pkgs.lib.cleanSourceFilter path type);
              src = ./reader;
            };
            nativeBuildInputs = [ pkgs.makeWrapper pkgs.jdk ];

            buildPhase = ''
              mkdir -p ./dist/META-INF 
              echo "Main-Class: reader.PrintRFIDReader.PrintRFIDTags" > ./dist/META-INF/MANIFEST.MF
              javac -cp ./vendor/zebra/lib/Symbol.RFID.API3.jar -d ./dist ./PrintRFIDReader/PrintRFIDTags.java
              cd dist
              jar -cmvf META-INF/MANIFEST.MF reader.jar reader/PrintRFIDReader/PrintRFIDTags.class
            '';

            installPhase = ''
              mkdir -pv $out/share/java $out/bin
              cp -r $TMP/source/vendor $out/share/vendor
              cp -r $TMP/source/dist/reader.jar $out/share/java/reader.jar
              makeWrapper ${pkgs.jre}/bin/java $out/bin/reader \
                --add-flags "-cp $out/share/vendor/zebra/lib/Symbol.RFID.API3.jar -jar $out/share/java/reader.jar" \
                --set _JAVA_OPTIONS "-Djava.library.path=$out/share/vendor/zebra/lib/x86_64" \
                --set LD_LIBRARY_PATH ${pkgs.lib.makeLibraryPath [ "$out/share/vendor/zebra/lib/x86_64" ]}
            '';
          };
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

              # Java
              pkgs.jdk
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
                  cargo fmt --manifest-path main/Cargo.toml --all -- --check
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
                  cargo fmt --manifest-path main/Cargo.toml --all
                  ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt flake.nix
                '';
              }
              {
                name = "reader:start";
                help = "Boot reader service";
                command = ''
                  LD_LIBRARY_PATH="reader/vendor/zebra/lib/x86_64" \
                  java -Djava.library.path="reader/vendor/zebra/lib/x86_64" \
                       -cp reader/vendor/zebra/lib/Symbol.RFID.API3.jar \
                       reader/PrintRFIDReader/PrintRFIDTags.java
                '';
              }
            ];
            env = [
              {
                name = "DIESEL_CONFIG_FILE";
                value = "main/diesel.toml";
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
