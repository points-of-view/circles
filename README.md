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

## Production setup
### Raspberry Pi 5 (Raspberry Pi OS/Debian Bookworm)
To run the circles application, you should:
* Set the Ethernet ipv4 settings from DHCP to Local-Link
* In file manager 'Preferences' > 'General' > Uncheck 'Don't ask options on launch executable file'
* Install the package through an AppImage Launcher ([example]([url](https://www.makeuseof.com/add-appimages-to-linux-system-menu/)))
* Put an icon on the Desktop


 <p xmlns:cc="http://creativecommons.org/ns#" xmlns:dct="http://purl.org/dc/terms/"><span property="dct:title">Circles</span> © 2024 by <span property="cc:attributionName">Tree company, Levuur, DYPALL, Danes je nov dan, stad Sint-Niklaas</span> is licensed under <a href="https://creativecommons.org/licenses/by/4.0/?ref=chooser-v1" target="_blank" rel="license noopener noreferrer" style="display:inline-block;">CC BY 4.0<img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1" alt=""><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/by.svg?ref=chooser-v1" alt=""></a></p> 