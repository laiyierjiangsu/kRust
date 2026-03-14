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

- `platform/windows/rust_core/`: current Windows-specific Rust `cdylib` sample
- `platform/windows/rust_build/`: Visual Studio makefile wrapper project that runs Cargo for the Windows sample
- `platform/windows/cpp_host/`: Visual Studio C++ console host for the Windows sample
- `common/`: shared code for future platform samples
- `common/include/rust_core.h`: shared C ABI header consumed by platform hosts
- `platform/windows/kRust.sln`: Visual Studio solution for the Windows sample entrypoint

## Machine Setup

1. Install Visual Studio 2019 or 2022.
2. Add the `Desktop development with C++` workload.
3. Ensure `MSVC v142` tools are installed, even on Visual Studio 2022.
4. Ensure Windows SDK `10.0.22621.0` is installed.
5. Install Rust and add the MSVC target:
   - `rustup toolchain install stable-x86_64-pc-windows-msvc`
   - `rustup target add x86_64-pc-windows-msvc`

## Build and Debug Flow

1. Open `platform/windows/kRust.sln` in Visual Studio 2019 or 2022.
2. Select `Debug | x64`.
3. Build the solution.
   - `platform/windows/rust_build` runs `cargo build`
   - `platform/windows/cpp_host` links against `rust_core.dll.lib`
   - post-build copies `rust_core.dll` and `rust_core.pdb` next to `cpp_host.exe`
   - Visual Studio intermediate outputs live under `platform/windows/build/`
4. Set `cpp_host` as the startup project.
5. Place a breakpoint in `platform/windows/cpp_host/main.cpp` and in `platform/windows/rust_core/src/lib.rs`.
6. Start debugging with `F5` and step into `rust_add`.

## Constraints

- First phase supports only pure integer functions.
- First phase supports only `Debug | x64`.
- The repository baseline is pinned to `v142` and the explicit Windows SDK version.
- Cargo remains the Rust build system; Visual Studio orchestrates the workflow.

