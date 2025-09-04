# Circles
## Introduction

### Description
Circles is an interactive phygital tool developed to conduct polls and quizes about relevant topics in society. This repository contains the application developed on the Tauri framework. For more in-depth information please refer to the [Installation Toolkit](https://docs.google.com/document/d/1k6dQ_dArw55UN4PFimkeRfo-HTY-j-_cZE4hhaQofas/edit?usp=sharing) which contains the exact materials, steps for set-up and tips to use the installation.

### The structure
This repository contains 4 main folders that define the structure of the program:
*  data: contains all public and persistent data that is shared within the application and required to run the application
*  frontend: contains all the frontend code of the Tauri application
*  main: contains all the backend code of the Tauri application
*  projects: contains the copy of the projects that were developed to run with the application

## Setup

### Requirements
This application is developped to run on a Raspberry Pi 5 with Raspberry Pi OS installed. The application is an interface that's made to connect with an RFID Reader, the Zebra FX9600. Other RFID readers using the LLRP may work but are not tested. For an extensive list of the exact materials, please refer to the Installation Toolkit linked above.

### Recommendations
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
Compile the source code for your specific platform (Windows, MacOS, ...) by using the `yarn tauri build` command.

 <p xmlns:cc="http://creativecommons.org/ns#" xmlns:dct="http://purl.org/dc/terms/"><span property="dct:title">Circles</span> © 2024 by <span property="cc:attributionName">Tree company, Levuur, DYPALL, Danes je nov dan, stad Sint-Niklaas</span> is licensed under <a href="https://creativecommons.org/licenses/by/4.0/?ref=chooser-v1" target="_blank" rel="license noopener noreferrer" style="display:inline-block;">CC BY 4.0<img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1" alt=""><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/by.svg?ref=chooser-v1" alt=""></a></p> 
 <img src="https://www.eacea.ec.europa.eu/sites/default/files/styles/embed_large_2x/public/2022-11/EN%20Co-Funded%20by%20the%20EU_POS.png?itok=l9sN2_3F" alt="Co-funded by the European Union">