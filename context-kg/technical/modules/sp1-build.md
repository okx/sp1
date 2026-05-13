---
name: "sp1-build"
description: "Module design for sp1-build: build-script helper for compiling SP1 programs to RISC-V ELF"
---
# sp1-build Module

## Responsibilities

- Compile SP1 programs to RISC-V ELF targeting `riscv64im-succinct-zkvm-elf`
- Provide `include_elf!` macro for embedding ELF at compile time
- Support Docker-based reproducible builds
- Manage toolchain installation and version detection

## NOT Responsible For

- Running proofs or verification
- RISC-V execution
- Any runtime functionality

## Core Entities

| Entity | Key Fields | Description |
|--------|-----------|-------------|
| `DEFAULT_TARGET` | `riscv64im-succinct-zkvm-elf` | Fixed compilation target |
| `include_elf!` | macro | Embed ELF bytes at compile time |

## Dependencies

- Require to reference arch/dependency.md for full dependency details

## Relevant Flows

- Require to reference core-flows/ for flows involving this module

## Module-Specific Pitfalls

[Pitfall] `--all-features` flag forces a specific build branch — feature flags may not be the right solution (TODO)

[Pitfall] `trim-paths` not yet supported — workaround in place for reproducible builds
