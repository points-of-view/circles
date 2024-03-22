# Zebra Reader

This program uses the CMake build system to generate the build files for your platform. 

## Set up the build environment

```bash
cmake -S . -B build/
```

This will generate platform-specific Make files in the build directory that can be used to build the project: 

## Building

```bash
cmake --build build
```

This should create a binary executable `zebra-reader` in the `/build` directory.
