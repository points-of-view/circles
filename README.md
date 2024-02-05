# Erasmus

## Setup
It's recommended to use nix when working on erasmus, this will ensure that you have all the necessary dependencies and that we all use the same version of dependencies.

Refer to the [the nixos documentation](https://nixos.org/download#nix-install-macos) setup nix.

Once you have nix installed, you can run `nix develop` or use [direnv](https://direnv.net/) to load the devshell.

## Local development
* Install dependencies with `yarn install`
* Run the app with `yarn tauri dev`

> [!NOTE]
> Make sure you have rust and node installed when not using Nix to manage your environment.
