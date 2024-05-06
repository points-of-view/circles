# Circles

## Setup
It's recommended to use nix when working on circles, this will ensure that you have all the necessary dependencies and that we all use the same version of dependencies.

Refer to the [the nixos documentation](https://nixos.org/download#nix-install-macos) setup nix.

Once you have nix installed, you can run `nix develop` or use [direnv](https://direnv.net/) to load the devshell.

## Local development
* Install dependencies with `yarn install`
* Run the app with `yarn tauri dev`

If you don't want to (or can't) use an actual RFID reader, set an environment variable `MOCK_RFID_READER=1` to skip this process.

> [!NOTE]
> When not using Nix to manage your environment:
> * Make sure you have rust, node, java, and the diesel-cli installed.
> * Make sure you have the necessary environment variables set. See [`flake.nix`](./flake.nix) for details.

### Tests
You can run the tests with `cargo test`

## Production set-up

### Raspberry Pi 5 (Raspberry Pi OS/Debian Bookworm)
For running the circles application you should:
- Set the Ethernet ipv4 settings from DHCP to Local-Link
- In file manager 'Preferences' > 'General' > Uncheck 'Don't ask options on launch executable file'
- Install the package through an AppImage Launcher ([example]([url](https://www.makeuseof.com/add-appimages-to-linux-system-menu/)))
- Put an icon on the Desktop