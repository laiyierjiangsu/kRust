# kRust VS C++ / Rust Debug Sample

This repository contains a minimal Windows sample for calling Rust from C++ and debugging the call path from Visual Studio.

## Shared Baseline

- Visual Studio 2019 or Visual Studio 2022
- Desktop development with C++ workload
- MSVC v142 build tools installed on both machines
- Windows 10 SDK `10.0.22621.0`
- Rust stable toolchain
- Rust target `x86_64-pc-windows-msvc`
- Supported configuration in phase 1: `Debug | x64`

## Repository Layout

- `rust_core/`: Rust `cdylib` exporting pure integer functions through a C ABI
- `rust_build/`: Visual Studio makefile wrapper project that runs Cargo
- `cpp_host/`: Visual Studio C++ console host that links against the Rust import library
- `kRust.sln`: Visual Studio solution containing both projects

## Machine Setup

1. Install Visual Studio 2019 or 2022.
2. Add the `Desktop development with C++` workload.
3. Ensure `MSVC v142` tools are installed, even on Visual Studio 2022.
4. Ensure Windows SDK `10.0.22621.0` is installed.
5. Install Rust and add the MSVC target:
   - `rustup toolchain install stable-x86_64-pc-windows-msvc`
   - `rustup target add x86_64-pc-windows-msvc`

## Build and Debug Flow

1. Open `kRust.sln` in Visual Studio 2019 or 2022.
2. Select `Debug | x64`.
3. Build the solution.
   - `rust_build` runs `cargo build`
   - `cpp_host` links against `rust_core.dll.lib`
   - post-build copies `rust_core.dll` and `rust_core.pdb` next to `cpp_host.exe`
4. Set `cpp_host` as the startup project.
5. Place a breakpoint in `cpp_host/main.cpp` and in `rust_core/src/lib.rs`.
6. Start debugging with `F5` and step into `rust_add`.

## Constraints

- First phase supports only pure integer functions.
- First phase supports only `Debug | x64`.
- The repository baseline is pinned to `v142` and the explicit Windows SDK version.
- Cargo remains the Rust build system; Visual Studio orchestrates the workflow.

