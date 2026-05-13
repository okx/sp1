---
name: "sp1-core-executor"
description: "Module design for sp1-core-executor: RISC-V VM execution with precompile support"
---
# sp1-core-executor Module

## Responsibilities

- Implement cycle-accurate RISC-V VM (64-bit) via `CoreVM<M: ExecutionMode>`
- Execute ALU, branch, jump, load/store, ecall instructions
- Support `SupervisorMode` (trusted, no page protection) and `UserMode` (page protection checks)
- Generate `ExecutionRecord` (execution trace) with typed event vectors
- Handle syscalls: HALT, WRITE, HINT_LEN/READ, COMMIT, ENTER/EXIT_UNCONSTRAINED, and all precompiles
- Manage execution state: PC, clock, memory, page protections, I/O streams
- Support shard splitting via `SplitOpts` and `ExecutionRecord::defer()`
- Provide JIT compilation support via `sp1-jit`

## NOT Responsible For

- Generating AIR traces directly (that is sp1-core-machine + sp1-hypercube)
- ZK proving (that is sp1-hypercube)
- Recursion or proof composition (that is sp1-prover)

## Core Entities

| Entity | Key Fields | Description |
|--------|-----------|-------------|
| `CoreVM<M>` | state, record, program | RISC-V VM parameterized on execution mode |
| `ExecutionRecord` | cpu_events, add_events, mul_events, precompile_events, public_values | Shard execution trace |
| `ExecutionState` | pc, clk, memory, page_protections, input_stream | Live runtime state |
| `Opcode` | u8 (ADD=0..UNIMP=52) | SP1 custom instruction encoding |
| `SyscallCode` | u32 | System call identifier |
| `Instruction` | opcode, op_a, op_b, op_c, imm_b, imm_c | SP1 custom-encoded instruction |
| `ForkState` | global_clk, clk, pc, memory_diff | Unconstrained block rollback snapshot |

## Dependencies

- Require to reference arch/dependency.md for full dependency details

## Relevant Flows

- Require to reference core-flows/ for flows involving this module

## Module-Specific Pitfalls

[Pitfall] Register X0 must never be written non-zero — `rw()` forces value=0 for X0; violating this breaks RISC-V compliance

[Pitfall] `CLK_INC` is 8 per cycle; syscalls add 256 — clock ordering is critical for memory access timestamp correctness

[Pitfall] `HALT_PC` is 1 (not 0) — it is intentionally not 4-aligned to be an invalid instruction address

[Pitfall] Unconstrained blocks must be properly exited — `EndInUnconstrained` error if program halts inside unconstrained mode

[Pitfall] `InvalidShardingState` when running in non-sharding mode but encountering shard boundaries — ensure correct executor configuration
