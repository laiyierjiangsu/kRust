# Tasks: VS C++ Rust Debug Sample

## 1. Define baseline

- [x] 1.1 Document the supported Visual Studio versions: 2019 and 2022
- [x] 1.2 Pin the C++ baseline to `v142`
- [x] 1.3 Pin a shared Windows SDK version
- [x] 1.4 Pin Rust to `x86_64-pc-windows-msvc`
- [x] 1.5 Limit the first iteration to `Debug | x64`

## 2. Create Rust sample library

- [x] 2.1 Initialize a Rust library crate for the sample
- [x] 2.2 Configure the crate as a `cdylib`
- [x] 2.3 Export a minimal set of pure integer functions using `extern "C"`
- [x] 2.4 Ensure debug symbols are produced for stepping into Rust

## 3. Create Visual Studio solution structure

- [x] 3.1 Create a Visual Studio solution at the repository root
- [x] 3.2 Add a C++ console host project
- [x] 3.3 Add a Rust wrapper/build project to the solution
- [x] 3.4 Define project dependencies so Rust builds before C++
- [x] 3.5 Reorganize the Windows sample, including `kRust.sln`, under `platform/windows/` and keep shared assets under `common/`

## 4. Wire C++ to Rust

- [x] 4.1 Add a C-compatible header for the Rust functions
- [x] 4.2 Link the C++ host against the Rust import library
- [x] 4.3 Ensure the Rust DLL is copied or made discoverable at runtime
- [x] 4.4 Implement a minimal C++ call path that invokes the Rust functions

## 5. Validate debugging workflow

- [x] 5.1 Verify the C++ host starts under Visual Studio debugging
- [x] 5.2 Verify stepping from C++ into Rust source
- [x] 5.3 Verify Rust symbols are loaded predictably
- [x] 5.4 Verify the workflow on Visual Studio 2019 and Visual Studio 2022

## 6. Document machine setup

- [x] 6.1 Document the required Visual Studio workloads and optional components
- [x] 6.2 Document the required Rust toolchain and target
- [x] 6.3 Document the expected build and debug flow
- [x] 6.4 Document known constraints and non-goals for the sample
- [x] 6.5 Update repository layout documentation for the `platform/` and `common/` structure
