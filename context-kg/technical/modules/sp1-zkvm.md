---
name: "sp1-zkvm"
description: "Module design for sp1-zkvm: guest library and entrypoint for programs running inside the zkVM"
---
# sp1-zkvm Module

Spans `zkvm/lib` and `zkvm/entrypoint`.

## Responsibilities

- Provide guest-side library (`sp1-lib`) with syscall wrappers for I/O, hinting, committing, verification
- Define zkVM program entrypoint with `#[sp1_zkvm::entrypoint]` macro
- Provide `io::read()`, `io::write()`, `io::commit()` for guest programs
- Support in-zkVM proof verification via `sp1_zkvm::lib::verify::verify_sp1_proof()`

## NOT Responsible For

- Host-side proving or verification
- VM implementation (that is sp1-core-executor)
- Network communication

## Core Entities

| Entity | Key Fields | Description |
|--------|-----------|-------------|
| `sp1_zkvm::io` | read, write, commit, hint | Guest I/O primitives |
| `#[sp1_zkvm::entrypoint]` | (proc macro) | Program entry point |

## Dependencies

- Require to reference arch/dependency.md for full dependency details

## Relevant Flows

- Require to reference core-flows/ for flows involving this module

## Module-Specific Pitfalls

None identified
