# poly_ui

[![GitHub Actions](https://github.com/PiotrMoscicki/poly_ui_proto/workflows/main%20CI/badge.svg)](https://github.com/PiotrMoscicki/poly_ui_proto/actions)
[![GitHub Actions](https://github.com/PiotrMoscicki/poly_ui_proto/workflows/dev%20CI/badge.svg)](https://github.com/PiotrMoscicki/poly_ui_proto/actions)
[![License: LGPL v3](https://img.shields.io/badge/License-LGPL%20v3-blue.svg)](https://www.gnu.org/licenses/lgpl-3.0)
[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/PiotrMoscicki/poly_ui_proto)

User interface library for Rust using SDL2

## Quick start
### Windows
Clone and run. The build script (build.rs) will copy necessary libraries to the root dir of the cloned repository.

### Linux
Install necessary SDL2 libraries:
```sh
sudo apt-get install -y libsdl2-dev
sudo apt-get install -y libsdl2-ttf-dev
```
Clone and run.

### macOS
Install necessary SDL2 libraries:
```terminal
brew install SDL2
brew install SDL2_ttf
```
Clone and run.

##
After running you should see a test window:
![Image](/media/README/example_run.bmp "icon")
