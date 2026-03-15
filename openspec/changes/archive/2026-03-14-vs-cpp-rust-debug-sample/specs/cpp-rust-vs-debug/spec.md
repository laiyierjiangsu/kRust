# Capability: C++ to Rust Debug Sample

## Added Requirements

### Requirement: Shared Visual Studio baseline

The sample project MUST build on both Visual Studio 2019 and Visual Studio 2022 by targeting a shared baseline configuration.

The shared baseline MUST define:

- the C++ platform toolset
- the Windows SDK version
- the Rust target triple
- the supported architecture
- the supported configuration set for the first phase

#### Scenario: Open solution on either supported IDE

- **GIVEN** a developer machine with either Visual Studio 2019 or Visual Studio 2022 installed
- **AND** the machine satisfies the repository baseline prerequisites
- **WHEN** the developer opens the solution
- **THEN** the solution is loadable without requiring project migration to a newer toolset

### Requirement: Rust project must be managed inside the solution

The solution MUST include a Rust-related project that participates in the Visual Studio build graph.

#### Scenario: Build Solution includes Rust

- **GIVEN** the solution is opened in a supported Visual Studio version
- **WHEN** the developer runs Build Solution
- **THEN** the Rust build project executes before the C++ host project
- **AND** the Rust native outputs needed by the host are produced before C++ link time

### Requirement: C++ host calls Rust pure functions

The sample MUST demonstrate C++ calling Rust through a C ABI using pure functions only in the first phase.

#### Scenario: Call a Rust pure function from C++

- **GIVEN** the solution has been built successfully
- **WHEN** the C++ host invokes a Rust-exported pure function with integer arguments
- **THEN** the function returns the expected integer result

### Requirement: Visual Studio debugging crosses the language boundary

The sample MUST support starting under the C++ host debugger and stepping into Rust source.

#### Scenario: Step from C++ into Rust

- **GIVEN** the solution is built in `Debug | x64`
- **AND** Rust debug symbols are available to the debugger
- **WHEN** the developer starts debugging from the C++ host and steps into a Rust call
- **THEN** the debugger enters Rust source for the called function