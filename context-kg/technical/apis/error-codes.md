---
name: "error-codes"
description: "Error types across SDK, prover, executor, verifier, and network modules"
---
# Error Codes

## SDK Verification Errors (`SP1VerificationError`)

| Variant | Description |
|---------|-------------|
| `InvalidPublicValues` | Public values hash mismatch (neither SHA2 nor Blake3 matches) |
| `VersionMismatch(String)` | `sp1_version` in proof does not match `SP1_CIRCUIT_VERSION` |
| `Core(anyhow::Error)` | Core machine shard verification failure |
| `Recursion(anyhow::Error)` | Recursion proof verification failure |
| `Plonk(anyhow::Error)` | PLONK BN254 verification failure |
| `Groth16(anyhow::Error)` | Groth16 BN254 verification failure |
| `Other(anyhow::Error)` | Unexpected verification error |
| `UnexpectedExitCode(u32)` | Exit code does not match expected status code |

## Executor Errors (`ExecutionError`)

| Variant | Description |
|---------|-------------|
| `InvalidMemoryAccess(Opcode, u64)` | Invalid memory access for given opcode and address |
| `InvalidMemoryAccessUntrustedProgram(u64)` | Untrusted program address not aligned to 4 bytes |
| `UnsupportedSyscall(u32)` | Unimplemented syscall code |
| `Breakpoint()` | Breakpoint instruction encountered |
| `ExceededCycleLimit(u64)` | Execution exceeded configured cycle limit |
| `InvalidSyscallUsage(u64)` | Syscall called in unconstrained mode |
| `Unimplemented()` | UNIMP opcode encountered |
| `EndInUnconstrained()` | Program ended inside unconstrained block |
| `UnconstrainedCycleLimitExceeded(u64)` | Unconstrained block cycle limit exceeded |
| `UnexpectedExitCode(u32)` | Program exited with unexpected code |
| `InstructionNotFound()` | Instruction not found (page protect off) |
| `InvalidShardingState()` | Shard boundary in non-sharding mode |
| `UnhandledTrap(TrapError)` | Trap without valid handler |
| `TooMuchMemory()` | SP1 program consumes too much memory |
| `Other(String)` | Generic error |

## Executor Trap Errors (`TrapError`)

| Variant | Description |
|---------|-------------|
| `PagePermissionViolation(u64)` | Page permission check failed |

## Verifier Errors (`verifier::Error`)

| Variant | Description |
|---------|-------------|
| `InvalidWitness` | Invalid witness data |
| `InvalidXLength` | Invalid x coordinate length |
| `InvalidData` | Invalid input data |
| `InvalidPoint` | Invalid point in subgroup check |
| `FailedToGetFrFromRandomBytes` | Fr conversion failure |
| `Field(FieldError)` | BN254 field operation error |
| `Group(GroupError)` | BN254 group operation error |
| `Curve(CurveError)` | BN254 curve operation error |
| `InvalidProgramVkeyHash` | Program vkey hash mismatch |

## Network Errors (`network::Error`)

| Variant | Description |
|---------|-------------|
| `SimulationFailed` | Program execution simulation failed |
| `RequestUnexecutable { request_id }` | Proof request cannot be executed |
| `RequestUnfulfillable { request_id }` | Proof request cannot be fulfilled |
| `RequestTimedOut { request_id }` | Proof request timed out |
| `RequestAuctionTimedOut { request_id }` | Auction phase timed out (no prover bid) |
| `RpcError(Status)` | gRPC transport error |
| `Other(anyhow::Error)` | Unknown network error |

## Worker Task Errors (`TaskError`)

| Variant | Retry Behavior | Description |
|---------|---------------|-------------|
| `Retryable(anyhow::Error)` | Auto-retry | Network/transport errors (reqwest, tonic Internal/Unavailable/Unknown/Cancelled/DeadlineExceeded/ResourceExhausted/Aborted/DataLoss) |
| `Fatal(anyhow::Error)` | No retry | Logic errors, invalid input, permanent failures |
| `Execution(ExecutionError)` | No retry | VM execution failure |
