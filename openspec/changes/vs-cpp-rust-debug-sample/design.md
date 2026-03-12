# Design: VS C++ Rust Debug Sample

## Overview

The sample will use a split-responsibility model:

- Cargo remains responsible for Rust compilation
- Visual Studio remains responsible for solution orchestration, C++ build, startup, and debugging

This design keeps Rust aligned with standard ecosystem tooling while still making Rust appear as a managed project inside the Visual Studio solution.

## Architecture

```text
kRust.sln
©À©¤ rust_build
©¦  ©¸©¤ invokes cargo build for rust_core
©¸©¤ cpp_host
   ©¸©¤ links rust_core import library and starts debugging

rust_core (Cargo crate)
  -> rust_core.dll
  -> rust_core.lib
  -> rust_core.pdb
```

Runtime flow:

```text
Visual Studio F5
  -> launches cpp_host.exe
  -> cpp_host calls Rust C ABI function
  -> debugger loads Rust symbols
  -> Step Into enters Rust source
```

## Repository Layout

Planned layout:

```text
kRust/
©À©¤ rust_core/
©¦  ©À©¤ Cargo.toml
©¦  ©¸©¤ src/lib.rs
©À©¤ rust_build/
©¦  ©¸©¤ rust_build.vcxproj
©À©¤ cpp_host/
©¦  ©À©¤ cpp_host.vcxproj
©¦  ©À©¤ main.cpp
©¦  ©¸©¤ include/rust_core.h
©¸©¤ kRust.sln
```

## Rust Library Design

Rust will be compiled as a `cdylib` targeting `x86_64-pc-windows-msvc`.

The first version intentionally exposes only pure integer functions through a stable C ABI, for example:

- `rust_add(int a, int b) -> int`
- `rust_sub(int a, int b) -> int`
- `rust_mul(int a, int b) -> int`

Reasons:

- keeps ABI simple
- isolates the investigation to build, linking, symbol loading, and stepping
- avoids early complexity around ownership and cross-language memory rules

## Visual Studio Design

### Shared Baseline

The solution must target the following shared baseline:

- Visual Studio support: 2019 and 2022
- C++ toolset: `v142`
- Windows SDK: pinned to one explicit version shared by both machines (implemented as `10.0.22621.0`)
- architecture: `x64`
- configuration for phase 1: `Debug`

### Rust Project in Solution

The Rust side will be represented in the solution by a wrapper project instead of relying on a first-class Visual Studio Rust project type.

The wrapper project is responsible for:

- Build: run `cargo build`
- Rebuild: clean and then build
- Clean: remove Rust outputs needed by the sample workflow

This makes Rust visible in Solution Explorer and part of Build Solution without replacing Cargo.

### C++ Host Project

The C++ host project is the startup project and depends on the Rust wrapper project.

Responsibilities:

- include the generated or hand-written C header
- link against the Rust import library
- ensure the Rust DLL is available at runtime
- provide a simple call path that can be stepped through in the debugger

## Build and Output Strategy

The build graph should be:

```text
Build Solution
  -> rust_build
  -> cpp_host
```

Output handling should separate:

- Rust native outputs in Cargo's standard target directory
- C++ runnable outputs in the Visual Studio output directory

The design must include a deterministic handoff so that:

- the linker finds the Rust import library
- the runtime finds the Rust DLL
- the debugger finds the Rust PDB

## Compatibility Strategy

To keep the project working on both machines:

- pin the C++ project to `v142`
- avoid automatic migration to `v143`
- avoid implicit SDK selection
- pin Rust to the MSVC target
- avoid absolute machine-specific paths

The governing rule is:

```text
the repository defines the baseline;
the machine must satisfy it
```

## Non-Goals

This design does not attempt to:

- make Visual Studio the primary Rust dependency manager
- model advanced Rust types across FFI
- support multiple architecture/configuration matrices in the first phase
- optimize release distribution
