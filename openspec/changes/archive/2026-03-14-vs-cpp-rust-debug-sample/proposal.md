# Change Proposal: VS C++ Rust Debug Sample

## Why

This repository needs a minimal Windows-native sample that demonstrates how a C++ application can call into a Rust library and be debugged as one workflow from Visual Studio.

The sample must be usable on two developer machines with different IDE versions:

- machine A uses Visual Studio 2022
- machine B uses Visual Studio 2019

The change should therefore optimize for the lowest common working baseline rather than the newest available tooling.

## What Changes

Create a sample project that:

- exposes a small set of pure functions from Rust through a C ABI
- hosts those functions in a Visual Studio C++ application
- includes both the C++ host and a Rust build project inside one Visual Studio solution
- supports stepping from C++ into Rust during debugging
- is constrained to a shared Windows baseline that works on both VS2019 and VS2022
- organizes platform-specific sample code, including the Visual Studio solution entrypoint, under `platform/windows/` and shared assets under `common/`

## Scope

In scope:

- a Rust library compiled for `x86_64-pc-windows-msvc`
- a C++ console host application
- a Visual Studio solution that includes a Rust build wrapper project and the C++ host
- project dependency ordering so Rust builds before C++
- a documented shared baseline for toolset, SDK, architecture, and debug flow

Out of scope:

- complex FFI types such as strings, structs, handles, or callbacks
- GUI hosting
- Win32 support
- release packaging
- CI automation

## Success Criteria

The change is successful when:

1. the solution opens on both Visual Studio 2019 and Visual Studio 2022
2. both environments can build the sample using the same shared baseline
3. the C++ host can call Rust pure functions successfully
4. a developer can start debugging from the C++ host and step into Rust source

## Risks

- Visual Studio 2022 may silently encourage migration to newer toolsets; the project must stay pinned to the shared baseline
- symbol loading can fail if Rust outputs, DLL paths, or PDB paths are not organized predictably
- cross-machine drift can occur if the Windows SDK or Rust toolchain are left implicit
