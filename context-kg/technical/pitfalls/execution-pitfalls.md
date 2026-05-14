---
name: "execution-pitfalls"
description: "Known pitfalls and warnings related to RISC-V execution"
---
# Execution Pitfalls

[Pitfall] **Register X0 write**: Register X0 must never be written with a non-zero value — `rw()` forces value=0 when register == X0. Trigger: RISC-V instruction targeting X0. Correct approach: this is handled automatically; but custom instructions must respect this. Affected module: `sp1-core-executor`

[Pitfall] **Clock increment**: `CLK_INC` is 8 per instruction cycle, and syscalls add 256 additionally. Trigger: assuming uniform clock increments. Correct approach: account for syscall clock bump in timestamp calculations. Affected module: `sp1-core-executor`

[Pitfall] **HALT_PC sentinel**: `HALT_PC` is 1 (not 0, not 4-aligned) — intentionally an invalid instruction address to signal halt. Trigger: checking `pc == 0` for halt detection. Correct approach: check `pc == HALT_PC` (1). Affected module: `sp1-core-executor`

[Pitfall] **Unconstrained mode exit**: Programs must properly exit unconstrained blocks — `EndInUnconstrained` error if program halts inside. Trigger: using `ENTER_UNCONSTRAINED` syscall without matching `EXIT_UNCONSTRAINED`. Correct approach: always pair enter/exit calls. Affected module: `sp1-core-executor`

[Pitfall] **Memory consumption**: `TooMuchMemory` error if SP1 program allocates excessive memory. Trigger: large data structures or unbounded allocation in guest code. Correct approach: optimize guest memory usage; use streaming patterns. Affected module: `sp1-core-executor`

[Pitfall] **Untrusted program alignment**: In UserMode, untrusted program instructions must be 4-byte aligned — `InvalidMemoryAccessUntrustedProgram` error otherwise. Trigger: corrupted or misaligned code in user programs. Affected module: `sp1-core-executor`

[Pitfall] **Sharding state mismatch**: `InvalidShardingState` when running executor in non-sharding mode but encountering shard boundaries. Trigger: incorrect executor configuration. Correct approach: ensure executor mode matches intended operation. Affected module: `sp1-core-executor`

[Warning] **Context lifetime**: `ExecutionContext` has a TODO to remove the lifetime and change stdout/stderr to accept channels. Source: `core/executor/src/context.rs`. Affected module: `sp1-core-executor`

[Warning] **Deprecated record method**: `ExecutionRecord` has a deprecated method marked TODO for removal. Source: `core/executor/src/record.rs`. Affected module: `sp1-core-executor`
