# Circles

Circles is an interactive phygital tool developed to conduct polls and quizes about relevant topics in society. This repository contains the application developed on the Tauri framework. For more in-depth information please refer to the [Installation Toolkit](installation_toolkit.pdf) which contains the exact materials, steps for set-up and tips to use the installation.

### The structure
This repository contains 4 main folders that define the structure of the program:
*  data: contains all public and persistent data that is shared within the application and required to run the application
*  frontend: contains all the frontend code of the Tauri application
*  main: contains all the backend code of the Tauri application
*  projects: contains the copy of the projects that were developed to run with the application

## Setup

### Requirements
This application is developped to run on a Raspberry Pi 5 with Raspberry Pi OS installed. The application is an interface that's made to connect with an RFID Reader, the Zebra FX9600. Other RFID readers using the LLRP may work but are not tested. For an extensive list of the exact materials, please refer to the Installation Toolkit linked above.

This application can run on all systems that tauri supports (though we only provide a build for Arm64-Linux). The application is an interface that connects with an RFID reader through LLRP. We made this using the Zebra FX9600. Other RFID readers from Zebra using LLRP may work but are not tested. Readers from other brands probably won't work, since we rely on some custom extensions to LLRP. For an extensive list of the exact materials, please refer to the Installation Toolkit linked above.
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
* Download the Arm64-Linux release
* Set the Ethernet ipv4 settings from DHCP to Local-Link
* In file manager 'Preferences' > 'General' > Uncheck 'Don't ask options on launch executable file'
* Install the package through an AppImage Launcher ([example]([url](https://www.makeuseof.com/add-appimages-to-linux-system-menu/)))
* Put an icon on the Desktop

### Other systems
Compile the source code for your specific platform (Windows, MacOS, ...) by using the `yarn tauri build` command. Look at tauri's guides for help on how to make production builds: https://v1.tauri.app/v1/guides/building/
 <img src="https://www.eacea.ec.europa.eu/sites/default/files/styles/embed_large_2x/public/2022-11/EN%20Co-Funded%20by%20the%20EU_POS.png?itok=l9sN2_3F" alt="Co-funded by the European Union">