---
name: "sp1-core-machine"
description: "Module design for sp1-core-machine: AIR constraint columns and RISC-V chip definitions"
---
# sp1-core-machine Module

## Responsibilities

- Define `TrustMode` trait with `SupervisorMode` and `UserMode` implementations
- Provide AIR constraint columns for all RISC-V instruction types
- Define `RiscvAir` chip collection via `MachineAir` implementations
- Provide instruction column types per trust mode (adapter, syscall, page protection)
- Re-export `SP1RecursionProof` and I/O types (`SP1Stdin`)

## NOT Responsible For

- Executing the VM (that is sp1-core-executor)
- PCS commitment or sumcheck (that is sp1-hypercube and slop crates)
- User-facing API (that is sp1-sdk)

## Core Entities

| Entity | Key Fields | Description |
|--------|-----------|-------------|
| `TrustMode` | IS_TRUSTED, associated column types | Trait for trusted vs untrusted execution |
| `SupervisorMode` | IS_TRUSTED=true, EmptyCols for extras | Trusted program mode |
| `UserMode` | IS_TRUSTED=false, full protection cols | Untrusted program mode |
| `RiscvAir` | (chip enum) | Collection of all RISC-V AIR chips |

## Dependencies

- Require to reference arch/dependency.md for full dependency details

## Relevant Flows

- Require to reference core-flows/ for flows involving this module

## Module-Specific Pitfalls

[Pitfall] `SupervisorMode` uses `EmptyCols` for all extra column types — switching between modes requires rebuilding the entire machine

[Pitfall] `UserMode` adds trap/page-protection columns — the mprotect feature flag must be consistently enabled across the pipeline
